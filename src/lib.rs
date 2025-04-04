// lib.rs
//
// Copyright (C) 2025  Douglas P Lau
#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

mod html;

pub use html::{Elem, Html, VoidElem, opt_ref, opt_str};
