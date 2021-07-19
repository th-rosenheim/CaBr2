use std::collections::HashMap;

use tauri::{async_runtime, plugin::Plugin, Invoke, Params, Window};

use cabr2_types::SubstanceData;

use crate::{
  error::Result,
  handler,
  types::{ProviderInfo, SearchArguments, SearchResponse, SearchType},
};

#[tauri::command]
pub async fn get_available_providers() -> Result<Vec<ProviderInfo>> {
  handler::get_available_providers().await
}

#[tauri::command]
pub async fn search_suggestions(provider: String, pattern: String, search_type: SearchType) -> Result<Vec<String>> {
  handler::get_quick_search_suggestions(provider, search_type, pattern).await
}

#[tauri::command]
pub async fn search(provider: String, arguments: SearchArguments) -> Result<Vec<SearchResponse>> {
  handler::get_search_results(provider, arguments).await
}

#[tauri::command]
pub async fn get_substance_data(provider: String, identifier: String) -> Result<SubstanceData> {
  handler::get_substance_data(provider, identifier).await
}

pub struct Search<M: Params> {
  invoke_handler: Box<dyn Fn(Invoke<M>) + Send + Sync>,
}

impl<M: Params> Search<M> {
  pub fn new() -> Self {
    // Must be finished when this function exits, because other init functions depend on the result of this.
    async_runtime::block_on(handler::init_providers());
    Search {
      invoke_handler: Box::new(tauri::generate_handler![
        get_available_providers,
        search_suggestions,
        search,
        get_substance_data,
      ]),
    }
  }

  pub fn get_provider_mapping(&self) -> HashMap<String, String> {
    handler::get_provider_mapping()
  }
}

impl<M: Params> Plugin<M> for Search<M> {
  fn name(&self) -> &'static str {
    "cabr2_search"
  }

  fn extend_api(&mut self, message: Invoke<M>) {
    (self.invoke_handler)(message)
  }

  fn created(&mut self, _: Window<M>) {
    log::trace!("plugin created");
  }
}
