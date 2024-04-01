use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub(crate) fn process_enum_from(input: DeriveInput) -> TokenStream {
    // get the ident
    let ident = input.ident;
    // get generics
    let generics = input.generics;
    // get enum variants
    let variants = match input.data {
        syn::Data::Enum(data) => data.variants,
        _ => panic!("EnumFrom only works on enums"),
    };
    // for each variant, get the ident and fields
    let from_impls = variants.iter().map(|variant| {
        let var = &variant.ident;
        match &variant.fields {
            syn::Fields::Unnamed(fields) => {
                // only support one field
                if fields.unnamed.len() != 1 {
                    quote! {}
                } else {
                    let field = fields.unnamed.first().expect("should have 1 field");
                    let ty = &field.ty;
                    quote! {
                        impl #generics  From<#ty> for #ident #generics {
                            fn from(v: #ty) -> Self {
                                #ident::#var(v)
                            }
                        }
                    }
                }
            }

            _ => quote! {},
        }
    });

    // quote return proce-macro2 TokenTtream so we need to convert it to TokenStream
    quote! {
        #(#from_impls)*
    }
}
