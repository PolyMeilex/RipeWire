// Copyright The pipewire-rs Contributors.
// SPDX-License-Identifier: MIT

use std::path::PathBuf;

use bindgen::callbacks::EnumVariantCustomBehavior;
use convert_case::Casing;

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
        match original_item_name {
            "spa_choice_type" => Some("SpaChoiceType".to_string()),
            "spa_prop" => Some("SpaProp".to_string()),
            "spa_data_type" => Some("SpaDataType".to_string()),
            "spa_param_route" => Some("SpaParamRoute".to_string()),
            "spa_param_type" => Some("SpaParamType".to_string()),
            "spa_io_type" => Some("SpaIoType".to_string()),
            "spa_param_io" => Some("SpaParamIo".to_string()),
            "spa_media_type" => Some("SpaMediaType".to_string()),
            "spa_media_subtype" => Some("SpaMediaSubtype".to_string()),
            "spa_format" => Some("SpaFormat".to_string()),
            _ => None,
        }
    }

    fn enum_variant_behavior(
        &self,
        _enum_name: Option<&str>,
        original_variant_name: &str,
        _variant_value: bindgen::callbacks::EnumVariantValue,
    ) -> Option<EnumVariantCustomBehavior> {
        if original_variant_name == "_SPA_DATA_LAST" {
            Some(EnumVariantCustomBehavior::Hide)
        } else if original_variant_name.starts_with("SPA_PROP_") {
            if original_variant_name.contains("_START") {
                Some(EnumVariantCustomBehavior::Hide)
            } else {
                None
            }
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
        let Some(enum_name) = enum_name else {
            return None;
        };

        let variant = match enum_name {
            "enum spa_choice_type" => original_variant_name.strip_prefix("SPA_CHOICE_"),
            "enum spa_prop" => original_variant_name.strip_prefix("SPA_PROP_"),
            "enum spa_data_type" => original_variant_name.strip_prefix("SPA_DATA_"),
            "enum spa_param_route" => original_variant_name.strip_prefix("SPA_PARAM_ROUTE_"),
            "enum spa_param_type" => original_variant_name.strip_prefix("SPA_PARAM_"),
            "enum spa_param_io" => original_variant_name.strip_prefix("SPA_PARAM_IO_"),
            "enum spa_io_type" => original_variant_name.strip_prefix("SPA_IO_"),
            "enum spa_media_type" => original_variant_name.strip_prefix("SPA_MEDIA_TYPE_"),
            "enum spa_media_subtype" => original_variant_name.strip_prefix("SPA_MEDIA_SUBTYPE_"),
            "enum spa_format" => original_variant_name.strip_prefix("SPA_FORMAT_"),
            _ => None,
        };

        variant.map(|v| v.to_case(convert_case::Case::UpperCamel))
    }
}

fn run_bindgen(libs: &system_deps::Dependencies) {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    let builder = bindgen::Builder::default()
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Use `usize` for `size_t`. This behavior of bindgen changed because it is not
        // *technically* correct, but is the case in all architectures supported by Rust.
        .size_t_is_usize(true)
        .ignore_functions()
        .ignore_methods()
        .allowlist_type("spa_choice_type")
        .rustified_enum("spa_choice_type")
        .allowlist_type("spa_pod")
        .allowlist_type("spa_format")
        .rustified_enum("spa_format")
        .allowlist_type("spa_prop")
        .rustified_enum("spa_prop")
        .allowlist_type("spa_data_type")
        .rustified_enum("spa_data_type")
        .allowlist_type("spa_param_route")
        .rustified_enum("spa_param_route")
        .allowlist_type("spa_param_type")
        .rustified_enum("spa_param_type")
        .allowlist_type("spa_param_io")
        .rustified_enum("spa_param_io")
        .allowlist_type("spa_io_type")
        .rustified_enum("spa_io_type")
        .allowlist_type("spa_media_type")
        .rustified_enum("spa_media_type")
        .allowlist_type("spa_media_subtype")
        .rustified_enum("spa_media_subtype")
        .allowlist_var("SPA_POD_PROP_FLAG.*")
        .allowlist_var("SPA_TYPE_.*")
        .blocklist_item("SPA_TYPE_INFO_.*")
        .blocklist_item("SPA_TYPE_INTERFACE_.*")
        .parse_callbacks(Box::new(ParseCallbacks))
        .prepend_enum_name(false)
        .layout_tests(false)
        .derive_eq(true);

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
