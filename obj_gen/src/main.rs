use libspa_consts::{
    SpaEnum, SpaFormat, SpaParamBuffers, SpaParamIo, SpaParamLatency, SpaParamMeta,
    SpaParamPortConfig, SpaParamProcessLatency, SpaParamProfile, SpaParamRoute, SpaParamTag,
    SpaProfiler, SpaProp, SpaPropInfo, SpaType,
};

use proc_macro2::TokenStream;
use quote::{format_ident, quote};

mod json;
mod typed;
mod untyped;

fn get_key_enum_type(object_ty: SpaType) -> Option<TokenStream> {
    let out = match object_ty {
        SpaType::ObjectPropInfo => {
            quote!(SpaPropInfo)
        }
        SpaType::ObjectProps => quote!(SpaProp),
        SpaType::ObjectFormat => quote!(SpaFormat),
        SpaType::ObjectParamBuffers => quote!(SpaParamBuffers),
        SpaType::ObjectParamMeta => quote!(SpaParamMeta),
        SpaType::ObjectParamIo => quote!(SpaParamIo),
        SpaType::ObjectParamProfile => quote!(SpaParamProfile),
        SpaType::ObjectParamPortConfig => quote!(SpaParamPortConfig),
        SpaType::ObjectParamRoute => quote!(SpaParamRoute),
        SpaType::ObjectProfiler => quote!(SpaProfiler),
        SpaType::ObjectParamLatency => quote!(SpaParamLatency),
        SpaType::ObjectParamProcessLatency => {
            quote!(SpaParamProcessLatency)
        }
        SpaType::ObjectParamParamTag => quote!(SpaParamTag),
        _ => return None,
    };

    Some(out)
}

fn get_key_enum(object_ty: SpaType, key: u32) -> Option<TokenStream> {
    macro_rules! get_key {
        ($type: ident) => {
            if let SpaEnum::Value(key) = SpaEnum::<$type>::from_raw(key) {
                let key = format_ident!("{key:?}");
                quote!($type::#key)
            } else {
                return None;
            }
        };
    }

    let out = match object_ty {
        SpaType::ObjectPropInfo => {
            get_key!(SpaPropInfo)
        }
        SpaType::ObjectProps => get_key!(SpaProp),
        SpaType::ObjectFormat => get_key!(SpaFormat),
        SpaType::ObjectParamBuffers => get_key!(SpaParamBuffers),
        SpaType::ObjectParamMeta => get_key!(SpaParamMeta),
        SpaType::ObjectParamIo => get_key!(SpaParamIo),
        SpaType::ObjectParamProfile => get_key!(SpaParamProfile),
        SpaType::ObjectParamPortConfig => get_key!(SpaParamPortConfig),
        SpaType::ObjectParamRoute => get_key!(SpaParamRoute),
        SpaType::ObjectProfiler => get_key!(SpaProfiler),
        SpaType::ObjectParamLatency => get_key!(SpaParamLatency),
        SpaType::ObjectParamProcessLatency => {
            get_key!(SpaParamProcessLatency)
        }
        SpaType::ObjectParamParamTag => get_key!(SpaParamTag),
        _ => return None,
    };

    Some(out)
}

fn spa_strip_parent_name<'a>(parent: &str, field: &'a str) -> Option<&'a str> {
    let out = field.strip_prefix(parent)?;
    if out.is_empty() {
        return None;
    }

    let out = &out[1..];
    if out.is_empty() {
        return None;
    }

    Some(out)
}

fn spa_short_name(v: &str) -> Option<&str> {
    let pos = &v.rfind(':').unwrap();
    let out = v[*pos + 1..].trim_end();

    if out.is_empty() {
        return None;
    }

    Some(out)
}

struct DisplayToIdent<T>(T);
impl<T: std::fmt::Display> quote::IdentFragment for DisplayToIdent<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        self.0.fmt(f)
    }
}

fn snake_case(v: &str) -> impl quote::IdentFragment + '_ {
    DisplayToIdent(heck::AsSnakeCase(v))
}

fn camel_case(v: &str) -> impl quote::IdentFragment + '_ {
    DisplayToIdent(heck::AsUpperCamelCase(v))
}

fn main() {
    // typed::run();
    untyped::run();
}
