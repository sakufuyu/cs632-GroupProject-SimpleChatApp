use std::time::{SystemTime, UNIX_EPOCH};
use std::collections::HashMap;

// Message structure
#[derive(Clone, Debug)]
pub struct Message {
    pub sender_id: u64,
    pub sender_name: String,
    pub content: String,
    pub timestamp: u64,
}

impl Message {
    pub fn new(sender_id: u64, sender_name: String, content: String) -> Self {
        Message {
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

// User manager
pub struct UserManager {
    users: HashMap<u64, User>,
    next_id: u64,
}

impl UserManager {
    pub fn new() -> Self {
        UserManager {
            users: HashMap::new(),
            next_id: 1,
        }
    }

    pub fn add_user(&mut self, name: String) -> u64 {
        let id = self.next_id;
        self.users.insert(id, User::new(id, name));
        self.next_id += 1;
        id
    }

    pub fn get_user(&self, id: u64) -> Option<&User> {
        self.users.get(&id)
    }

    pub fn list_users(&self) {
        println!("Current users:");
        for (id, user) in &self.users {
            println!("  ID: {} - Name: {}", id, user.name);
        }
    }
}

// Chat event
pub enum ChatEvent {
    NewMessage(Message),
    SearchByUser(u64),
    SearchByKeyword(String),
    SwitchUser(u64),
    AddUser(String),
    ListUsers,
    Exit,
}