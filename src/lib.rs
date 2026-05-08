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
#![cfg_attr(test, feature(macro_attr))]
#![allow(incomplete_features)]

#[cfg(feature = "generic_const_arguments")]
#[cfg(feature = "type_info")]
#[cfg(feature = "ptr_metadata")]
#[cfg(feature = "maybe_uninit_array_assume_init")]
pub mod fields;
#[cfg(test)]
mod tests;

struct Assert<const CONDITION: bool>;
