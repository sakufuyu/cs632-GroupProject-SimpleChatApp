mod models;
mod chat_engine;
mod simulator;

use std::io::{self, Write};
use models::{ChatEvent, Message, UserManager};
use chat_engine::ChatEngine;
use simulator::ChatSimulator;


fn main() {
    println!("Simple Chat app");

    // Get user name at startup
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    let mut user_name = String::new();
    io::stdin().read_line(&mut user_name).expect("Failed to read input");
    let user_name = user_name.trim().to_string();

    // Initialize user manager
    let mut user_manager = UserManager::new();
    
    // Add predefined users
    user_manager.add_user("Akito".to_string());
    user_manager.add_user("Kazuki".to_string());
    user_manager.add_user("Miku".to_string());
    user_manager.add_user("Chisato".to_string());
    user_manager.add_user("Yumi".to_string());
    let current_user_id = user_manager.add_user(user_name);

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
    println!("  /switch <id> - Switch to user ID");
    println!("  /add <name> - Add new user");
    println!("  /list - List all users");
    println!("  /exit - Exit the application");

    let mut current_user_id = current_user_id;

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().unwrap_or_default();
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let event = parse_input(&input, current_user_id, &user_manager);

        match event {
            ChatEvent::Exit => {
                chat_engine.process_event(ChatEvent::Exit);
                break;
            },
            ChatEvent::SwitchUser(user_id) => {
                if user_manager.get_user(user_id).is_some() {
                    current_user_id = user_id;
                    println!("Switched to user ID: {}", user_id);
                } else {
                    println!("User ID {} not found", user_id);
                }
            },
            ChatEvent::AddUser(name) => {
                let new_id = user_manager.add_user(name.clone());
                println!("Added user '{}' with ID: {}", name, new_id);
            },
            ChatEvent::ListUsers => {
                user_manager.list_users();
            },
            _ => {
                chat_engine.process_event(event);
            }
        }
    }
}

fn parse_input(input: &str, current_user_id: u64, user_manager: &UserManager) -> ChatEvent {
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
    } else if input.starts_with("/switch") {
        if let Ok(user_id) = input[8..].parse::<u64>() {
            return ChatEvent::SwitchUser(user_id);
        }
    } else if input.starts_with("/add") {
        let name = input[5..].to_string();
        return ChatEvent::AddUser(name);
    } else if input.starts_with("/list") {
        return ChatEvent::ListUsers;
    } else {
        let user = user_manager.get_user(current_user_id).unwrap();
        let msg = Message::new(
            current_user_id,
            user.name.clone(),
            input.to_string(),
        );
        return ChatEvent::NewMessage(msg);
    }

    // Default case
    ChatEvent::Exit
}