#![feature(adt_const_params)]
#![feature(unsized_const_params)]

use std::marker::PhantomData;

use seq_macro::seq;

pub trait TParse
where
    Self: Sized,
{
    // TODO: handle errors better
    /// Option<(Self, advanced by)>
    fn tparse(input: &str) -> Option<(Self, usize)>;
}

/// A compile-time unique string
#[derive(Debug, PartialEq)]
pub struct TStr<const STR: &'static str>;
impl<const STR: &'static str> TStr<STR> {
    pub fn str(&self) -> &'static str {
        STR
    }
}
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

/// Matches any single unicode character between START and END
#[derive(Debug)]
pub struct RangedChar<const START: char, const END: char>(pub char);
impl<const START: char, const END: char> TParse for RangedChar<START, END> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let (char, offset) = char::tparse(input)?;
        (START..=END)
            .contains(&char)
            .then_some((Self(char), offset))
    }
}

/// Tries each child parser in order, returning the first successful match
/// ```rust
/// Or!(EnumName, VariantName1 = P1, VariantName2 = P2, ...)
/// ```
// TODO: support for omitting the variant name in simple cases
#[macro_export]
macro_rules! Or {
    {$enum:ident, $($variant:ident = $ty:ty),+} => {
        #[derive(Debug)]
        enum $enum {
            $(
                $variant($ty),
            )+
        }
        impl TParse for $enum {
            fn tparse(input: &str) -> Option<(Self, usize)> {
                $(
                    if let Some(parsed) = <$ty>::tparse(input) {
                        return Some((Self::$variant(parsed.0), parsed.1))
                    }
                )+
                None
            }
        }
    };
}

// tuples
macro_rules! _impl_tparse_for_tuple_inner {
    ($($generic:ident),+ $(,)?) => {
        impl<$($generic: TParse),+> TParse for ($($generic),+) {
            fn tparse(input: &str) -> Option<(Self, usize)> {
                let mut offset = 0;

                Some((($(
                    {
                        let (parsed, new_offset) = $generic::tparse(&input[offset..])?;
                        offset += new_offset;
                        parsed
                    },
                )+), offset))
            }
        }
    };
}
seq!(I in 2..=32 {
    #(
        seq!(J in 1..=I {
            _impl_tparse_for_tuple_inner!(
                #(P~J,)*
            );
        });
    )*
});

impl<P: TParse> TParse for Vec<P> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let mut out = Vec::new();
        let mut offset = 0;

        while let Some((parsed, new_offset)) = P::tparse(&input[offset..]) {
            out.push(parsed);
            offset += new_offset;
        }

        Some((out, offset))
    }
}

/// Matches at least N consecutive occurrences of P
#[derive(Debug)]
pub struct VecN<const N: usize, P>(pub Vec<P>);
impl<const N: usize, P: TParse> TParse for VecN<N, P> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let (vec, offset) = Vec::tparse(input)?;
        vec.len().ge(&N).then_some((Self(vec), offset))
    }
}

impl<P: TParse> TParse for Option<P> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let parsed = P::tparse(input);

        match parsed {
            None => Some((None, 0)),
            Some((parsed, offset)) => Some((Some(parsed), offset)),
        }
    }
}

/// Lookahead: matches if P does, but without consuming input
#[derive(Debug)]
pub struct Is<P: TParse>(PhantomData<P>);
impl<P: TParse> TParse for Is<P> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        P::tparse(input).map(|_| (Self(PhantomData), 0))
    }
}

/// Negative lookahead: matches if P does *not*, without consuming input
#[derive(Debug)]
pub struct IsNot<P: TParse>(PhantomData<P>);
impl<P: TParse> TParse for IsNot<P> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        match P::tparse(input) {
            None => Some((Self(PhantomData), 0)),
            Some(_) => None,
        }
    }
}

/// Matches if P matched the entire input
#[derive(Debug)]
pub struct AllConsumed<P: TParse>(pub P);
impl<P: TParse> TParse for AllConsumed<P> {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        let (parsed, offset) = P::tparse(input)?;
        offset.eq(&input.len()).then_some((Self(parsed), offset))
    }
}

/// Always matches, contains the remaining length of the input
pub struct RemainingLength(pub usize);
impl TParse for RemainingLength {
    fn tparse(input: &str) -> Option<(Self, usize)> {
        Some((Self(input.len()), 0))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple_concat() {
        type test_str = TStr<"test_str">;

        let tests = [
            ("ctest_str", Some((('c', TStr), 9))),
            ("test_str", None),
            ("\0test_str\0\0\0", Some((('\0', TStr), 9))),
        ];

        for (input, output) in tests {
            let parsed = <(char, test_str)>::tparse(input);
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

    #[test]
    fn test_csv() {
        type Field = (Option<TStr<"-">>, VecN<1, RangedChar<'0', '9'>>);
        type Record = (Field, Vec<(TStr<",">, Field)>, TStr<"\n">);

        type File = AllConsumed<Vec<Record>>;

        let input = "65279,1179403647,1463895090
31415927,27182817,-1618034
-40,-27315
13,42
65537
";
        let parsed = File::tparse(input);
        assert!(parsed.is_some());
    }
}
