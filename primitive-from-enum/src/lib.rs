#![cfg_attr(not(feature = "std"), no_std)]
pub use primitive_enum_derive::{FromU8, PrimitiveFromEnum};
#[cfg(feature = "std")]
use std::{error::Error, fmt};

/**
Need for complex Enums, which includes other data:
```
use primitive_enum::{PrimitiveFromEnum, FromU8};

#[derive(PrimitiveFromEnum)]
#[primitive(Primitive)]
enum Complex {
    A(String),
    B(u32),
    C
}
#[derive(FromU8, Clone, Copy)]
#[repr(u8)]
enum Primitive {
    A,
    B,
    C,
}
```
PrimitiveEnum should be equivalent for Complex, but without variants inner data
 */
pub trait PrimitiveFromEnum {
    type PrimitiveEnum: TryFrom<u8> + UnsafeFromU8;

    fn get_primitive_enum(&self) -> Self::PrimitiveEnum;

    /// get primitive enum name in string format
    fn primitive_name() -> &'static str;
}

pub trait UnsafeFromU8: PartialEq<u8> + Sized {
    unsafe fn from_unsafe(_: u8) -> Self;

    // get enum name in string format
    fn name() -> &'static str;
}

#[cfg_attr(feature = "std", derive(Debug))]
pub struct EnumFromU8Error;

#[cfg(feature = "std")]
impl fmt::Display for EnumFromU8Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EnumFromU8Error")
    }
}

#[cfg(feature = "std")]
impl Error for EnumFromU8Error {}
