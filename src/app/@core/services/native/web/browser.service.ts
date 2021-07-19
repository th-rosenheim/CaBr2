import { Observable, of } from 'rxjs';
import { Injectable } from '@angular/core';

import { init } from 'cabr2_wasm';
import Logger from 'src/app/@core/utils/logger';

import { INativeService } from '../native.interface';

const logger = new Logger('service.browser');

init();

const logCall = (name: string) => logger.error('called inaccessible tauri service: [', name, ']');

@Injectable()
export class BrowserService implements INativeService {
  openUrl(url: string, _?: string): Promise<void> {
    window.open(url, '_blank');
    return Promise.resolve();
  }

  open(_?: any): Observable<string | string[]> {
    const inputField = document.createElement('input');
    inputField.type = 'file';
    inputField.style.display = 'none';
    document.body.append(inputField);

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
