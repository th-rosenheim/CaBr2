use std::collections::HashMap;

use cabr2_types::SubstanceData;

use crate::{
  error::Result,
  handler,
  types::{ProviderInfo, SearchArguments, SearchResponse, SearchType},
};

pub fn init() {
  handler::init_providers();
}

pub fn get_provider_mapping() -> HashMap<String, String> {
  handler::get_provider_mapping()
}

pub fn get_available_providers() -> Result<Vec<ProviderInfo>> {
  handler::get_available_providers()
}

pub fn search_suggestions(provider: String, pattern: String, search_type: SearchType) -> Result<Vec<String>> {
  handler::get_quick_search_suggestions(provider, search_type, pattern)
}

pub fn search(provider: String, arguments: SearchArguments) -> Result<Vec<SearchResponse>> {
  handler::get_search_results(provider, arguments)
}

pub fn get_substance_data(provider: String, identifier: String) -> Result<SubstanceData> {
  handler::get_substance_data(provider, identifier)
}
