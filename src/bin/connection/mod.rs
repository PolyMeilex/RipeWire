use std::{
    io::{self, IoSlice, IoSliceMut},
    mem,
    os::{
        fd::{AsRawFd, RawFd},
        unix::net::UnixStream,
    },
    path::Path,
};

use nix::sys::socket::{self, ControlMessage, MsgFlags};

pub const MAX_FDS_OUT: usize = 28;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Header {
    pub object_id: u32,
    pub opcode: u32,
    pub len: usize,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Message {
    pub header: Header,
    pub body: Vec<u8>,
}

pub struct Connection {
    pub stream: UnixStream,
}

impl Connection {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        UnixStream::connect(path).map(|stream| Self { stream })
    }

    pub fn send_msg(&mut self, bytes: &[u8], fds: &[RawFd]) -> io::Result<usize> {
        // let flags = MsgFlags::MSG_DONTWAIT | MsgFlags::MSG_NOSIGNAL;
        let flags = MsgFlags::MSG_NOSIGNAL;
        let iov = [IoSlice::new(bytes)];

        if !fds.is_empty() {
            let cmsgs = [ControlMessage::ScmRights(fds)];
            Ok(socket::sendmsg::<()>(
                self.stream.as_raw_fd(),
                &iov,
                &cmsgs,
                flags,
                None,
            )?)
        } else {
            Ok(socket::sendmsg::<()>(
                self.stream.as_raw_fd(),
                &iov,
                &[],
                flags,
                None,
            )?)
        }
    }

    pub fn rcv_msg(&mut self) -> Vec<Message> {
        let mut buffer = vec![0u8; 50000];
        let mut cmsg = nix::cmsg_space!([RawFd; MAX_FDS_OUT]);

        let mut iov = [IoSliceMut::new(&mut buffer)];

        let msg = nix::sys::socket::recvmsg::<()>(
            self.stream.as_raw_fd(),
            &mut iov,
            Some(&mut cmsg),
            MsgFlags::MSG_CMSG_CLOEXEC | socket::MsgFlags::MSG_NOSIGNAL,
        )
        .unwrap();

        // let received_fds = msg.cmsgs().flat_map(|cmsg| match cmsg {
        //     socket::ControlMessageOwned::ScmRights(s) => s,
        //     _ => Vec::new(),
        // });

        // let fds: Vec<_> = received_fds.collect();
        // dbg!(fds);

        let bytes = msg.bytes;

        let mut buff = &buffer[..bytes];

        let mut messages = Vec::new();

        let hdr_size = 16;
        while let Some((b, msg)) = Self::read_msg(buff, hdr_size) {
            buff = b;

            messages.push(msg);
        }

        messages
    }

    fn read_msg(buff: &[u8], hdr_size: usize) -> Option<(&[u8], Message)> {
        if buff.len() < 4 * mem::size_of::<u32>() && buff.len() < hdr_size {
            return None;
        }

        let (object_id, opcode, len) = {
            let buff = unsafe { ::std::slice::from_raw_parts_mut(buff.as_ptr() as *mut u32, 4) };
            let object_id = buff[0];

            // O -> opcode
            // L -> len
            // Stored in u32 like so:
            // OOLLLLLL
            let opcode = buff[1] >> 24;
            let len = (buff[1] & 0x00ffffff) as usize;

            // ref: https://gitlab.freedesktop.org/pipewire/pipewire/-/blob/master/src/modules/module-protocol-native/connection.c#L501
            if object_id == 0 && opcode == 1 {
                if buff[3] >= 4 {
                    unimplemented!("Old version of the protocol");
                    // let hdr_size = 8;
                } else {
                    // let hdr_size = 16;
                }
            }

            (object_id, opcode, len)
        };

        let header = Header {
            object_id,
            opcode,
            len,
        };

        let buff = &buff[hdr_size..];

        let body = if len > 0 {
            let buff = buff[..len].to_vec();
            // let (_buff, value) = PodDeserializer::deserialize_from::<Value>(&buff[..len]).unwrap();
            // Somevalue
            Some(buff)
        } else {
            None
        }?;

        let buff = &buff[len..];

        let msg = Message { header, body };

        Some((buff, msg))
    }
}

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }
}
