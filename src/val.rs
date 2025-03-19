use std::path::{Path, PathBuf};

use crate::err::Error;

/// A trait for parsing command line argument values into various types
///
/// This trait provides a unified interface for converting string values
/// from command line arguments into different Rust types. Each implementation
/// handles type-specific parsing logic and error reporting.
pub trait ValueParser {
    type Error;
    type Out<'a>;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error>;
}

/// Converts an optional string value into a required value
///
/// # Arguments
/// * `val` - An optional string reference from command line arguments
///
/// # Returns
/// Returns the string reference if present, or an Error if None
pub fn value_or_err(val: Option<&String>) -> Result<&String, Error> {
    val.ok_or_else(|| Error::from("Except value, found None"))
}

/// Implementation of ValueParser for boolean values
///
/// Returns true if any value is present, false otherwise
impl ValueParser for bool {
    type Error = Error;

    type Out<'a> = bool;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        Ok(val.is_some())
    }
}

/// Implementation of ValueParser for String values
///
/// Returns a cloned String from the input value
impl ValueParser for String {
    type Error = Error;

    type Out<'a> = String;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        value_or_err(val).cloned()
    }
}

/// Implementation of ValueParser for string slices
///
/// Returns a reference to the input string's contents
impl ValueParser for &'_ str {
    type Error = Error;

    type Out<'a> = &'a str;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        value_or_err(val).map(|v| v.as_str())
    }
}

/// Implementation of ValueParser for PathBuf values
///
/// Converts the input string into a PathBuf
impl ValueParser for PathBuf {
    type Error = Error;

    type Out<'a> = PathBuf;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        value_or_err(val).map(|v| PathBuf::from(&v))
    }
}

/// Implementation of ValueParser for Path references
///
/// Returns a reference to a Path created from the input string
impl ValueParser for Path {
    type Error = Error;

    type Out<'a> = &'a Path;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        value_or_err(val).map(|v| Path::new(v.as_str()))
    }
}

macro_rules! impl_for {
    ($type:ty) => {
        impl ValueParser for $type {
            type Error = Error;

            type Out<'a> = $type;

            fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
                value_or_err(val)?.parse::<$type>().map_err(|e| {
                    Error::from(format!(
                        "Can not parsing value to {}: {e:?}",
                        stringify!($type)
                    ))
                })
            }
        }
    };
}

impl_for!(i8);
impl_for!(i16);
impl_for!(i32);
impl_for!(i64);
impl_for!(i128);
impl_for!(u8);
impl_for!(u16);
impl_for!(u32);
impl_for!(u64);
impl_for!(u128);
impl_for!(usize);
impl_for!(isize);
impl_for!(f64);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_or_err() {
        assert!(value_or_err(None).is_err());
        let s = String::from("test");
        assert_eq!(value_or_err(Some(&s)).unwrap(), &s);
    }

    #[test]
    fn test_bool_parser() {
        assert!(!bool::parse(None).unwrap());
        assert!(bool::parse(Some(&String::from(""))).unwrap());
    }

    #[test]
    fn test_string_parser() {
        assert!(String::parse(None).is_err());
        let s = String::from("test");
        assert_eq!(String::parse(Some(&s)).unwrap(), s);
    }

    #[test]
    fn test_str_parser() {
        assert!(<&str>::parse(None).is_err());
        let s = String::from("test");
        assert_eq!(<&str>::parse(Some(&s)).unwrap(), "test");
    }

    #[test]
    fn test_path_parser() {
        assert!(Path::parse(None).is_err());
        let s = String::from("/test/path");
        assert_eq!(Path::parse(Some(&s)).unwrap(), Path::new("/test/path"));
    }

    #[test]
    fn test_pathbuf_parser() {
        assert!(PathBuf::parse(None).is_err());
        let s = String::from("/test/path");
        assert_eq!(
            PathBuf::parse(Some(&s)).unwrap(),
            PathBuf::from("/test/path")
        );
    }

    #[test]
    fn test_number_parsers() {
        // Test integer types
        assert_eq!(i32::parse(Some(&String::from("42"))).unwrap(), 42);
        assert_eq!(u32::parse(Some(&String::from("42"))).unwrap(), 42);
        assert!(i32::parse(Some(&String::from("abc"))).is_err());

        // Test floating point
        assert_eq!(f64::parse(Some(&String::from("3.143"))).unwrap(), 3.143);
        assert!(f64::parse(Some(&String::from("abc"))).is_err());
    }
}
