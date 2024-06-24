use std::error::Error;

pub enum GenErrorType {
    Config,
    Custom,
}

pub struct GenError {
    pub type_: GenErrorType,
    pub message: String,
}

impl GenError {
    pub fn new(message: String) -> GenError {
        GenError {
            message,
            type_: GenErrorType::Custom,
        }
    }

    pub fn new_config(message: String) -> GenError {
        GenError {
            message,
            type_: GenErrorType::Config,
        }
    }
}

impl std::fmt::Display for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.type_ {
            GenErrorType::Config => write!(f, "configuration error: {}", self.message),
            GenErrorType::Custom => write!(f, "{}", self.message),
        }
    }
}

impl std::fmt::Debug for GenError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.type_ {
            GenErrorType::Config => write!(f, "configuration error: {}", self.message),
            GenErrorType::Custom => write!(f, "{}", self.message),
        }
    }
}

impl std::error::Error for GenError {
    fn description(&self) -> &str {
        match self.type_ {
            GenErrorType::Config => "configuration error",
            GenErrorType::Custom => &self.message,
        }
    }
}

impl From<String> for GenError {
    fn from(message: String) -> GenError {
        GenError::new(message)
    }
}

impl From<&str> for GenError {
    fn from(message: &str) -> GenError {
        GenError::new(message.to_string())
    }
}

impl From<Box<dyn Error>> for GenError {
    fn from(error: Box<dyn Error>) -> GenError {
        GenError::new(error.to_string())
    }
}
