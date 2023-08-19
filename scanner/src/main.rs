use std::io::BufRead;

use heck::{ToShoutySnakeCase, ToUpperCamelCase};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quick_xml::events::attributes::Attributes;
use quick_xml::events::Event as XmlEvent;
use quick_xml::reader::Reader;
use quote::quote;

fn main() {
    let has_opcode = quote! {
        pub trait HasOpCode {
            const OPCODE: u8;
        }
    }
    .to_string();

    let client = gen(include_str!("../../protocol/pw_client.xml"));
    let client_node = gen(include_str!("../../protocol/pw_client_node.xml"));
    let core = gen(include_str!("../../protocol/pw_core.xml"));
    let device = gen(include_str!("../../protocol/pw_device.xml"));
    let factory = gen(include_str!("../../protocol/pw_factory.xml"));
    let link = gen(include_str!("../../protocol/pw_link.xml"));
    let module = gen(include_str!("../../protocol/pw_module.xml"));
    let node = gen(include_str!("../../protocol/pw_node.xml"));
    let port = gen(include_str!("../../protocol/pw_port.xml"));
    let registry = gen(include_str!("../../protocol/pw_registry.xml"));

    let full = [
        has_opcode,
        client,
        client_node,
        core,
        device,
        factory,
        link,
        module,
        node,
        port,
        registry,
    ]
    .join("\n");

    println!("{full}");
}

fn to_doc_attr(text: &str) -> TokenStream {
    let text = text.lines().map(str::trim).collect::<Vec<_>>().join("\n");
    let text = text.trim();

    if text.is_empty() {
        quote!()
    } else {
        quote!(#[doc = #text])
    }
}

fn gen(src: &str) -> String {
    let protocol = Protocol::parse(src);

    let mut out = String::new();

    for interface in protocol.interfaces.iter() {
        out += &format!("pub mod {} {{", interface.name);
        out += "use super::*;";

        for enu in interface.enums.iter() {
            let name = enu.name.to_upper_camel_case();
            let ty = pw_ty_to_rust_ty(&enu.ty).expect("Unknown Type").to_string();

            if enu.bitfield {
                out += "bitflags::bitflags! {\n";
                out += &" ".repeat(4);
                out += "#[derive(Debug, Clone, Copy, pod_derive::PodBitflagDeserialize)]\n";
                out += &" ".repeat(4);
                out += &format!("pub struct {name}: {ty} {{\n");

                for entry in enu.entries.iter() {
                    out += &" ".repeat(4 * 2);
                    out += &format!(
                        "const {} = {};\n",
                        entry.name.to_shouty_snake_case(),
                        entry.value
                    );
                }

                out += &" ".repeat(4);
                out += &format!("}}");
                out += "\n}\n";

                out += "\n\n";
            } else {
                let varians = enu.entries.iter().map(|entry| {
                    let name = entry.name.to_upper_camel_case();
                    let value: usize = entry.value.parse().unwrap();

                    let name = Ident::new(&name, Span::call_site());
                    let value = Literal::usize_unsuffixed(value);

                    quote!(#name = #value)
                });

                let name = Ident::new(&name, Span::call_site());
                let ty = Ident::new(&ty, Span::call_site());

                out += &quote! {
                    #[derive(Debug, Clone, Copy)]
                    #[repr(#ty)]
                    pub enum #name {
                        #(#varians),*
                    }
                }
                .to_string();

                out += "\n";
            }

            out += "\n";
        }

        let methods = interface
            .methods
            .iter()
            .enumerate()
            .map(|(opcode, method)| {
                let name = method.name.to_upper_camel_case();
                let name = Ident::new(&name, Span::call_site());
                let opcode = Literal::u8_unsuffixed(opcode as u8);

                if method.ty == "permission_list" {
                    let doc = to_doc_attr(&method.description.txt);

                    quote! {
                        #doc
                        #[derive(Debug, Clone)]
                        pub struct #name(pub pod::permissions::Permissions);

                        impl pod::serialize::PodSerialize for #name {
                            fn serialize<O: std::io::Write + std::io::Seek>(
                                &self,
                                serializer: pod::serialize::PodSerializer<O>,
                            ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
                                self.0.serialize(serializer)
                            }
                        }

                        impl HasOpCode for #name {
                            const OPCODE: u8 = #opcode;
                        }
                    }
                } else {
                    let fields = method.fields.iter().map(|field| {
                        let name = Ident::new(&field.name, Span::call_site());

                        let ty = if let Some(ty) = pw_ty_to_rust_ty(&field.ty) {
                            ty
                        } else {
                            let ty = field.ty.to_upper_camel_case();
                            let ty = Ident::new(&ty, Span::call_site());
                            quote!(#ty)
                        };

                        quote! {
                            pub #name: #ty
                        }
                    });

                    let doc = to_doc_attr(&method.description.txt);

                    quote! {
                        #doc
                        #[derive(Debug, Clone, pod_derive::PodSerialize)]
                        pub struct #name {
                            #(#fields),*
                        }

                        impl HasOpCode for #name {
                            const OPCODE: u8 = #opcode;
                        }
                    }
                }
            });

        let methods = quote! {
            pub mod methods {
                use super::*;
                #(#methods)*
            }
        };

        out += &methods.to_string();

        let events = interface.event.iter().enumerate().map(|(opcode, event)| {
            let name = event.name.to_upper_camel_case();
            let name = Ident::new(&name, Span::call_site());
            let opcode = Literal::u8_unsuffixed(opcode as u8);

            let doc = to_doc_attr(&event.description.txt);

            let fields = event
                .fields
                .iter()
                .map(|field| {
                    let name = Ident::new(&field.name, Span::call_site());

                    let ty = if let Some(ty) = pw_ty_to_rust_ty(&field.ty) {
                        ty
                    } else {
                        let ty = field.ty.to_upper_camel_case();
                        let ty = Ident::new(&ty, Span::call_site());
                        quote!(#ty)
                    };

                    let res = quote! {
                        pub #name: #ty
                    };

                    res
                })
                .collect::<Vec<_>>();

            quote! {
                #doc
                #[derive(Debug, Clone, pod_derive::PodDeserialize)]
                pub struct #name {
                    #(#fields),*
                }

                impl HasOpCode for #name {
                    const OPCODE: u8 = #opcode;
                }
            }
        });

        let event_variants = interface.event.iter().enumerate().map(|(_, event)| {
            let name = event.name.to_upper_camel_case();
            let name = Ident::new(&name, Span::call_site());
            let doc = to_doc_attr(&event.description.txt);

            quote! {
                #doc
                #name(events::#name)
            }
        });

        let events = quote! {
            pub mod events {
                use super::*;
                #(#events)*
            }

            #[derive(Debug, Clone, pod_derive::EventDeserialize)]
            pub enum Event {
                #(#event_variants),*
            }
        };

        out += &events.to_string();

        out += &format!("}}\n");
    }

    out
}

