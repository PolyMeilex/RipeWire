pub trait HasOpCode {
    const OPCODE: u8;
}

pub mod pw_client;
pub mod pw_client_node;
pub mod pw_core;
pub mod pw_device;
pub mod pw_factory;
pub mod pw_link;
pub mod pw_module;
pub mod pw_node;
pub mod pw_port;
pub mod pw_registry;
