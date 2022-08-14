use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("None error")]
    NoneError,

    #[error("convert function errror")]
    ConvertFunctionError,

    #[error(transparent)]
    SerdeJsonError(#[from] serde_json::Error),

    #[error("{0:?}")]
    JsValueError(wasm_bindgen::JsValue),
}

impl From<wasm_bindgen::JsValue> for Error {
    fn from(e: wasm_bindgen::JsValue) -> Self {
        Error::JsValueError(e)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
