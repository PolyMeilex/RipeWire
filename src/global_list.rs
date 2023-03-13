use crate::protocol;

#[derive(Debug, Default)]
pub struct GlobalList {
    pub globals: Vec<protocol::pw_registry::event::Global>,
}

impl GlobalList {
    pub fn handle_event(&mut self, event: &protocol::pw_registry::Event) {
        match event {
            protocol::pw_registry::Event::Global(global) => {
                self.globals.push(global.clone());
            }
            protocol::pw_registry::Event::GlobalRemove(_) => {
                //
            }
        }
    }

    pub fn len(&self) -> usize {
        self.globals.len()
    }
}
