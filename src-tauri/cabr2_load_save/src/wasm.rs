use cabr2_types::ProviderMapping;

pub use crate::types::{CaBr2Document, DocumentTypes};
use crate::{
  error::Result,
  handler::{self, init_handlers},
};

pub fn init(provider_mapping: ProviderMapping) {
  init_handlers(provider_mapping)
}

pub fn save_document(file_type: String, document: CaBr2Document) -> Result<Vec<u8>> {
  log::debug!("type: {}", file_type);
  log::trace!("doc: {:#?}", document);

  handler::save_document(file_type.as_str(), document)
}

pub fn load_document(file_type: String, doc: Vec<u8>) -> Result<CaBr2Document> {
  log::debug!("filename: {:?}", file_type);

  handler::load_document(&file_type, doc)
}

pub fn get_available_document_types() -> Result<DocumentTypes> {
  handler::get_available_document_types()
}
