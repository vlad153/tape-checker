use std::process::{Command, Output};
use std::str;

use regex::Regex;

use crate::service::{TapeResponse, Tape, TapeState, TapeStatusInfo};
use crate::exceptions::{ExecuteTapeAppError, TapeAppError};

//const WAITING_PATTERNS: &'static[&'static str] = &[];
const BUSY_PATTERN: &'static[&'static str] = &["Device or resource busy"];

#[derive(Debug)]
pub struct CommandTapeService;

impl TapeStatusInfo for CommandTapeService {

    fn get_tape_status(&self, tape: &mut Tape) -> Result<TapeResponse, TapeAppError> {

        if !tape.get_path().exists() {
            return Err(TapeAppError::ExecuteTapeAppError(ExecuteTapeAppError::new("Path to file don't exists")))
        }

        let path_to_tape = match tape.get_path().to_str() {
            Some(path_to_tape) => path_to_tape,
            None => return Err(TapeAppError::ExecuteTapeAppError(ExecuteTapeAppError::new("Empty path"))),
        };
        if let Ok(command) = Command::new("mt")
            .arg("-f")
            .arg(path_to_tape)
            .arg("status")
            .output() {
            Ok(match_mt_text_and_state(tape, command)?)
        } else {
            Err(TapeAppError::ExecuteTapeAppError(ExecuteTapeAppError::new("Error execute command to get tape status")))
        }
    }
}


fn match_mt_text_and_state(
    tape: &mut Tape,
    mt_response: Output
) -> Result<TapeResponse, TapeAppError> {
    if mt_response.status.success() {
        match String::from_utf8(mt_response.stdout) {
            Ok(execution_response) => {
                Ok(get_response_form_execution(tape, execution_response.as_str()))
            },
            Err(_) => Err(
                TapeAppError::ExecuteTapeAppError(
                    ExecuteTapeAppError::new("Error to convert execute message")
                )
            ),
        }
    } else {
        match String::from_utf8(mt_response.stderr.into()) {
            Ok(error_execution_response) => {
                Ok(get_response_form_execution(tape, error_execution_response.as_str()))
            },
            Err(_) => Err(
                TapeAppError::ExecuteTapeAppError(
                    ExecuteTapeAppError::new("Error to convert error message")
                )
            ),
        }
    }
}

fn get_response_form_execution(
    tape: &mut Tape,
    execution_response: &str
) -> TapeResponse {
    let state = match_response(execution_response);
    tape.set_state(state);
    form_response_from_state(tape, execution_response)
}

fn match_response(command_response: &str) -> TapeState {
    for pattern in BUSY_PATTERN.iter() {
        let busy_re = Regex::new(pattern).expect("Should be valid pattern");
        if busy_re.is_match(command_response) {
            return TapeState::Busy;
        }
    }
    TapeState::Unknown
}

fn form_response_from_state(tape: &Tape, command_response: &str) -> TapeResponse {
    TapeResponse::new(
        tape.clone(),
        command_response.to_string()
    )

}
