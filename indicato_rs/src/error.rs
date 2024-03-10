#[derive(Debug, PartialEq)]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fin_error() {
        let error = FinError::new(FinErrorType::InvalidInput, "Invalid input");
        assert_eq!(error.error_type, FinErrorType::InvalidInput);
        assert_eq!(error.message, "Invalid input");
        assert_eq!(format!("{}", error), "Error: InvalidInput - Invalid input");
        assert_eq!(format!("{:?}", error), "FinError { error_type: InvalidInput, message: \"Invalid input\" }");
    }
}