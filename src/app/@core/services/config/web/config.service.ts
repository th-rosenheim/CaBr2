import { Observable, of } from 'rxjs';
import { Injectable } from '@angular/core';

import { ConfigModel } from 'src/app/@core/models/config.model';
import { get_program_version } from 'cabr2_wasm';
import { GHSSymbols } from 'src/app/@core/models/global.model';

import { IConfigService } from '../config.interface';

/* eslint-disable @typescript-eslint/no-unused-vars */
// TODO use vars and remove this lines
@Injectable()
export class ConfigWebService implements IConfigService {
  getProgramVersion(): Observable<string> {
    return of(get_program_version());
  }

  getConfig(): Observable<ConfigModel> {
    return new Observable((sub) => {
      const config = localStorage.getItem('config');
      if (config) {
        sub.next(new ConfigModel(JSON.parse(config).global));
      } else {
        sub.next(new ConfigModel());
      }
    });
  }

  saveConfig(config: ConfigModel): Observable<void> {
    return new Observable((sub) => {
      localStorage.setItem('config', JSON.stringify(config));
      sub.next();
    });
  }

  getHazardSymbols(): Observable<GHSSymbols> {
    return new Observable((sub) => {
      const symbols = new Map<string, string>([
        ['ghs01', 'ghs_symbols/ghs01.png'],
        ['ghs02', 'ghs_symbols/ghs02.png'],
        ['ghs03', 'ghs_symbols/ghs03.png'],
        ['ghs04', 'ghs_symbols/ghs04.png'],
        ['ghs05', 'ghs_symbols/ghs05.png'],
        ['ghs06', 'ghs_symbols/ghs06.png'],
        ['ghs07', 'ghs_symbols/ghs07.png'],
        ['ghs08', 'ghs_symbols/ghs08.png'],
        ['ghs09', 'ghs_symbols/ghs09.png'],
      ]);

      sub.next(symbols);
    });
  }

  getPromptHtml(name: string): Observable<string> {
    throw new Error('Method not implemented.');
  }

}
