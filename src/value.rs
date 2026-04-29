// value.rs
//
// Copyright (C) 2025-2026  Douglas P Lau
//
use std::borrow::Cow;

/// Character iterator
enum CharIter<'a> {
    /// Borrowed string slice
    Borrowed(&'a str),
    /// Owned string
    Owned(String),
}

/// A value of an attribute or text content
pub struct Value<'a> {
    iter: CharIter<'a>,
}

impl Value<'_> {
    /// Get character iterator
    pub(crate) fn chars(&'_ self) -> impl Iterator<Item = char> {
        match &self.iter {
            CharIter::Borrowed(s) => s.chars(),
            CharIter::Owned(s) => s.chars(),
        }
    }

    /// Encode value to an attribute
    ///
    /// These characters will be replaced with entities:
    ///
    /// - `&` ⇨ `&amp;`
    /// - `"` ⇨ `&quot;`
    pub fn encode_attr(&'_ self, attr: &mut String) {
        for c in self.chars() {
            match c {
                '&' => attr.push_str("&amp;"),
                '"' => attr.push_str("&quot;"),
                _ => attr.push(c),
            }
        }
    }

    /// Encode value to character data
    ///
    /// These characters will be replaced with entities:
    ///
    /// - `&` ⇨ `&amp;`
    /// - `<` ⇨ `&lt;`
    /// - `>` ⇨ `&gt;`
    pub fn encode_cdata(&'_ self, cdata: &mut String) {
        for c in self.chars() {
            match c {
                '&' => cdata.push_str("&amp;"),
                '<' => cdata.push_str("&lt;"),
                '>' => cdata.push_str("&gt;"),
                _ => cdata.push(c),
            }
        }
    }

    /// Encode value to character data with length limit
    ///
    /// These characters will be replaced with entities:
    ///
    /// - `&` ⇨ `&amp;`
    /// - `<` ⇨ `&lt;`
    /// - `>` ⇨ `&gt;`
    pub fn encode_cdata_len(&'_ self, cdata: &mut String, len: usize) {
        for c in self.chars().take(len) {
            match c {
                '&' => cdata.push_str("&amp;"),
                '<' => cdata.push_str("&lt;"),
                '>' => cdata.push_str("&gt;"),
                _ => cdata.push(c),
            }
        }
    }

    /// Encode value to a comment
    ///
    /// These characters will be replaced with entities:
    ///
    /// - `-` ⇨ `&hyphen;`
    /// - `<` ⇨ `&lt;`
    /// - `>` ⇨ `&gt;`
    pub fn encode_comment(&'_ self, comment: &mut String) {
        for c in self.chars() {
            match c {
                '-' => comment.push_str("&hyphen;"),
                '<' => comment.push_str("&lt;"),
                '>' => comment.push_str("&gt;"),
                _ => comment.push(c),
            }
        }
    }
}

impl<'c> From<&'c str> for Value<'c> {
    fn from(v: &'c str) -> Self {
        Value {
            iter: CharIter::Borrowed(v),
        }
    }
}

impl From<String> for Value<'_> {
    fn from(v: String) -> Self {
        Value {
            iter: CharIter::Owned(v),
        }
    }
}

impl<'c> From<&'c String> for Value<'c> {
    fn from(v: &'c String) -> Self {
        Value {
            iter: CharIter::Borrowed(v),
        }
    }
}

impl<'c> From<Cow<'c, str>> for Value<'c> {
    fn from(v: Cow<'c, str>) -> Self {
        match v {
            Cow::Borrowed(v) => Value {
                iter: CharIter::Borrowed(v),
            },
            Cow::Owned(v) => Value {
                iter: CharIter::Owned(v),
            },
        }
    }
}

impl From<char> for Value<'_> {
    fn from(v: char) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<bool> for Value<'_> {
    fn from(v: bool) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<i8> for Value<'_> {
    fn from(v: i8) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<u8> for Value<'_> {
    fn from(v: u8) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<i16> for Value<'_> {
    fn from(v: i16) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<u16> for Value<'_> {
    fn from(v: u16) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<i32> for Value<'_> {
    fn from(v: i32) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<u32> for Value<'_> {
    fn from(v: u32) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<i64> for Value<'_> {
    fn from(v: i64) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<u64> for Value<'_> {
    fn from(v: u64) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<i128> for Value<'_> {
    fn from(v: i128) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<u128> for Value<'_> {
    fn from(v: u128) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<isize> for Value<'_> {
    fn from(v: isize) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<usize> for Value<'_> {
    fn from(v: usize) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<f32> for Value<'_> {
    fn from(v: f32) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}

impl From<f64> for Value<'_> {
    fn from(v: f64) -> Self {
        Value {
            iter: CharIter::Owned(v.to_string()),
        }
    }
}
