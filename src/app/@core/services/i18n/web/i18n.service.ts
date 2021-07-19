import { Observable, of } from 'rxjs';
import { Injectable } from '@angular/core';

import Logger from 'src/app/@core/utils/logger';

import { II18nService, LocalizedStrings, LocalizedStringsHeader } from '../i18n.interface';

const logger = new Logger('i18n-service-web');

@Injectable()
export class I18nWebService implements II18nService {
  getAvailableLanguages(): Observable<LocalizedStringsHeader[]> {
    return of([
      { name: 'Boarisch', locale: 'de_by' },
      { name: 'Deutsch', locale: 'de_de' },
      { name: 'English', locale: 'en_us' },
      { name: 'Hrvatski', locale: 'hr_hr' },
      { name: 'Русский', locale: 'ru_ru' },
    ]);
  }

  getLocalizedStrings(language: string): Observable<LocalizedStrings> {
    return new Observable((sub) => {
      import(`translations/${language}.json`)
        .then((lang) => sub.next(lang.strings))
        .catch((err) => {
          sub.error(err);
          logger.error(err);
        });
    });
  }

}
