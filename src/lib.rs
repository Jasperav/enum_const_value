use quote::quote;
use syn::{DeriveInput, Data};
use quote::format_ident;

#[proc_macro_derive(EnumConstValue)]
pub fn enum_const_value(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let derive_input: DeriveInput = syn::parse(input).unwrap();
    let original_enum_type_name = &derive_input.ident;
    let const_enum_type_ident = format_ident!("{}ConstValue", &original_enum_type_name);
    let (idents_and_matcher, original_matcher): (Vec<_>, Vec<_>) = match derive_input.data {
        Data::Enum(e) => {
            e
                .variants
                .into_iter()
                .enumerate()
                .map(|(index, variant)| {
                    let index = index as i32;
                    let variant_name = variant.ident;
                    let original_enum_matcher = quote! {
                        &#original_enum_type_name::#variant_name(_) => #index
                    };
                    let const_enum_matcher = quote! {
                        &#const_enum_type_ident::#variant_name => #index
                    };

                    ((variant_name, const_enum_matcher), original_enum_matcher)
                })
                .unzip()
        }
        _ => panic!("Only enums are supported")
    };
    let (variant_names, const_matcher): (Vec<_>, Vec<_>) = idents_and_matcher.into_iter().unzip();
    let tokens = quote! {
        impl #original_enum_type_name {
            pub fn const_value(&self) -> i32 {
                match self {
                    #(#original_matcher),*
                }
            }
        }
        pub enum #const_enum_type_ident {
            #(#variant_names),*
        }

        impl #const_enum_type_ident {
            pub fn value_for_variant(variant: &#const_enum_type_ident) -> i32 {
                match variant {
                    #(#const_matcher),*
                }
            }

            pub fn value(&self) -> i32 {
                #const_enum_type_ident::value_for_variant(self)
            }
        }
    };

    tokens.into()
}