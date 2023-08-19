use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(PodSerialize)]
pub fn pod_serialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = if let Data::Struct(s) = input.data {
        s.fields
            .into_iter()
            .map(|field| field.ident.unwrap())
            .map(|field| quote!(s.serialize_field(&self.#field)?;))
    } else {
        unimplemented!("Not a struct")
    };

    let expanded = quote! {
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

#[proc_macro_derive(PodDeserialize)]
pub fn pod_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out: Vec<_> = if let Data::Struct(s) = input.data {
        s.fields
            .into_iter()
            .map(|field| field.ident.unwrap())
            .map(|field| quote!(
                #field: struct_deserializer.deserialize_field()?.ok_or(pod::deserialize::DeserializeError::PropertyMissing)?
            ))
            .collect()
    } else {
        unimplemented!("Not a struct")
    };

    let expanded = quote! {
        impl<'de> pod::deserialize::PodDeserialize<'de> for #name {
            fn deserialize(
                deserializer: pod::deserialize::PodDeserializer<'de>,
            ) -> Result<
                (Self, pod::deserialize::DeserializeSuccess<'de>),
                pod::deserialize::DeserializeError<&'de [u8]>,
            >
            where
                Self: Sized,
            {
                struct TestVisitor;
                impl<'de> pod::deserialize::Visitor<'de> for TestVisitor {
                    type Value = #name;
                    type ArrayElem = std::convert::Infallible;

                    fn visit_struct(
                        &self,
                        struct_deserializer: &mut pod::deserialize::StructPodDeserializer<'de>,
                    ) -> Result<Self::Value, pod::deserialize::DeserializeError<&'de [u8]>>
                    {
                        Ok(Self::Value {
                            #(#out),*
                        })
                    }
                }

                deserializer.deserialize_struct(TestVisitor)
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(PodBitflagDeserialize)]
pub fn pod_bitflag_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl<'de> pod::deserialize::PodDeserialize<'de> for #name {
            fn deserialize(
                deserializer: pod::deserialize::PodDeserializer<'de>,
            ) -> Result<
                (Self, pod::deserialize::DeserializeSuccess<'de>),
                pod::deserialize::DeserializeError<&'de [u8]>,
            >
            where
                Self: Sized,
            {
                struct TestVisitor;
                impl<'de> pod::deserialize::Visitor<'de> for TestVisitor {
                    type Value = #name;
                    type ArrayElem = std::convert::Infallible;

                    fn visit_int(
                        &self,
                        v: i32,
                    ) -> Result<Self::Value, pod::deserialize::DeserializeError<&'de [u8]>>
                    {
                        Ok(Self::Value::from_bits_retain(v as _))
                    }

                    fn visit_long(
                        &self,
                        v: i64,
                    ) -> Result<Self::Value, pod::deserialize::DeserializeError<&'de [u8]>>
                    {
                        Ok(Self::Value::from_bits_retain(v as _))
                    }
                }

                match std::mem::size_of::<Self>() {
                    8 => deserializer.deserialize_long(TestVisitor),
                    4 => deserializer.deserialize_int(TestVisitor),
                    _ => unreachable!(),
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(EventDeserialize)]
pub fn event_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let out = if let Data::Enum(e) = input.data {
        e.variants
            .into_iter()
            .map(|enu| enu.ident)
            .enumerate()
            .map(|(opcode, variant)| {
                let opcode = opcode as u8;
                quote!(#opcode => Self::#variant(pod::deserialize::PodDeserializer::deserialize_from(&value)?.1))
            })
    } else {
        unimplemented!("Not a struct")
    };

    let expanded = quote! {
        impl #name {
            pub fn from(opcode: u8, value: &[u8]) -> Result<Self, pod::deserialize::DeserializeError<&[u8]>> {
                let this = match opcode {
                    #(#out,)*
                    _ => return Err(pod::deserialize::DeserializeError::InvalidType),
                };

                Ok(this)
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
