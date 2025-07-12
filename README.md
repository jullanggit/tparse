Type-level parser combinators.

The following basic parsers are implemented:
- `TStr<const STR: &'static str>`
  - Exactly matches the compile-time string STR
- `char`
  - Matches any single unicode character

The following parser combinators are implemented:
- `Or!(EnumName, P1, P2, ...)`
  - Tries each child parser in order, returning the first successful match
- `Concat!(StructName, P1, P2, ...)`
  - Matches each child parser in order
- `Vec<P>`
  - Match any number of consecutive occurrences of P
- `VecN<P>`
  - Match at least N consecutive occurrences of P
- `Option<P>`
  - Always matches, returns None if P failed and the result if P succeeded
- `Is<P>`
  - Lookahead: matches if P does, but without consuming input
- `IsNot<P>`
  - Negative lookahead: matches if P does *not*, but without consuming input
- `AllConsumed<P>`
  - Match if P matched the entire input

> [!NOTE]
Currently requires the nightly compiler
