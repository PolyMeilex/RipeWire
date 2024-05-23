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

#[proc_macro_derive(HasName)]
pub fn has_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    quote! {
        impl HasName for #name {
            const NAME: &'static str = stringify!(#name);
        }
    }
    .into()
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

#[proc_macro_derive(PodDeserialize, attributes(fd))]
pub fn pod_deserialize(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (fds, out) = if let Data::Struct(ref s) = input.data {
        let fds = s
            .fields
            .iter()
            .filter(|field| {
                for attr in field.attrs.iter() {
                    if attr.path.segments[0].ident.to_string().as_str() == "fd" {
                        return true;
                    }
                }

                false
            })
            .map(|field| {
                let field = field.ident.as_ref().unwrap();

                quote! {
                    self.#field.fd = fds.get(self.#field.id as usize).copied();
                }
            });

        let deserialize_fields = s.fields
            .iter()
            .map(|field| field.ident.as_ref().unwrap())
            .map(|field| quote!(
                #field: struct_deserializer.deserialize_field()?.ok_or(pod::deserialize::DeserializeError::PropertyMissing)?
            ));

        (fds, deserialize_fields)
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

        // TODO: trait this up
        impl #name {
            pub(super) fn load_fds(&mut self, fds: &[std::os::fd::RawFd]) {
                #(#fds)*
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

    let (fds, out) = if let Data::Enum(ref e) = input.data {
        let fds = e.variants.iter().map(|variant| {
            let variant = &variant.ident;
            quote! {
                Self::#variant(event) => {
                    event.load_fds(fds);
                }
            }
        });

        let deserialize = e.variants
            .iter()
            .enumerate()
            .map(|(opcode, variant)| {
                let opcode = opcode as u8;
                let variant = &variant.ident;
                quote!(#opcode => Self::#variant(pod::deserialize::PodDeserializer::deserialize_from(&value)?.1))
            });

        (fds, deserialize)
    } else {
        unimplemented!("Not a struct")
    };

    let expanded = quote! {
        impl #name {
            pub fn from<'a>(opcode: u8, value: &'a [u8], fds: &[std::os::fd::RawFd]) -> Result<Self, pod::deserialize::DeserializeError<&'a [u8]>> {
                let mut this = match opcode {
                    #(#out,)*
                    _ => return Err(pod::deserialize::DeserializeError::UnsupportedType),
                };

                this._load_fds(fds);

                Ok(this)
            }

            fn _load_fds(&mut self, fds: &[std::os::fd::RawFd]) {
                match self {
                    #(#fds,)*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}

#[proc_macro_derive(EventDeserialize2)]
pub fn event_deserialize2(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let (fds, out) = if let Data::Enum(ref e) = input.data {
        let fds = e.variants.iter().map(|variant| {
            let variant = &variant.ident;
            quote! {
                Self::#variant(event) => {
                    event.load_fds(fds);
                }
            }
        });

        let deserialize = e.variants.iter().enumerate().map(|(opcode, variant)| {
            let opcode = opcode as u8;
            let variant = &variant.ident;
            quote!(#opcode => Self::#variant(EventDeserialize::deserialize(pod)?))
        });

        (fds, deserialize)
    } else {
        unimplemented!("Not a struct")
    };

    let expanded = quote! {
        impl #name {
            pub fn deserialize_event(opcode: u8, pod: &mut pod_simple::PodDeserializer, fds: &[std::os::fd::RawFd]) -> Result<Self, EventDeserializeError> {
                let mut this = match opcode {
                    #(#out,)*
                    _ => todo!("opcode: {opcode}"),
                };

                this._load_fds2(fds);

                Ok(this)
            }

            fn _load_fds2(&mut self, fds: &[std::os::fd::RawFd]) {
                match self {
                    #(#fds,)*
                }
            }
        }
    };

    // Hand the output tokens back to the compiler
    TokenStream::from(expanded)
}
