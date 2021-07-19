import { Component, OnInit, QueryList, ViewChildren } from '@angular/core';
import { first } from 'rxjs/operators';
import { MatDialog } from '@angular/material/dialog';
import { MatTableDataSource } from '@angular/material/table';

import { CdkDragDrop, moveItemInArray } from '@angular/cdk/drag-drop';
import { Data, Source, SubstanceData } from '../@core/models/substances.model';
import { Provider, ProviderMapping, SearchArgument } from '../@core/services/provider/provider.model';
import { AlertService } from '../@core/services/alertsnackbar/altersnackbar.service';
import { GlobalModel } from '../@core/models/global.model';
import { INativeService } from '../@core/services/native/native.interface';
import { IProviderService } from '../@core/services/provider/provider.interface';
import { LocalizedStrings } from '../@core/services/i18n/i18n.interface';
import Logger from '../@core/utils/logger';

import { EditSubstanceDataComponent } from '../edit-substance-data/edit-substance-data.component';
import { SearchDialogComponent } from './search-dialog/search-dialog.component';
import { SelectedSearchComponent } from './selected-search/selected-search.component';

const logger = new Logger('search');

const GESTIS_URL_RE = new RegExp('https:\\/\\/gestis-api\\.dguv\\.de\\/api\\/article\\/(de|en)\\/(\\d{6})');

@Component({
  selector: 'app-search',
  templateUrl: './search.component.html',
  styleUrls: ['./search.component.scss'],
})
export class SearchComponent implements OnInit {
  @ViewChildren(SelectedSearchComponent)
  selectedSearchComponents!: QueryList<SelectedSearchComponent>;

  res: SearchArgument[] = [];

  addButtonHover = false;

  strings!: LocalizedStrings;

  providerMapping!: ProviderMapping;

  providers: Provider[] = [];

  index!: number;

  substanceData: SubstanceData[] = [];

  displayedColumns = ['edited', 'name', 'cas', 'source', 'actions'];

  dataSource!: MatTableDataSource<SubstanceData>;

  constructor(
    private providerService: IProviderService,
    private nativeService: INativeService,
    private alertService: AlertService,
    private dialog: MatDialog,
    public globals: GlobalModel,
  ) {
    this.globals.localizedStringsObservable.subscribe((strings) => (this.strings = strings));
  }

  ngOnInit(): void {
    this.globals.substanceDataObservable.subscribe((data) => {
      this.dataSource = new MatTableDataSource(data);
    });

    this.providerService.providerMappingsObservable.subscribe((providerMap) => {
      logger.debug(providerMap);
      this.providerMapping = providerMap;
      this.providers = Array.from(providerMap.values()).filter((provider) => provider.identifier !== 'custom');
    });
  }

  openDialog(index: number): void {
    const providerIdentifier = this.providers[index].identifier;
    const currentSearchComponent = this.selectedSearchComponents.find((_, i) => i === index);
    const searchArguments = currentSearchComponent?.getSearchArguments();

    const dialogRef = this.dialog.open(SearchDialogComponent, {
      data: {
        arguments: searchArguments,
        providerIdentifier,
      },
      panelClass: ['unselectable', 'undragable'],
    });

    dialogRef.afterClosed().subscribe((result) => {
      if (result) {
        this.providerService.substanceData(providerIdentifier, result.zvgNumber).subscribe(
          (value) => {
            const cas = this.modifiedOrOriginal(value.cas);
            if (
              cas &&
              this.globals.substanceDataSubject.getValue().some((s) => cas === this.modifiedOrOriginal(s.cas))
            ) {
              logger.warning('substance with same cas number already present:', cas);
              this.alertService.error(this.strings.error.substanceWithCASExist);
              return;
            }
            const data = [...this.globals.substanceDataSubject.getValue(), value];
            this.dataSource.connect().next(data);
            this.globals.substanceDataSubject.next(data);
          },
          (err) => {
            logger.error('could not get substance information:', err);
            this.alertService.error(this.strings.error.substanceLoadData);
          },
        );

        currentSearchComponent?.clear();
      }
    });
  }

  openEditDialog(origData: SubstanceData): void {
    origData.checked = true;
    this.dialog
      .open(EditSubstanceDataComponent, {
        data: origData,
        maxWidth: 1500,
        minWidth: 800,
        maxHeight: 900,
        minHeight: 300,
        autoFocus: false,
      })
      .afterClosed()
      .subscribe(
        (substanceData?: SubstanceData) => {
          // substanceData is only filled if editing was successful
          if (substanceData) {
            const newData = this.globals.substanceDataSubject.getValue();
            const index = newData.indexOf(origData);
            newData[index] = substanceData;
            this.globals.substanceDataSubject.next(newData);
          }
        },
        (err) => {
          logger.error('editing substance failed:', err);
          this.alertService.error(this.strings.error.editSubstance);
        },
      );
  }

  removeSubstance(event: MouseEvent, data: SubstanceData): void {
    event.preventDefault();
    event.stopPropagation();

    this.globals.substanceDataObservable.pipe(first()).subscribe((value) => {
      const index = value.indexOf(data);
      value.splice(index, 1);
      this.globals.substanceDataSubject.next(value);
      this.dataSource.connect().next(value);
    });
  }

  sourceButtonDisabled(source: Source): boolean {
    return source.provider === 'custom' || !this.providerMapping.has(source.provider);
  }

  openSource(event: MouseEvent, source: Source): void {
    event.preventDefault();
    event.stopPropagation();
    const matches = source.url.match(GESTIS_URL_RE);
    if (matches) {
      // TODO (#526) move the url to providers
      this.nativeService.openUrl(`https://gestis.dguv.de/data?name=${matches[2]}&lang=${matches[1]}`);
    }
  }

  addCustomSubstanceData(): void {
    const data = [
      ...this.globals.substanceDataSubject.getValue(),
      // create new custom SubstanceData
      new SubstanceData({ checked: true }),
    ];
    this.dataSource.connect().next(data);
    this.globals.substanceDataSubject.next(data);
  }

  getProviderName(source: Source): string {
    if (source.provider === 'custom') {
      return this.strings.search.customSubstance;
    } else {
      const provider = this.providerMapping.get(source.provider);
      return provider ? provider.name : `${source.provider} (${this.strings.search.unsupportedProviderInfo})`;
    }
  }

  drop(event: CdkDragDrop<string[]> | any) {
    moveItemInArray(this.dataSource.data, event.previousIndex, event.currentIndex);
    const data = this.dataSource.data.slice();
    this.dataSource.connect().next(data);
    this.globals.substanceDataSubject.next(data);
  }

  // TODO move to SubstanceData class
  private modifiedOrOriginal<T>(obj: Data<T>): T {
    return obj.modifiedData ?? obj.originalData;
  }
}
