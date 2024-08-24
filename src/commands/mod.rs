pub mod handler;

pub mod common {
    pub enum Command {
        Insert,
        Get,
        Delete,
        Nop,
    }
    
}