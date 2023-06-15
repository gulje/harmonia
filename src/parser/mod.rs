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
use crate::lexer::Token;

pub struct Parser {
    tokens: Vec<Token>,
    current_token: Option<Token>,
    selected_tracks: Vec<String>,
    selected_playlists: Vec<String>,
}

impl Parser {
    /// # Overview
    /// The `new` function conjures a new instance of the `Parser` struct.
    ///
    /// # Usage
    /// To create a new `Parser` instance and begin your parsing journey,
    /// simply invoke the `Parser::new` function and pass the `tokens`.
    ///
    /// ```rust
    /// let tokens: Vec<Token> = /* Obtain tokens */;
    /// let mut parser = Parser::new(tokens);
    /// ```
    pub fn new(tokens: Vec<Token>) -> Self {
        let selected_tracks = vec![];
        let selected_playlists = vec![];
        Self {
            tokens,
            current_token: None,
            selected_tracks,
            selected_playlists,
        }
    }

    /// To advance the `Parser` and move to the next token, simply invoke the `advance` function.
    /// It will update the `current_token` accordingly.
    fn advance(&mut self) {
        if self.tokens.is_empty() {
            self.current_token = None;
        } else {
            self.current_token = Some(self.tokens.remove(0));
        }
    }

    /// # Overview
    /// The `parse` function is responsible for orchestrating the parsing process within the `Parser` instance.
    /// It traverses through the tokens and performs different actions based on the encountered token type.
    /// It gracefully handles keywords, identifiers, literals, operators, punctuation, and newlines,
    /// unlocking the secrets of the parsed data.
    ///
    /// # Usage
    /// To initiate the parsing process, simply invoke the `parse` function on the `Parser` instance.
    /// It will navigate through the tokens, matching patterns and executing the corresponding actions.
    /// The magic of parsing will unfold before your eyes.
    ///
    /// ```rust
    /// let mut parser = Parser::new(/* Provide tokens */);
    ///
    /// // Begin the parsing adventure
    /// parser.parse();
    /// ```
    pub fn parse(&mut self) {
        self.advance();
        while let Some(token) = &self.current_token {
            match token {
                Token::Keyword(keyword) => match keyword.as_str() {
                    "create" => {
                        let expr = self.parse_create();
                        if expr.tracks.is_empty() {
                            println!("Create playlist '{}'", expr.playlist_name);
                        } else {
                            println!(
                                "Create playlist '{}' with tracks: {:?}",
                                expr.playlist_name, expr.tracks
                            );
                        }
                    }
                    "merge" => {
                        let expr = self.parse_merge();

                        if expr.playlists.len() < 2 {
                            panic!("Expected two or more playlists");
                        } else {
                            println!("Merge playlists {:?} into {}", expr.playlists, expr.into);
                        }
                    }
                    _ => {
                        self.advance();
                    }
                },
                Token::Identifier(_identifier) => {}
                Token::Literal(_literal) => {}
                Token::Operator(_operator) => {}
                Token::Punctuation(_punctuation) => {}
                Token::StringLiteral(_string_literal) => {}
                Token::Newline => {
                    self.advance();
                }
            }
        }
    }

    /// The `parse_create` function is responsible for parsing the creation expression within the `Parser` instance.
    /// It captures the playlist name and optionally extracts the tracks associated with it.
    /// This function gracefully handles different scenarios based on the encountered tokens,
    /// ensuring a harmonious creation process.
    fn parse_create(&mut self) -> CreateExpr {
        self.advance();
        self.capture_keyword("playlist");
        let playlist_name = self.capture_string_literal("Expected playlist name");

        self.skip_newline();

        if let Some(Token::Keyword(keyword)) = &self.current_token {
            if keyword == "with" {
                self.advance();
                let tracks = self.capture_tracks();

                CreateExpr {
                    playlist_name,
                    tracks,
                }
            } else {
                panic!("Invalid syntax")
            }
        } else {
            CreateExpr {
                playlist_name,
                tracks: vec![],
            }
        }
    }

