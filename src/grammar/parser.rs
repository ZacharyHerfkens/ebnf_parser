//! Parse an ebnf grammar file.
//! The grammar is defined as:
//!     grammar = { rule } ;
//!     rule = ident ':' expr ';' ;
//!     expr = seq { '|' seq } ;
//!     seq = { item } ;
//!     item = ident | terminal | '(' expr ')' | '[' expr ']' | '{' expr '}' ;
//! 
//! Non-terminals are represented by an Ident, and terminals by a Terminal.
//! Terminals must be enclosed in single quotes, and may contain any character
//! except a single quote.
//! Special characters may be included in terminals by using escape the escape
//! sequences:
//!     \t - tab
//!     \n - newline
//!     \r - carriage return
//!     \0 - null
//!     \' - single quote
//!     \\ - backslash