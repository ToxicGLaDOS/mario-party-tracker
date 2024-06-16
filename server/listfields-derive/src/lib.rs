use proc_macro::{self, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{braced, parenthesized, parse::{Parse, ParseStream}, parse_macro_input, spanned::Spanned, token::Brace, AngleBracketedGenericArguments, Data::{Enum, Struct, Union}, DataEnum, DeriveInput, Error, GenericArgument, Ident, PathArguments, PathSegment, Token, Type, TypePath, Variant};

#[derive(Debug)]
struct EnumParsed {
    name: Ident,
    variants: Vec<Var>
}

#[derive(Debug)]
struct Var {
    name: String,
    ty: Ty
}

#[derive(Debug)]
struct Ty {
    name: String
}

impl Parse for Ty {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;

        Ok(Ty{
            name: ty.to_token_stream().to_string()
        })
    }
}

impl Parse for Var {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        parenthesized!(content in input);

        let ty: Ty = content.parse()?;

        Ok(Var{
            name: name.to_string(),
            ty
        })   
    }
}

impl Parse for EnumParsed {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        input.parse::<Token![enum]>()?;

        let content;
        let ident: Ident = input.parse()?;
        braced!(content in input);
        let parsed_variants = content.parse_terminated(Var::parse, Token![,])?;
        let mut variants = Vec::new();
        for parsed_variant in parsed_variants {
            variants.push(parsed_variant);
        }

        return Ok(EnumParsed {
            name: ident,
            variants
        });
    }
}

#[proc_macro_derive(ListFields)]
pub fn list_fields_macro(input: TokenStream) -> TokenStream {
    let enum_parsed: EnumParsed = parse_macro_input!(input as EnumParsed);
    let enum_ident = enum_parsed.name;
    let name = enum_ident.to_string();
    let variants = enum_parsed.variants;

    let mut variant_tokens = Vec::new();

    for variant in variants {
        let v_name = variant.name;
        let v_ty = variant.ty.name;
        
        variant_tokens.push(
            quote! {
                Variant {
                    name: #v_name.to_string(),
                    ty: #v_ty.to_string()
                }
            }
        );
    }

    let mut output = quote! {
        impl ListFields for #enum_ident {
            fn answer() -> &'static str {
                "myine"
            }

            fn list_fields() -> EnumData {
                let mut v = Vec::new();

                #( v.push(#variant_tokens); )*
                EnumData {
                    name: #name.to_string(),
                    variants: v
                }
            }
        }
    };

    output.into()
}
