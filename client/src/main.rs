mod lib;

use std::io;
use lib::{delete, get, insert};

fn main() -> io::Result<()> {
    // Example usage of sending commands to the server
    insert("abc", "audi");
    get("abc");
    delete("abc");
    Ok(())
}
