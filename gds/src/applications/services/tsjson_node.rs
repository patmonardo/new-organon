use napi_derive::napi;

/// Node/N-API adapter for the core TS-JSON boundary (`services::tsjson`).
///
/// This should stay wafer-thin: JSON in / JSON out, no application logic.
#[napi]
pub fn invoke(request_json: String) -> napi::Result<String> {
    Ok(crate::applications::services::tsjson::invoke(request_json))
}

/// Convenience: returns the Rust crate version.
#[napi]
pub fn version() -> String {
    crate::applications::services::tsjson::version()
}


