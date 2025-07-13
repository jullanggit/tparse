Type-level parser combinators.

## Parsers
- `TStr<STR>`
  - Exactly matches the compile-time string STR
- `char`
  - Matches any single unicode character
- `RangedChar<START, END>`
  - Matches any single unicode character between START and END

## Parser Combinators
- `Or!(EnumName, P1, P2, ...)`
  - Tries each child parser in order, returning the first successful match
- `Or!(EnumName, VariantName1 = P1, VariantName2 = P2, ...)`
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

## Examples
A CSV file containing integers
```rust
type OptionMinus = Option<TStr<"-">>;
type Digits = VecN<1, RangedChar<'0', '9'>>;
Concat! {Field, OptionMinus, Digits};

type Comma = TStr<",">;
Concat!(CommaField, Comma, Field);
type CommaFields = Vec<CommaField>;

type Newline = TStr<"\n">;
Concat!(Record, Field, CommaFields, Newline);

type File = AllConsumed<Vec<Record>>;

let input = "65279,1179403647,1463895090
31415927,27182817,-1618034
-40,-27315
13,42
65537
";
let parsed = File::tparse(input);
assert!(parsed.is_some());
```

> [!NOTE]
Currently requires the nightly compiler
