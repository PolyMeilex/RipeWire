//! Proxy used to snif the protocol, used whenever one is to lazy to read the C implementation

#![allow(clippy::single_match)]

use std::{
    collections::{HashMap, VecDeque},
    io::{IoSlice, IoSliceMut},
    mem::MaybeUninit,
    os::{
        fd::{BorrowedFd, IntoRawFd, RawFd},
        unix::net::{UnixListener, UnixStream},
    },
    sync::{Arc, Mutex},
};

use pod::deserialize::PodDeserializerKind;
use ripewire::{connection::Message, object_map::ObjectType};
use rustix::net::{
    RecvAncillaryBuffer, RecvAncillaryMessage, RecvFlags, SendAncillaryBuffer,
    SendAncillaryMessage, SendFlags,
};

// pub const MAX_BUFFER_SIZE: usize = 1024 * 32;
pub const MAX_BUFFER_SIZE: usize = 500000;
pub const MAX_FDS: usize = 1024;
pub const MAX_FDS_MSG: usize = 28;

fn recvmsg<'a>(stream: &UnixStream, buffer: &'a mut [u8]) -> (&'a [u8], VecDeque<RawFd>) {
    let mut space = [MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(MAX_FDS_MSG))];
    let mut cmsgs = RecvAncillaryBuffer::new(&mut space);
    let mut iov = [IoSliceMut::new(buffer)];

    let len = rustix::net::recvmsg(stream, &mut iov, &mut cmsgs, RecvFlags::CMSG_CLOEXEC)
        .unwrap()
        .bytes;

    let mut fds = VecDeque::new();

    let received_fds = cmsgs
        .drain()
        .filter_map(|cmsg| match cmsg {
            RecvAncillaryMessage::ScmRights(fds) => Some(fds),

            _ => todo!(),
        })
        .flatten()
        .map(|fd| fd.into_raw_fd());
    fds.extend(received_fds);

    (&buffer[..len], fds)
}

fn sendmsg(stream: &UnixStream, bytes: &[u8], fds: VecDeque<RawFd>) {
    let fds: Vec<_> = fds
        .iter()
        .map(|fd| unsafe { BorrowedFd::borrow_raw(*fd) })
        .collect();
    let iov = [IoSlice::new(bytes)];

    let mut space = vec![MaybeUninit::uninit(); rustix::cmsg_space!(ScmRights(fds.len()))];
    let mut cmsgs = SendAncillaryBuffer::new(&mut space);

    cmsgs.push(SendAncillaryMessage::ScmRights(&fds));

    rustix::net::sendmsg(stream, &iov, &mut cmsgs, SendFlags::NOSIGNAL).unwrap();
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
            buffer.fill(0);
            let (bytes, fds) = recvmsg(&client, &mut buffer);

            let mut reader = bytes;
            let mut fds_read = fds.clone();
            while let Some((rest, msg)) = ripewire::connection::read_msg(reader, &mut fds_read) {
                reader = rest;
                inspect_method(&objects, &interfaces, &msg);
                // pod::dbg_print::dbg_print(&msg.body);
            }

            sendmsg(&server, bytes, fds);
        }
    });

    let server_in = std::thread::spawn({
        let client = client.clone();
        let server = server.clone();
        let mut buffer = vec![0u8; MAX_BUFFER_SIZE];
        let objects = objects.clone();
        let interfaces = interfaces();
        move || loop {
            buffer.fill(0);
            let (bytes, fds) = recvmsg(&server, &mut buffer);

            let mut reader = bytes;
            let mut fds_read = fds.clone();
            while let Some((rest, msg)) = ripewire::connection::read_msg(reader, &mut fds_read) {
                reader = rest;
                inspect_event(&objects, &interfaces, &msg);
                // pod::dbg_print::dbg_print(&msg.body);
            }

            sendmsg(&client, bytes, fds);
        }
    });

    client_in.join().unwrap();
    server_in.join().unwrap();
}

