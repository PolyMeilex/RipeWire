use std::fmt::Write;

use libspa_consts::SpaType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use syn::Ident;

use crate::{camel_case, json, snake_case};

fn print_entry_builder(entry: &json::Entry, out: &mut impl Write) {
    let ident = format_ident!(
        "{}Builder",
        camel_case(crate::spa_short_name(&entry.name).unwrap())
    );

    let properties: Vec<_> = get_properties(entry);
    let obj_type = SpaType::from_raw(entry.r#type).unwrap();

    let setters = properties.iter().map(|(ident, info)| {
        let doc = get_doc(info);

        let set_call = if let Some(key) = crate::get_key_enum(obj_type, info.r#type) {
            quote!(self.set(#key, value))
        } else {
            let ty = info.r#type;
            quote!(self.set_raw(#ty, value))
        };

        quote! {
            #doc
            fn #ident(&self, value: OwnedPod) {
                #set_call
            }
        }
    });

    let doc = format!(" {}", entry.name);

    let set_raw = quote! {
        fn set_raw(&self, id: u32, value: OwnedPod) {
            todo!("set {id} to {value:?}")
        }
    };

    let set_typed = crate::get_key_enum_type(obj_type).map(|key| {
        quote! {
            fn set(&self, key: #key, value: OwnedPod) {
                self.set_raw(key.to_u32().unwrap(), value)
            }
        }
    });

    let src = quote! {
        #[doc = #doc]
        #[derive(Debug)]
        pub struct #ident<'a>(pub std::marker::PhantomData<&'a ()>);
        impl #ident<'_> {
            #set_raw
            #set_typed

            #(#setters)*
        }
    };

    writeln!(out, "{src}").unwrap();
    writeln!(out).unwrap();
}

fn print_entry(entry: &json::Entry, out: &mut impl Write) {
    let ident = format_ident!(
        "{}",
        camel_case(crate::spa_short_name(&entry.name).unwrap())
    );

    let properties: Vec<_> = get_properties(entry);
    let obj_type = SpaType::from_raw(entry.r#type).unwrap();

    let getters = properties.iter().map(|(ident, info)| {
        let doc = get_doc(info);

        let get_call = if let Some(key) = crate::get_key_enum(obj_type, info.r#type) {
            quote!(self.get(#key))
        } else {
            let ty = info.r#type;
            quote!(self.get_raw(#ty))
        };

        quote! {
            #doc
            fn #ident(&self) -> Option<PodDeserializer<'a>> {
                #get_call
            }
        }
    });

    let doc = format!(" {}", entry.name);

    let get_raw = quote! {
        fn get_raw(&self, id: u32) -> Option<PodDeserializer<'a>> {
            self.0.clone().find(|v| v.key == id).map(|v| v.value)
        }
    };

    let get_typed = crate::get_key_enum_type(obj_type).map(|key| {
        quote! {
            fn get(&self, key: #key) -> Option<PodDeserializer<'a>> {
                self.get_raw(key.to_u32().unwrap())
            }
        }
    });

    let src = quote! {
        #[doc = #doc]
        #[derive(Debug)]
        pub struct #ident<'a>(pub PodObjectDeserializer<'a>);
        impl<'a> #ident<'a> {
            #get_raw
            #get_typed

            #(#getters)*
        }
    };

    writeln!(out, "{src}").unwrap();
    writeln!(out).unwrap();
}

fn get_properties(entry: &json::Entry) -> Vec<(Ident, &json::SpaTypeInfo)> {
    entry
        .properties
        .iter()
        .filter(|info| SpaType::from_raw(info.parent).unwrap() != SpaType::None)
        .filter_map(|info| {
            let ident = crate::spa_strip_parent_name(&entry.name, &info.name)?;
            let ident = if ident == "type" {
                format_ident!("ty")
            } else {
                format_ident!("{}", snake_case(ident))
            };
            Some((ident, info))
        })
        .collect()
}

fn get_doc(info: &json::SpaTypeInfo) -> TokenStream {
    let name_doc = format!(" name: {}", info.name);
    let return_doc = format!(" returns: {:?}", SpaType::from_raw(info.parent).unwrap());

    let values_doc = {
        let values_label_doc = (!info.values.is_empty()).then_some(quote!(#[doc = " values:"]));
        let values_doc = info.values.iter().map(|v| {
            let doc = format!("  {}: {:?}", v.r#type, v.name);
            quote!(#doc)
        });
        quote! {
            #values_label_doc
            #(#[doc = #values_doc])*
        }
    };

    quote! {
        #[doc = #name_doc]
        #[doc = #return_doc]
        #values_doc
    }
}

pub fn run() {
    let path = std::env::args().nth(1).expect("path to json");
    let src = std::fs::read_to_string(path).unwrap();
    let json: Vec<json::Entry> = serde_json::from_str(&src).unwrap();

    println!("// =============");
    println!("// This file is autogenerated by obj_gen");
    println!("// DO NOT EDIT");
    println!("// =============");
    println!();
    println!("use super::*;");
    println!();

    for e in json.iter() {
        let mut code_gen = String::new();
        print_entry(e, &mut code_gen);
        let code_gen = prettyplease::unparse(&syn::parse_str::<syn::File>(&code_gen).unwrap());
        println!("{code_gen}");
        println!();
    }

    println!("pub mod builder {{");
    println!("use super::*;");
    println!();
    for e in json.iter() {
        let mut code_gen = String::new();
        print_entry_builder(e, &mut code_gen);
        let code_gen = prettyplease::unparse(&syn::parse_str::<syn::File>(&code_gen).unwrap());
        println!("{code_gen}");
        println!();
    }
    println!("}}");
}
