use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use crate::models::{User, Message};

pub struct ChatSimulator {
    users: Vec<User>,
}

impl ChatSimulator {
    pub fn new() -> Self {
        ChatSimulator {
            users: vec![
                User::new(1, "Akito".to_string()),
                User::new(2, "Kazuki".to_string()),
                User::new(3, "Miku".to_string()),
                User::new(4, "Chisato".to_string()),
                User::new(5, "Yumi".to_string()),
            ],
        }
    }

    pub fn start_simulation(&self, messages: Arc<Mutex<Vec<Message>>>) {
        let users = self.users.clone();

        thread::spawn(move || {
            let sample_messages = vec![
                (1, "Hello everyone!"),
                (2, "Hi!, how are you?"),
                (1, "I'm good, thanks for asking!"),
                (3, "Hello folks, what are we discussing today?"),
                (4, "Just catching up, Charlie."),
                (5, "Let's talk about Rust programming."),
            ];

            for (i, (user_id, content)) in sample_messages.iter().enumerate() {
                let user = users.iter().find(|u| u.id == *user_id).unwrap();

                // Create a new message
                let msg = Message::new(
                    i as u64 + 1,
                    *user_id,
                    user.name.clone(),
                    content.to_string(),
                );

                // Add message to the store
                let mut message_store = messages.lock().unwrap();
                message_store.push(msg.clone());
                drop(message_store); // Release the lock

                // Display the message
                println!("[{}] {}: {}", msg.id, msg.sender_name, msg.content);

                thread::sleep(Duration::from_secs(1));
            }
        });
    }
}