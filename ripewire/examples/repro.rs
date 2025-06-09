use std::os::fd::AsRawFd;

use libspa_consts::SpaParamType;
use ripewire::{
    connection::MessageBuffer,
    context::Context,
    global_list::GlobalList,
    protocol::{
        pw_client_node::{
            self,
            methods::{NodeFlags, NodeInfo, NodeInfoChangeMask, UpdateChangeMask},
        },
        pw_core, ParamFlags, ParamInfo, PwDictionary,
    },
    proxy::PwClientNode,
    HashMapExt,
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

    // line:1
    core.hello(&mut ctx);

    // line:2
    client.update_properties(
        &mut ctx,
        PwDictionary::from_arr([
            "application.name",
            "pipewire_playground",
            "application.process.binary",
            "pipewire_playground",
            "application.language",
            "en_US.UTF-8",
            "application.process.id",
            "206854",
            "application.process.user",
            "poly",
            "application.process.host",
            "fedora",
            "window.x11.display",
            ":0",
            "log.level",
            "0",
            "cpu.max-align",
            "64",
            "default.clock.rate",
            "48000",
            "default.clock.quantum",
            "1024",
            "default.clock.min-quantum",
            "32",
            "default.clock.max-quantum",
            "2048",
            "default.clock.quantum-limit",
            "8192",
            "default.clock.quantum-floor",
            "4",
            "default.video.width",
            "640",
            "default.video.height",
            "480",
            "default.video.rate.num",
            "25",
            "default.video.rate.denom",
            "1",
            "clock.power-of-two-quantum",
            "true",
            "link.max-buffers",
            "64",
            "mem.warn-mlock",
            "false",
            "mem.allow-mlock",
            "true",
            "settings.check-quantum",
            "false",
            "settings.check-rate",
            "false",
            "core.version",
            "1.4.4",
            "core.name",
            "pipewire-poly-206854",
        ]),
    );

    // line:61
    let node: PwClientNode = core.create_object(
        &mut ctx,
        pw_core::methods::CreateObject {
            factory_name: "client-node".into(),
            interface: "PipeWire:Interface:ClientNode".into(),
            version: 6,
            properties: PwDictionary::from_arr([
                "media.type",
                "Midi",
                "media.category",
                "Filter",
                "media.role",
                "DSP",
                "media.name",
                "midi-dump",
                "node.name",
                "pipewire_playground",
                "node.want-driver",
                "true",
            ]),
            new_id: 0,
        },
    );

    // line:62
    node.send(
        &mut ctx,
        pw_client_node::methods::Update {
            // line:63
            change_mask: UpdateChangeMask::PARAMS | UpdateChangeMask::INFO,
            params: vec![],
            info: Some(NodeInfo {
                max_input_ports: 4294967295,
                max_output_ports: 4294967295,
                change_mask: NodeInfoChangeMask::FLAGS
                    | NodeInfoChangeMask::PROPS
                    | NodeInfoChangeMask::PARAMS,
                flags: NodeFlags::RT,
                // line:77
                props: PwDictionary::from_dict([
                    ("media.category", "Filter"),
                    ("media.name", "midi-dump"),
                    ("media.role", "DSP"),
                    ("media.type", "Midi"),
                    ("node.name", "pipewire_playground"),
                    ("node.want-driver", "true"),
                ]),
                // line:85
                params: vec![
                    ParamInfo {
                        id: SpaParamType::PropInfo.into(),
                        flags: ParamFlags::empty(),
                    },
                    ParamInfo {
                        id: SpaParamType::Props.into(),
                        flags: ParamFlags::WRITE,
                    },
                    ParamInfo {
                        id: SpaParamType::ProcessLatency.into(),
                        flags: ParamFlags::empty(),
                    },
                    ParamInfo {
                        id: SpaParamType::EnumFormat.into(),
                        flags: ParamFlags::empty(),
                    },
                    ParamInfo {
                        id: SpaParamType::Format.into(),
                        flags: ParamFlags::WRITE,
                    },
                ],
            }),
        },
    );
}
