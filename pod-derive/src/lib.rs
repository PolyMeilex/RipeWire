use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(EventDeserialize)]
pub fn event_deserialize(input: TokenStream) -> TokenStream {
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
