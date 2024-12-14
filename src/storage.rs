use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::{HashSet, HashMap};

use crate::service::Tape;

#[derive(Debug)]
pub struct UserSubscription {
    pub telegram_id: u64,
    pub chat_id: i64,
}

pub type Subscribers = Arc<Mutex<HashMap<u64, UserSubscription>>>;
pub type Tapes = Arc<Mutex<HashMap<PathBuf, Tape>>>;
pub type ValidUsers = Arc<HashSet<u64>>;

