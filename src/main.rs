// Rust Book Multithreaded Web Server Exercise
// Kaitlin Berryman

// std::io::prelude lets us read and write to the stream
use std::io::prelude::*;

// bring standard library's filesystem module into scope
use std::fs;

use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // listen at `127.0.0.1:7878` for incoming TCP streams
    // in the address, before the colon is an IP representing the computer & 7878 is the port
    // bind works like the new fn -- it returns a new TcpListener instance (wrapped in a Result)
    // in networking, connecting to a new port is called "binding to a port"
    // use unwrap to stop the program if errors happen
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // When it gets an incoming stream, call handle_connection and pass stream to it
    // The incoming method on TcpListener returns an iterator that gives a stream of TcpStream
    // A single stream is a open connection between a client and server
    // A connection is the name for the full request / response process
    // TcpStream will read from itself to see what the client sent and then allow us to write our response to the stream
    // This for loop will process each stream and produce a series of streams for us to handle
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // println!("Connection established :D")

        handle_connection(stream);
    }
}

// stream is mut because TcpStream's internal state may change
fn handle_connection(mut stream: TcpStream) {
    // declare a buffer on the stack 1024 bytes in size
    let mut buffer = [0; 1024];

    // read bytes from TcpStream and put in buffer
    stream.read(&mut buffer).unwrap();

    // hardcode the data coresponding to the / request
    // the b converts the raw bytes we get into a byte string
    let home_path_data = b"GET / HTTP/1.1\r\n";



    /*if buffer.starts_with(home_path_data) {
        status_line = "HTTP/1.1 200 OK";
        filename = "hello.html";
    } else {
        status_line = "HTTP/1.1 404 NOT FOUND";
        filename = "404.html";
    }*/

    let (status_line, filename) = if buffer.starts_with(home_path_data) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };


    // read html file and save it to a string
    // unwrap stops the program if errors happen
    let contents = fs::read_to_string(filename).unwrap();

    // Add file's contents to the body of response
    // Ensure a HTTP response by adding Content-Length header set to the size of our response body
    // ^^ in this case response body size is the size of hello.html
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
