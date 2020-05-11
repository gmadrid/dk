use anyhow::Error;
use std::convert::TryFrom;
use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, Div, Mul, Range, Sub};

macro_rules! unit_binop {
    ($name:ident, $class:ident, $binop: ident) => {
        impl<T> $class<T> for $name
        where
            T: Into<$name>,
        {
            type Output = $name;
            fn $binop(self, rhs: T) -> Self::Output {
                $name(u16::$binop(self.0, rhs.into().0))
            }
        }

        impl $class<i32> for $name {
            type Output = $name;
            fn $binop(self, rhs: i32) -> Self::Output {
                $name(u16::$binop(self.0, rhs as u16))
            }
        }

        impl $class<u32> for $name {
            type Output = u32;
            fn $binop(self, rhs: u32) -> Self::Output {
                u32::$binop(u32::from(self.0), rhs)
            }
        }
    };
}

macro_rules! define_unit {
    ($name:ident) => {
        #[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
        pub struct $name(u16);
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl From<u8> for $name {
            fn from(f: u8) -> Self {
                $name(f.into())
            }
        }

        impl From<u16> for $name {
            fn from(f: u16) -> Self {
                $name(f.into())
            }
        }

        impl TryFrom<u32> for $name {
            type Error = Error;
            fn try_from(v: u32) -> Result<Self, Self::Error> {
                Ok($name(u16::try_from(v)?))
            }
        }

        impl TryFrom<usize> for $name {
            type Error = Error;
            fn try_from(v: usize) -> Result<Self, Self::Error> {
                Ok($name(u16::try_from(v)?))
            }
        }

        impl From<$name> for usize {
            fn from(v: $name) -> Self {
                v.0.into()
            }
        }

        impl From<$name> for u32 {
            fn from(v: $name) -> Self {
                v.0.into()
            }
        }

        impl IntoIterator for $name {
            type Item = Self;
            type IntoIter = UnitIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                UnitIter::new(self)
            }
        }

        unit_binop!($name, Add, add);
        unit_binop!($name, Sub, sub);
        unit_binop!($name, Mul, mul);
        unit_binop!($name, Div, div);
    };
}

macro_rules! allow_unit_conversion {
    ($from: ident, $to: ident) => {
        impl From<$from> for $to {
            fn from(f: $from) -> Self {
                $to(f.0)
            }
        }
    };
}

define_unit!(Rows);
define_unit!(Cols);
define_unit!(Height);
define_unit!(Width);

allow_unit_conversion!(Width, Cols);
allow_unit_conversion!(Cols, Width);
allow_unit_conversion!(Height, Rows);
allow_unit_conversion!(Rows, Height);

pub struct UnitIter<T>(Range<u16>, PhantomData<T>);

impl<T> UnitIter<T>
where
    T: Into<usize>,
{
    fn new(t: T) -> UnitIter<T> {
        UnitIter(
            Range {
                start: 0_u16,
                end: t.into() as u16,
            },
            PhantomData::<T>,
        )
    }
}

impl<T> Iterator for UnitIter<T>
where
    T: From<u16>,
{
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|v| Self::Item::from(v))
    }
}

impl<T> DoubleEndedIterator for UnitIter<T>
where
    T: From<u16>,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.0.next_back().map(|v| Self::Item::from(v))
    }
}
