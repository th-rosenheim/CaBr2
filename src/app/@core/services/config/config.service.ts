import { DomSanitizer } from '@angular/platform-browser';
import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { ConfigModel } from '../../models/config.model';
import { GHSSymbols } from '../../models/global.model';

import { IConfigService } from './config.interface';
import { INativeService } from '../native/native.interface';
import { map } from 'rxjs/operators';

@Injectable()
export class ConfigService implements IConfigService {
  constructor(private nativeService: INativeService, private sanitizer: DomSanitizer) { }

  getProgramVersion(): Observable<string> {
    return this.nativeService.promisified('plugin:cabr2_config|get_program_version');
  }

  getConfig(): Observable<ConfigModel> {
    return this.nativeService.promisified('plugin:cabr2_config|get_config');
  }

  saveConfig(config: ConfigModel): Observable<void> {
    return this.nativeService.promisified('plugin:cabr2_config|save_config', { config });
  }

  getHazardSymbols(): Observable<GHSSymbols> {
    return this.nativeService.promisified<GHSSymbols>('plugin:cabr2_config|get_hazard_symbols')
      .pipe(map((symbols) => {
        const newSymbols = new Map();

        // symbols is just an object
        new Map(Object.entries(symbols))
          .forEach((s, k) => newSymbols.set(k, this.sanitizer.bypassSecurityTrustResourceUrl(s)));

        return newSymbols;
      }));
  }

  getPromptHtml(name: string): Observable<string> {
    return this.nativeService.promisified('plugin:cabr2_config|get_prompt_html', { name });
  }
}
