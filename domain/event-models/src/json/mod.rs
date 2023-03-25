use serde::{Deserialize, Serialize};

use crate::EventModel;

/// Lossy, as we don't bring over Schemas, and we downgrade
/// non-supported Interface types to blank.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonV0_1_0BetaTransfer {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JsonV1_0_0Transfer {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JsonImport {
    V0_1_0Beta(JsonV0_1_0BetaTransfer),
    V1_0_0(JsonV1_0_0Transfer),
}

pub fn import(json: &str) -> JsonImport {
    todo!()
}

#[derive(Debug, Clone, Serialize)]
pub enum JsonExport {
    V1_0_0(JsonV1_0_0Transfer),
}

pub fn export(model: &JsonExport) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::JsonImport;

    #[test]
    fn json_import_v0_1_0_beta() {
        let json = r#"
          {
            "spec-version": "0.1.0-beta",
            "event-model": {}
          }"#;

        let import: JsonImport = serde_json::from_str(json);
    }
}
