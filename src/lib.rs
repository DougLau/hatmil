// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[macro_use]
mod macros;

pub mod html;
mod page;
mod path;
mod value;

pub use page::Page;
pub use path::PathDef;
pub use value::Value;
