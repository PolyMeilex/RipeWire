// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use bindgen::callbacks::EnumVariantCustomBehavior;

fn main() {
    let libs = system_deps::Config::new()
        .probe()
        .expect("Cannot find libraries");

    run_bindgen(&libs);
}

#[derive(Debug)]
struct ParseCallbacks;

impl bindgen::callbacks::ParseCallbacks for ParseCallbacks {
    fn item_name(&self, original_item_name: &str) -> Option<String> {
        Some(heck::AsUpperCamelCase(original_item_name).to_string())
    }

    fn add_derives(&self, info: &bindgen::callbacks::DeriveInfo<'_>) -> Vec<String> {
        match info.kind {
            bindgen::callbacks::TypeKind::Enum => {
                vec![
                    "num_derive::FromPrimitive".into(),
                    "num_derive::ToPrimitive".into(),
                ]
            }
            _ => vec![],
        }
    }

    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        if original_variant_name == "_SPA_DATA_LAST"
            || (original_variant_name.starts_with("SPA_PROP_")
                && original_variant_name.contains("_START"))
        {
            Some(EnumVariantCustomBehavior::Hide)
        } else {
            None
        }
    }

    fn enum_variant_name(
        &self,
        enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<String> {
        let prefix = if let Some(enum_name) = enum_name?.strip_prefix("enum ") {
            match enum_name {
                "spa_choice_type" | "spa_data_type" | "spa_param_type" | "spa_io_type"
                | "spa_control_type" | "spa_meta_type" => &format!(
                    "{}_",
                    heck::AsShoutySnekCase(enum_name.strip_suffix("_type").unwrap())
                ),
                _ => &format!("{}_", heck::AsShoutySnekCase(enum_name)),
            }
        } else {
            return None;
        };

        Some(heck::AsUpperCamelCase(original_variant_name.strip_prefix(prefix)?).to_string())
    }
}

fn run_bindgen(libs: &system_deps::Dependencies) {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .parse_callbacks(Box::new(ParseCallbacks))
        // Use `usize` for `size_t`. This behavior of bindgen changed because it is not
        // *technically* correct, but is the case in all architectures supported by Rust.
        .size_t_is_usize(true)
        .ignore_functions()
        .ignore_methods()
        .prepend_enum_name(false)
        .layout_tests(false)
        .derive_eq(true);

    for name in [
        "spa_rectangle",
        "spa_fraction",
        "spa_io_video_size",
        "spa_io_clock",
        "spa_io_segment_bar",
        "spa_io_segment_video",
        "spa_io_segment",
        "spa_io_position",
    ] {
        builder = builder.allowlist_type(name);
    }

    for name in [
        "spa_choice_type",
        "spa_format",
        "spa_prop",
        "spa_prop_info",
        "spa_data_type",
        "spa_param_route",
        "spa_param_type",
        "spa_param_io",
        "spa_io_type",
        "spa_media_type",
        "spa_media_subtype",
        "spa_direction",
        "spa_param_availability",
        "spa_control_type",
        "spa_meta_type",
        "spa_param_tag",
        "spa_param_buffers",
        "spa_param_meta",
        "spa_param_profile",
        "spa_param_port_config",
        "spa_profiler",
        "spa_param_latency",
        "spa_param_process_latency",
        "pw_link_state",
        "pw_node_state",
        "spa_param_port_config_mode",
        "spa_bluetooth_audio_codec",
        "spa_audio_format",
        "spa_audio_iec958_codec",
        "spa_video_format",
        "spa_video_interlace_mode",
        "spa_video_multiview_mode",
        "spa_video_multiview_flags",
    ] {
        builder = builder.allowlist_type(name).rustified_enum(name);
    }

    builder = builder.bitfield_enum("spa_video_multiview_flags");

    let builder = libs
        .iter()
        .iter()
        .flat_map(|(_, lib)| lib.include_paths.iter())
        .fold(builder, |builder, l| {
            let arg = format!("-I{}", l.to_string_lossy());
            builder.clang_arg(arg)
        });

    let bindings = builder.generate().expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let out_path = PathBuf::from("./src/gen/");
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
