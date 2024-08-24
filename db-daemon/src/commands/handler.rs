use super::common::Command;

pub fn commands_parser(command: &str) -> Command {
    match command {
        "get" => Command::Get,
        "delete" => Command::Delete,
        "insert" => Command::Insert,
        _ => Command::Nop,
    }
}
