//! Proxy used to snif the protocol, used whenever one is to lazy to read the C implementation

#![allow(clippy::single_match)]

use std::{
    collections::HashMap,
    io::{IoSlice, IoSliceMut},
    os::{
        fd::{AsRawFd, RawFd},
        unix::net::{UnixListener, UnixStream},
    },
    sync::{Arc, Mutex},
};

use nix::sys::socket::{self, ControlMessage, ControlMessageOwned, MsgFlags};
use pod_simple::{deserialize::PodDeserializerKind, PodDeserializer};
use ripewire::{connection::Message, protocol::EventDeserialize as _};

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

    let objects = Arc::new(Mutex::new(objects()));
    let client_in = std::thread::spawn({
        let client = client.clone();
        let server = server.clone();
        let mut buffer = vec![0u8; MAX_BUFFER_SIZE];
        let objects = objects.clone();
        let interfaces = interfaces();
        move || loop {
            let (bytes, cmsgs) = recvmsg(&client, &mut buffer);

            let mut reader = bytes;
            while let Some((rest, msg)) = ripewire::connection::read_msg(reader) {
                reader = rest;
                inspect_method(&objects, &interfaces, &msg);
                // pod_simple::dbg_print::dbg_print(&msg.body);
            }

            sendmsg(&server, bytes, &cmsgs);
        }
    });

    let server_in = std::thread::spawn({
        let client = client.clone();
        let server = server.clone();
        let mut buffer = vec![0u8; MAX_BUFFER_SIZE];
        let objects = objects.clone();
        let interfaces = interfaces();
        move || loop {
            let (bytes, cmsgs) = recvmsg(&server, &mut buffer);

            let mut reader = bytes;
            while let Some((rest, msg)) = ripewire::connection::read_msg(reader) {
                reader = rest;
                inspect_event(&objects, &interfaces, &msg);
                // pod_simple::dbg_print::dbg_print(&msg.body);
            }

            sendmsg(&client, bytes, &cmsgs);
        }
    });

    client_in.join().unwrap();
    server_in.join().unwrap();
}

fn inspect_method(objects: &Mutex<Objects>, interfaces: &Interfaces, msg: &Message) {
    let mut objects = objects.lock().unwrap();
    if let Some(interface) = objects.get(&msg.header.object_id) {
        print!("{}@{}.", strip_prefix(interface), msg.header.object_id);

        if let Some((methods, _events)) = interfaces.get(interface.as_str()) {
            let method = methods.get(&msg.header.opcode).unwrap();
            print!("{}", method);

            match interface.as_str() {
                "PipeWire:Interface:Core" => match *method {
                    "GetRegistry" => {
                        let (pod, _) = PodDeserializer::new(&msg.body);
                        let PodDeserializerKind::Struct(mut pod) = pod.kind() else {
                            unreachable!("Non struct method call");
                        };

                        let _version = pod.next().unwrap();
                        let new_id = pod.next().unwrap();
                        let PodDeserializerKind::Int(new_id) = new_id.kind() else {
                            unreachable!("Non int new_id");
                        };

                        objects.insert(new_id as u32, "PipeWire:Interface:Registry".to_string());
                    }
                    "CreateObject" => {
                        let (pod, _) = PodDeserializer::new(&msg.body);
                        let PodDeserializerKind::Struct(mut pod) = pod.kind() else {
                            unreachable!("Non struct method call");
                        };

                        let _factory_name = pod.next().unwrap().as_str().unwrap();
                        let interface_type = pod.next().unwrap().as_str().unwrap();
                        let _version = pod.next().unwrap();
                        let _props = pod.next().unwrap();
                        let new_id = pod.next().unwrap();
                        let PodDeserializerKind::Int(new_id) = new_id.kind() else {
                            unreachable!("Non int new_id");
                        };

                        objects.insert(new_id as u32, interface_type.to_string());
                    }
                    _ => {}
                },
                "PipeWire:Interface:Registry" => match *method {
                    "Bind" => {
                        let (pod, _) = PodDeserializer::new(&msg.body);
                        let PodDeserializerKind::Struct(mut pod) = pod.kind() else {
                            unreachable!("Non struct method call");
                        };

                        let _id = pod.next().unwrap();
                        let interface_type = pod.next().unwrap().as_str().unwrap();
                        let interface_type = interface_type.to_string();

                        let _version = pod.next().unwrap();
                        let new_id = pod.next().unwrap();
                        let PodDeserializerKind::Int(new_id) = new_id.kind() else {
                            unreachable!("Non int new_id");
                        };

                        objects.insert(new_id as u32, interface_type);
                    }
                    _ => {}
                },
                _ => {}
            }
        } else {
            print!("{}", msg.header.opcode);
        }
        println!("()");
    } else {
        println!("Header: {:?}", msg.header);
    }
}

