// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use bindgen::callbacks::EnumVariantCustomBehavior;
use convert_case::{Case, Casing};

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
        let rename = [
            "spa_pod",
            "spa_choice_type",
            "spa_prop",
            "spa_data_type",
            "spa_param_route",
            "spa_param_type",
            "spa_io_type",
            "spa_param_io",
            "spa_media_type",
            "spa_media_subtype",
            "spa_format",
            "spa_rectangle",
            "spa_fraction",
            "spa_direction",
            "spa_param_availability",
            "spa_control_type",
            "spa_meta_type",
            "pw_link_state",
            "pw_node_state",
        ];

        if rename.contains(&original_item_name) {
            Some(original_item_name.to_case(Case::UpperCamel))
        } else {
            None
        }
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
        let prefix = match enum_name? {
            "enum spa_choice_type" => "SPA_CHOICE_",
            "enum spa_prop" => "SPA_PROP_",
            "enum spa_data_type" => "SPA_DATA_",
            "enum spa_param_route" => "SPA_PARAM_ROUTE_",
            "enum spa_param_type" => "SPA_PARAM_",
            "enum spa_param_io" => "SPA_PARAM_IO_",
            "enum spa_io_type" => "SPA_IO_",
            "enum spa_media_type" => "SPA_MEDIA_TYPE_",
            "enum spa_media_subtype" => "SPA_MEDIA_SUBTYPE_",
            "enum spa_format" => "SPA_FORMAT_",
            "enum spa_direction" => "SPA_DIRECTION_",
            "enum spa_param_availability" => "SPA_PARAM_AVAILABILITY_",
            "enum spa_control_type" => "SPA_CONTROL_",
            "enum spa_meta_type" => "SPA_META_",
            "enum pw_link_state" => "PW_LINK_STATE_",
            "enum pw_node_state" => "PW_NODE_STATE_",
            _ => return None,
        };

        Some(
            original_variant_name
                .strip_prefix(prefix)?
                .to_case(Case::UpperCamel),
        )
    }
}

fn run_bindgen(libs: &system_deps::Dependencies) {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let mut builder = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .parse_callbacks(Box::new(ParseCallbacks))
        // Use `usize` for `size_t`. This behavior of bindgen changed because it is not
        // *technically* correct, but is the case in all architectures supported by Rust.
        .size_t_is_usize(true)
        .ignore_functions()
        .ignore_methods()
        .allowlist_type("spa_pod")
        .allowlist_type("spa_rectangle")
        .allowlist_type("spa_fraction")
        .prepend_enum_name(false)
        .layout_tests(false)
        .derive_eq(true);

    for name in [
        "spa_choice_type",
        "spa_format",
        "spa_prop",
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
        "pw_link_state",
        "pw_node_state",
    ] {
        builder = builder.allowlist_type(name).rustified_enum(name);
    }

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
