use std::{convert::Infallible, path::PathBuf, time::Duration};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio::fs;
use uuid::Uuid;
use warp::{hyper::StatusCode, Reply};

use cabr2_types::ProviderMapping;

use crate::{handler, types::CaBr2Document};

pub const DOWNLOAD_FOLDER: &str = "/tmp/cabr2_server/created";
pub const CACHE_FOLDER: &str = "/tmp/cabr2_server/cache";

pub struct LoadSave;

impl LoadSave {
  pub fn new(provider_mapping: ProviderMapping) -> Self {
    handler::init_handlers(provider_mapping);
    LoadSave
  }
}

pub async fn handle_available_document_types() -> Result<impl Reply, Infallible> {
  match handler::get_available_document_types() {
    Ok(res) => Ok(warp::reply::with_status(warp::reply::json(&res), StatusCode::OK)),
    Err(err) => Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(err.to_string())),
      StatusCode::BAD_REQUEST,
    )),
  }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoadDocumentBody {
  file_type: String,
  document: String,
}

pub async fn handle_load_document(body: LoadDocumentBody) -> Result<impl Reply, Infallible> {
  lazy_static! {
    static ref TMP: PathBuf = PathBuf::from(CACHE_FOLDER);
  }

  let mut path;
  loop {
    path = TMP.clone();
    path.push(&serde_json::to_string(&Uuid::new_v4()).unwrap().replace('"', ""));
    let path = path.with_extension(&body.file_type);

    if !path.exists() {
      break;
    }
  }

  let contents = fs::read(&path).await;

  if contents.is_err() {
    return Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(contents.err().unwrap().to_string())),
      StatusCode::INTERNAL_SERVER_ERROR,
    ));
  }

  let reply = match fs::write(&path, body.document).await {
    Ok(_) => match handler::load_document(
      path.extension().unwrap_or_default().to_str().unwrap_or_default(),
      contents.unwrap(),
    ) {
      Ok(res) => Ok(warp::reply::with_status(warp::reply::json(&res), StatusCode::OK)),
      Err(err) => Ok(warp::reply::with_status(
        warp::reply::json(&Value::String(err.to_string())),
        StatusCode::BAD_REQUEST,
      )),
    },
    Err(err) => Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(err.to_string())),
      StatusCode::INTERNAL_SERVER_ERROR,
    )),
  };

  fs::remove_file(&path)
    .await
    .unwrap_or_else(|err| log::error!("removing file '{:?}' failed: {}", path, err.to_string()));

  reply
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SaveDocumentBody {
  file_type: String,
  document: CaBr2Document,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct SaveDocumentResponse {
  download_url: String,
}

pub async fn handle_save_document(body: SaveDocumentBody) -> Result<impl Reply, Infallible> {
  lazy_static! {
    static ref TMP: PathBuf = PathBuf::from(DOWNLOAD_FOLDER);
  }

  let mut path;
  let mut uuid_str;
  loop {
    path = TMP.clone();
    uuid_str = serde_json::to_string(&Uuid::new_v4()).unwrap().replace('"', "");
    path.push(&uuid_str);
    let path = path.with_extension(&body.file_type);

    if !path.exists() {
      break;
    }
  }

  let contents = match handler::save_document(body.file_type.as_str(), body.document) {
    Ok(contents) => contents,
    Err(err) => {
      return Ok(warp::reply::with_status(
        warp::reply::json(&Value::String(err.to_string())),
        StatusCode::INTERNAL_SERVER_ERROR,
      ))
    }
  };

  let res = fs::write(&path, contents).await;

  match res {
    Ok(_) => Ok(warp::reply::with_status(
      warp::reply::json(&SaveDocumentResponse {
        #[cfg(not(debug_assertions))]
        download_url: format!("https://api.cabr2.de/download/{}.{}", uuid_str, body.file_type),
        #[cfg(debug_assertions)]
        download_url: format!("http://localhost:3030/download/{}.{}", uuid_str, body.file_type),
      }),
      StatusCode::CREATED,
    )),
    Err(err) => Ok(warp::reply::with_status(
      warp::reply::json(&Value::String(err.to_string())),
      StatusCode::BAD_REQUEST,
    )),
  }
}

pub async fn cleanup_thread() {
  // I'm sorry for the number of match statements.
  // This logic should ignore every error and continue working on the next file
  // but we want to know what went wrong.

  loop {
    let mut res = match fs::read_dir(DOWNLOAD_FOLDER).await {
      Ok(iter) => iter,
      Err(err) => {
        log::error!("{:?}", err);
        break;
      }
    };

    // go through all generated files and remove all that are older than 24 hours
    loop {
      let path = res.next_entry().await.unwrap_or_default();
      match path {
        Some(path) => match path.metadata().await {
          Ok(data) => match data.modified() {
            Ok(c_time) => match c_time.elapsed() {
              Ok(dur) => {
                if dur.as_secs() > 86400 {
                  match fs::remove_file(path.path()).await {
                    Ok(_) => log::debug!("deleted file: {:?}", path.path()),
                    Err(e) => log::error!("{:?}", e),
                  };
                }
              }
              Err(e) => log::error!("{:?}", e),
            },
            Err(e) => log::error!("{:?}", e),
          },
          Err(e) => log::error!("{:?}", e),
        },
        None => break,
      }
    }

    // check every 15 minutes
    tokio::time::sleep(Duration::from_secs(900)).await;
  }
}
