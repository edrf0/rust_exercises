use std::io;
use std::net::{UdpSocket, SocketAddr};

/// 1. Send a text message via UDP to a target socket address.
pub fn send_udp_message(target: SocketAddr, msg: &str) -> io::Result<usize> {
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    socket.send_to(msg.as_bytes(), target)
}

/// 2. Receive a UDP packet from a socket and return the message as a String.
pub fn receive_udp_message(socket: &UdpSocket) -> io::Result<String> {
    let mut buffer = [0; 512];
    let (amt, _src) = socket.recv_from(&mut buffer)?;
    Ok(String::from_utf8_lossy(&buffer[..amt]).to_string())
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_udp_messaging() {
        // Bind a receiver socket to a random available port
        let receiver_socket = UdpSocket::bind("127.0.0.1:0").unwrap();
        let receiver_addr = receiver_socket.local_addr().unwrap();

        // Send a message to our receiver socket
        send_udp_message(receiver_addr, "Hello UDP!").unwrap();

        // Receive the message
        let received_msg = receive_udp_message(&receiver_socket).unwrap();
        assert_eq!(received_msg, "Hello UDP!");
    }
}