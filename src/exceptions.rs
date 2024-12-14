use core::fmt;


#[derive(Debug, Clone)]
pub struct ExecuteTapeAppError {
    message: &'static str
}

impl ExecuteTapeAppError {
    pub fn new(message: &'static str) -> ExecuteTapeAppError {
        ExecuteTapeAppError { message }
    }
}

impl fmt::Display for ExecuteTapeAppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

#[derive(Debug, Clone)]
pub struct ExecuteCommandTapeError {
    message: String
}

impl ExecuteCommandTapeError {
    pub fn new(message: String) -> ExecuteCommandTapeError {
        ExecuteCommandTapeError {
            message
        }
    }
}

impl fmt::Display for ExecuteCommandTapeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }

}


impl From<ExecuteTapeAppError> for ExecuteCommandTapeError {
    fn from(value: ExecuteTapeAppError) -> Self {
        ExecuteCommandTapeError { message: value.message.to_string() }
    }

}

#[derive(Debug)]
pub enum TapeAppError {
    ExecuteTapeAppError(ExecuteTapeAppError),
    ExecuteCommandTapeError(ExecuteCommandTapeError)
}
