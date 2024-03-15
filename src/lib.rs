extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, Ident};

fn builder_methods(fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>) -> proc_macro2::TokenStream {
    let methods = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! {
            pub fn #name(mut self, #name: #ty) -> Self {
                self.#name = Some(#name);
                self
            }
        }
    });

    quote! { #(#methods)* }
}

fn builder_struct(fields: &syn::punctuated::Punctuated<Field, syn::token::Comma>, builder_name: &Ident) -> proc_macro2::TokenStream {
    let fields = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: Option<#ty> }
    });

    quote! {
        pub struct #builder_name {
            #(#fields),*
        }
    }
}

#[proc_macro_derive(Builder)]
pub fn builder(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let builder_name = syn::Ident::new(&format!("{}Builder", name), name.span());

    let fields = if let Data::Struct(data_struct) = input.data {
        match data_struct.fields {
            Fields::Named(fields) => fields.named,
            _ => unimplemented!("Only named fields are supported"),
        }
    } else {
        unimplemented!("Only structs are supported");
    };

    let builder_struct = builder_struct(&fields, &builder_name);
    let builder_methods = builder_methods(&fields);

    let field_names = fields.iter().map(|f| &f.ident);
    let field_accesses = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { self.#name.unwrap_or_default() }
    });

    let build_method = quote! {
        pub fn build(self) -> #name {
            #name {
                #(#field_names: #field_accesses),*
            }
        }
    };

    let field_names_for_builder_impl = fields.iter().map(|f| &f.ident);
    let builder_impl = quote! {
        impl #name {
            pub fn builder() -> #builder_name {
                #builder_name {
                    #(#field_names_for_builder_impl: None),*
                }
            }
        }
    };

    let expanded = quote! {
        #builder_impl

        #builder_struct

        impl #builder_name {
            #builder_methods

            #build_method
        }
    };

    TokenStream::from(expanded)
}
