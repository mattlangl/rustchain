use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Decode)]
pub fn decode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_decode_macro(&ast)
}

fn impl_decode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Decode for #name {
            fn decode_binary<R: Read, D: Decoder<#name>>(reader: &mut R, decoder: D) -> Result<Box<#name>, std::io::Error> {
                decoder.decode(reader)
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(Encode)]
pub fn encode_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_encode_macro(&ast)
}

fn impl_encode_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Encode for #name {
            fn encode_binary<W: Write, E: Encoder<#name>>(&self, writer: &mut W, encoder: E) -> Result<(), std::io::Error> {
                encoder.encode(writer, self)
            }
        }
    };
    gen.into()
}