extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Fields, Ident};

fn get_primitive_name(attrs: &[Attribute]) -> (Ident, String) {
    attrs
        .iter()
        .find_map(|attr| {
            if !attr.path().is_ident("primitive") {
                return None;
            }

            let ident: Ident = attr.parse_args().unwrap();
            let name = ident.to_string();

            Some((ident.clone(), name))
        })
        .expect("complex enums must include primitive type name")
}

/// #[primitive = PrimitiveName]
#[proc_macro_derive(PrimitiveFromEnum, attributes(primitive))]
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
                let (primitive_name, primitive_name_s) = get_primitive_name(&ast.attrs);

                let len = data_enum.variants.len();

                let mut get_primitive_enum: Vec<TokenStream> = Vec::with_capacity(len);

                for variant in &data_enum.variants {
                    let variant_name = &variant.ident;

                    match &variant.fields {
                        Fields::Unit => {
                            get_primitive_enum.push(quote! {
                                #name::#variant_name => #primitive_name::#variant_name,
                            });
                        }
                        Fields::Unnamed(fields) => {
                            let len = fields.unnamed.len();
                            if len == 1 {
                                get_primitive_enum.push(quote! {
                                    #name::#variant_name(_) => #primitive_name::#variant_name,
                                });
                            } else {
                                let underscores = vec![quote! { ,_ }; len - 1];
                                get_primitive_enum.push(quote! {
                                    #name::#variant_name(_ #(#underscores)*) => #primitive_name::#variant_name,
                                });
                            }
                        }
                        Fields::Named(fields) => {
                            let fields = &fields
                                .named
                                .iter()
                                .map(|f| {
                                    let ident = f.ident.as_ref().unwrap();
                                    quote! { #ident: _, }
                                })
                                .collect::<Vec<_>>();
                            get_primitive_enum.push(quote! {
                                #name::#variant_name{ #(#fields)* } => #primitive_name::#variant_name,
                            });
                        }
                    };
                }

                let gen = quote! {
                    impl primitive_enum::PrimitiveFromEnum for #name {
                        type PrimitiveEnum = #primitive_name;
                        #[inline]
                        fn get_primitive_enum(&self) -> Self::PrimitiveEnum {
                            match self {
                                #(#get_primitive_enum)*
                            }
                        }
                        #[inline]
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

#[proc_macro_derive(FromU8, attributes(primitive))]
pub fn derive_from_u8(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(stream as DeriveInput);

    let name = &ast.ident;
    let name_s = &ast.ident.to_string();
    let data = &ast.data;

    match data {
        Data::Enum(data_enum) => {
            let is_simple_enum = data_enum.variants.iter().all(|item| item.fields.is_empty());
            if is_simple_enum {
                let mut variants: Vec<TokenStream> = Vec::with_capacity(data_enum.variants.len());
                let mut try_variants: Vec<TokenStream> =
                    Vec::with_capacity(data_enum.variants.len());

                for variant in &data_enum.variants {
                    let ident = &variant.ident;
                    let var = quote! {
                        u if #name::#ident == u => #name::#ident,
                    };
                    variants.push(var);
                    try_variants.push(quote! {
                        u if #name::#ident == u => Ok(#name::#ident),
                    });
                }

                let gen = quote! {
                    impl PartialEq<u8> for #name {
                        fn eq(&self, other: &u8) -> bool {
                            *self as u8 == *other
                        }
                    }
                    impl primitive_enum::UnsafeFromU8 for #name {
                        #[inline]
                        unsafe fn from_unsafe(u: u8) -> Self {
                            match u {
                                #(#variants)*
                                _ => panic!("UnsafeFromU8 from_unsafe undefined value: {}", u),
                            }
                        }
                        #[inline]
                        fn name() -> &'static str {
                            #name_s
                        }
                    }
                    impl core::convert::TryFrom<u8> for #name {
                        type Error = primitive_enum::EnumFromU8Error;
                        fn try_from(value: u8) -> Result<Self, Self::Error> {
                            match value {
                                #(#try_variants)*
                                _ => Err(primitive_enum::EnumFromU8Error),
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
