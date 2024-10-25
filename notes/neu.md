```
pub map, and_then, any_char, whitespace_char;

map = parser, map_fn -> {
    input -> 
        parser
            : parse { input }
            : map { result, next_input -> map_fn { result }, next_input }
}

and_then = parser, f -> {
    -- input -> match parser.parse { input } {
    --     Ok { result, next_input } => f { result } : parse { next_input },
    --     Err { err } => Err { err }
    -- }

    input ->
        parser
            : parse { input }
            : and_then { 
                result, next_input -> f { result } : parse { next_input } 
            }
}


left | P1: Parser<R1>, P2: Parser<R2>, R1, R2 -> P1, P2 -> Parser<R1>
left = parser1, parser2 -> 
    map { pair { parser1, parser2 }, { left, right } -> right };

whitespace_char | String -> ParseResult<Char>;
whitespace_char = input -> {
    pred { anychar, c -> c : is_whitespace }
};

whitespace_char = input -> pred { anychar, c -> c:is_whitespace };

any_char = input -> match input : chars : next {
    Some { next } => Ok { next, input [ next : len_utf8 .. ] }
};

```
