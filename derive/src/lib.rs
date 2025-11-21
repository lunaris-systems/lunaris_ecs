use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Component, attributes(component))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Hardcode the path to point to our re-export
    let crate_path = quote! { lunaris_ecs::bevy_ecs };

    let expanded = quote! {
        impl #impl_generics #crate_path::component::Component for #name #ty_generics #where_clause {
            const STORAGE_TYPE: #crate_path::component::StorageType = #crate_path::component::StorageType::Table;
            type Mutability = #crate_path::component::Mutable;
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Resource, attributes(resource))]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let crate_path = quote! { lunaris_ecs::bevy_ecs };

    let expanded = quote! {
        impl #impl_generics #crate_path::prelude::Resource for #name #ty_generics #where_clause {}
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Event, attributes(event))]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let crate_path = quote! { lunaris_ecs::bevy_ecs };

    let expanded = quote! {
        impl #impl_generics #crate_path::event::Event for #name #ty_generics #where_clause {
            type Trigger<'a> = #crate_path::event::GlobalTrigger;
        }
    };

    TokenStream::from(expanded)
}
