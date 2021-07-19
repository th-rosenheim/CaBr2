import { Injectable } from '@angular/core';
import { Observable } from 'rxjs';

import { CaBr2Document, DocumentTypes } from '../loadSave.model';
import { get_available_document_types, load_document } from 'cabr2_wasm';
import { ILoadSaveService } from '../loadSave.interface';

/* eslint-disable @typescript-eslint/no-unused-vars */
// TODO use vars and remove this lines

@Injectable()
export class LoadSaveService implements ILoadSaveService {
  saveDocument(fileType: string, filename: string, document: CaBr2Document): Observable<string> {
    throw new Error('Method not implemented.');
  }

  loadDocument(filename: string): Observable<CaBr2Document> {
    const file = filename as unknown as File;
    const fileType = getFileType(file);

    const reader = new FileReader();

    switch (fileType) {
      case 'cb2':
        return new Observable((sub) => {
          reader.onload = () => sub.next(JSON.parse(reader.result as string));
          reader.readAsText(file);
        });

      default:
        return new Observable((sub) => {
          reader.onload = () => {
            const res = reader.result as ArrayBuffer;
            const contents = load_document(fileType, new Uint8Array(res));
            sub.next(JSON.parse(contents));
          };
          reader.readAsArrayBuffer(file);
        });
    }
  }

  getAvailableDocumentTypes(): Observable<DocumentTypes> {
    return new Observable((sub) => {
      sub.next(JSON.parse(get_available_document_types()));
    });
  }

}

const getFileType = (file: File): string => {
  const fileTypeSplit = file.name.split('.');
  return fileTypeSplit[fileTypeSplit.length - 1];
};

interface SaveDocumentResponse {
  downloadUrl: string;
}
