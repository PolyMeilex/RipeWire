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

#[derive(Debug, Clone)]
pub struct Header {
    pub object_id: u32,
    pub opcode: u8,
    pub len: u32,
    pub seq: u32,
    pub n_fds: u32,
}

impl Header {
    pub fn deserialize(bytes: &[u32]) -> Self {
        let object_id = bytes[0];

        // opc -> message opcode
        // len -> message lenght
        //
        // Stored in u32 like so:
        // opc: 11111111000000000000000000000000
        // len: 00000000111111111111111111111111
        let (opcode, len) = (bytes[1] >> 24, (bytes[1] & 0x00ffffff));

        let seq = bytes[2];
        let n_fds = bytes[3];

        Self {
            object_id,
            opcode: opcode as u8,
            len,
            seq,
            n_fds,
        }
    }

    pub fn serialize(&self) -> [u8; 16] {
        let header_buffer = [0; 16];
        let buffer =
            unsafe { ::std::slice::from_raw_parts_mut(header_buffer.as_ptr() as *mut u32, 4) };

        let opcode = self.opcode as u32;

        buffer[0] = self.object_id;
        buffer[1] = (opcode << 24) | (self.len & 0x00ffffff);
        buffer[2] = self.seq;
        buffer[3] = self.n_fds;

        header_buffer
    }
}

#[derive(Debug, Clone)]
pub struct Message {
    pub header: Header,
    pub body: Vec<u8>,
}

pub struct Connection {
    stream: UnixStream,
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

    pub fn rcv_msg(&mut self) -> io::Result<(Vec<Message>, Vec<RawFd>)> {
        let mut buffer = vec![0u8; 500000];
        let mut cmsg = nix::cmsg_space!([RawFd; MAX_FDS_OUT]);

        let mut iov = [IoSliceMut::new(&mut buffer)];

        let msg = nix::sys::socket::recvmsg::<()>(
            self.stream.as_raw_fd(),
            &mut iov,
            Some(&mut cmsg),
            MsgFlags::MSG_CMSG_CLOEXEC | socket::MsgFlags::MSG_NOSIGNAL,
        )?;

        let received_fds: Vec<RawFd> = msg
            .cmsgs()
            .flat_map(|cmsg| match cmsg {
                socket::ControlMessageOwned::ScmRights(s) => s,
                _ => Vec::new(),
            })
            .collect();

        let bytes = msg.bytes;

        let mut buff = &buffer[..bytes];

        let mut messages = Vec::new();

        let hdr_size = 16;
        while let Some((b, msg)) = Self::read_msg(buff, hdr_size) {
            buff = b;

            messages.push(msg);
        }

        Ok((messages, received_fds))
    }

    fn read_msg(buff: &[u8], hdr_size: usize) -> Option<(&[u8], Message)> {
        if buff.len() < 4 * mem::size_of::<u32>() && buff.len() < hdr_size {
            return None;
        }

        let header = {
            let buff = unsafe { ::std::slice::from_raw_parts_mut(buff.as_ptr() as *mut u32, 4) };

            let header = Header::deserialize(&buff);

            // if core hello message
            if header.object_id == 0 && header.opcode == 1 {
                // Check the type of the pod in the message. Old versions
                // should not have 0 there, new versions keep the number of file
                // descriptors, which should be 0 for the first message.
                //
                // Although libpipewire checks if the pod size is > 4 because "the unit test adds one fd
                // in the first message.". So I guess we'll do that as well.
                if header.n_fds >= 4 {
                    unimplemented!("Old version of the protocol");
                    // let hdr_size = 8;
                } else {
                    // let hdr_size = 16;
                }
            }

            header
        };

        let buff = &buff[hdr_size..];

        let len = header.len as usize;
        let body = if len > 0 {
            let buff = buff[..len].to_vec();
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
