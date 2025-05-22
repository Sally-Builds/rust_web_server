use std::{fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}};
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }


}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    if buffer.starts_with(get) {
        let mut file = File::open("index.html").unwrap();
        let mut html_contents = String::new();
        file.read_to_string(&mut html_contents).unwrap();
    
        // println!("{}", html_contents);
    
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            html_contents.len(),
            html_contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let mut file = File::open("404.html").unwrap();
        let mut html_contents = String::new();
        file.read_to_string(&mut html_contents).unwrap();

        let response = format!(
            "HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            html_contents.len(),
            html_contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

}