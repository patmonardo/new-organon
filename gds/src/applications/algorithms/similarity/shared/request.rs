use crate::concurrency::Concurrency;
use serde_json::Value;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Stream,
    Stats,
    Mutate,
    Write,
    Estimate,
}

impl Mode {
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            "stream" => Some(Self::Stream),
            "stats" => Some(Self::Stats),
            "mutate" => Some(Self::Mutate),
            "write" => Some(Self::Write),
            "estimate" => Some(Self::Estimate),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CommonRequest {
    pub graph_name: String,
    pub mode: Mode,
    pub concurrency: Concurrency,
    pub estimate_submode: Option<String>,
}

impl CommonRequest {
    pub fn parse(request: &Value) -> Result<Self, String> {
        let graph_name = request
            .get("graphName")
            .and_then(|v| v.as_str())
            .ok_or_else(|| "Missing 'graphName' parameter".to_string())?
            .to_string();

        let mode_str = request
            .get("mode")
            .and_then(|v| v.as_str())
            .unwrap_or("stream");
        let mode = Mode::parse(mode_str).ok_or_else(|| format!("Invalid mode '{mode_str}'"))?;

        let concurrency_raw = request
            .get("concurrency")
            .and_then(|v| v.as_u64())
            .unwrap_or(1) as usize;
        let concurrency = Concurrency::new(concurrency_raw)
            .ok_or_else(|| "concurrency must be > 0".to_string())?;

        let estimate_submode = request
            .get("submode")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        Ok(Self {
            graph_name,
            mode,
            concurrency,
            estimate_submode,
        })
    }
}

pub fn get_u64(request: &Value, key: &str) -> Option<u64> {
    request.get(key).and_then(|v| v.as_u64())
}

pub fn get_f64(request: &Value, key: &str) -> Option<f64> {
    request.get(key).and_then(|v| v.as_f64())
}

pub fn get_str<'a>(request: &'a Value, key: &str) -> Option<&'a str> {
    request.get(key).and_then(|v| v.as_str())
}

pub fn get_array<'a>(request: &'a Value, key: &str) -> Option<&'a Vec<Value>> {
    request.get(key).and_then(|v| v.as_array())
}
