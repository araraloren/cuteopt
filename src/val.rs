use std::path::{Path, PathBuf};

use crate::err::Error;

pub trait ValueParser {
    type Error;
    type Out<'a>;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error>;
}

pub fn value_or_err(val: Option<&String>) -> Result<&String, Error> {
    val.ok_or_else(|| Error::from("Except value, found None"))
}

impl ValueParser for bool {
    type Error = Error;

    type Out<'a> = bool;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        Ok(val.is_some())
    }
}

impl ValueParser for String {
    type Error = Error;

    type Out<'a> = String;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        value_or_err(val).cloned()
    }
}

impl ValueParser for &'_ str {
    type Error = Error;

    type Out<'a> = &'a str;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        value_or_err(val).map(|v| v.as_str())
    }
}

impl ValueParser for PathBuf {
    type Error = Error;

    type Out<'a> = PathBuf;

    fn parse(val: Option<&String>) -> Result<Self::Out<'_>, Self::Error> {
        value_or_err(val).map(|v| PathBuf::from(&v))
    }
}

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
