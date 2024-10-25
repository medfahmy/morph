```
impl Parser {
    parse_ident | Self ref mut, String -> Result<Stmt>;
    parse_ident = self ref mut, ident -> {
        token = self.peek ? Ok { Stmt.Expr { Expr.Ident { ident } } };

        match token.kind {
            Assign => {
                self : bump;
                expr = self : parse_expr ?;
                Ok { Stmt.Binding { ident, expr } }
            },
            Pipe => todo(),
            _ => Err { 
                "Expected binding or type signature",
                self.peek,
            }
        }
    }
}
```
