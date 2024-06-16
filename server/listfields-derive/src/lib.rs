use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{braced, parenthesized, parse::{Parse, ParseStream}, parse_macro_input, Attribute, Error, Ident, LitStr, Token, Type, Visibility, MetaList};

enum ObjectParsed {
    EnumParsed(EnumParsed),
    StructParsed(StructParsed)
}

#[derive(Debug)]
struct EnumParsed {
    name: Ident,
    variants: Vec<Var>
}

#[derive(Debug)]
struct StructParsed {
    name: Ident,
    fields: Vec<Field>
}

#[derive(Debug)]
struct Var {
    name: String,
    ty: Ty
}

#[derive(Debug)]
struct Ty {
    ident: Ident,
    name: String
}

#[derive(Debug)]
struct Field {
    name: String,
    ty: String,
}

impl Parse for Ty {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ty: Type = input.parse()?;
        match ty {
            Type::Path(path_type) => {
                let segment = &path_type.path.segments[0];
                match &segment.arguments {
                    syn::PathArguments::AngleBracketed(angle_bracket_args) => {
                        match &angle_bracket_args.args[0] {
                            syn::GenericArgument::Type(ty) => {
                                match ty {
                                    Type::Path(path_type) => {
                                        Ok(Ty{
                                            ident: path_type.path.segments[0].ident.clone(),
                                            name: path_type.path.segments[0].ident.to_string()
                                        })
                                    },
                                    _ => {
                                        Err(Error::new(input.span(), "Expected type path"))
                                    }
                                }
                            },
                            _ => {
                                Err(Error::new(input.span(), "Expected type in angle brackets"))
                            }
                        }
                    },
                    syn::PathArguments::None => {
                        Ok(Ty{
                            ident: segment.ident.clone(),
                            name: segment.ident.to_string()
                        })
                    },
                    _ => {
                        Err(Error::new(input.span(), "Expected angle brackets"))
                    }
                }
            }
            _ => {
                Err(Error::new(input.span(), "Expected type path"))
            }
        }
    }
}

impl Parse for Var {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();

        let mut name = None;
        if lookahead.peek(Token![#]){
            let attr = Attribute::parse_outer(input)?;

            Attribute::parse_nested_meta(&attr[0], |meta| {
                let value = meta.value()?;
                let rename: LitStr = value.parse()?;

                name = Some(rename.value());
                Ok(())
            })?;
        }

        let ident: Ident = input.parse()?;

        let content;
        parenthesized!(content in input);

        let ty: Ty = content.parse()?;

        if name.is_none() {
            name = Some(ident.to_string());
        }
        Ok(Var{
            name: name.unwrap(),
            ty
        })
    }
}

impl Parse for Field {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Ty = input.parse()?;

        return Ok(Field{
            name: name.to_string(),
            ty: ty.name
        })
    }
}

impl Parse for ObjectParsed {
    fn parse(input: ParseStream) -> Result<Self, syn::Error> {
        let lookahead = input.lookahead1();
        if lookahead.peek(Token![#]){
            let _ = Attribute::parse_outer(input);
        }

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![pub]) {
            let _ = input.parse::<Visibility>();
        }

        let lookahead = input.lookahead1();
        if lookahead.peek(Token![enum]) {
            input.parse::<Token![enum]>()?;
            let content;
            let ident: Ident = input.parse()?;
            braced!(content in input);
            let parsed_variants = content.parse_terminated(Var::parse, Token![,])?;
            let mut variants = Vec::new();
            for parsed_variant in parsed_variants {
                variants.push(parsed_variant);
            }

            return Ok(ObjectParsed::EnumParsed(
                EnumParsed {
                    name: ident,
                    variants
                }
            ));
        }
        else if lookahead.peek(Token![struct]) {
            input.parse::<Token![struct]>()?;
            let content;
            let ident: Ident = input.parse()?;
            braced!(content in input);
            let parsed_fields = content.parse_terminated(Field::parse, Token![,])?;

            let mut fields = Vec::new();
            for parsed_field in parsed_fields {
                fields.push(parsed_field);
            }

            return Ok(ObjectParsed::StructParsed(
                StructParsed {
                    name: ident,
                    fields
                }
            ));
        }
        else {
            return Err(lookahead.error());
        }
    }
}

#[proc_macro_derive(ListFields)]
pub fn list_fields_macro(input: TokenStream) -> TokenStream {
    let object_parsed: ObjectParsed = parse_macro_input!(input as ObjectParsed);
    match object_parsed {
        ObjectParsed::EnumParsed( EnumParsed{ name, variants }) => {
            let name_str = name.to_string();

            let mut variant_tokens = Vec::new();

            for variant in variants {
                let v_name = variant.name;
                let v_ty_name = variant.ty.name;
                let v_ty_ident = variant.ty.ident;

                variant_tokens.push(
                    quote! {
                        Variant {
                            name: #v_name.to_string(),
                            ty: #v_ty_name.to_string(),
                            type_data: #v_ty_ident::list_fields()
                        }
                    }
                );
            }

            let output = quote! {
                impl ListFields for #name {
                    fn list_fields() -> ObjectData {
                        let mut v = Vec::new();

                        #( v.push(#variant_tokens); )*
                        ObjectData::EnumData( EnumData {
                            name: #name_str.to_string(),
                            variants: v,
                        })
                    }
                }
            };

            output.into()
        },
        ObjectParsed::StructParsed( StructParsed{ name, fields } ) => {
            let mut field_tokens = Vec::new();

            for field in fields {
                let f_name = field.name;
                let f_ty = field.ty;
                field_tokens.push(
                    quote! {
                        Field {
                            name: #f_name.to_string(),
                            ty: #f_ty.to_string()
                        }
                    }
                )
            }

            let output = quote! {
                impl ListFields for #name {
                    fn list_fields() -> ObjectData {
                        let mut v = Vec::new();

                        #( v.push(#field_tokens); )*

                        ObjectData::Fields(v)
                    }
                }
            };

            output.into()
        }
    }
}
