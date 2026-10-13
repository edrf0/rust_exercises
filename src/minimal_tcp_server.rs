use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream, SocketAddr};
use std::thread;

/// 1. Start a simple TCP server that listens, accepts one connection,
/// reads data, and echoes it back. Returns the assigned SocketAddr so clients know where to connect.
pub fn start_echo_server() -> io::Result<(TcpListener, SocketAddr)> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;
    let listener_clone = listener.try_clone()?;

    // We spawn a thread to handle one incoming client connection for testing purposes
    thread::spawn(move || {
        if let Ok((mut stream, _)) = listener_clone.accept() {
            let mut buffer = [0; 512];
            if let Ok(n) = stream.read(&mut buffer) {
                let _ = stream.write_all(&buffer[..n]);
            }
        }
    });

    Ok((listener, addr))
}

/// 2. Connect to a TCP server address, send a message, and read the response back as a String.
pub fn send_message_to_server(addr: SocketAddr, msg: &str) -> io::Result<String> {
    match TcpStream::connect(addr) {
        Ok(mut stream) => {
            stream.write_all(msg.as_bytes())?;
            let mut buffer = [0; 512];
            let bytes_read = stream.read(&mut buffer)?;
            Ok(String::from_utf8_lossy(&buffer[..bytes_read]).to_string())
        },
        Err(e) => Err(e),
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tcp_echo() {
        // Start server
        let (_listener, addr) = start_echo_server().unwrap();

        // Send message as client
        let response = send_message_to_server(addr, "Hello, Network!").unwrap();

        assert_eq!(response, "Hello, Network!");
    }
}