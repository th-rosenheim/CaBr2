import { BehaviorSubject } from 'rxjs';
import { Injectable } from '@angular/core';

import { DomSanitizer, SafeResourceUrl } from '@angular/platform-browser';
import { getDefaultStrings, LocalizedStrings } from '../services/i18n/i18n.interface';
import { Header } from '../interfaces/Header';
import { SearchResult } from '../services/provider/provider.model';
import { SubstanceData } from './substances.model';

@Injectable()
export class GlobalModel {
  ghsSymbols: GHSSymbols = new Map();
  ghsSymbolKeys: string[] = [];

  localizedStringsSubject = new BehaviorSubject<LocalizedStrings>(getDefaultStrings());
  localizedStringsObservable = this.localizedStringsSubject.asObservable();

  headerSubject = new BehaviorSubject<Header>({
    assistant: '',
    documentTitle: '',
    labCourse: '',
    name: '',
    organisation: '',
    place: '',
    preparation: '',
  });
  headerObservable = this.headerSubject.asObservable();

  searchResultSubject = new BehaviorSubject<SearchResult[]>([]);
  searchResultObservable = this.searchResultSubject.asObservable();

  substanceDataSubject = new BehaviorSubject<SubstanceData[]>([]);
  substanceDataObservable = this.substanceDataSubject.asObservable();

  humanAndEnvironmentDangerSubject = new BehaviorSubject<string[]>([]);
  humanAndEnvironmentDangerObservable = this.humanAndEnvironmentDangerSubject.asObservable();

  rulesOfConductSubject = new BehaviorSubject<string[]>([]);
  rulesOfConductObservable = this.rulesOfConductSubject.asObservable();

  inCaseOfDangerSubject = new BehaviorSubject<string[]>([]);
  inCaseOfDangerObservable = this.inCaseOfDangerSubject.asObservable();

  disposalSubject = new BehaviorSubject<string[]>([]);
  disposalObservable = this.disposalSubject.asObservable();

  ghsSymbolsSubject = new BehaviorSubject<GHSSymbols>(new Map<string, string>());
  ghsSymbolsObservable = this.ghsSymbolsSubject.asObservable();

  constructor(private sanitizer: DomSanitizer) { }

  setGHSSymbols(newSymbols: GHSSymbols) {
    this.ghsSymbols = newSymbols;
    this.ghsSymbolKeys = Array.from(newSymbols.keys());

    this.ghsSymbolKeys.sort();
  }
}

export type GHSSymbols = Map<string, SafeResourceUrl>;
