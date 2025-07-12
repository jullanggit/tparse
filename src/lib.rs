#![feature(adt_const_params)]
#![feature(unsized_const_params)]

pub trait TParse
where
    Self: Sized,
{
    // TODO: handle errors better
    /// Option<(Self, advanced by)>
    fn tparse(input: &str) -> Option<(Self, usize)>;
}

/// A type containing a compile-time unique string
#[derive(Debug, PartialEq)]
pub struct TStr<const STR: &'static str>;
impl<const STR: &'static str> TParse for TStr<STR> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let len = STR.len();
        if len > input.len() {
            return None;
        }
        input[0..len].eq(STR).then_some((Self, len))
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

/// Greedily matches on the options
#[macro_export]
macro_rules! Or {
    {$enum:ident, $($ty:ident),+} => {
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

/// Match on all arguments followed by each other
#[macro_export]
macro_rules! Concat {
    ($struct:ident, $($ty:ident),+) => {
        struct $struct($($ty,)+);
        impl TParse for $struct {
            fn tparse(input: &str) -> Option<(Self, usize)> {
                let mut offset = 0;

                $(
                    let ($ty, new_offset) = $ty::tparse(&input[offset..])?;
                    offset += new_offset;
                )+

                Some((Self($($ty,)+), offset))
            }
        }
    };
}

impl<T: TParse> TParse for Vec<T> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let mut out = Vec::new();
        let mut offset = 0;

        while let Some((parsed, new_offset)) = T::tparse(&input[offset..]) {
            out.push(parsed);
            offset += new_offset;
        }

        Some((out, offset))
    }
}

/// A Vec containing at least N elements.
/// This is only enforced when parsing using tparse.
struct VecContaining<const N: usize, T>(pub Vec<T>);
impl<const N: usize, T: TParse> TParse for VecContaining<N, T> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let (vec, offset) = Vec::tparse(input)?;
        vec.len().ge(&N).then_some((Self(vec), offset))
    }
}

impl<T: TParse> TParse for Option<T> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let parsed = T::tparse(input);

        match parsed {
            None => Some((None, 0)),
            Some((parsed, offset)) => Some((Some(parsed), offset)),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_concat() {
        type test_str = TStr<"test_str">;
        Concat!(TestStruct, char, test_str);

        let tests = [
            ("ctest_str", Some((TestStruct('c', TStr), 9))),
            ("test_str", None),
            ("\0test_str\0\0\0", Some((TestStruct('\0', TStr), 9))),
        ];

        for (input, output) in tests {
            let parsed = TestStruct::tparse(input);
            match (parsed, output) {
                (None, None) => {}
                (Some(parsed), Some(output)) => {
                    assert_eq!(parsed.0.0, output.0.0);
                    assert_eq!(parsed.0.1, output.0.1);
                    assert_eq!(parsed.1, output.1);
                }
                _ => panic!(),
            }
        }
    }
}
