use std::{
    io::{self, Write, Read},
    net::{TcpListener, TcpStream, SocketAddr, Ipv4Addr, IpAddr},
};
use std::borrow::Cow;
use simple_http::http::request;
use crate::request::HttpRequest;

fn create_socket() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 5500)
}

fn handle_client(stream: &mut TcpStream) -> io::Result<()> {
    let mut buffer: [u8; 1024] = [0; 1024]; // Correct the size to 1024 for the buffer
    stream.read(&mut buffer)?; // Removed `buf:` since it's incorrect syntax in Rust
    


let buf_str: Cow<str> = String::from_utf8_lossy(&buffer);
// let request: HttpRequest = request::HttpRequest::new(request: &buf_str)?; // Correctly call `HttpRequest::new`
let request = HttpRequest::new(&buf_str)?;
let response= request.response()?;

println!("{:?}", &response);
println!("{}", &response.response_body);

let body = response.response_body.clone();

stream.write(&mut body.as_bytes())?;
// stream.write_all(buf_str.as_bytes())?; // Use write_all to write the buffer content

    stream.flush()?;
    
    Ok(())
}


fn serve(socket: SocketAddr) -> io::Result<()> {
    let listener = TcpListener::bind(socket)?;
    let mut counter: u32 = 0;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                match std::thread::spawn(move || handle_client(&mut stream)).join() {
                    Ok(_) => {
                        counter += 1;
                        println!("Connected stream... {}", counter);
                    }
                    Err(_) => continue,
                }
            }
            Err(_) => continue,
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let socket = create_socket();
    serve(socket)?;
    Ok(())
}
