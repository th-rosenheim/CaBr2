import { Observable, of } from 'rxjs';
import { Injectable } from '@angular/core';

import { INativeService } from '../native.interface';
import Logger from '../../../utils/logger';

const logger = new Logger('service.browser');

const logCall = (name: string) => logger.error('called inaccessible tauri service: [', name, ']');

@Injectable()
export class BrowserService implements INativeService {
  openUrl(url: string, _?: string): Promise<void> {
    window.open(url, '_blank');
    return Promise.resolve();
  }

  open(_?: any): Observable<string | string[]> {
    const inputField = new HTMLInputElement();
    inputField.style.display = 'none';
    document.append(inputField);

    return new Observable((sub) => {
      inputField.addEventListener(
        'change',
        (event: any) => {
          sub.next(event.target.files[0]);
        },
        { once: true },
      );
      inputField.click();
    });
  }

  save(options?: any): Observable<string | string[]> {
    return of(options.filter ?? 'cb2');
  }

  promisified<T>(_cmd: string, _args?: any): Observable<T> {
    logCall('promisified');
    return new Observable();
  }
}
