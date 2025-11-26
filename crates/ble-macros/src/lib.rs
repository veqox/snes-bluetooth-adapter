use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Expr, Field, ItemStruct, MetaNameValue, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
};

struct Args {
    ogf: Expr,
    ocf: Expr,
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let vars = Punctuated::<MetaNameValue, Token![,]>::parse_terminated(input)?;

        let mut ogf = None;
        let mut ocf = None;

        for v in vars {
            if let Some(ident) = v.path.get_ident() {
                match ident.to_string().as_str() {
                    "ogf" => ogf = Some(v.value.clone()),
                    "ocf" => ocf = Some(v.value.clone()),
                    arg => {
                        return Err(syn::Error::new(
                            v.span(),
                            format!("unknown argument {}", arg),
                        ));
                    }
                }
            }
        }

        Ok(Self {
            ogf: ogf.ok_or_else(|| syn::Error::new(input.span(), "missing ogf"))?,
            ocf: ocf.ok_or_else(|| syn::Error::new(input.span(), "missing ocf"))?,
        })
    }
}

#[proc_macro_attribute]
pub fn command(args: TokenStream, item: TokenStream) -> TokenStream {
    let args: Args = syn::parse(args).expect("failed to parse args");
    let input = parse_macro_input!(item as ItemStruct);

    let name = &input.ident;
    let ocf = args.ocf;
    let ogf = args.ogf;

    let fields: Vec<&Field> = input.fields.iter().collect();

    let new_args = fields.iter().fold(quote! {}, |new_args, f| {
        let ty = &f.ty;
        let ident = &f.ident;
        quote! { #new_args #ident: #ty, }
    });

    let new_fields = fields.iter().fold(quote! {}, |new_fields, f| {
        let ident = &f.ident;
        quote! { #new_fields #ident, }
    });

    let param_len_expr = fields.iter().fold(quote! { 0 }, |param_len_expr, f| {
        let ty = &f.ty;
        quote! { #param_len_expr + core::mem::size_of::<#ty>() }
    });

    let write_expr = fields.iter().fold(quote! {}, |write_expr, f| {
        let ident = &f.ident;
        quote! {
            #write_expr
            self.#ident.write_into(w)?;
        }
    });

    quote! {
        #input

        impl #name {
            pub fn new(#new_args) -> Self {
                Self {
                    #new_fields
                }
            }
        }

        impl Command for #name {
            const OCF: u16 = #ocf;
            const OGF: u16 = #ogf;
            const PARAM_LEN: u8 = (#param_len_expr) as u8;

            fn write_payload<W: Write>(&self, w: &mut W) -> Result<(), W::Error> {
                #write_expr
                Ok(())
            }
        }
    }
    .into()
}