    /// The `parse_merge` function is responsible for parsing the merging expression within the `Parser` instance.
    /// It captures the source playlists and the destination playlist where they will be merged.
    /// This function gracefully handles different scenarios based on the encountered tokens, ensuring a seamless
    /// merging process.
    fn parse_merge(&mut self) -> MergeExpr {
        self.advance();

        let playlists = self.capture_playlists();

        self.skip_newline();

        if let Some(Token::Keyword(keyword)) = &self.current_token {
            if keyword == "into" {
                self.advance();

                let into = self.capture_string_literal("Expected playlist name");

                MergeExpr { playlists, into }
            } else {
                panic!("Invalid syntax")
            }
        } else {
            panic!("Invalid syntax")
        }
    }

    /// The `skip_newline` function is responsible for skipping over newline tokens encountered during the parsing process within the `Parser` instance.
    /// It checks the current token and advances the parsing cursor if a newline token is found.
    /// This function helps maintain the flow of parsing by effortlessly handling newlines.
    fn skip_newline(&mut self) {
        if self.current_token == Some(Token::Newline) {
            self.advance();
        }
    }

    /// The `capture_tracks` function is responsible for capturing the tracks within the parsing process of the `Parser` instance.
    fn capture_tracks(&mut self) -> Vec<String> {
        let mut tracks = Vec::new();

        if let Some(Token::Keyword(keyword)) = &self.current_token {
            if keyword == "selected" {
                self.advance();

                if let Some(Token::Keyword(keyword)) = &self.current_token {
                    if keyword == "tracks" {
                        tracks = self.selected_tracks.clone();
                        self.advance();
                        return tracks;
                    }
                }

                panic!("Invalid syntax")
            }
        }

        loop {
            let track_name = self.capture_string_literal("Expected a track");
            tracks.push(track_name);

            if let Some(Token::Punctuation(punctuation)) = &self.current_token {
                if punctuation == "," {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        tracks
    }

    /// The `capture_playlists` function is responsible for capturing the playlists within the parsing process of the `Parser` instance.
    fn capture_playlists(&mut self) -> Vec<String> {
        let mut playlists = Vec::new();

        if let Some(Token::Keyword(keyword)) = &self.current_token {
            if keyword == "selected" {
                self.advance();

                if let Some(Token::Keyword(keyword)) = &self.current_token {
                    if keyword == "playlists" {
                        playlists = self.selected_playlists.clone();
                        self.advance();
                        return playlists;
                    }
                }

                panic!("Invalid syntax")
            }
        }

        loop {
            let playlist_name = self.capture_string_literal("Expected a playlist");
            playlists.push(playlist_name);

            if let Some(Token::Punctuation(punctuation)) = &self.current_token {
                if punctuation == "," {
                    self.advance();
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        playlists
    }

    /// The `capture_keyword` function is responsible for capturing the keywords within the parsing process of the `Parser` instance.
    fn capture_keyword(&mut self, expected: &str) {
        if let Some(Token::Keyword(value)) = self.current_token.take() {
            if value == expected {
                self.advance();
            } else {
                Self::error(&format!("Expected keyword: '{expected}'"));

                panic!()
            }
        } else {
            Self::error(&format!("Expected keyword: '{expected}'"));

            panic!()
        }
    }

    /// The `capture_string_literal` function is responsible for capturing the string literals within the parsing process of the `Parser` instance.
    fn capture_string_literal(&mut self, error_msg: &str) -> String {
        if let Some(Token::StringLiteral(value)) = self.current_token.take() {
            self.advance();
            value
        } else {
            Self::error(error_msg);

            panic!()
        }
    }

    /// The `error` function is responsible for raising an error and terminating the program flow with the provided error message.
    fn error(message: &str) {
        panic!("{}", message)
    }
}

/// The `CreateExpr` struct represents the result of parsing a playlist
/// creation expression within the `Parser` instance.
struct CreateExpr {
    playlist_name: String,
    tracks: Vec<String>,
}

/// The `MergeExpr` struct represents the result of parsing a playlist merging
/// expression within the `Parser` instance.
struct MergeExpr {
    playlists: Vec<String>,
    into: String,
}
