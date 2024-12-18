use std::{
    fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, time::Duration,
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // handle_connection(stream);

        // test "/"" endpoint waits until "/sleep" endpoint completes.
        // ex: hit "/sleep" endpoint then hit "/" endpoint
        handle_connection_with_sleep(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {_http_request:#?}");

    let status_line = "HTTP/1.1 200 OK";
    let contents = fs::read_to_string("hello.html").unwrap();
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_connection_with_sleep(mut stream: TcpStream) {
    // Membaca request menggunakan BufReader
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();
    reader.read_line(&mut request_line).unwrap();

    println!("Request line: {}", request_line.trim_end()); // Debug untuk melihat request line

    // Memproses request berdasarkan request_line
    let (status_line, filename) = match request_line.trim_end() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    // Membaca file dan mengirimkan response
    let contents = std::fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}