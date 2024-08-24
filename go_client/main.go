package main

import (
	"bufio"
	"fmt"
	"net"
	"strings"
)

func sendCommand(command string) {
	// Define server address
	server := "localhost:8080"

	// Connect to the server
	conn, err := net.Dial("tcp", server)
	if err != nil {
		fmt.Println("Error connecting to server:", err)
		return
	}
	defer conn.Close()

	// Send the command
	fmt.Fprintf(conn, command+"\n")

	// Read the response
	response, err := bufio.NewReader(conn).ReadString('\n')
	if err != nil {
		fmt.Println("Error reading response:", err)
		return
	}

	fmt.Println("Received:", strings.TrimSpace(response))
}

func insert(path string, value string) {
	sendCommand("insert" + " " + path + " " + value)
}

func get(path string) {
	sendCommand("get" + " " + path)
}

func main() {
	// Example usage
	insert("aaaaa", "audi")
	insert("aaaab", "bmw")
	insert("aaaac", "audi")
	insert("aaaad", "audi")
	insert("aaabc", "audi")
	insert("aaabd", "audi")
	insert("aaabe", "audi")
	insert("aaabf", "audi")
	insert("aazss", "audi")
	insert("qwerty", "audi")
	insert("qqqwe", "audi")
	insert("fffrt", "audi")
	insert("adifb", "bmw")
	insert("dalknjv", "audi")
	insert("dsvjkn", "audi")
	insert("cxvcx", "audi")
	insert("asadsadbc", "audi")
	insert("ad", "audi")

	get("aaaaa")
	get("aaaab")
	get("aaaac")
	get("aaaad")
	get("aaabc")
	get("aaabd")
	get("aaabe")
	get("aaabf")
	get("aazss")
	get("qwerty")
	get("qqqwe")
	get("fffrt")
	get("adifb")
	get("dalknjv")
	get("dsvjkn")
	get("cxvcx")
	get("asadsadbc")
	get("ad")
}
