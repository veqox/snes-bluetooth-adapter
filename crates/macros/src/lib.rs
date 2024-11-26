#![no_std]

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Variant};

#[proc_macro_derive(FromU8)]
pub fn from_u8(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let data = match input.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("#[derive(FromU8)] can only be used with enums"),
    };

    let match_arms = data.variants.into_iter().map(|v: Variant| {
        let ident = v.ident;

        let discriminant = match v.discriminant {
            Some((_, expr)) => expr,
            None => panic!("Enum variants must have assigned discriminant values"),
        };

        quote! {
            #discriminant => #name::#ident,
        }
    });

    let expanded = quote! {
        impl From<u8> for #name {
            fn from(value: u8) -> Self {
                match value {
                    #(#match_arms)*
                    _ => panic!("Invalid value for {}: {}", stringify!(#name), value),
                }
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(IntoU8)]
pub fn into_u8(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;

    let expanded = quote! {
        impl Into<u8> for #name {
            fn into(self) -> u8 {
                self as u8
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Size)]
pub fn size(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = match &ast.data {
        Data::Struct(s) => &s.fields,
        _ => unimplemented!("Only works for structs"),
    };

    let size = match &fields {
        Fields::Named(fields) => fields.named.iter().fold(quote! { 0 }, |size, field| {
            let ty = &field.ty;
            quote! { #size + core::mem::size_of::<#ty>() }
        }),
        Fields::Unnamed(fields) => fields.unnamed.iter().fold(quote! { 0 }, |size, field| {
            let ty = &field.ty;
            quote! { #size + core::mem::size_of::<#ty>() }
        }),

        Fields::Unit => quote! { 0 },
    };

    TokenStream::from(quote! {
        impl #name {
            pub const fn size(&self) -> usize {
                #size
            }
        }
    })
}
