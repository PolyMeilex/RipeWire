use pod::dictionary::Dictionary;
use std::os::fd::AsRawFd;

use ripewire::{
    connection::MessageBuffer, context::Context, global_list::GlobalList, object_map::ObjectType,
    protocol::pw_core, proxy::PwDevice,
};

struct State {
    globals: GlobalList,
    is_done: bool,
}

fn main() {
    let mut ctx = Context::<State>::connect("/run/user/1000/pipewire-0").unwrap();
    ripewire::set_blocking(ctx.as_raw_fd(), true);

    let core = ctx.core();
    let client = ctx.client();

    core.hello(&mut ctx);
    client.update_properties(
        &mut ctx,
        Dictionary::from([
            ("application.name", "ripewire"),
            ("application.process.binary", "ripewire"),
        ]),
    );

    let registry = core.get_registry(&mut ctx);

    core.sync(&mut ctx, 0, 0);

    let mut state = State {
        globals: GlobalList::default(),
        is_done: false,
    };

    ctx.set_object_callback(&registry, |state, _ctx, _registry, event| {
        state.globals.handle_event(&event);
    });
    ctx.set_object_callback(&core, |state, ctx, core, event| match event {
        pw_core::Event::Done(done) if done.id == Some(0) && done.seq == 0 => {
            state.is_done = true;
        }
        pw_core::Event::Ping(ping) => {
            core.pong(ctx, ping.id, ping.seq);
        }
        _ => {}
    });

    let mut buffer = MessageBuffer::new();
    loop {
        buffer.clear();
        let (messages, fds) = ctx.rcv_msg(&mut buffer).unwrap();
        for msg in messages {
            ctx.dispatch_event(&mut state, msg, &fds);
        }

        if state.is_done {
            break;
        }
    }

    let device = state
        .globals
        .iter()
        .filter(|g| g.interface == ObjectType::Device)
        .find(|g| {
            g.properties.get("device.name").map(String::as_str)
                == Some("alsa_card.pci-0000_0b_00.6")
        });

    if let Some(global) = device {
        let device: PwDevice = registry.bind(&mut ctx, global);

        device.set_param(
            &mut ctx,
            pod::params::RouteParamBuilder::route()
                .index(4)
                .device(4)
                .props(
                    pod::props::PropsBuilder::new()
                        .mute(false)
                        .volume(0.5)
                        .build(),
                )
                .build(),
        );

        state.is_done = true;
    }
}
