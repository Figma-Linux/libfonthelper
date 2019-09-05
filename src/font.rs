extern crate serde;
extern crate serde_json;

use super::types::FontEntry;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Serialize, Deserialize)]
pub struct Font {
  pub path: String,
  pub entries: Vec<FontEntry>,
}

impl Font {
  pub fn to_json(&self) -> String {
    let result = json!({
      &self.path: self.entries
    })
    .to_string();

    result.to_string()
  }
}
