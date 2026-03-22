// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[macro_use]
mod macros;

mod definition;
pub mod html;
mod poly;
pub mod svg;
mod tree;
mod value;

#[allow(deprecated)]
pub use tree::Page;

pub use definition::PathDefBuilder;
pub use poly::PolyPointBuilder;
pub use tree::Tree;
pub use value::Value;
