// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[macro_use]
mod macros;

mod definition;
pub mod html;
mod page;
mod poly;
pub mod svg;
mod value;

pub use definition::PathDefBuilder;
pub use page::Page;
pub use poly::PolyPointBuilder;
pub use value::Value;
