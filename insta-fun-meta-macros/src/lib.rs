use proc_macro::TokenStream;
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::{Expr, Ident, Result, Token, punctuated::Punctuated};

struct MetadataField {
    name: Ident,
    value: Expr,
}

impl Parse for MetadataField {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let value: Expr = input.parse()?;
        Ok(Self { name, value })
    }
}

struct MetadataInput {
    fields: Punctuated<MetadataField, Token![,]>,
}

impl Parse for MetadataInput {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(Self {
            fields: Punctuated::<MetadataField, Token![,]>::parse_terminated(input)?,
        })
    }
}

fn resolve_insta_fun_root() -> proc_macro2::TokenStream {
    let found = crate_name("insta-fun").or_else(|_| crate_name("insta_fun"));

    match found {
        Ok(FoundCrate::Itself) => quote!(::insta_fun),
        Ok(FoundCrate::Name(name)) => {
            let ident = format_ident!("{}", name.replace('-', "_"));
            quote!(::#ident)
        }
        Err(_) => quote!(::insta_fun),
    }
}

#[proc_macro]
pub fn insta_fun_meta(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as MetadataInput);
    let crate_root = resolve_insta_fun_root();

    let expanded_fields = input.fields.iter().map(|field| {
        let name = field.name.to_string();
        let value = &field.value;
        quote! {
            #crate_root::meta::MetaField::new(#name, #value)
        }
    });

    quote! {
        #crate_root::meta::SnapshotMetadata::new(vec![
            #(#expanded_fields),*
        ])
    }
    .into()
}
