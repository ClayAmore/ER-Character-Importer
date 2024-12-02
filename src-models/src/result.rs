use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum JsResult {
    Ok(String),
    Err(String),
}

impl JsResult {
    pub fn ok(value: impl Serialize) -> String {
        serde_json::to_string(&JsResult::Ok(serde_json::to_string(&value).unwrap())).unwrap()
    }

    pub fn error(error: impl Serialize) -> String {
        serde_json::to_string(&JsResult::Err(serde_json::to_string(&error).unwrap())).unwrap()
    }
}
