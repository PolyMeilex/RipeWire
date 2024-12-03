//! Pipewire objects map

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Client,
    ClientEndpoint,
    ClientNode,
    ClientSession,
    Core,
    Device,
    Endpoint,
    EndpointLink,
    EndpointStream,
    Factory,
    Link,
    Metadata,
    Module,
    Node,
    Port,
    Profiler,
    Registry,
    Session,
    Other(String),
}

impl ObjectType {
    pub fn from_interface_name(name: &str) -> Self {
        match name {
            "PipeWire:Interface:Client" => Self::Client,
            "PipeWire:Interface:ClientEndpoint" => Self::ClientEndpoint,
            "PipeWire:Interface:ClientNode" => Self::ClientNode,
            "PipeWire:Interface:ClientSession" => Self::ClientSession,
            "PipeWire:Interface:Core" => Self::Core,
            "PipeWire:Interface:Device" => Self::Device,
            "PipeWire:Interface:Endpoint" => Self::Endpoint,
            "PipeWire:Interface:EndpointLink" => Self::EndpointLink,
            "PipeWire:Interface:EndpointStream" => Self::EndpointStream,
            "PipeWire:Interface:Factory" => Self::Factory,
            "PipeWire:Interface:Link" => Self::Link,
            "PipeWire:Interface:Metadata" => Self::Metadata,
            "PipeWire:Interface:Module" => Self::Module,
            "PipeWire:Interface:Node" => Self::Node,
            "PipeWire:Interface:Port" => Self::Port,
            "PipeWire:Interface:Profiler" => Self::Profiler,
            "PipeWire:Interface:Registry" => Self::Registry,
            "PipeWire:Interface:Session" => Self::Session,
            _ => Self::Other(name.to_string()),
        }
    }

    pub fn as_interface_name(&self) -> &str {
        match self {
            Self::Client => "PipeWire:Interface:Client",
            Self::ClientEndpoint => "PipeWire:Interface:ClientEndpoint",
            Self::ClientNode => "PipeWire:Interface:ClientNode",
            Self::ClientSession => "PipeWire:Interface:ClientSession",
            Self::Core => "PipeWire:Interface:Core",
            Self::Device => "PipeWire:Interface:Device",
            Self::Endpoint => "PipeWire:Interface:Endpoint",
            Self::EndpointLink => "PipeWire:Interface:EndpointLink",
            Self::EndpointStream => "PipeWire:Interface:EndpointStream",
            Self::Factory => "PipeWire:Interface:Factory",
            Self::Link => "PipeWire:Interface:Link",
            Self::Metadata => "PipeWire:Interface:Metadata",
            Self::Module => "PipeWire:Interface:Module",
            Self::Node => "PipeWire:Interface:Node",
            Self::Port => "PipeWire:Interface:Port",
            Self::Profiler => "PipeWire:Interface:Profiler",
            Self::Registry => "PipeWire:Interface:Registry",
            Self::Session => "PipeWire:Interface:Session",
            Self::Other(name) => name,
        }
    }
}

/// The representation of a protocol object
#[derive(Debug, Clone)]
pub struct Object<Data> {
    /// Interface name of this object
    pub interface: ObjectType,
    /// Version of this object
    pub version: u32,
    /// ObjectData associated to this object (ex: its event queue client side)
    pub data: Data,
}

#[derive(Debug, Default)]
pub struct SlotMap<T> {
    slots: Vec<Option<T>>,
}

impl<T> SlotMap<T> {
    pub fn new() -> Self {
        Self { slots: Vec::new() }
    }

    pub fn get(&self, id: u32) -> Option<&T> {
        self.slots.get(id as usize).and_then(|obj| obj.as_ref())
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut T> {
        self.slots.get_mut(id as usize).and_then(|obj| obj.as_mut())
    }

    pub fn remove(&mut self, id: u32) {
        if let Some(place) = self.slots.get_mut(id as usize) {
            *place = None;
        }
    }

    pub fn replace(&mut self, id: u32, value: T) {
        let id = id as usize;
        match id.cmp(&self.slots.len()) {
            Ordering::Less => {
                self.slots[id] = Some(value);
            }
            Ordering::Equal => {
                self.slots.push(Some(value));
            }
            Ordering::Greater => {
                self.slots.resize_with(id + 1, || None);
                self.slots[id] = Some(value);
            }
        }
    }

    /// Insert given object for given id
    ///
    /// Can fail if the requested id is not the next free id of this store.
    /// (In which case this is a protocol error)
    pub fn insert_at(&mut self, id: u32, value: T) -> Result<(), ()> {
        let id = id as usize;
        match id.cmp(&self.slots.len()) {
            Ordering::Greater => Err(()),
            Ordering::Equal => {
                self.slots.push(Some(value));
                Ok(())
            }
            Ordering::Less => {
                let previous = &mut self.slots[id];
                if !previous.is_none() {
                    return Err(());
                }
                *previous = Some(value);
                Ok(())
            }
        }
    }

    /// Allocate a new id for an object in the client namespace
    pub fn insert_new(&mut self, value: T) -> u32 {
        match self.slots.iter().position(Option::is_none) {
            Some(id) => {
                self.slots[id] = Some(value);
                id as u32
            }
            None => {
                self.slots.push(Some(value));
                (self.slots.len() - 1) as u32
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (u32, &T)> {
        self.slots
            .iter()
            .enumerate()
            .flat_map(|(idx, obj)| obj.as_ref().map(|obj| (idx as u32, obj)))
    }
}

/// A holder for the object store of a connection
///
/// Keeps track of which object id is associated to which
/// interface object, and which is currently unused.
#[derive(Debug)]
pub struct ObjectMap<Data> {
    objects: SlotMap<Object<Data>>,
}

impl<Data> ObjectMap<Data> {
    /// Create a new empty object map
    pub fn new() -> Self {
        Self {
            objects: SlotMap::new(),
        }
    }

    /// Find an object in the store
    pub fn find(&self, id: u32) -> Option<&Object<Data>> {
        self.objects.get(id)
    }

    /// Find an object in the store
    pub fn find_mut(&mut self, id: u32) -> Option<&mut Object<Data>> {
        self.objects.get_mut(id)
    }

    /// Remove an object from the store
    ///
    /// Does nothing if the object didn't previously exists
    pub fn remove(&mut self, id: u32) {
        self.objects.remove(id)
    }

    /// Allocate a new id for an object in the client namespace
    pub fn insert_new(&mut self, object: Object<Data>) -> u32 {
        self.objects.insert_new(object)
    }

    pub fn all_objects(&self) -> impl Iterator<Item = (u32, &Object<Data>)> {
        self.objects.iter()
    }
}
