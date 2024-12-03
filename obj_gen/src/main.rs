use std::fmt::Write;

use libspa_consts::{SpaEnum, SpaType};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

static ID_TO_ENUM_MAP: &[(&str, &str)] = &[
    ("Spa:Pod:Object:Param:Props", "SpaProp"),
    ("Spa:Enum:Direction", "SpaDirection"),
    ("Spa:Enum:ParamPortConfigMode", "SpaParamPortConfigMode"),
    ("Spa:Enum:BluetoothAudioCodec", "SpaBluetoothAudioCodec"),
    ("Spa:Enum:MediaType", "SpaMediaType"),
    ("Spa:Enum:MediaSubtype", "SpaMediaSubtype"),
    ("Spa:Enum:AudioFormat", "SpaAudioFormat"),
    ("Spa:Enum:AudioIEC958Codec", "SpaAudioIec958Codec"),
    ("Spa:Enum:VideoFormat", "SpaVideoFormat"),
    ("Spa:Enum:VideoInterlaceMode", "SpaVideoInterlaceMode"),
    ("Spa:Enum:AudioAMRBandMode", "SpaAudioAmrBandMode"),
    ("Spa:Enum:AudioWMAProfile", "SpaAudioWmaProfile"),
    ("Spa:Enum:AudioAACStreamFormat", "SpaAudioAacStreamFormat"),
    ("Spa:Enum:ParamBitorder", "SpaParamBitorder"),
];

static PROP_NAME_TYPE_OVERRIDE: &[(&str, SpaType)] = &[
    // Sometimes Choice sometimes Id
    ("Spa:Pod:Object:Param:Format:Audio:format", SpaType::Pod),
    // Sometimes Choice sometimes Int
    ("Spa:Pod:Object:Param:Format:Audio:rate", SpaType::Pod),
    // Sometimes Choice sometimes Int
    ("Spa:Pod:Object:Param:Format:Audio:channels", SpaType::Pod),
    // Sometimes Choice sometimes Id
    ("Spa:Pod:Object:Param:Format:mediaType", SpaType::Pod),
    // Sometimes Choice sometimes Id (TODO)
    ("Spa:Pod:Object:Param:Format:mediaSubtype", SpaType::Pod),
    // Sometimes Choice sometimes Id (TODO)
    ("Spa:Pod:Object:Param:Format:Audio:position", SpaType::Pod),
    // Sometimes Choice sometimes Id (TODO)
    (
        "Spa:Pod:Object:Param:Format:Audio:iec958Codec",
        SpaType::Pod,
    ),
];

