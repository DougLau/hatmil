// chariter.rs
//
// Copyright (C) 2025  Douglas P Lau
//
use std::borrow::Cow;

/// Character iterator
pub enum CharIter<'a> {
    /// Borrowed string slice
    Borrowed(&'a str),
    /// Owned string
    Owned(String),
}

impl CharIter<'_> {
    /// Get character iterator
    pub fn chars(&'_ self) -> impl Iterator<Item = char> {
        match self {
            CharIter::Borrowed(s) => s.chars(),
            CharIter::Owned(s) => s.chars(),
        }
    }
}

impl<'c> From<&'c str> for CharIter<'c> {
    fn from(v: &'c str) -> Self {
        CharIter::Borrowed(v)
    }
}

impl From<String> for CharIter<'_> {
    fn from(v: String) -> Self {
        CharIter::Owned(v)
    }
}

impl<'c> From<&'c String> for CharIter<'c> {
    fn from(v: &'c String) -> Self {
        CharIter::Borrowed(v)
    }
}

impl<'c> From<Cow<'c, str>> for CharIter<'c> {
    fn from(v: Cow<'c, str>) -> Self {
        match v {
            Cow::Borrowed(v) => CharIter::Borrowed(v),
            Cow::Owned(v) => CharIter::Owned(v),
        }
    }
}

impl From<char> for CharIter<'_> {
    fn from(v: char) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<bool> for CharIter<'_> {
    fn from(v: bool) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<i8> for CharIter<'_> {
    fn from(v: i8) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<u8> for CharIter<'_> {
    fn from(v: u8) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<i16> for CharIter<'_> {
    fn from(v: i16) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<u16> for CharIter<'_> {
    fn from(v: u16) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<i32> for CharIter<'_> {
    fn from(v: i32) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<u32> for CharIter<'_> {
    fn from(v: u32) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<i64> for CharIter<'_> {
    fn from(v: i64) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<u64> for CharIter<'_> {
    fn from(v: u64) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<i128> for CharIter<'_> {
    fn from(v: i128) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<u128> for CharIter<'_> {
    fn from(v: u128) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<isize> for CharIter<'_> {
    fn from(v: isize) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<usize> for CharIter<'_> {
    fn from(v: usize) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<f32> for CharIter<'_> {
    fn from(v: f32) -> Self {
        CharIter::Owned(v.to_string())
    }
}

impl From<f64> for CharIter<'_> {
    fn from(v: f64) -> Self {
        CharIter::Owned(v.to_string())
    }
}
