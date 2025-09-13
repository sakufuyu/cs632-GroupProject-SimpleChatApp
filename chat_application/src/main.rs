mod models;
mod chat_engine;
mod simulator;

use std::io::{self, Write};
use models::{ChatEvent, Message};
use chat_engine::ChatEngine;
use simulator::ChatSimulator;


fn main() {
    println!("Simple Chat app");

    // Initialize the chat engine
    let chat_engine = ChatEngine::new();
    let messages = chat_engine.get_messages();

    // Initialize the chat simulator
    let simulator = ChatSimulator::new();
    simulator.start_simulation(messages.clone());

    // Main loop
    println!("Chat Application");
    println!("Commands: ");
    println!("  /user <id> - Search messages by user ID");
    println!("  /search <keyword> - Search messages by keyword");
    println!("  /exit - Exit the application");

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let event = parse_input(&input);

        if let ChatEvent::Exit = &event {
            chat_engine.process_event(event);
            break;
        } else {
            chat_engine.process_event(event);
        }
    }
}

fn parse_input(input: &str) -> ChatEvent {
    let input = input.trim();

    if input.starts_with("/exit") {
        return ChatEvent::Exit;
    } else if input.starts_with("/user") {
        if let Ok(user_id) = input[6..].parse::<u64>() {
            return ChatEvent::SearchByUser(user_id);
        }
    } else if input.starts_with("/search") {
        let keyword = input[8..].to_string();
        return ChatEvent::SearchByKeyword(keyword);
    } else {
        let current_user_id = 1;
        use std::time::{SystemTime, UNIX_EPOCH};
        let msg = Message {
            id: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
            sender_id: current_user_id,
            sender_name: "You".to_string(),
            content: input.to_string(),
            timestamp: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };
        return ChatEvent::NewMessage(msg);
    }

    // Default case
    ChatEvent::Exit
}