static PROP_NAME_TO_ENUM_MAP: &[(&str, &str)] = &[
    (
        "Spa:Pod:Object:Param:Format:Video:multiviewMode",
        "SpaVideoMultiviewMode",
    ),
    // TODO: Don't use SpaEnum for bitflags
    (
        "Spa:Pod:Object:Param:Format:Video:multiviewFlags",
        "SpaVideoMultiviewFlags",
    ),
];

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
    fn postprocess(&mut self) {
        for prop in self.properties.iter_mut() {
            if let Some((_, to)) = PROP_NAME_TYPE_OVERRIDE
                .iter()
                .find(|(k, _)| *k == prop.name)
            {
                prop.values.clear();
                prop.parent = SpaEnum::<_, u32>::Value(*to).as_raw();
            }
        }
    }

    fn print(&self, out: &mut impl Write) {
        let ident = format_ident!("{}", camel_case(spa_short_name(&self.name).unwrap()));
        let ident_str = ident.to_string();

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

            let rs_spa_type = spa_type_to_rs(spa_type, info);
            let get = if let Some(call) = spa_type_to_as_call(spa_type, info) {
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

        // TODO: Remove duplication
        let props_dbg = self.properties.iter().filter_map(|info| {
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

            let src = quote! {
                opt_fmt!(f, self.#ident);
            };

            Some(src)
        });

        let doc = format!(" {}", self.name);

        let src = quote! {
            #[doc = #doc]
            pub struct #ident<'a>(pub PodObjectDeserializer<'a>);
            impl<'a> #ident<'a> {
                fn get(&self, id: u32) -> Option<PodDeserializer> {
                    self.0.clone().find(|v| v.key == id).map(|v| v.value)
                }

                #(#props)*
            }


            impl<'a> std::fmt::Debug for #ident<'a> {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    let mut f = f.debug_struct(#ident_str);
                    #(#props_dbg)*
                    f.finish()
                }
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
    let res = spa_type_to_rs(parent, info).to_string();

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

        if spa_extract_known_enum_name(parent, info).is_some() {
            return quote! {
                #[doc = #doc]
            };
        } else if let Some(enum_name) = spa_extract_enum_name(parent, info) {
            doc += "\n";
            doc += "    enum: ";
            doc += enum_name;
        }
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

fn spa_extract_enum_name(parent: SpaType, info: &SpaTypeInfo) -> Option<&str> {
    if parent == SpaType::Id && !info.values.is_empty() {
        let variant_name = &info.values[0].name;
        let id = variant_name.rfind(':').unwrap();
        let enum_name = &variant_name[..id];
        Some(enum_name)
    } else {
        None
    }
}

fn spa_extract_known_enum_name(parent: SpaType, info: &SpaTypeInfo) -> Option<&str> {
    spa_extract_enum_name(parent, info)
        .and_then(|name| ID_TO_ENUM_MAP.iter().find(|(a, _)| *a == name))
        .or_else(|| PROP_NAME_TO_ENUM_MAP.iter().find(|(a, _)| *a == info.name))
        .map(|(_, v)| v)
        .copied()
}

fn spa_type_to_as_call(parent: SpaType, info: &SpaTypeInfo) -> Option<TokenStream> {
    let out = match parent {
        SpaType::Bool => quote!(as_bool()),
        SpaType::Id => {
            if spa_extract_known_enum_name(parent, info).is_some() {
                quote!(as_id().map(SpaEnum::from_raw))
            } else {
                quote!(as_id())
            }
        }
        SpaType::Int => quote!(as_i32()),
        SpaType::Long => quote!(as_i64()),
        SpaType::Float => quote!(as_f32()),
        SpaType::Double => quote!(as_f64()),
        SpaType::String => quote!(as_str()),
        SpaType::Rectangle => quote!(as_rectangle()),
        SpaType::Fraction => quote!(as_fraction()),
        SpaType::Fd => quote!(as_fd()),
        SpaType::Choice => quote!(as_choice()),
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

    let name = &info.name;
    Some(quote!(#out.map_err(|err| unreachable!("{}: {err}", #name)).ok()))
}

fn spa_type_to_rs(parent: SpaType, info: &SpaTypeInfo) -> TokenStream {
    match parent {
        SpaType::Bool => quote!(bool),
        SpaType::Id => {
            if let Some(enum_name) = spa_extract_known_enum_name(parent, info) {
                let name = format_ident!("{enum_name}");
                quote!(SpaEnum<#name>)
            } else {
                quote!(u32)
            }
        }
        SpaType::Int => quote!(i32),
        SpaType::Long => quote!(i64),
        SpaType::Float => quote!(f32),
        SpaType::Double => quote!(f64),
        SpaType::String => quote!(&BStr),
        SpaType::Rectangle => quote!(SpaRectangle),
        SpaType::Fraction => quote!(SpaFraction),
        SpaType::Fd => quote!(i64),
        SpaType::Choice => quote!(PodChoiceDeserializer),

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

    out += &quote! {
        macro_rules! opt_fmt {
            ($f: ident, $self: ident.$key: ident) => {
                if let Some(v) = $self.$key() {
                    $f.field(stringify!($key), &v);
                }
            };
        }
    }
    .to_string();

    for mut e in json {
        e.postprocess();
        e.print(&mut out);
    }

    let out = prettyplease::unparse(&syn::parse_str::<syn::File>(&out).unwrap());
    println!("{out}");
}
