use std::net::{TcpListener, TcpStream};

fn handle_client(_stream: TcpStream) {
    println!("HEY");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:5487").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => println!("{:?}", e),
        }
    }
}