fn inspect_event(objects: &Mutex<Objects>, interfaces: &Interfaces, msg: &Message) {
    let objects = objects.lock().unwrap();

    print!("-> ");
    if let Some(interface) = objects.get(&msg.header.object_id) {
        print!("{}@{}.", strip_prefix(interface), msg.header.object_id);

        if let Some((_methods, events)) = interfaces.get(interface.as_str()) {
            let event = events.get(&msg.header.opcode).unwrap();

            match interface.as_str() {
                "PipeWire:Interface:Core" => inspect_core_event(msg.header.opcode, msg),
                "PipeWire:Interface:Registry" => inspect_registry_event(event, msg),
                _ => {
                    println!("{}()", event);
                }
            }
        } else {
            println!("{}()", msg.header.opcode);
        }
    } else {
        println!("Header: {:?}", msg.header);
    }
}

fn inspect_core_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_core;
    let (mut pod, _) = PodDeserializer::new(&msg.body);
    match pw_core::Event::deserialize_event(opcode, &mut pod, &[]).unwrap() {
        pw_core::Event::Info(v) => println!("{v:#?}"),
        pw_core::Event::Done(v) => println!("{v:?}"),
        pw_core::Event::Ping(v) => println!("{v:?}"),
        pw_core::Event::Error(v) => println!("{v:?}"),
        pw_core::Event::RemoveId(v) => println!("{v:?}"),
        pw_core::Event::BoundId(v) => println!("{v:?}"),
        pw_core::Event::AddMem(v) => println!("{v:#?}"),
        pw_core::Event::RemoveMem(v) => println!("{v:?}"),
        pw_core::Event::BoundProps(v) => println!("{v:#?}"),
    }
}

fn inspect_registry_event(event: &str, msg: &Message) {
    use ripewire::protocol::pw_registry;
    let (mut pod, _) = PodDeserializer::new(&msg.body);
    match event {
        "Global" => {
            let global = pw_registry::events::Global::deserialize(&mut pod).unwrap();
            println!("{global:#?}");
        }
        _ => {
            println!("{}()", event);
        }
    }
}

type Methods = HashMap<u8, &'static str>;
type Events = HashMap<u8, &'static str>;
type Interfaces = HashMap<&'static str, (Methods, Events)>;
type Objects = HashMap<u32, String>;

fn pw_core() -> (Methods, Events) {
    (
        HashMap::from([
            (1, "Hello"),
            (2, "Sync"),
            (3, "Pong"),
            (4, "Error"),
            (5, "GetRegistry"),
            (6, "CreateObject"),
            (7, "Destroy"),
        ]),
        HashMap::from([
            (0, "Info"),
            (1, "Done"),
            (2, "Ping"),
            (3, "Error"),
            (4, "RemoveId"),
            (5, "BoundId"),
            (6, "AddMem"),
            (7, "RemoveMem"),
            (8, "BoundProps"),
        ]),
    )
}

fn pw_registry() -> (Methods, Events) {
    (
        HashMap::from([
            (1, "Bind"),
            (2, "Destroy"),
            //
        ]),
        HashMap::from([
            (0, "Global"),
            (1, "GlobalRemove"),
            //
        ]),
    )
}

fn pw_client() -> (Methods, Events) {
    (
        HashMap::from([
            (1, "Error"),
            (2, "UpdateProperties"),
            (3, "GetPermissions"),
            (4, "UpdatePermissions"),
        ]),
        HashMap::from([
            (0, "Info"),
            (1, "Permissions"),
            //
        ]),
    )
}

fn pw_client_node() -> (Methods, Events) {
    (
        HashMap::from([
            (1, "GetNode"),
            (2, "Update"),
            (3, "PortUpdate"),
            (4, "SetActive"),
            (5, "Event"),
            (6, "PortBuffers"),
        ]),
        HashMap::from([
            (0, "Transport"),
            (1, "SetParam"),
            (2, "SetIO"),
            (3, "Event"),
            (4, "Command"),
            (5, "AddPort"),
            (6, "RemovePort"),
            (7, "PortSetParam"),
            (8, "UseBuffers"),
            (9, "PortSetIO"),
            (10, "SetActivation"),
            (11, "PortSetMixInfo"),
        ]),
    )
}

fn interfaces() -> Interfaces {
    HashMap::from([
        ("PipeWire:Interface:Core", pw_core()),
        ("PipeWire:Interface:Registry", pw_registry()),
        ("PipeWire:Interface:Client", pw_client()),
        ("PipeWire:Interface:ClientNode", pw_client_node()),
    ])
}

fn objects() -> Objects {
    HashMap::from([
        (0, "PipeWire:Interface:Core".to_string()),
        (1, "PipeWire:Interface:Client".to_string()),
        //
    ])
}

fn strip_prefix(interface: &str) -> &str {
    interface
        .strip_prefix("PipeWire:Interface:")
        .unwrap_or(interface)
}
