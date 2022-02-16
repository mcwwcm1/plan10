use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use futures::join;
use tokio;

#[tokio::main]
async fn main() {
    // Async bodies return a 'future' that must be executed in the future.
    print!("Server or Client? (s/c): ");
    stdout().flush().expect("Flush failed.");

    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Did not enter a correct string");
    match input.trim() {
        "server" | "s" => server_actor().await,

        "client" | "c" => client_actor().await,

        _ => println!("Unknown input"),
    }
}

async fn server_actor() {
    // Create the communication channel for this actor.
    // Also known as the 'mailbox' for the actor
    let (_tx, _rx): (Sender<i32>, Receiver<i32>) = channel();

    println!("Opening receiver socket...");
    let socket = UdpSocket::bind("127.0.0.1:15076").await.expect("Couldn't bind to address");

    let mut buf = [0; 100];

    let mut clients = HashMap::new();

    loop {
        println!("Awaiting packets...");
        let (amt, src) = socket.recv_from(&mut buf).await.expect("Didn't receive data");
        let msg = String::from_utf8_lossy(&buf[..amt]); // Should probably handle errors rather than using the lossy method

        match &msg[..3] {
            "reg" => {
                let name = msg[4..].to_string();
                println!("Registered client: {name}");
                clients.insert(src, name);
            }
            "msg" => { 
                let formatted_message = format!("[{}] {}", clients[&src], msg[4..].to_string());
                println!("{formatted_message}");
                for (adr, _nam) in &clients {
                    socket.send_to(formatted_message.as_bytes() , &adr).await.expect("Couldn't send message");
                }
            },
            "ech" => { socket
                        .send_to(msg[4..].as_bytes(), src).await
                        .expect("Couldn't send message"); },
            _ => println!("Unrecognized command: {msg}"),
        }
    }
}

async fn client_actor() {
    // Create the communication channel for this actor.
    // Also known as the 'mailbox' for the actor
    let (_tx, _rx): (Sender<i32>, Receiver<i32>) = channel();

    print!("Username:  ");
    stdout().flush().expect("Flush failed.");

    let mut username = String::new();
    stdin()
        .read_line(&mut username)
        .expect("Did not enter a correct string");

        println!("Opening sender socket...");
        let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 15070)),
        SocketAddr::from(([127, 0, 0, 1], 15071)),
    ];
    let socket = UdpSocket::bind(&addrs[..]).await.expect("couldn't bind to address");

    let mut reg_command = "reg ".to_string();
    reg_command.push_str(&username.trim());
    socket
        .send_to(reg_command.as_bytes(), "127.0.0.1:15076").await
        .expect("Couldn't send message");

    join!(message_sender_service(&socket), receive_packet_service(&socket));
}

async fn message_sender_service(socket: &UdpSocket) {
    loop {
        print!(">  ");
        stdout().flush().expect("Flush failed.");

        let input = tokio::task::spawn_blocking(|| {
            let mut input = String::new();
            stdin()
                .read_line(&mut input)
                .expect("Did not enter a correct string");

            input
        }).await.unwrap();
        
        socket
            .send_to(&input.as_bytes(), "127.0.0.1:15076").await
            .expect("Couldn't send message");
    }
}

async fn receive_packet_service(socket: &UdpSocket) {
    loop {
        let mut buf : [u8; 100] = [0; 100];
        let (amt, src) = socket.recv_from(&mut buf).await.expect("Didn't receive data");
        println!("[{src}] {}", String::from_utf8_lossy(&buf[..amt]));
    }
}