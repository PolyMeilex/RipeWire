use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

fn op_code(attrs: &[syn::Attribute], name: &syn::Ident) -> proc_macro2::TokenStream {
    let op_code = attrs.iter().find(|attr| {
        let Some(ident) = attr.path.get_ident() else {
            return false;
        };
        ident == "op_code"
    });

    if let Some(op_code) = op_code {
        let syn::Meta::List(mut value) = op_code.parse_meta().unwrap() else {
            todo!("not list")
        };
        let syn::NestedMeta::Lit(value) = value.nested.pop().unwrap().into_value() else {
            todo!("not lit")
        };
        let syn::Lit::Int(value) = value else {
            todo!("not int")
        };

        let out: u8 = value.base10_parse().unwrap();

        quote! {
            impl HasOpCode for #name {
                const OPCODE: u8 = #out;
            }
        }
    } else {
        quote!()
    }
}

#[proc_macro_derive(HasOpCode, attributes(op_code))]
pub fn has_opcode(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    op_code(&input.attrs, &name).into()
}

#[proc_macro_derive(PodSerialize, attributes(op_code, fd))]
pub fn pod_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let op_code = op_code(&input.attrs, &name);

    let out = if let Data::Struct(s) = input.data {
        s.fields
            .into_iter()
            .map(|field| field.ident.unwrap())
            .map(|field| quote!(s.serialize_field(&self.#field)?;))
    } else {
        unimplemented!("Not a struct")
    };

    let expanded = quote! {
        #op_code
        impl pod::serialize::PodSerialize for #name {
            fn serialize<O: std::io::Write + std::io::Seek>(
                &self,
                serializer: pod::serialize::PodSerializer<O>,
                flatten: bool,
            ) -> Result<pod::serialize::SerializeSuccess<O>, pod::serialize::GenError> {
                let mut s = serializer.serialize_struct(flatten)?;
                #(#out)*
                s.end()
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(EventDeserialize)]
pub fn event_deserialize2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = if let Data::Enum(ref e) = input.data {
        e.variants.iter().map(|variant| {
            let variant = &variant.ident;
            quote!(events::#variant::OPCODE => Self::#variant(
                Deserialize::deserialize(pod, fds).map_err(|error| {
                    EventDeserializeError {
                        interface: Self::INTERFACE,
                        event: stringify!(#variant),
                        error,
                    }
                })?
            ))
        })
    } else {
        unimplemented!("Not a struct")
    };

    let expanded = quote! {
        impl #name {
            pub fn deserialize(opcode: u8, pod: &mut pod::PodDeserializer, fds: &[std::os::fd::RawFd]) -> Result<Self, EventDeserializeError> {
                let mut this = match opcode {
                    #(#out,)*
                    _ => todo!("opcode: {opcode}"),
                };

                Ok(this)
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
