use chrono::{DateTime, Duration, Local};
use diesel::MysqlConnection;

use serde::{Deserialize, Serialize};
use std::ops::Add;

use crate::settings::action::get_setting;

use crate::error::request_error::RequestError;
use actix_web::http::HeaderMap;
use rust_embed::RustEmbed;
use std::fs::read;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(RustEmbed)]
#[folder = "$CARGO_MANIFEST_DIR/resources"]
pub struct Resources;

impl Resources {
    pub fn file_get(file: &str) -> Vec<u8> {
        let buf = Path::new("resources").join(file);
        if buf.exists() {
            return read(buf).unwrap();
        } else {
            return Resources::get(file).unwrap().data.to_vec();
        }
    }
    pub fn file_get_string(file: &str) -> String {
        let vec = Resources::file_get(file);
        return String::from_utf8(vec).unwrap();
    }
}

pub fn installed(conn: &MysqlConnection) -> Result<(), RequestError> {
    let installed: bool = bool::from_str(std::env::var("INSTALLED").unwrap().as_str()).unwrap();
    if installed {
        return Ok(());
    }
    let option = get_setting("INSTALLED", &conn)?;
    if option.is_none() {
        return Err(RequestError::UnInstalled);
    }
    std::env::set_var("INSTALLED", "true");
    return Ok(());
}

pub fn get_current_time() -> i64 {
    Local::now().timestamp_millis()
}

pub fn get_current_date_time() -> String {
    let local: DateTime<Local> = Local::now();
    let format = local.format("%B %d %Y %H:%M");
    format.to_string()
}

pub fn default_expiration() -> i64 {
    let time = Local::now();
    time.add(Duration::days(30)).timestamp_millis()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmailChangeRequest {
    pub email_username: Option<String>,
    pub email_password: Option<String>,
    pub email_host: Option<String>,
    pub encryption: Option<String>,
    pub from: Option<String>,
    pub port: Option<i64>,
}

pub fn get_accept(header_map: &HeaderMap) -> Result<Option<String>, RequestError> {
    let option = header_map.get("accept");
    if option.is_none() {
        return Ok(None);
    }
    let x = option.unwrap().to_str();
    if x.is_err() {}
    let header = x.unwrap().to_string();
    Ok(Some(header))
}

pub fn get_storage_location() -> PathBuf {
    return PathBuf::from(std::env::var("STORAGE_LOCATION").unwrap());
}
