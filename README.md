<p align="center">
 <img src="https://i.imgur.com/qk7jHzB.png" width="200" />
 <h1 align="center">RipeWire</h1>
 <h3 align="center">PipeWire client library in idiomatic Rust</h3>
</p>

## WIP...

Simple proof of concept (definitely not final API): 
```rs
#[derive(Default)]
struct State {
    globals: GlobalList,
}

impl State {
    pub fn core_event(
        &mut self,
        context: &mut Context<Self>,
        core: PwCore,
        event: pw_core::Event,
    ) {
        match event {
            pw_core::Event::Done(done) => {
                if done.id == 0 && done.seq == 0 {
                    // Let's print globals advertised by the registry
                    dbg!(&self.globals)
                }
            }
            _ => {}
        }
    }

    pub fn registry_event(
        &mut self,
        context: &mut Context<Self>,
        registry: PwRegistry,
        event: pw_registry::Event,
    ) {
        self.globals.handle_event(&event);
    }
}

let mut ctx = Context::connect("/run/user/1000/pipewire-0").unwrap();
let core = ctx.core();
let client = ctx.client();

core.hello(&mut ctx);

client.update_properties(
    &mut ctx,
    Dictionary::from([
        ("application.name", "RipeWire"),
        ("application.process.binary", "ripewire"),
    ]),
);

let registry = core.get_registry(&mut ctx);

core.sync(&mut ctx, 0, 0);

ctx.set_object_callback(&core, State::core_event);
ctx.set_object_callback(&registry, State::registry_event);

ctx.set_object_callback(&client, |state, client, event| {
    println!("You can use closures as well: {:?}", event);
});

let mut state = State::default();

loop { // any event loop or async runtime of your choice
    let (messages, fds) = ctx.rcv_msg();

    for msg in messages {
        ctx.dispatch_event(&mut state, msg, &fds);
    }
}
```
