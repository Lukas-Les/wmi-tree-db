use std::sync::{Arc, Mutex};

use crate::commands::handler;
use crate::commands::common;
use crate::dsa::closed_char_tree::{ClosedTree, Vehicle, CarType};


pub async fn handle_request(tree: &Arc<Mutex<ClosedTree>>, request: &str) -> String {
    println!("request: {}", request);
    let mut parts = request.split_whitespace();
    let command_str = parts.next().unwrap_or("");
    println!("command: {}", command_str);
    let path = parts.next().unwrap_or("");
    let value = parts.next().unwrap_or("");
    println!("value: {}", value);

    let mut tree = tree.lock().unwrap(); // Acquire lock for safe mutable access
    let command = handler::commands_parser(command_str);
    match command {
        common::Command::Insert => {
            let make = match value {
                "audi" => &Vehicle::Car(CarType::Audi),
                "bmw" => &Vehicle::Car(CarType::Bmw),
                _ => panic!("WRONG MAKE"),
            };
            tree.insert(path, make);
            format!("Inserted value {} at path {}\n", value, path)
        }
        common::Command::Get => {
            println!("getting {}", path);
            if let Some(result) = tree.get(path) {
                format!("Value at path {}: {}\n", path, result)
            } else {
                format!("No value found at path {}\n", path)
            }
        }
        common::Command::Delete => {
            tree.deep_delete(path);
            format!("Deleted value at path {}\n", path)
        }
        _ => "Unknown command\n".to_string(),
    }
}
