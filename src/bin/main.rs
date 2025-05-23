use std::{fs::File, io::{Read, Write}, net::{TcpListener, TcpStream}, time::Duration};
use std::thread;
use hello_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        })

    }


}


fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    // println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n", "index.html")
    }else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK\r\n", "index.html")
    }
    else {
        ("HTTP/1.1 404 NOT FOUND\r\n", "404.html")
    };

    let mut file = File::open(filename).unwrap();
        let mut html_contents = String::new();
        file.read_to_string(&mut html_contents).unwrap();
    
        // println!("{}", html_contents);
    
        let response = format!(
            "{}Content-Type: text/html\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            html_contents.len(),
            html_contents
        );

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();

}