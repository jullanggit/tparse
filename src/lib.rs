#![feature(adt_const_params)]
#![feature(unsized_const_params)]

use std::marker::PhantomData;

struct TStr<const STR: &'static str>();

trait TParse
where
    Self: Sized,
{
    // TODO: handle errors better
    /// Option<(Self, advanced by)>
    fn tparse(input: &str) -> Option<(Self, usize)>;
}

impl<const STR: &'static str> TParse for TStr<STR> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let str = STR;
        let len = str.len();
        (&input[0..len] == str).then_some((Self(), len))
    }
}

impl TParse for char {
    /// Match any character
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let mut iter = input.char_indices();
        let (_, char) = iter.next()?;
        let offset = iter.next().map_or(input.len(), |(offset, _)| offset);

        Some((char, offset))
    }
}

/// Greedily match on the options
macro_rules! OR {
    ($enum:ident, $($ty:ident),+) => {
        enum $enum {
            $(
                $ty($ty),
            )+
        }
        impl TParse for $enum {
            fn tparse(input: &str) -> Option<(Self, usize)> {
                $(
                    if let Some(parsed) = $ty::tparse(input) {
                        return Some((Self::$ty(parsed.0), parsed.1))
                    }
                )+
                None
            }
        }
    };
}
