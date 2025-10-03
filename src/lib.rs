// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod chariter;
mod html;
mod svg;

pub use chariter::CharIter;
pub use html::{Elem, Html, VoidElem};
pub use svg::Svg;
