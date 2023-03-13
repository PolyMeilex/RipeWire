use std::{io::Cursor, os::unix::net::UnixListener};

use connection::{Connection, Header};
use pod::{
    deserialize::PodDeserializer,
    serialize::{PodSerialize, PodSerializer},
    Value,
};

mod connection;

#[derive(Debug)]
pub struct Message {
    pub header: Header,
    pub body: Value,
}

#[derive(Debug)]
enum Kind {
    In(Message),
    Out(Message),
}

fn main() {
    // let dump = std::fs::read_to_string("./dump.ron").unwrap();

    // let dump: Vec<Kind> = ron::from_str(&dump).unwrap();

    // let mut conn = Connection::connect("/run/user/1000/pipewire-0").unwrap();

    // for msg in &dump[..6] {
    //     if let Kind::In(msg) = msg {
    //         dbg!(msg);

    //         conn.send_msg(
    //             &create_msg(msg.header.object_id, msg.header.opcode, &msg.body),
    //             &[],
    //         )
    //         .unwrap();
    //     }
    //     //
    // }

    // loop {}
}

fn run_proxy() {
    std::fs::remove_file("/run/user/1000/pipewire-1").ok();
    let listener = UnixListener::bind("/run/user/1000/pipewire-1").unwrap();

    let mut root = Connection::connect("/run/user/1000/pipewire-0").unwrap();

    let (stream, _add) = listener.accept().unwrap();

    let mut conn = Connection { stream };

    let mut out = Vec::new();

    loop {
        let msg = conn.rcv_msg();

        for msg in msg {
            // root.send_msg(bytes, fds)
            println!("client: {:?}", &msg.header);

            let value: Value = PodDeserializer::deserialize_from(&msg.body).unwrap().1;

            println!("{value:?}");

            root.send_msg(
                &create_msg(msg.header.object_id, msg.header.opcode, &value),
                &[],
            )
            .unwrap();

            out.push(Kind::In(Message {
                header: msg.header,
                body: value,
            }));
            write_file(&out);
        }

        let msg = root.rcv_msg();

        for msg in msg {
            // root.send_msg(bytes, fds)
            println!("server: {:?}", &msg.header);

            let value: Value = PodDeserializer::deserialize_from(&msg.body).unwrap().1;

            conn.send_msg(
                &create_msg(msg.header.object_id, msg.header.opcode, &value),
                &[],
            )
            .unwrap();

            out.push(Kind::Out(Message {
                header: msg.header,
                body: value,
            }));
            write_file(&out);
        }
    }
}

fn write_file(list: &[Kind]) {
    // let file = ron::ser::to_string_pretty(list, ron::ser::PrettyConfig::default()).unwrap();

    // std::fs::write("./dump.ron", &file).unwrap();
}

fn header(object_id: u32, opcode: u32, len: u32) -> [u8; 16] {
    let header_buffer = [0; 16];
    let buffer = unsafe { ::std::slice::from_raw_parts_mut(header_buffer.as_ptr() as *mut u32, 2) };

    let opcode = opcode << 24;

    buffer[0] = object_id;
    buffer[1] = opcode | len;

    header_buffer
}

pub fn create_msg(object_id: u32, opcode: u32, value: &impl PodSerialize) -> Vec<u8> {
    let (pod, _size) = PodSerializer::serialize(Cursor::new(Vec::new()), value).unwrap();
    let mut pod = pod.into_inner();

    let header = header(object_id, opcode, pod.len() as u32);

    let mut msg = header.to_vec();
    msg.append(&mut pod);

    msg
}
