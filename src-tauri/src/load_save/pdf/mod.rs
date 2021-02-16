use std::{
  fs::OpenOptions,
  io::{BufWriter, Read, Write},
  path::PathBuf,
  sync::{mpsc, Arc, Mutex},
  thread,
};

use lazy_static::lazy_static;
use wkhtmltopdf::{Orientation, PageSize, PdfApplication, Size};

use super::{
  error::{LoadSaveError, Result},
  types::{CaBr2Document, Saver},
};

pub struct PDF;

impl Saver for PDF {
  fn save_document(&self, filename: PathBuf, document: CaBr2Document) -> Result<()> {
    lazy_static! {
      static ref PDF_THREAD_CHANNEL: Arc<Mutex<(mpsc::SyncSender<(String, String)>, mpsc::Receiver<Result<Vec<u8>>>)>> =
        Arc::new(Mutex::new(init_pdf_application()));
    }

    let title = document.header.document_title.clone();
    match render_doc(document) {
      Err(e) => Err(LoadSaveError::RenderError(e.to_string())),
      Ok((page1, page2)) => {
        let channels = PDF_THREAD_CHANNEL.lock().unwrap();

          channels
            .0
            .send((htmls, title))
            .expect("sending data to pdf thread failed");

          let pdf: Vec<u8> = channels.1.recv().expect("receiving data from pdf thread failed")?;

          let file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(filename)?;
          let mut writer = BufWriter::new(file);
          writer.write_all(&pdf)?;

        Ok(())
      }
    }
  }
}

const HTML_TEST: &str = r#""#;

/// render_doc get CaBr2Document and return html (dummy at the moment)
fn render_doc(document: CaBr2Document) -> Result<(String, String)> {
  fn html(placeholder: &str) -> String {
    format!("<html><body>foo {}</body></html>", placeholder)
  }

  log::warn!("dummy values");

  Ok((html("1"), html("2")))
}

fn init_pdf_application() -> (mpsc::SyncSender<(String, String)>, mpsc::Receiver<Result<Vec<u8>>>) {
  let (tauri_tx, pdf_rx) = mpsc::sync_channel(0);
  let (pdf_tx, tauri_rx) = mpsc::sync_channel(0);

  /* #region  pdf thread */

  thread::spawn(move || {
    log::debug!("[pdf_thread]: initializing pdf application");
    let mut pdf_app = match PdfApplication::new() {
      Ok(app) => app,
      Err(e) => {
        log::error!("initialization of pdf application failed");
        pdf_tx
          .send(Err(LoadSaveError::PdfError(e)))
          .expect("pdf thread could not send data");
        return;
      }
    };

    loop {
      log::trace!("[pdf_thread]: waiting for html to convert");
      let (html, title) = pdf_rx.recv().expect("pdf thread could not receive data");
      log::trace!("[pdf_thread]: got html");

      // needed for rust to resolve types
      let title: String = title;

      let mut buf = Vec::new();

      let result = match pdf_app
        .builder()
        .page_size(PageSize::A4)
        .orientation(Orientation::Portrait)
        .margin(Size::Millimeters(50))
        .title(&title)
        .build_from_html(&html)
      {
        Ok(mut pdfout) => match pdfout.read_to_end(&mut buf) {
          Ok(_) => Ok(buf),
          Err(e) => Err(LoadSaveError::IOError(e)),
        },
        Err(e) => Err(LoadSaveError::PdfError(e)),
      };

      log::trace!("[pdf_thread]: sending result");
      pdf_tx.send(result).expect("pdf thread could not send data");
      log::trace!("[pdf_thread]: finished");
    }
  });

  /* #endregion */

  (tauri_tx, tauri_rx)
}