fn inspect_method(objects: &Mutex<Objects>, interfaces: &Interfaces, msg: &Message) {
    print!("<- ");
    let mut objects = objects.lock().unwrap();
    if let Some(interface) = objects.get(&msg.header.object_id) {
        print!("{:?}@{}.", interface, msg.header.object_id);

        if let Some((methods, _events)) = interfaces.get(interface) {
            let method = methods.get(&msg.header.opcode).unwrap();
            print!("{}", method);

            match interface {
                ObjectType::Client => {
                    print!(" {:#?}", msg.body);
                }
                ObjectType::Core => {
                    print!(" {:?}", msg.body);
                    match *method {
                        "GetRegistry" => {
                            let pod = &msg.body;
                            let PodDeserializerKind::Struct(mut pod) = pod.kind() else {
                                unreachable!("Non struct method call");
                            };

                            let _version = pod.next().unwrap();
                            let new_id = pod.next().unwrap();
                            let PodDeserializerKind::Int(new_id) = new_id.kind() else {
                                unreachable!("Non int new_id");
                            };

                            objects.insert(new_id as u32, ObjectType::Registry);
                        }
                        "CreateObject" => {
                            let pod = &msg.body;
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

                            objects.insert(
                                new_id as u32,
                                ObjectType::from_interface_name(&interface_type.to_string()),
                            );
                        }
                        _ => {}
                    }
                }
                ObjectType::Registry => {
                    print!(" {:?}", msg.body);
                    match *method {
                        "Bind" => {
                            let pod = &msg.body;
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

                            objects.insert(
                                new_id as u32,
                                ObjectType::from_interface_name(&interface_type.to_string()),
                            );
                        }
                        _ => {}
                    }
                }
                ObjectType::ClientNode => {
                    print!(" ");
                    inspect_client_node_method(msg.header.opcode, msg);
                }
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
        print!("{:?}@{}.", interface, msg.header.object_id);

        match interface {
            ObjectType::Core => inspect_core_event(msg.header.opcode, msg),
            ObjectType::Client => inspect_client_event(msg.header.opcode, msg),
            ObjectType::ClientNode => inspect_client_node_event(msg.header.opcode, msg),
            ObjectType::Device => inspect_device_event(msg.header.opcode, msg),
            ObjectType::Factory => inspect_factory_event(msg.header.opcode, msg),
            ObjectType::Link => inspect_link_event(msg.header.opcode, msg),
            ObjectType::Module => inspect_module_event(msg.header.opcode, msg),
            ObjectType::Node => inspect_node_event(msg.header.opcode, msg),
            ObjectType::Port => inspect_port_event(msg.header.opcode, msg),
            ObjectType::Registry => inspect_registry_event(msg.header.opcode, msg),
            _ => {
                if let Some(event) = interfaces
                    .get(interface)
                    .and_then(|(_, events)| events.get(&msg.header.opcode))
                {
                    println!("{}()", event);
                } else {
                    println!("{}()", msg.header.opcode);
                }
            }
        }
    } else {
        println!("Header: {:?}", msg.header);
    }
}

fn inspect_core_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_core::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
        Event::Done(v) => println!("{v:?}"),
        Event::Ping(v) => println!("{v:?}"),
        Event::Error(v) => println!("{v:?}"),
        Event::RemoveId(v) => println!("{v:?}"),
        Event::BoundId(v) => println!("{v:?}"),
        Event::AddMem(v) => println!("{v:#?}"),
        Event::RemoveMem(v) => println!("{v:?}"),
        Event::BoundProps(v) => println!("{v:#?}"),
    }
}

fn inspect_client_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_client::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
        Event::Permissions(v) => println!("{v:#?}"),
    }
}

fn inspect_client_node_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_client_node::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Transport(v) => println!("{v:#?}"),
        Event::SetParam(v) => println!("{v:#?}"),
        Event::SetIo(v) => println!("{v:#?}"),
        Event::Event(v) => println!("{v:#?}"),
        Event::Command(v) => println!("{v:#?}"),
        Event::AddPort(v) => println!("{v:#?}"),
        Event::RemovePort(v) => println!("{v:#?}"),
        Event::PortSetParam(v) => println!("{v:#?}"),
        Event::PortUseBuffers(v) => println!("{v:#?}"),
        Event::PortSetIo(v) => println!("{v:#?}"),
        Event::SetActivation(v) => println!("{v:#?}"),
        Event::PortSetMixInfo(v) => println!("{v:#?}"),
    }
}

fn inspect_device_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_device::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
        Event::Param(v) => println!("{v:#?}"),
    }
}

fn inspect_factory_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_factory::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
    }
}

fn inspect_link_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_link::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
    }
}

fn inspect_module_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_module::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
    }
}

fn inspect_node_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_node::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
        Event::Param(v) => println!("{v:#?}"),
    }
}

fn inspect_port_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_port::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Info(v) => println!("{v:#?}"),
        Event::Param(v) => println!("{v:#?}"),
    }
}

fn inspect_registry_event(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_registry::Event;
    let mut pod = msg.body.clone();
    match Event::deserialize(opcode, &mut pod, &msg.fds).unwrap() {
        Event::Global(v) => println!("{v:#?}"),
        Event::GlobalRemove(v) => println!("{v:?}"),
    }
}

fn inspect_client_node_method(opcode: u8, msg: &Message) {
    use ripewire::protocol::pw_client_node::methods;
    let mut pod = msg.body.clone();
    match opcode {
        2 => {
            let msg = methods::Update::deserialize(&mut pod).unwrap();
            print!("{msg:#?}");
        }
        3 => {
            let msg = methods::PortUpdate::deserialize(&mut pod).unwrap();
            print!("{msg:#?}");
        }
        _ => {}
    }
}

type Methods = HashMap<u8, &'static str>;
type Events = HashMap<u8, &'static str>;
type Interfaces = HashMap<ObjectType, (Methods, Events)>;
type Objects = HashMap<u32, ObjectType>;

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
        (ObjectType::Core, pw_core()),
        (ObjectType::Registry, pw_registry()),
        (ObjectType::Client, pw_client()),
        (ObjectType::ClientNode, pw_client_node()),
    ])
}

fn objects() -> Objects {
    HashMap::from([
        (0, ObjectType::Core),
        (1, ObjectType::Client),
        //
    ])
}
