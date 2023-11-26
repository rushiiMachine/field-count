use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt};
use syn::{parse_macro_input, ItemEnum, ItemStruct};

#[proc_macro_derive(FieldCount)]
pub fn derive_field_count(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemStruct);

    let name = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let field_count = input.fields.iter().count();

    let output = quote! {
        impl #impl_generics FieldCount for #name #ty_generics #where_clause {
            fn field_count() -> usize {
                #field_count
            }
        }

        impl #impl_generics #name #ty_generics #where_clause {
            const fn field_count() -> usize {
                #field_count
            }
        }
    };

    TokenStream::from(output)
}

#[proc_macro_derive(EnumFieldCount)]
pub fn derive_field_count_enum(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemEnum);

    let enum_name = &input.ident;
    let variants = input.variants;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let mut match_arms = quote! {};

    variants.iter().for_each(|v| {
        let variant_name = &v.ident;
        let field_count = v.fields.iter().count();
        let fields_tuple = if field_count > 0 {
            quote!((..))
        } else {
            quote! {}
        };
        let match_arm = quote! { #enum_name::#variant_name #fields_tuple => #field_count, };
        match_arms.append_all(match_arm);
    });

    let output = quote! {
        impl #impl_generics EnumFieldCount for #enum_name #ty_generics #where_clause {
            fn field_count(&self) -> usize {
                match *self {
                    #match_arms
                }
            }
        }
    };

    TokenStream::from(output)
}
