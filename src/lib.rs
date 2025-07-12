use std::marker::PhantomData;

use tstr::{StrValue, TS, TStr};

trait TParse {
    // TODO: handle errors better
    /// Option<(Self, advanced by)>
    fn tparse(input: &str) -> Option<(Self, usize)>;
}

impl<T> TParse for TStr<T>
where
    TStr<T>: StrValue,
{
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let str = Self::STR;
        let len = str.len();
        (input[0..len] == str).then((Self::NEW, len))
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
                    if let Some(parsed) = $ty::parse(input) {
                        return Some((Self::$ty(parsed.0), parsed.1))
                    }
                )+
                None
            }
        }
    };
}

OR!(TestEnum, char, TStr<T>);
