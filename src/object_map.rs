//! Pipewire objects map

use std::cmp::Ordering;

/// The representation of a protocol object
#[derive(Debug, Clone)]
pub struct Object<Data> {
    /// Interface name of this object
    pub interface: &'static str,
    /// Version of this object
    pub version: u32,
    /// ObjectData associated to this object (ex: its event queue client side)
    pub data: Data,
}

/// A holder for the object store of a connection
///
/// Keeps track of which object id is associated to which
/// interface object, and which is currently unused.
#[derive(Debug, Default)]
pub struct ObjectMap<Data> {
    objects: Vec<Option<Object<Data>>>,
}

impl<Data: Clone> ObjectMap<Data> {
    /// Create a new empty object map
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// Find an object in the store
    pub fn find(&self, id: u32) -> Option<Object<Data>> {
        if id == 0 {
            None
        } else {
            self.objects.get((id - 1) as usize).and_then(Clone::clone)
        }
    }

    /// Remove an object from the store
    ///
    /// Does nothing if the object didn't previously exists
    pub fn remove(&mut self, id: u32) {
        if id == 0 {
            // nothing
        } else if let Some(place) = self.objects.get_mut((id - 1) as usize) {
            *place = None;
        }
    }

    /// Insert given object for given id
    ///
    /// Can fail if the requested id is not the next free id of this store.
    /// (In which case this is a protocol error)
    pub fn insert_at(&mut self, id: u32, object: Object<Data>) -> Result<(), ()> {
        if id == 0 {
            Err(())
        } else {
            insert_in_at(&mut self.objects, (id - 1) as usize, object)
        }
    }

    /// Allocate a new id for an object in the client namespace
    pub fn client_insert_new(&mut self, object: Object<Data>) -> u32 {
        insert_in(&mut self.objects, object) + 1
    }

    /// Mutably access an object of the map
    pub fn with<T, F: FnOnce(&mut Object<Data>) -> T>(&mut self, id: u32, f: F) -> Result<T, ()> {
        if id == 0 {
            Err(())
        } else if let Some(&mut Some(ref mut obj)) = self.objects.get_mut((id - 1) as usize) {
            Ok(f(obj))
        } else {
            Err(())
        }
    }

    pub fn all_objects(&self) -> impl Iterator<Item = (u32, &Object<Data>)> {
        self.objects
            .iter()
            .enumerate()
            .flat_map(|(idx, obj)| obj.as_ref().map(|obj| (idx as u32 + 1, obj)))
    }
}

// insert a new object in a store at the first free place
fn insert_in<Data>(store: &mut Vec<Option<Object<Data>>>, object: Object<Data>) -> u32 {
    match store.iter().position(Option::is_none) {
        Some(id) => {
            store[id] = Some(object);
            id as u32
        }
        None => {
            store.push(Some(object));
            (store.len() - 1) as u32
        }
    }
}

// insert an object at a given place in a store
fn insert_in_at<Data>(
    store: &mut Vec<Option<Object<Data>>>,
    id: usize,
    object: Object<Data>,
) -> Result<(), ()> {
    match id.cmp(&store.len()) {
        Ordering::Greater => Err(()),
        Ordering::Equal => {
            store.push(Some(object));
            Ok(())
        }
        Ordering::Less => {
            let previous = &mut store[id];
            if !previous.is_none() {
                return Err(());
            }
            *previous = Some(object);
            Ok(())
        }
    }
}
