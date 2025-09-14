use std::time::{SystemTime, UNIX_EPOCH};

// Message structure
#[derive(Clone, Debug)]
pub struct Message {
    pub id: u64,
    pub sender_id: u64,
    pub sender_name: String,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    pub fn new(id: u64, sender_id: u64, sender_name: String, content: String) -> Self {
        Message {
            id,
            sender_id,
            sender_name,
            content,
            timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs()
        }
    }
}

// User structure
#[derive(Clone, Debug)]
pub struct User {
    pub id: u64,
    pub name: String,
    pub created_at: u64,
}

impl User {
    pub fn new(id: u64, name: String) -> Self {
        User {
            id,
            name,
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}

// Chat event
pub enum ChatEvent {
    NewMessage(Message),
    SearchByUser(u64),
    SearchByKeyword(String),
    Exit,
}