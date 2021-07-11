use std::collections::HashMap;

use wasm_bindgen::prelude::*;

use cabr2_load_save::wasm::CaBr2Document;

pub type Result<T> = std::result::Result<T, JsValue>;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

/// must be run first
#[wasm_bindgen]
pub fn init() {
  #[cfg(feature = "debug_build")]
  console_error_panic_hook::set_once();

  cabr2_load_save::wasm::init(HashMap::new())
}

#[wasm_bindgen]
pub fn save_document(file_type: String, document: String) -> Result<Vec<u8>> {
  let document: CaBr2Document = serde_json::from_str(&document).unwrap();
  let res = cabr2_load_save::wasm::save_document(file_type, document).unwrap();
  Ok(res)
}

#[wasm_bindgen]
pub fn load_document(file_type: String, doc: Vec<u8>) -> Result<String> {
  let document = match cabr2_load_save::wasm::load_document(file_type, doc) {
    Ok(document) => document,
    Err(err) => return Err(JsValue::from(err.to_string())),
  };

  match serde_json::to_string(&document) {
    Ok(res) => Ok(res),
    Err(err) => Err(JsValue::from(err.to_string())),
  }
}

#[wasm_bindgen]
pub fn get_available_document_types() -> Result<String> {
  let types = cabr2_load_save::wasm::get_available_document_types().unwrap();
  Ok(serde_json::to_string(&types).unwrap())
}