fn pw_ty_to_rust_ty(ty: &str) -> Option<TokenStream> {
    Some(match ty {
        "bool" => quote!(bool),
        "int" => quote!(i32),
        "long" => quote!(i64),
        "uint" => quote!(u32),
        "ulong" => quote!(u64),
        "string" => quote!(String),
        "dict" => quote!(pod::dictionary::Dictionary),
        "struct" => quote!(pod::pod_struct::Struct),
        "permission_list" => quote!(pod::permissions::Permissions),
        "permission_flags" => quote!(pod::permissions::PermissionFlags),
        "value" => quote!(pod::Value),
        "id" => quote!(pod::utils::Id),
        "fd" => quote!(pod::utils::Fd),
        ty => {
            if ty.starts_with("array(") && ty.ends_with(")") {
                let ty = ty
                    .strip_prefix("array(")
                    .unwrap()
                    .strip_suffix(")")
                    .unwrap();

                let ty = pw_ty_to_rust_ty(ty).expect("Unknown Type");
                quote!(pod::array::Array<#ty>)
            } else {
                return None;
            }
        }
    })
}

#[derive(Debug, Default)]
struct Protocol {
    pub interfaces: Vec<Interface>,
}

impl Protocol {
    fn parse(src: &str) -> Protocol {
        let mut reader = Reader::from_str(src);
        reader.trim_text(true);

        let mut protocol = Protocol::default();

        loop {
            // NOTE: this is the generic case when we don't know about the input BufRead.
            // when the input is a &str or a &[u8], we don't actually need to use another
            // buffer, we could directly call `reader.read_event()`
            match reader.read_event_into(&mut Vec::new()) {
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                // exits the loop when reaching end of file
                Ok(XmlEvent::Eof) => break,

                Ok(XmlEvent::Start(e)) => {
                    match e.name().0 {
                        b"interface" => {
                            let interface = Interface::parse(&mut reader, e.attributes());
                            protocol.interfaces.push(interface);
                        }
                        _ => {}
                    }

                    //
                }
                Ok(XmlEvent::Text(e)) => {
                    dbg!(e.unescape().unwrap().into_owned());
                }

                // There are several other `Event`s we do not consider here
                _ => (),
            }
        }

        protocol
    }
}

#[derive(Debug, Default)]
pub struct Interface {
    pub name: String,
    pub methods: Vec<Method>,
    pub event: Vec<Event>,
    pub enums: Vec<Enum>,
}

impl Interface {
    fn parse<R: BufRead>(reader: &mut Reader<R>, attrs: Attributes) -> Interface {
        let mut interface = Interface::default();
        for attr in attrs.filter_map(|res| res.ok()) {
            match attr.key.0 {
                b"name" => interface.name = decode_utf8_or_panic(attr.value.into_owned()),
                _ => {}
            }
        }

        loop {
            match reader.read_event_into(&mut Vec::new()) {
                Ok(XmlEvent::Empty(bytes)) => match bytes.name().0 {
                    b"method" => {
                        interface
                            .methods
                            .push(Method::parse(reader, bytes.attributes(), true));
                    }
                    b"event" => {
                        interface
                            .event
                            .push(Event::parse(reader, bytes.attributes(), true));
                    }
                    _ => {}
                },
                Ok(XmlEvent::Start(bytes)) => match bytes.name().0 {
                    b"method" => {
                        interface
                            .methods
                            .push(Method::parse(reader, bytes.attributes(), false));
                    }
                    b"event" => {
                        interface
                            .event
                            .push(Event::parse(reader, bytes.attributes(), false));
                    }
                    b"enum" => {
                        interface
                            .enums
                            .push(Enum::parse(reader, bytes.attributes()));
                    }
                    _ => {}
                },
                Ok(XmlEvent::End(bytes)) if bytes.name().0 == b"interface" => break,
                _ => {}
            }
        }

        interface
    }
}

#[derive(Debug, Default)]
pub struct Description {
    pub txt: String,
}

impl Description {
    fn parse<R: BufRead>(reader: &mut Reader<R>) -> Self {
        let mut description = Self::default();

        loop {
            match reader.read_event_into(&mut Vec::new()) {
                Ok(XmlEvent::Text(bytes)) => {
                    description
                        .txt
                        .push_str(&bytes.unescape().unwrap_or_default());
                }
                Ok(XmlEvent::End(bytes)) if bytes.name().0 == b"description" => break,
                _ => {}
            }
        }

        description
    }
}

#[derive(Debug, Default)]
pub struct Method {
    pub name: String,
    pub ty: String,
    pub description: Description,
    pub fields: Vec<Field>,
}

impl Method {
    fn parse<R: BufRead>(reader: &mut Reader<R>, attrs: Attributes, empty: bool) -> Method {
        let mut method = Method::default();
        for attr in attrs.filter_map(|res| res.ok()) {
            match attr.key.0 {
                b"name" => method.name = decode_utf8_or_panic(attr.value.into_owned()),
                b"type" => method.ty = decode_utf8_or_panic(attr.value.into_owned()),
                _ => {}
            }
        }

        if !empty {
            loop {
                match reader.read_event_into(&mut Vec::new()) {
                    Ok(XmlEvent::Start(bytes)) => match bytes.name().0 {
                        b"description" => {
                            method.description = Description::parse(reader);
                        }
                        b"field" => {
                            method
                                .fields
                                .push(Field::parse(reader, bytes.attributes(), false))
                        }
                        _ => {}
                    },
                    Ok(XmlEvent::Empty(bytes)) => match bytes.name().0 {
                        b"field" => {
                            method
                                .fields
                                .push(Field::parse(reader, bytes.attributes(), true))
                        }
                        _ => {}
                    },
                    Ok(XmlEvent::End(bytes)) if bytes.name().0 == b"method" => break,
                    _ => {}
                }
            }
        }

        method
    }
}

#[derive(Debug, Default)]
pub struct Event {
    pub name: String,
    pub ty: String,
    pub description: Description,
    pub fields: Vec<Field>,
}

impl Event {
    fn parse<R: BufRead>(reader: &mut Reader<R>, attrs: Attributes, empty: bool) -> Self {
        let mut method = Self::default();
        for attr in attrs.filter_map(|res| res.ok()) {
            match attr.key.0 {
                b"name" => method.name = decode_utf8_or_panic(attr.value.into_owned()),
                b"type" => method.ty = decode_utf8_or_panic(attr.value.into_owned()),
                _ => {}
            }
        }

        if !empty {
            loop {
                match reader.read_event_into(&mut Vec::new()) {
                    Ok(XmlEvent::Start(bytes)) => match bytes.name().0 {
                        b"description" => {
                            method.description = Description::parse(reader);
                        }
                        b"field" => {
                            method
                                .fields
                                .push(Field::parse(reader, bytes.attributes(), false))
                        }
                        _ => {}
                    },
                    Ok(XmlEvent::Empty(bytes)) => match bytes.name().0 {
                        b"field" => {
                            method
                                .fields
                                .push(Field::parse(reader, bytes.attributes(), true))
                        }
                        _ => {}
                    },
                    Ok(XmlEvent::End(bytes)) if bytes.name().0 == b"event" => break,
                    _ => {}
                }
            }
        }

        method
    }
}

#[derive(Debug, Default)]
pub struct Field {
    pub name: String,
    pub ty: String,
}

impl Field {
    fn parse<R: BufRead>(reader: &mut Reader<R>, attrs: Attributes, empty: bool) -> Field {
        let mut field = Field::default();
        for attr in attrs.filter_map(|res| res.ok()) {
            match attr.key.0 {
                b"name" => field.name = decode_utf8_or_panic(attr.value.into_owned()),
                b"type" => field.ty = decode_utf8_or_panic(attr.value.into_owned()),
                _ => {}
            }
        }

        if !empty {
            // loop {
            //     match reader.read_event_into(&mut Vec::new()) {
            //         Ok(Event::Start(bytes)) => match bytes.name().0 {
            //             b"field" => {}
            //             _ => {}
            //         },
            //         Ok(Event::End(bytes)) if bytes.name().0 == b"field" => break,
            //         _ => {}
            //     }
            // }
        }

        field
    }
}

#[derive(Debug, Default)]
pub struct Enum {
    pub name: String,
    pub ty: String,
    pub bitfield: bool,
    pub entries: Vec<EnumEntry>,
}

impl Enum {
    fn parse<R: BufRead>(reader: &mut Reader<R>, attrs: Attributes) -> Self {
        let mut this = Self::default();
        for attr in attrs.filter_map(|res| res.ok()) {
            match attr.key.0 {
                b"name" => this.name = decode_utf8_or_panic(attr.value.into_owned()),
                b"type" => this.ty = decode_utf8_or_panic(attr.value.into_owned()),
                b"bitfield" => {
                    this.bitfield = decode_utf8_or_panic(attr.value.into_owned()) == "true"
                }
                _ => {}
            }
        }

        loop {
            match reader.read_event_into(&mut Vec::new()) {
                Ok(XmlEvent::Empty(bytes)) => match bytes.name().0 {
                    b"entry" => {
                        this.entries.push(EnumEntry::parse(bytes.attributes()));
                    }
                    _ => {}
                },
                Ok(XmlEvent::End(bytes)) if bytes.name().0 == b"enum" => break,
                _ => {}
            }
        }

        this
    }
}

#[derive(Debug, Default)]
pub struct EnumEntry {
    pub name: String,
    pub value: String,
}

impl EnumEntry {
    fn parse(attrs: Attributes) -> Self {
        let mut this = Self::default();
        for attr in attrs.filter_map(|res| res.ok()) {
            match attr.key.0 {
                b"name" => this.name = decode_utf8_or_panic(attr.value.into_owned()),
                b"value" => this.value = decode_utf8_or_panic(attr.value.into_owned()),
                _ => {}
            }
        }
        this
    }
}

fn decode_utf8_or_panic(txt: Vec<u8>) -> String {
    match String::from_utf8(txt) {
        Ok(txt) => txt,
        Err(e) => panic!(
            "Invalid UTF8: '{}'",
            String::from_utf8_lossy(&e.into_bytes())
        ),
    }
}
