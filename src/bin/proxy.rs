use std::{
    os::{
        fd::AsRawFd,
        unix::net::{UnixListener, UnixStream},
    },
    sync::Arc,
};

use pod::{deserialize::PodDeserializer, Value};

use ripewire::connection;

fn main() {
    std::fs::remove_file("/run/user/1000/pipewire-1").ok();
    let listener = UnixListener::bind("/run/user/1000/pipewire-1").unwrap();

    let server = Arc::new(UnixStream::connect("/run/user/1000/pipewire-0").unwrap());

    let (stream, _add) = listener.accept().unwrap();
    let client = Arc::new(stream);

    let client_in = std::thread::spawn({
        let client = client.clone();
        let server = server.clone();
        move || loop {
            let mut fds = [nix::poll::PollFd::new(
                client.as_raw_fd(),
                nix::poll::PollFlags::POLLIN,
            )];
            nix::poll::poll(&mut fds, -1).unwrap();

            let (msgs, fds) = connection::rcv_msg(&client).unwrap();
            for msg in msgs {
                match PodDeserializer::deserialize_from::<Value>(&msg.body) {
                    Ok(value) => {
                        println!("{:?}", value.1);
                    }
                    Err(err) => {
                        println!("Err: {:?}", err);
                    }
                }
                connection::send_msg(&server, &msg_raw(msg), &fds).unwrap();
            }
        }
    });

    let server_in = std::thread::spawn({
        let client = client.clone();
        let server = server.clone();
        move || loop {
            let mut fds = [nix::poll::PollFd::new(
                server.as_raw_fd(),
                nix::poll::PollFlags::POLLIN,
            )];
            nix::poll::poll(&mut fds, -1).unwrap();

            let (msgs, fds) = connection::rcv_msg(&server).unwrap();
            for msg in msgs {
                match PodDeserializer::deserialize_from::<Value>(&msg.body) {
                    Ok(value) => {
                        println!("{:?}", value.1);
                    }
                    Err(err) => {
                        println!("Err: {:?}", err);
                    }
                }
                connection::send_msg(&client, &msg_raw(msg), &fds).unwrap();
            }
        }
    });

    client_in.join().unwrap();
    server_in.join().unwrap();
}

pub fn msg_raw(mut msg: ripewire::connection::Message) -> Vec<u8> {
    let mut bytes = msg.header.serialize().to_vec();
    bytes.append(&mut msg.body);
    bytes
}
