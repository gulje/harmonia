use std::str::Chars;

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
    pub fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current_char = chars.next();

        Lexer {
            input: chars,
            current_char,
        }
    }

    fn advance(&mut self) {
        self.current_char = self.input.next();
    }

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
