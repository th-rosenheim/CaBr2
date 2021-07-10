use std::{fs, path::PathBuf};

use tauri::{plugin::Plugin, Invoke, Params, Window};

use cabr2_types::ProviderMapping;

use crate::{
  error::{LoadSaveError, Result},
  handler::{self, init_handlers},
  types::{CaBr2Document, DocumentTypes},
};

#[tauri::command]
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

  let res = handler::save_document(file_type.as_str(), document)?;

  let res = fs::write(&filename, res);

  if res.is_err() {
    log::warn!("failed to write file '{:?}': {:?}", filename, res);
  }

  Ok(res?)
}

#[tauri::command]
pub fn load_document(filename: PathBuf) -> Result<CaBr2Document> {
  log::debug!("filename: {:?}", filename);

  let contents = fs::read(&filename);

  if contents.is_err() {
    log::warn!("failed to read file '{:?}': {:?}", filename, contents);
  }

  handler::load_document(
    filename.extension().unwrap_or_default().to_str().unwrap_or_default(),
    contents?,
  )
}

#[tauri::command]
pub fn get_available_document_types() -> Result<DocumentTypes> {
  handler::get_available_document_types()
}

pub struct LoadSave<M: Params> {
  invoke_handler: Box<dyn Fn(Invoke<M>) + Send + Sync>,
}

impl<M: Params> LoadSave<M> {
  pub fn new(provider_mapping: ProviderMapping) -> Self {
    init_handlers(provider_mapping);
    LoadSave {
      invoke_handler: Box::new(tauri::generate_handler![
        save_document,
        load_document,
        get_available_document_types,
      ]),
    }
  }
}

impl<M: Params> Plugin<M> for LoadSave<M> {
  fn name(&self) -> &'static str {
    "cabr2_load_save"
  }

  fn extend_api(&mut self, message: Invoke<M>) {
    (self.invoke_handler)(message)
  }

  fn created(&mut self, _: Window<M>) {
    log::trace!("plugin created");
  }
}
