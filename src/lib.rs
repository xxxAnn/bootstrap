//! We are trying to parse the .tb specification
//! The .tb specification is a text file that describes the parsers
//! 
//! Parsers are defined as 
//! ```
//! parser(<name>) ...code...
//! ```
//! Checkers are defined as
//! ```
//! check(<name>) ...code...
//! ```
//! Where the code is a sequence of combinators and parsers
//! Combinators can take parsers as arguments
//! Parsers can take conditions as arguments
//! Conditions are either literals or checkers
//! Checkers are a combination of checkers and combinators
//! Literals are either strings, characters or numbers
//! 
//! We parse the following base parsers:
//! - `char`     Params: literal-character
//! - `string`   Params: literal-string
//! - `take`     Params: literal-number
//! - `while`    Params: either a literal or a checker
//! - `until`    Params: either a literal or a checker
//! 
//! We have the following combinations
//! (a, b) -> a followed by b
//! [a, b] -> a or b
//! (*a)   -> zero or more a
//! (+a)   -> one or more a
//! 
//! The parser module (bootstrap version) should return a list of parsers and a list of checkers
//! Each parser should be equivalent to a Combinator enum which contains Combinator/Parser/Checkers...\
use nom;

