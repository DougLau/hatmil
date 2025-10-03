// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod html;
mod svg;
mod value;

pub use html::{Elem, Html, VoidElem};
pub use svg::Svg;
pub use value::Value;
