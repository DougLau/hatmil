// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod html;
mod path;
mod svg;
mod value;

pub use html::{Elem, Html, VoidElem};
pub use path::PathDef;
pub use svg::Svg;
pub use value::Value;
