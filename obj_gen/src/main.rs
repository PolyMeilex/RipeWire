use std::fmt::Write;

use libspa_consts::SpaType;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

#[derive(Debug, serde::Deserialize)]
struct SpaTypeInfo {
    r#type: u32,
    parent: u32,
    name: String,
    #[allow(unused)]
    #[serde(default)]
    values: Vec<SpaTypeInfo>,
}

#[derive(Debug, serde::Deserialize)]
struct Entry {
    name: String,
    properties: Vec<SpaTypeInfo>,
}

impl Entry {
    fn print(&self, out: &mut impl Write) {
        let ident = format_ident!("{}", camel_case(spa_short_name(&self.name).unwrap()));

        let props = self.properties.iter().filter_map(|info| {
            let ident = spa_strip_parent_name(&self.name, &info.name)?;
            let ident = if ident == "type" {
                format_ident!("ty")
            } else {
                format_ident!("{}", snake_case(ident))
            };

            let spa_type = SpaType::from_raw(info.parent).unwrap();

            if spa_type == SpaType::None {
                return None;
            }

            let ty = info.r#type;

            let rs_spa_type = spa_type_to_rs(spa_type);
            let get = if let Some(call) = spa_type_to_as_call(spa_type) {
                quote! {
                    self.get(#ty)?.#call
                }
            } else {
                quote! {
                    self.get(#ty)
                }
            };

            let doc = spa_type_doc(spa_type, info);

            let src = quote! {
                #doc
                fn #ident(&self) -> Option<#rs_spa_type> {
                    #get
                }
            };

            Some(src)
        });

        let doc = format!(" {}", self.name);

        let src = quote! {
            #[doc = #doc]
            struct #ident<'a>(PodObjectDeserializer<'a>);
            impl<'a> #ident<'a> {
                fn get(&self, id: u32) -> Option<PodDeserializer> {
                    todo!("{id}")
                }

                #(#props)*
            }
        };

        writeln!(out, "{src}").unwrap();
        writeln!(out).unwrap();
    }
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

fn spa_type_doc(parent: SpaType, info: &SpaTypeInfo) -> TokenStream {
    let res = spa_type_to_rs(parent).to_string();

    let mut doc = " ".to_string();

    if parent == SpaType::Array {
        doc += &info.name;
        doc += "\n";
        doc += &format!("    parent: Array<{}>", info.values[0].name);
        doc += "\n";

        return quote! {
            #[doc = #doc]
        };
    } else if res == "PodDeserializer" || res == "PodObjectDeserializer" {
        doc += &info.name;
        doc += "\n";
        doc += &format!("    parent: {:?}", parent);

        if info.values.is_empty() {
            doc += "\n";
        }
    } else {
        doc += &info.name;
    }

    if !info.values.is_empty() && (res == "PodDeserializer" || parent == SpaType::Id) {
        doc += "\n";
        for v in info.values.iter() {
            doc += &format!("    value-{}: {:?}\n", v.r#type, v.name);
        }
    }

    quote! {
        #[doc = #doc]
    }
}

fn spa_type_to_as_call(parent: SpaType) -> Option<TokenStream> {
    let out = match parent {
        SpaType::Bool => quote!(as_bool()),
        SpaType::Id => quote!(as_id()),
        SpaType::Int => quote!(as_i32()),
        SpaType::Long => quote!(as_i64()),
        SpaType::Float => quote!(as_f32()),
        SpaType::Double => quote!(as_f64()),
        SpaType::String => quote!(as_str()),
        SpaType::Rectangle => quote!(as_rectangle()),
        SpaType::Fraction => quote!(as_fraction()),
        SpaType::Fd => quote!(as_fd()),
        SpaType::Array => quote!(as_array()),
        SpaType::Struct => quote!(as_struct()),
        SpaType::Object => quote!(as_object()),
        SpaType::Sequence => quote!(as_sequence()),

        SpaType::ObjectFormat => {
            quote! {
                as_object().map(Format)
            }
        }
        SpaType::ObjectProps => {
            quote! {
                as_object().map(Props)
            }
        }
        _ => return None,
    };

    Some(quote!(#out.ok()))
}

fn spa_type_to_rs(parent: SpaType) -> TokenStream {
    match parent {
        SpaType::Bool => quote!(bool),
        SpaType::Id => quote!(u32),
        SpaType::Int => quote!(i32),
        SpaType::Long => quote!(i64),
        SpaType::Float => quote!(f32),
        SpaType::Double => quote!(f64),
        SpaType::String => quote!(&BStr),
        SpaType::Rectangle => quote!(SpaRectangle),
        SpaType::Fraction => quote!(SpaFraction),
        SpaType::Fd => quote!(i64),

        SpaType::Array => quote!(PodArrayDeserializer),
        SpaType::Struct => quote!(PodStructDeserializer),
        SpaType::Object => quote!(PodObjectDeserializer),
        SpaType::Sequence => quote!(PodSequenceDeserializer),
        SpaType::Pod => quote!(PodDeserializer),
        SpaType::ObjectFormat => quote!(Format),
        SpaType::ObjectProps => quote!(Props),

        todo => todo!("{todo:?} unsupported"),
    }
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

fn main() {
    let path = std::env::args().nth(1).expect("path to json");
    let src = std::fs::read_to_string(path).unwrap();
    let json: Vec<Entry> = serde_json::from_str(&src).unwrap();

    let mut out = String::new();

    out += "use super::*;";
    out += "\n";

    for e in json {
        e.print(&mut out);
    }

    let out = prettyplease::unparse(&syn::parse_str::<syn::File>(&out).unwrap());
    println!("{out}");
}
