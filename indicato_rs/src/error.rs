#[derive(Debug)]
pub enum FinErrorType {
    DivideByZero,
    InvalidInput,
    InvalidOperation,
}

#[derive(Debug)]
pub struct FinError {
    pub error_type: FinErrorType,
    pub message: String,
}

impl FinError {
    pub fn new(error_type: FinErrorType, message: &str) -> Self {
        Self {
            error_type,
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for FinError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Error: {:?} - {}", self.error_type, self.message)
    }
}
