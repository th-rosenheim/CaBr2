use std::convert::Infallible;

use serde::Deserialize;
use serde_json::Value;
use warp::{hyper::StatusCode, Reply};

use crate::handler;

pub async fn handle_hazard_symbols() -> Result<impl Reply, Infallible> {
  match handler::get_hazard_symbols() {
    Ok(res) => Ok(warp::reply::with_status(warp::reply::json(&res), StatusCode::OK)),
    Err(err) => Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(err.to_string())),
      StatusCode::BAD_REQUEST,
    )),
  }
}

pub async fn handle_program_version() -> Result<impl Reply, Infallible> {
  Ok(warp::reply::with_status(
    warp::reply::json(&env!("CARGO_PKG_VERSION")),
    StatusCode::OK,
  ))
}

pub async fn handle_prompt_html(body: PromptHtmlBody) -> Result<impl Reply, Infallible> {
  match handler::get_prompt_html(body.name) {
    Ok(res) => Ok(warp::reply::with_status(warp::reply::json(&res), StatusCode::OK)),
    Err(err) => Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(err.to_string())),
      StatusCode::BAD_REQUEST,
    )),
  }
}

pub async fn handle_available_languages() -> Result<impl Reply, Infallible> {
  match handler::get_available_languages() {
    Ok(res) => Ok(warp::reply::with_status(warp::reply::json(&res), StatusCode::OK)),
    Err(err) => Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(err.to_string())),
      StatusCode::BAD_REQUEST,
    )),
  }
}

pub async fn handle_localized_strings(body: LocalizedStringsBody) -> Result<impl Reply, Infallible> {
  match handler::get_localized_strings(body.language) {
    Ok(res) => Ok(warp::reply::with_status(warp::reply::json(&res), StatusCode::OK)),
    Err(err) => Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(err.to_string())),
      StatusCode::BAD_REQUEST,
    )),
  }
}

// Body definitions

#[derive(Debug, Deserialize)]
pub struct PromptHtmlBody {
  name: String,
}

#[derive(Debug, Deserialize)]
pub struct LocalizedStringsBody {
  language: String,
}
