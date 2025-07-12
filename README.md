Type-level parser combinators.

The following basic parsers are implemented:
- `TStr<STR>`
  - Exactly matches the compile-time string STR
- `char`
  - Matches any single unicode character
- `RangedChar<START, END>`
  - Matches any single unicode character between START and END

The following parser combinators are implemented:
- `Or!(EnumName, P1, P2, ...)`
  - Tries each child parser in order, returning the first successful match
- `Concat!(StructName, P1, P2, ...)`
  - Matches each child parser in order
- `Vec<P>`
  - Matches any number of consecutive occurrences of P
- `VecN<P>`
  - Matches at least N consecutive occurrences of P
- `Option<P>`
  - Always matches, returns None if P failed and the result if P succeeded
- `Is<P>`
  - Lookahead: matches if P does, but without consuming input
- `IsNot<P>`
  - Negative lookahead: matches if P does *not*, but without consuming input
- `AllConsumed<P>`
  - Matches if P matched the entire input

> [!NOTE]
Currently requires the nightly compiler
