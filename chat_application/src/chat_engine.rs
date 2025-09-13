use std::sync::{Arc, Mutex};
use crate::models::{Message, ChatEvent};

pub struct ChatEngine {
    messages: Arc<Mutex<Vec<Message>>>,
}

impl ChatEngine {
    pub fn new() -> Self {
        ChatEngine {
            messages: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn get_messages(&self) -> Arc<Mutex<Vec<Message>>> {
        Arc::clone(&self.messages)
    }

    pub fn process_event(&self, event: ChatEvent) {
        match event {
            ChatEvent::NewMessage(msg) => {
                let mut messages = self.messages.lock().unwrap();
                println!("[{}] {}: {}", msg.id, msg.sender_name, msg.content);
                messages.push(msg);
            },
            ChatEvent::SearchByUser(user_id) => {
                let messages = self.messages.lock().unwrap();
                println!("Messages from user {}:", user_id);
                for msg in messages.iter().filter(|m| m.sender_id == user_id) {
                    println!("[{}] {}: {}", msg.id, msg.sender_name, msg.content);
                }
            },
            ChatEvent::SearchByKeyword(keyword) => {
                let messages = self.messages.lock().unwrap();
                println!("Messages containing '{}':", keyword);
                for msg in messages.iter().filter(|m| m.content.to_lowercase().contains(&keyword.to_lowercase())) {
                    println!("[{}] {}: {}", msg.id, msg.sender_name, msg.content);
                }
            },
            ChatEvent::Exit => {
                println!("Exiting chat 👋");
            }
        }
    }
}