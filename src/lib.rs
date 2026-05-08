#![doc = include_str!("../README.md")]
#![no_std]
#![warn(clippy::pedantic)]
#![cfg_attr(feature = "generic_const_arguments", feature(min_generic_const_args))]
#![cfg_attr(feature = "generic_const_arguments", feature(generic_const_args))]
#![cfg_attr(feature = "type_info", feature(type_info))]
#![cfg_attr(feature = "ptr_metadata", feature(ptr_metadata))]
#![cfg_attr(
    feature = "maybe_uninit_array_assume_init",
    feature(maybe_uninit_array_assume_init)
)]
#![allow(incomplete_features)]
#![deny(missing_docs)]

#[cfg(feature = "generic_const_arguments")]
#[cfg(feature = "type_info")]
#[cfg(feature = "ptr_metadata")]
#[cfg(feature = "maybe_uninit_array_assume_init")]
/// Reflection of a type's fields as `dyn Trait`s.
pub mod fields;
#[cfg(test)]
mod tests;

#[allow(dead_code)]
struct Assert<const CONDITION: bool>;
