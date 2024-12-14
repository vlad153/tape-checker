use std::path::PathBuf;

use crate::exceptions::TapeAppError;

pub trait TapeStatusInfo {
    fn get_tape_status(&self, tape: &mut Tape) -> Result<TapeResponse, TapeAppError>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum TapeState {
    Busy,
//    Waiting,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Tape {
    path: PathBuf,
    state: TapeState,
}

impl Tape {
    pub fn new(path: PathBuf) -> Tape {
        Tape { path, state: TapeState::Unknown }
    }

    pub fn get_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn set_state(&mut self, state: TapeState) {
        self.state = state;
    }

    pub fn get_state(&self) -> &TapeState {
        &self.state
    }
}

#[derive(Debug)]
pub struct TapeResponse {
    tape: Tape,
    external_response: String,
    app_response: &'static str
}

impl TapeResponse {

    pub fn new(
        tape: Tape,
        external_response: String
    ) -> TapeResponse {
        match tape.state {
            TapeState::Busy => {
                TapeResponse {
                    tape,
                    external_response,
                    app_response: "Writing on tape process"
                }
            }
            /*
            TapeState::Waiting => {
                TapeResponse {
                    tape,
                    external_response,
                    app_response: "Waiting change tape"

                }
            }
*/
            TapeState::Unknown => {
                TapeResponse {
                    tape,
                    external_response,
                    app_response: "Unknown problem"
                }
            }

        }

    }

    pub fn get_tape_status(&self) -> &TapeState {
        &self.tape.state
    }

    pub fn get_external_response(&self) -> String {
        self.external_response.clone()
    }

}
