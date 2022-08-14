use serde::{Deserialize, Serialize};
use wasm_bindgen::JsValue;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FlushOption {
    Pre,
    Post,
    Sync,
}

impl Default for FlushOption {
    fn default() -> Self {
        Self::Pre
    }
}

impl FlushOption {
    pub(crate) fn to_js_value(&self) -> JsValue {
        let s = match self {
            Self::Pre => "pre",
            Self::Post => "post",
            Self::Sync => "sync",
        };

        JsValue::from_str(s)
    }
}
