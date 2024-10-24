```
pub left, whitespace_char, map;


pair |  -> 

left | P1, P2 -> impl Parser<R1>
    where P1: Parser<R1>, 
          P2: Parser<R2>;
left = (parser1, parser2) -> map(pair(parser1, parser2), (left, right) -> right);



pub whitespace_char;

whitespace_char | String -> ParseResult<Char>;
whitespace_char = input -> {
    pred(anychar, c -> c.is_whitespace)
};


```
