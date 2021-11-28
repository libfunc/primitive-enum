extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::{TokenStream};
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, Data, DeriveInput, Field, Fields, Ident, Lit, Meta,
};

fn get_primitive_name(ast: &DeriveInput) -> (TokenStream, String) {
    ast.attrs
        .iter()
        .find_map(|attr| {
            attr.path.segments.first().and_then(|segment| {
                if segment.ident != "coming" {
                    return None;
                }
                match attr.parse_args::<Meta>() {
                    Ok(Meta::NameValue(name_value)) => {
                        if name_value.path.to_token_stream().to_string() != "primitive" {
                            return None;
                        }
                        if let Lit::Str(litstr) = name_value.lit {
                            let s = litstr.parse::<Ident>().unwrap();
                            let value = s.to_token_stream();
                            Some((value, s.to_string()))
                        } else {
                            None
                        }
                    }
                    Ok(_) => None,
                    Err(_) => None,
                }
            })
        })
        .expect("complex enums must include primitive type name!")
}

#[proc_macro_derive(PrimitiveFromEnum, attributes(coming))]
pub fn derive_primitive_from_enum(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(stream as DeriveInput);

    let name = &ast.ident;
    let data = &ast.data;

    match data {
        Data::Enum(data_enum) => {
            let is_simple_enum = data_enum.variants.iter().all(|item| item.fields.is_empty());

            if is_simple_enum {
                panic!("PrimitiveFromEnum only for non simple enum allow");
            } else {
                let (primitive_name, primitive_name_s) = get_primitive_name(&ast);
                let mut variants_names: Vec<(TokenStream, Option<Field>)> =
                    Vec::with_capacity(data_enum.variants.len());

                for variant in &data_enum.variants {
                    if variant.discriminant.is_some() {
                        // why? because discriminant number may not be equal to primitive number
                        panic!("enums variants with discriminant not support in current moment");
                    }
                    let fields = &variant.fields;
                    let fields = match fields {
                        Fields::Unit => None,
                        Fields::Unnamed(fields) => {
                            let len = fields.unnamed.len();
                            if len != 1 {
                                panic!("enums variants is currently support only with 1 unnamed fields");
                            }
                            let field = fields.unnamed.first().unwrap();
                            Some(field.clone())
                        }
                        Fields::Named(_) => {
                            panic!("enums named variants is currently not support");
                        }
                    };
                    let variant_name = &variant.ident;
                    variants_names.push((variant_name.to_token_stream(), fields));
                }

                let get_primitive_enum: Vec<TokenStream> = variants_names
                    .iter()
                    .map(|(variant_name, inner)| {
                        if inner.is_some() {
                            quote! {
                                #name::#variant_name(_) => #primitive_name::#variant_name,
                            }
                        } else {
                            quote! {
                                #name::#variant_name => #primitive_name::#variant_name,
                            }
                        }
                    })
                    .collect();

                let gen = quote! {
                    impl PrimitiveFromEnum for #name {
                        type PrimitiveEnum = #primitive_name;
                        fn get_primitive_enum(&self) -> Self::PrimitiveEnum {
                            match self {
                                #(#get_primitive_enum)*
                            }
                        }
                        fn primitive_name() -> &'static str {
                            #primitive_name_s
                        }
                    }
                };

                proc_macro::TokenStream::from(gen)
            }
        }
        _ => {
            panic!("PrimitiveFromEnum only for enum allow");
        }
    }
}

#[proc_macro_derive(FromU8, attributes(coming))]
pub fn derive_from_u8(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(stream as DeriveInput);

    let name = &ast.ident;
    let data = &ast.data;

    match data {
        Data::Enum(data_enum) => {
            let is_simple_enum = data_enum.variants.iter().all(|item| item.fields.is_empty());
            if is_simple_enum {
                let mut variants: Vec<TokenStream> =
                    Vec::with_capacity(data_enum.variants.len());

                let mut is_first = true;

                for variant in &data_enum.variants {
                    let ident = &variant.ident;
                    let var = if is_first {
                        is_first = false;
                        quote! {
                            if #name::#ident == u {
                                #name::#ident
                            }
                        }
                    } else {
                        quote! {
                            else if #name::#ident == u {
                                #name::#ident
                            }
                        }
                    };
                    variants.push(var);
                }

                let gen = quote! {
                    impl PartialEq<u8> for #name {
                        fn eq(&self, other: &u8) -> bool {
                            *self as u8 == *other
                        }
                    }
                    impl From<u8> for #name {
                        fn from(u: u8) -> Self {
                            #(#variants)*
                            else {
                                panic!("FromU8 from_u8 undefined value");
                            }
                        }
                    }
                };
                proc_macro::TokenStream::from(gen)
            } else {
                panic!("FromU8 only for simple enum allow (without nested data)");
            }
        }
        _ => {
            panic!("FromU8 only for enum allow");
        }
    }
}

