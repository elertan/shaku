//! shaku provides a derive macro to generate implementations of the
//! Component traits for data structures defined in your crate,
//! allowing them to be 'registered' and 'resolved' from a Container
//!
//! This functionality is based on Rust's #[derive] mechanism,
//! just like what you would use to automatically derive implementations
//! of the built-in Clone, Copy, Debug, or other traits.
//! It is able to generate implementations for most structs.
//! For now enums including ones with elaborate generic types or
//! trait bounds. On rare occasions, for an especially convoluted
//! type you may need to implement the trait manually.
//!
//! # How-to
//!
//! These derives require a Rust compiler version 1.31 or newer.
//!
//! - Add shaku = "0.2" as a dependency in Cargo.toml.
//! - If you have a main.rs, add `#[macro_use] extern crate shaku` there,
//!   or import `shaku::Component` as needed if using Rust 2018.
//! - Use `#[derive(Component)]` on structs that you want to flag as Component which you want to
//!   inject or be injected.
//! - Specify the interface this Component is implementing through `#[shaku(interface = MyTrait)]`
//!
//! # #[derive(Component)] macro
//!
//! Supported attributes
//! - interface: for a struct, name of the Trait that this Component will implement
//! - inject: for a struct's property, tag a property as being a dependency to another Component
//!   (currently only supports `Arc<Interface>`/`Arc<dyn Interface>` syntax)

extern crate proc_macro;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

mod component;
mod consts;
mod error;
mod parsing;
mod structures;

#[proc_macro_derive(Component, attributes(shaku))]
pub fn component(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    TokenStream::from(component::expand_derive_component(&input))
}
