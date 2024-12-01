use pod::dictionary::Dictionary;
use std::{io::Write as _, os::fd::AsRawFd};

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
        let msg = ctx.rcv_msg(&mut buffer).unwrap();
        ctx.dispatch_event(&mut state, msg);

        if state.is_done {
            break;
        }
    }

    let devices: Vec<_> = state
        .globals
        .iter()
        .filter(|g| g.interface == ObjectType::Device)
        .collect();

    let device = {
        println!("\nAvailable devices:");
        for (i, device) in devices.iter().enumerate() {
            println!(
                "{}: {} - {}",
                i,
                device
                    .properties
                    .get("device.name")
                    .map(String::as_str)
                    .unwrap_or("-"),
                device
                    .properties
                    .get("device.description")
                    .map(String::as_str)
                    .unwrap_or("-")
            );
        }
        print!("Please select input port: ");
        std::io::stdout().flush().unwrap();
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        Some(
            devices
                .get(input.trim().parse::<usize>().unwrap())
                .ok_or("invalid input port selected")
                .unwrap(),
        )
    };

    // let device = state
    //     .globals
    //     .iter()
    //     .filter(|g| g.interface == ObjectType::Device)
    //     .find(|g| {
    //         g.properties.get("device.name").map(String::as_str)
    //             == Some("alsa_card.pci-0000_0b_00.6")
    //     });

    if let Some(global) = device {
        let device: PwDevice = registry.bind(&mut ctx, global);

        // device.e
        ctx.set_object_callback(&device, |_state, _ctx, _device, event| {
            dbg!(event);
        });

        // device.enum_param(&mut ctx, SpaParamType::Route);
        //
        // state.is_done = false;
        // loop {
        //     let msg = ctx.rcv_msg(&mut buffer).unwrap();
        //     ctx.dispatch_event(&mut state, msg);
        //
        //     if state.is_done {
        //         break;
        //     }
        // }

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
