//! Proxy used to snif the protocol, used whenever one is to lazy to read the C implementation

use std::{
    io::{IoSlice, IoSliceMut},
    os::{
        fd::{AsRawFd, RawFd},
        unix::net::{UnixListener, UnixStream},
    },
    sync::Arc,
};

use nix::sys::socket::{self, ControlMessage, ControlMessageOwned, MsgFlags};

// pub const MAX_BUFFER_SIZE: usize = 1024 * 32;
pub const MAX_BUFFER_SIZE: usize = 500000;
pub const MAX_FDS: usize = 1024;
pub const MAX_FDS_MSG: usize = 28;

fn recvmsg<'a>(stream: &UnixStream, buffer: &'a mut [u8]) -> (&'a [u8], Vec<ControlMessageOwned>) {
    let (len, cmsgs) = {
        let mut cmsg = nix::cmsg_space!([RawFd; MAX_FDS_MSG]);
        let mut iov = [IoSliceMut::new(buffer)];

        let msg = socket::recvmsg::<()>(
            stream.as_raw_fd(),
            &mut iov,
            Some(&mut cmsg),
            MsgFlags::MSG_CMSG_CLOEXEC,
        )
        .unwrap();

        let cmsgs: Vec<ControlMessageOwned> = msg.cmsgs().collect();
        (msg.bytes, cmsgs)
    };

    (&buffer[..len], cmsgs)
}

fn sendmsg(stream: &UnixStream, bytes: &[u8], cmsgs: &[ControlMessageOwned]) {
    let cmsgs: Vec<ControlMessage> = cmsgs
        .iter()
        .map(|cmsg| match cmsg {
            ControlMessageOwned::ScmRights(s) => ControlMessage::ScmRights(s),
            _ => todo!(),
        })
        .collect();
    let iov = [IoSlice::new(bytes)];

    socket::sendmsg::<()>(
        stream.as_raw_fd(),
        &iov,
        &cmsgs,
        MsgFlags::MSG_NOSIGNAL,
        None,
    )
    .unwrap();
}

fn main() {
    std::fs::remove_file("/run/user/1000/pipewire-1").ok();
    let listener = UnixListener::bind("/run/user/1000/pipewire-1").unwrap();

    let server = Arc::new(UnixStream::connect("/run/user/1000/pipewire-0").unwrap());

    let (stream, _add) = listener.accept().unwrap();
    let client = Arc::new(stream);

    let client_in = std::thread::spawn({
        let client = client.clone();
        let server = server.clone();
        let mut buffer = vec![0u8; MAX_BUFFER_SIZE];
        move || loop {
            let (bytes, cmsgs) = recvmsg(&client, &mut buffer);

            let mut reader = bytes;
            let mut count = 0;
            while let Some((rest, _msg)) = ripewire::connection::read_msg(reader) {
                reader = rest;
                count += 1;
                // pod_simple::dbg_print::dbg_print(&_msg.body);
            }
            println!("MSGs bytes len: {}, count: {count}", bytes.len());

            sendmsg(&server, bytes, &cmsgs);
        }
    });

    let server_in = std::thread::spawn({
        let client = client.clone();
        let server = server.clone();
        let mut buffer = vec![0u8; MAX_BUFFER_SIZE];
        move || loop {
            let (bytes, cmsgs) = recvmsg(&server, &mut buffer);

            let mut reader = bytes;
            let mut count = 0;
            while let Some((rest, _msg)) = ripewire::connection::read_msg(reader) {
                reader = rest;
                count += 1;
                // pod_simple::dbg_print::dbg_print(&_msg.body);
            }
            println!(" -> MSGs bytes len: {}, count: {count}", bytes.len());

            sendmsg(&client, bytes, &cmsgs);
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
