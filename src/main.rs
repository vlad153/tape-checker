mod configuration;
mod commands;
mod service;
mod exceptions;
mod telegram;
mod storage;
mod check_state;

use std::{collections::HashMap, sync::Arc};
use tokio::sync::Mutex;

use commands::CommandTapeService;
use configuration::{get_tapes_paths, get_valid_user_form_env};
use service::Tape;
use storage::{Subscribers, Tapes};
use telegram::run_telegram_bot;


#[tokio::main]
async fn main() {
    let tape_service = CommandTapeService;
    let tape_service = Arc::new(tape_service);
    let subscribers: Subscribers = Arc::new(Mutex::new(HashMap::new()));
    let tapes_paths = get_tapes_paths();

    let mut tapes: HashMap<std::path::PathBuf, Tape> = HashMap::new();

    for tape_path in tapes_paths.iter() {
        tapes.insert(tape_path.clone(), Tape::new(tape_path.clone()));
    }

    let tapes: Tapes = Arc::new(Mutex::new(tapes));

    let valid_users_id = Arc::new(get_valid_user_form_env());
    run_telegram_bot(
        tape_service.clone(),
        tapes.clone(),
        valid_users_id,
        subscribers
    ).await;
    println!("Hello world");
}
