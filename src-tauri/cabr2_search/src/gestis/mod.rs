pub mod types;
pub mod xml_parser;

use cabr2_types::{Data, Source, SubstanceData};
use reqwest::blocking::{Client, Response};
use types::GestisResponse;

use self::types::GestisResponse;
use crate::{
  error::{Result, SearchError},
  types::{Provider, SearchArguments, SearchResponse, SearchType},
};

const BASE_URL: &str = "https://gestis-api.dguv.de/api";
const SEARCH_SUGGESTIONS: &str = "search_suggestions";
const SEARCH: &str = "search";
const ARTICLE: &str = "article";

// TODO: add runtime for async requests
pub struct Gestis {
  agent: Client,
}

impl Gestis {
  pub fn new(agent: Client) -> Gestis {
    Gestis { agent }
  }

  pub fn get_article(&self, identifier: String) -> Result<(GestisResponse, String)> {
    let url = format!("{}/{}/de/{}", BASE_URL, ARTICLE, identifier);
    let res = self.make_request(&url)?;

    Ok((res.json()?, url))
  }

  fn make_request(&self, url: &str) -> Result<Response> {
    match self
      .agent
      .get(url)
      // don't ask, just leave it
      // https://gestis.dguv.de/search -> webpack:///./src/api.ts?
      .bearer_auth("dddiiasjhduuvnnasdkkwUUSHhjaPPKMasd")
      .send()
    {
      Ok(response) => {
        let code = response.status();
        log::debug!(
          "{} {} - {}",
          code.as_u16(),
          code.canonical_reason().unwrap_or_default(),
          &url
        );
        Ok(response)
      }
      Err(err) => {
        log::error!("error when requesting url: {} -> {:?}", &url, err);
        if let Some(code) = err.status() {
          return match code.as_u16() {
            429 => Err(SearchError::RateLimit),
            _ => Err(SearchError::RequestError(code.as_u16())),
          };
        }
        Err(SearchError::Logged)
      }
    }
  }
}

impl Provider for Gestis {
  fn get_name(&self) -> String {
    "Gestis".into()
  }

  fn get_quick_search_suggestions(&self, search_type: SearchType, pattern: String) -> Result<Vec<String>> {
    let url = format!(
      "{}/{}/de?{}={}",
      BASE_URL,
      SEARCH_SUGGESTIONS,
      search_type.as_str(),
      pattern
    );
    let res = self.make_request(&url)?;

    Ok(res.json()?)
  }

  fn get_search_results(&self, arguments: SearchArguments) -> Result<Vec<SearchResponse>> {
    let args: Vec<String> = arguments
      .arguments
      .into_iter()
      .map(|a| format!("{}={}", a.search_type.as_str(), a.pattern))
      .collect();

    let url = format!(
      "{}/{}/de?{}&exact={}",
      BASE_URL,
      SEARCH,
      args.join("&"),
      arguments.exact,
    );
    let res = self.make_request(&url)?;

    Ok(res.json()?)
  }

  fn get_substance_data(&self, identifier: String) -> Result<cabr2_types::SubstanceData> {
    let (json, url) = self.get_article(identifier)?;

    let data = xml_parser::parse_response(&json)?;

    let res_data = SubstanceData {
      name: Data::new(json.name.clone()),
      alternative_names: json.aliases.into_iter().map(|a| a.name).collect(),
      cas: Data::new(data.cas),
      molecular_formula: Data::new(data.molecular_formula),
      molar_mass: Data::new(data.molar_mass),
      melting_point: Data::new(data.melting_point),
      boiling_point: Data::new(data.boiling_point),
      water_hazard_class: Data::new(data.water_hazard_class),
      lethal_dose: Data::new(data.lethal_dose),
      signal_word: Data::new(data.signal_word),
      mak: Data::new(data.mak),
      amount: None,
      h_phrases: Data::new(match data.h_phrases {
        Some(inner) => inner,
        None => Vec::new(),
      }),
      p_phrases: Data::new(match data.p_phrases {
        Some(inner) => inner,
        None => Vec::new(),
      }),
      symbols: Data::new(match data.symbols {
        Some(inner) => inner,
        None => Vec::new(),
      }),
      source: Source {
        provider: "gestis".into(),
        url,
        last_updated: chrono::Utc::now(),
      },

      checked: false,
    };

    Ok(res_data)
  }
}

impl SearchType {
  /// Returns the search type as the string that is used in the query parameters
  pub fn as_str(&self) -> &'static str {
    match self {
      SearchType::ChemicalName => "stoffname",
      SearchType::ChemicalFormula => "summenformel",
      SearchType::Numbers => "nummern",
      SearchType::FullText => "volltextsuche",
    }
  }
}

#[cfg(test)]
mod tests {
  use lazy_static::lazy_static;
  #[cfg(feature = "wasm")]
  use reqwest::blocking::ClientBuilder;
  #[cfg(not(feature = "wasm"))]
  use reqwest::ClientBuilder;

  use super::*;

  lazy_static! {
    static ref GESTIS: Gestis = Gestis::new(ClientBuilder::new().user_agent("cabr2/testing").build().unwrap());
  }

  #[test]
  fn test_suggestions_chemical_name() {
    assert_eq!(
      GESTIS
        .get_quick_search_suggestions(SearchType::ChemicalName, "cobaltnit".into())
        .unwrap(),
      vec!["cobaltnitrat"]
    );
  }

  #[test]
  fn test_suggestions_chemical_formula() {
    assert_eq!(
      GESTIS
        .get_quick_search_suggestions(SearchType::ChemicalFormula, "h2o".into())
        .unwrap(),
      vec!["h2o", "h2o2", "h2o2sr", "h2o2zn", "h2o3s", "h2o3se", "h2o4s", "h2o4se", "h2o4w", "h2o7s2"]
    );
  }

  #[test]
  fn test_suggestions_numbers() {
    assert_eq!(
      GESTIS
        .get_quick_search_suggestions(SearchType::Numbers, "5340".into())
        .unwrap(),
      vec!["5340", "53404-28-7", "53408-94-9"]
    );
  }
}
