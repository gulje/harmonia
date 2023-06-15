/*
 * Copyright (C) 2023 gulje <guljeee@yandex.com>
 *
 * This program is free software: you can redistribute it and/or modify it
 * under the terms of the GNU General Public License as published by the Free
 * Software Foundation, either version 3 of the License, or (at your option)
 * any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT
 * ANY WARRANTY; without even the implied warranty of  MERCHANTABILITY or
 * FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for
 * more details.
 *
 * You should have received a copy of the GNU General Public License along with
 * this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use std::str::Chars;

/// The `Token` enum captures the various categories of tokens that can be
/// encountered while performing lexical analysis.
///
#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Keyword(String),
    Identifier(String),
    Literal(String),
    Operator(String),
    Punctuation(String),
    StringLiteral(String),
    Newline,
}

/// The `Lexer` struct serves as a magical entity responsible for lexical analysis.
/// It takes the input characters as a stream and systematically transforms them into a sequence of meaningful tokens.
/// By applying the rules of the language's syntax, the `Lexer` captures the essence of
/// the input and facilitates further parsing and interpretation.
pub struct Lexer<'a> {
    input: Chars<'a>,
    current_char: Option<char>,
}

static KEYWORDS: &[&str] = &[
    "if",
    "else",
    "select",
    "tracks",
    "playlists",
    "albums",
    "define",
    "sorting",
    "duplication",
    "renaming",
    "exporting",
    "implementation",
    "create",
    "playlist",
    "merge",
    "into",
    "delete",
    "with",
    "rename",
    "to",
    "add",
    "remove",
    "shuffle",
    "export",
    "by",
    "asc",
    "dec",
    "sort",
    "detect",
    "list",
    "and",
    "duplicates",
    "in",
    "selected",
    "save",
    "function",
    "call",
    "tag",
    "shell",
];

impl<'a> Lexer<'a> {
    /// The `new` function creates a new `Lexer` instance, preparing it for lexical analysis.
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();

        Lexer {
            input: chars,
            current_char,
        }
    }

    /// Advances the `Lexer` to the next character in the input stream.
    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

    /// Tokenizes the input.
    pub fn lex(&mut self) -> Vec<Token> {
        let mut tokens = vec![];

        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\t' => {
                    self.advance();
                }
                '\n' => {
                    tokens.push(Token::Newline);
                    self.advance();
                }
                '+' | '-' | '*' | '/' => {
                    tokens.push(Token::Operator(ch.to_string()));
                    self.advance();
                }
                '(' | ')' | '{' | ',' => {
                    tokens.push(Token::Punctuation(ch.to_string()));
                    self.advance();
                }
                '<' | '>' | '=' | '!' => {
                    let mut compound_op = String::from(ch);
                    self.advance();

                    if let Some(next_ch) = self.current_char {
                        if next_ch == '=' {
                            compound_op.push(next_ch);
                            tokens.push(Token::Operator(compound_op));
                            self.advance();
                        } else {
                            tokens.push(Token::Operator(ch.to_string()));
                        }
                    } else {
                        tokens.push(Token::Operator(ch.to_string()));
                    }
                }
                '"' | '\'' => {
                    let quote_char = ch;
                    self.advance();

                    let mut value = String::new();
                    let mut escaped = false;
                    while let Some(c) = self.current_char {
                        if escaped {
                            match c {
                                'n' => value.push('\n'),
                                'r' => value.push('\r'),
                                't' => value.push('\t'),
                                _ => value.push(c),
                            }

                            escaped = false;
                        } else if c == '\\' {
                            escaped = true;
                        } else if c == quote_char {
                            self.advance();
                            break;
                        } else {
                            value.push(c);
                        }
                        self.advance();
                    }

                    tokens.push(Token::StringLiteral(value));
                }
                _ => {
                    if ch.is_alphabetic() {
                        let mut value = String::new();
                        while let Some(c) = self.current_char {
                            if c.is_alphanumeric() || c == '_' {
                                value.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }

                        if KEYWORDS.contains(&value.as_str()) {
                            tokens.push(Token::Keyword(value));
                        } else if value == "contains" {
                            tokens.push(Token::Operator(value));
                        } else {
                            tokens.push(Token::Identifier(value));
                        }
                    } else if ch.is_ascii_digit() {
                        let mut value = String::new();
                        while let Some(c) = self.current_char {
                            if c.is_ascii_digit() {
                                value.push(c);
                                self.advance();
                            } else {
                                break;
                            }
                        }
                        tokens.push(Token::Literal(value));
                    } else {
                        // error
                        self.advance();
                    }
                }
            }
        }

        tokens
    }
}
