pub mod request;
pub mod response;

pub use request::{get_array, get_f64, get_str, get_u64, CommonRequest, Mode};
pub use response::{err, timings_json};
