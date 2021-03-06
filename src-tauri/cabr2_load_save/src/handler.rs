use std::{
  collections::HashMap,
  path::PathBuf,
  sync::{Arc, Mutex},
};

use lazy_static::lazy_static;

use super::{
  error::{LoadSaveError, Result},
  types::{CaBr2Document, DocumentTypes, Loader, Saver},
};

lazy_static! {
  pub static ref REGISTERED_LOADERS: Arc<Mutex<HashMap<&'static str, Box<dyn Loader + Send + Sync>>>> =
    Arc::new(Mutex::new(HashMap::new()));
  pub static ref REGISTERED_SAVERS: Arc<Mutex<HashMap<&'static str, Box<dyn Saver + Send + Sync>>>> =
    Arc::new(Mutex::new(HashMap::new()));
}

pub fn save_document(file_type: String, filename: PathBuf, document: CaBr2Document) -> Result<()> {
  log::debug!("type: {}", file_type);
  log::debug!("filename: {:?}", filename);
  log::trace!("doc: {:#?}", document);

  let mut filename = filename;
  let mut filename_changed = false;
  if let Some(ext) = filename.extension() {
    if ext.to_str().unwrap() != file_type {
      let mut name = filename.file_name().unwrap().to_owned();
      name.push(".");
      name.push(&file_type);
      filename = filename.with_file_name(&name);
      filename_changed = true;
    }
  } else {
    filename.set_extension(&file_type);
    filename_changed = true;
  }

  if filename_changed {
    log::debug!("filename changed: {:?}", filename);
    if filename.exists() {
      return Err(LoadSaveError::FileExists(filename.to_string_lossy().into()));
    }
  }

  if let Some(saver) = REGISTERED_SAVERS.lock().unwrap().get(file_type.as_str()) {
    return saver.save_document(filename, document);
  }

  Err(LoadSaveError::UnknownFileType)
}

pub fn load_document(filename: PathBuf) -> Result<CaBr2Document> {
  log::debug!("filename: {:?}", filename);

  if let Some(extension) = filename.extension() {
    let extension = extension.to_str().unwrap();
    if let Some(loader) = REGISTERED_LOADERS.lock().unwrap().get(extension) {
      return loader.load_document(filename);
    }
  }

  Err(LoadSaveError::UnknownFileType)
}

pub fn get_available_document_types() -> Result<DocumentTypes> {
  Ok(DocumentTypes {
    load: REGISTERED_LOADERS
      .lock()
      .expect("couldn't get lock for REGISTERED_LOADERS")
      .keys()
      .map(|s| s.to_string())
      .collect(),
    save: REGISTERED_SAVERS
      .lock()
      .expect("couldn't get lock for REGISTERED_SAVERS")
      .keys()
      .map(|s| s.to_string())
      .collect(),
  })
}