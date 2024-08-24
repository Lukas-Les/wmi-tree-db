use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;


fn send_command(command: &str) -> io::Result<()> {
    // Connect to the server
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    println!("Connected to the server.");

    // Send the command to the server
    stream.write_all(command.as_bytes())?;
    stream.write_all(b"\n")?; // Send a newline to indicate the end of the command

    // Read the response from the server
    let mut reader = BufReader::new(&stream);
    let mut response = String::new();
    reader.read_line(&mut response)?;

    // Print the server's response
    println!("Server response: {}", response.trim());

    Ok(())
}

pub fn insert(path: &str, value: &str) -> io::Result<()> {
    let command = format!("insert {} {}", path, value);
    send_command(&command)
}

pub fn get(path: &str) -> io::Result<()> {
    let command = format!("get {}", path);
    send_command(&command)
}

pub fn delete(path: &str) -> io::Result<()> {
    let command = format!("delete {}", path);
    send_command(&command)
}
