Type-level parser combinators.

## Parsers
### TStr<STR>
Exactly matches the compile-time string STR
### char
Matches any single unicode character
### RangedChar<START, END>
Matches any single unicode character between START and END
### RemainingLength
Always matches, contains the remaining length of the input

## Parser Combinators
### (P1, P2, ..., P32)
Matches each child parser in order, up to length 32.
### Or!(EnumName, VariantName1 = P1, VariantName2 = P2, ...)
Creates an Enum with one variant per child parser.
Tries each child parser in order, returning the variant corresponding to the first successful match.
### Or<(P1, P2, ..., P16)>
Tries each child parser in order (up to 16), storing the first successful match.
Use .matcher() to match on the child parsers.
Can be used inside other matchers, as it doesn't require the creation of an enum.
#### Matcher
A matcher type-level guaranteeing a match on all possibilities.
Uses a builder-like pattern, but on tuples, to emulate an enum without having to create one, so this type (and thus `Or`) can be used inside other types.
Use AddMatcher<I>::add_matcher() to add a matcher for the parser at index I.
Once a matcher for all parsers is added, use .do_match().
### Vec<P>
Matches any number of consecutive occurrences of P
### VecN<P>
Matches at least N consecutive occurrences of P
### Option<P>
Always matches, returns None if P failed and the result if P succeeded
### Is<P>
Lookahead: matches if P does, but without consuming input
### IsNot<P>
Negative lookahead: matches if P does *not*, but without consuming input
### AllConsumed<P>
Matches if P matched the entire input

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
