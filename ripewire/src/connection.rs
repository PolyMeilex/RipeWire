use std::{
    collections::VecDeque,
    io::{self, IoSlice, IoSliceMut},
    mem::{self, MaybeUninit},
    os::{
        fd::{AsRawFd, BorrowedFd, OwnedFd, RawFd},
        unix::net::UnixStream,
    },
    path::Path,
};

use pod::PodDeserializer;
use rustix::net::{
    RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer,
    SendAncillaryMessage, SendFlags,
};

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
        // len -> message length
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
        let mut buffer = [0; 16 / mem::size_of::<u32>()];
        let opcode = self.opcode as u32;

        buffer[0] = self.object_id;
        buffer[1] = (opcode << 24) | (self.len & 0x00ffffff);
        buffer[2] = self.seq;
        buffer[3] = self.n_fds;

        unsafe { mem::transmute(buffer) }
    }
}

#[derive(Debug)]
pub struct Message<'a> {
    pub header: Header,
    pub body: PodDeserializer<'a>,
    pub footer: Option<PodDeserializer<'a>>,
    pub fds: Vec<OwnedFd>,
}

impl Message<'_> {
    pub fn raw_fds(&self) -> Vec<RawFd> {
        self.fds.iter().map(|fd| fd.as_raw_fd()).collect()
    }
}

pub struct MessageBuffer {
    buffer: Vec<u8>,
    fds: VecDeque<OwnedFd>,
    start: usize,
    end: usize,

    staging_buffer: Vec<u8>,
}

impl Default for MessageBuffer {
    fn default() -> Self {
        Self::new()
    }
}

impl MessageBuffer {
    pub fn new() -> Self {
        Self {
            buffer: vec![0u8; 4096],
            fds: VecDeque::new(),
            start: 0,
            end: 0,
            staging_buffer: Vec::new(),
        }
    }

    fn len(&self) -> usize {
        self.end - self.start
    }
}

pub struct Connection {
    stream: UnixStream,
}

impl Connection {
    pub fn connect<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        UnixStream::connect(path).map(Self::from_stream)
    }

    pub fn from_stream(stream: UnixStream) -> Self {
        Self { stream }
    }

    pub fn send_msg(&mut self, bytes: &[u8], fds: &[RawFd]) -> io::Result<usize> {
        send_msg(&self.stream, bytes, fds)
    }

    pub fn rcv_msg<'a>(&mut self, buffer: &'a mut MessageBuffer) -> io::Result<Message<'a>> {
        rcv_msg(&self.stream, buffer)
    }
}

impl AsRawFd for Connection {
    fn as_raw_fd(&self) -> RawFd {
        self.stream.as_raw_fd()
    }
}

pub fn read_header(buff: &[u8]) -> Option<(&[u8], Header)> {
    const HDR_SIZE: usize = 16;
    if buff.len() < 4 * mem::size_of::<u32>() && buff.len() < HDR_SIZE {
        return None;
    }

    let header = {
        let buff = unsafe { ::std::slice::from_raw_parts_mut(buff.as_ptr() as *mut u32, 4) };

        let header = Header::deserialize(buff);

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

    let buff = &buff[HDR_SIZE..];
    Some((buff, header))
}

fn read_body_and_footer<'a>(
    buff: &'a [u8],
    header: &Header,
) -> Option<(&'a [u8], PodDeserializer<'a>, Option<PodDeserializer<'a>>)> {
    let len = header.len as usize;
    let (body, footer) = {
        let body = &buff[..header.len as usize];
        debug_assert_eq!(body.len(), header.len as usize);

        let (body, footer) = pod::PodDeserializer::new(body);
        let footer = if footer.is_empty() {
            None
        } else {
            let (footer, rest) = pod::PodDeserializer::new(footer);
            debug_assert!(rest.is_empty());
            Some(footer)
        };

        (body, footer)
    };

    let buff = &buff[len..];

    Some((buff, body, footer))
}

fn read_fds(fds: &mut VecDeque<OwnedFd>, header: &Header) -> Vec<OwnedFd> {
    let mut msg_fds = Vec::with_capacity(header.n_fds as usize);
    for _ in 0..header.n_fds {
        match fds.pop_front() {
            Some(fd) => msg_fds.push(fd),
            None => todo!("missing fds"),
        }
    }
    msg_fds
}

pub fn read_msg<'a>(
    buff: &'a [u8],
    fds: &mut VecDeque<OwnedFd>,
) -> Option<(&'a [u8], Message<'a>)> {
    let (buff, header) = read_header(buff)?;
    let (buff, body, footer) = read_body_and_footer(buff, &header)?;
    let fds = read_fds(fds, &header);

    let msg = Message {
        header,
        body,
        footer,
        fds,
    };

    Some((buff, msg))
}

fn send_msg(stream: &UnixStream, bytes: &[u8], fds: &[RawFd]) -> io::Result<usize> {
    let iov = [IoSlice::new(bytes)];

    if !fds.is_empty() {
        let mut space = vec![MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(fds.len()))];
        let mut cmsgs = SendAncillaryBuffer::new(&mut space);

        // TODO: Just aceept BorrowedFd
        let fds = unsafe { std::mem::transmute::<&[RawFd], &[BorrowedFd]>(fds) };

        cmsgs.push(SendAncillaryMessage::ScmRights(fds));
        Ok(rustix::net::sendmsg(
            stream,
            &iov,
            &mut cmsgs,
            SendFlags::NOSIGNAL,
        )?)
    } else {
        let mut cmsgs = SendAncillaryBuffer::new(&mut []);
        Ok(rustix::net::sendmsg(
            stream,
            &iov,
            &mut cmsgs,
            SendFlags::NOSIGNAL,
        )?)
    }
}

fn fill_buf(stream: &UnixStream, buffer: &mut MessageBuffer, mut needed: usize) -> io::Result<()> {
    while needed > 0 {
        if buffer.start == buffer.end {
            buffer.start = 0;
            buffer.end = 0;

            let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(MAX_FDS_OUT))];
            let mut cmsgs = RecvAncillaryBuffer::new(&mut space);

            let mut iov = [IoSliceMut::new(&mut buffer.buffer)];

            let msg = rustix::net::recvmsg(stream, &mut iov, &mut cmsgs, RecvFlags::CMSG_CLOEXEC)?;

            let received_fds = cmsgs
                .drain()
                .filter_map(|cmsg| match cmsg {
                    RecvAncillaryMessage::ScmRights(fds) => Some(fds),
                    _ => None,
                })
                .flatten();
            buffer.fds.extend(received_fds);

            buffer.end = msg.bytes;

            if buffer.start == buffer.end {
                todo!("end of buffer");
            }
        }

        let read = needed.min(buffer.len());
        let start = buffer.start;
        buffer
            .staging_buffer
            .extend_from_slice(&buffer.buffer[start..start + read]);
        needed -= read;
        buffer.start += read;
    }

    Ok(())
}

fn rcv_msg<'a>(stream: &UnixStream, buffer: &'a mut MessageBuffer) -> io::Result<Message<'a>> {
    buffer.staging_buffer.clear();
    fill_buf(stream, buffer, 16)?;
    let (_, header) = read_header(&buffer.staging_buffer).unwrap();

    buffer.staging_buffer.clear();
    fill_buf(stream, buffer, header.len as usize)?;

    let (_, body, footer) = read_body_and_footer(&buffer.staging_buffer, &header).unwrap();

    let fds = read_fds(&mut buffer.fds, &header);

    Ok(Message {
        header,
        body,
        footer,
        fds,
    })
}
