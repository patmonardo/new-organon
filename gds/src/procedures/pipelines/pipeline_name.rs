use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineNameError {
    Blank,
    ContainsWhitespace,
}

impl fmt::Display for PipelineNameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Blank => write!(f, "pipelineName must not be blank"),
            Self::ContainsWhitespace => write!(f, "pipelineName must not contain whitespace"),
        }
    }
}

impl Error for PipelineNameError {}

impl PipelineName {
    pub fn parse(pipeline_name: &str) -> Result<Self, PipelineNameError> {
        if pipeline_name.trim().is_empty() {
            return Err(PipelineNameError::Blank);
        }
        if pipeline_name.chars().any(|c| c.is_whitespace()) {
            return Err(PipelineNameError::ContainsWhitespace);
        }
        Ok(Self(pipeline_name.to_string()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}
