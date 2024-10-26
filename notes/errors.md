Syntax Error: expected SEMICOLON [syntax-error]
Syntax Error: expected expression, item or let statement [syntax-error]

    Checking db v0.1.0 (/home/mohamed/ws/codd)
error[E0599]: no method named `get_key` found for reference `&Node` in the current scope
  --> src/main.rs:62:32
   |
62 |             let cmp = cmp(self.get_key(i), key);
   |                                ^^^^^^^ method not found in `&Node`

error[E0425]: cannot find function `cmp` in this scope
  --> src/main.rs:62:23
   |
62 |             let cmp = cmp(self.get_key(i), key);
   |                       ^^^ not found in this scope

Some errors have detailed explanations: E0425, E0599.
For more information about an error, try `rustc --explain E0425`.
error: could not compile `db` (bin "db") due to 2 previous errors



//    Compiling parcom v0.1.0 (/home/mohamed/ws/parcom)
// error[E0601]: `main` function not found in crate `parcom`
//   |
//   = note: consider adding a `main` function to `src/main.rs`
//
// For more information about this error, try `rustc --explain E0601`.
// error: could not compile `parcom` (bin "parcom") due to 1 previous error
// error[E0600]: cannot apply unary operator `!` to type `[(String, String); 2]`
//    --> src/lib.rs:288:33
//     |
// 288 |                       attributes: ![
//     |  _________________________________^
// 289 | |                         ("id".to_owned(), "container".to_owned()),
// 290 | |                         ("class".to_owned(), "h-full w-full mb-4".to_owned()),
// 291 | |                     ],
//     | |_____________________^ cannot apply unary operator `!`
//
// error[E0599]: the method `parse` exists for unit type `()`, but its trait bounds were not satisfied
//    --> src/lib.rs:296:20
//     |
// 296 |             parser.parse(r#"<div id="container" class="h-full w-full mb-4""#),
//     |                    ^^^^^ method cannot be called on `()` due to unsatisfied trait bounds
//     |
// note: the following trait bounds were not satisfied:
//       `(): Fn<(&str,)>`
//       `<() as FnOnce<(&str,)>>::Output = Result<(_, &str), &str>`
//    --> src/lib.rs:9:8
//     |
// 7   | impl<'a, F, Output> Parser<'a, Output> for F
//     |                     ------------------     -
// 8   | where
// 9   |     F: Fn(&'a str) -> ParseResult<'a, Output>,
//     |        ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//     |        |              |
//     |        |              unsatisfied trait bound introduced here
//     |        unsatisfied trait bound introduced here
//     = help: items from traits can only be used if the trait is implemented and in scope
// note: `Parser` defines an item `parse`, perhaps you need to implement it
//    --> src/lib.rs:3:1
//     |
// 3   | pub trait Parser<'a, Output> {
//     | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^
//
