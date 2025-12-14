/// Minimal logging facade used by GDS "applications" layer.
///
/// This intentionally stays lightweight (stdout/stderr) so GDS can be embedded
/// without requiring a concrete logging backend.
#[derive(Clone, Debug, Default)]
pub struct Log;

impl Log {
    pub fn new() -> Self {
        Self
    }

    pub fn info(&self, message: &str) {
        println!("INFO: {}", message);
    }

    pub fn warn(&self, message: &str) {
        eprintln!("WARN: {}", message);
    }

    pub fn error(&self, message: &str) {
        eprintln!("ERROR: {}", message);
    }
}
