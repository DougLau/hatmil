// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

#[macro_use]
mod content;
pub mod elem;
mod html;
mod path;
mod svg;
mod value;

pub use html::Page;
pub use path::PathDef;
pub use svg::Svg;
pub use value::Value;
