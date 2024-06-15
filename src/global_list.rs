use crate::protocol::pw_registry;

#[derive(Debug, Default)]
pub struct GlobalList {
    globals: Vec<pw_registry::events::Global>,
}

impl GlobalList {
    pub fn handle_event(&mut self, event: &pw_registry::Event) {
        match event {
            pw_registry::Event::Global(global) => {
                self.globals.push(global.clone());
            }
            pw_registry::Event::GlobalRemove(_) => {
                //
            }
        }
    }

    pub fn len(&self) -> usize {
        self.globals.len()
    }

    pub fn is_empty(&self) -> bool {
        self.globals.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &pw_registry::events::Global> {
        self.globals.iter()
    }
}
