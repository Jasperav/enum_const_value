use quote::quote;
use syn::{DeriveInput, Fields};
use quote::format_ident;

#[proc_macro_derive(EnumConstValue)]
pub fn enum_const_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let original_enum_type_name = &derive_input.ident;
    let const_enum_type_ident = format_ident!("{}ConstValue", &original_enum_type_name);
    let mut generate_const_enum = false;
    let enum_data = proc_macro2_helper::enum_data(derive_input.data);
    let retrieve_ident = enum_data
        .iter()
        .map(|variant| {
            let ident = &variant.ident;

            // TODO: not sure how this can be nicer
            match variant.fields {
                Fields::Unit => quote! {
                    Self::#ident => stringify!(#ident)
                },
                Fields::Unnamed(..) => quote! {
                    Self::#ident(_) => stringify!(#ident)
                },
                Fields::Named(..) => quote! {
                    Self::#ident { .. } => stringify!(#ident)
                }
            }
        })
        .collect::<Vec<_>>();

    let (idents_and_matcher, original_matcher): (Vec<_>, Vec<_>) = enum_data
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, variant)| {
            let index = index as i32;
            let variant_name = variant.ident;
            let tokens = match variant.fields {
                Fields::Unit => quote! {},
                Fields::Unnamed(..) => {
                    generate_const_enum = true;
                    quote! {
                            (..)
                        }
                }
                Fields::Named(..) => {
                    generate_const_enum = true;
                    quote! {
                            {..}
                        }
                }
            };

            let original_enum_matcher = quote! {
                &#original_enum_type_name::#variant_name#tokens => #index
            };
            let const_enum_matcher = quote! {
                &#const_enum_type_ident::#variant_name => #index
            };

            ((variant_name, const_enum_matcher), original_enum_matcher)
        })
        .unzip();
    let (variant_names, const_matcher): (Vec<_>, Vec<_>) = idents_and_matcher.into_iter().unzip();
    let mut tokens = quote! {
        impl #original_enum_type_name {
            pub fn retrieve_ident(&self) -> &'static str {
                match self {
                    #(#retrieve_ident),*
                }
            }

            pub fn const_value(&self) -> i32 {
                match self {
                    #(#original_matcher),*
                }
            }
        }
    };

    if generate_const_enum {
        tokens.extend(quote! {
            pub enum #const_enum_type_ident {
                #(#variant_names),*
            }

            impl #const_enum_type_ident {
                pub fn value_for_variant(variant: &#const_enum_type_ident) -> i32 {
                    match variant {
                        #(#const_matcher),*
                    }
                }

                pub fn const_value(&self) -> i32 {
                    #const_enum_type_ident::value_for_variant(self)
                }
            }
        });
    }

    tokens.into()
}