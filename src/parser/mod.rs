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
    pub fn new(tokens: Vec<Token>) -> Self {
        let selected_tracks = vec![];
        let selected_playlists = vec![];
        Parser {
            tokens,
            current_token: None,
            selected_tracks,
            selected_playlists,
        }
    }

    fn advance(&mut self) {
        if self.tokens.is_empty() {
            self.current_token = None;
        } else {
            self.current_token = Some(self.tokens.remove(0));
        }
    }

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
                            println!("Merge playlists {:?} into {}", expr.playlists, expr.into)
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

    fn skip_newline(&mut self) {
        if let Some(Token::Newline) = self.current_token {
            self.advance();
        }
    }

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
                    } else {
                        panic!("Invalid syntax")
                    }
                } else {
                    panic!("Invalid syntax")
                }
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
                    } else {
                        panic!("Invalid syntax")
                    }
                } else {
                    panic!("Invalid syntax")
                }
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

    fn capture_keyword(&mut self, expected: &str) {
        if let Some(Token::Keyword(value)) = self.current_token.take() {
            if value == expected {
                self.advance();
            } else {
                Parser::error(&format!("Expected keyword: '{expected}'"));

                panic!()
            }
        } else {
            Parser::error(&format!("Expected keyword: '{expected}'"));

            panic!()
        }
    }

    fn capture_string_literal(&mut self, error_msg: &str) -> String {
        if let Some(Token::StringLiteral(value)) = self.current_token.take() {
            self.advance();
            value
        } else {
            Parser::error(error_msg);

            panic!()
        }
    }

    fn error(message: &str) {
        panic!("{}", message)
    }
}

struct CreateExpr {
    playlist_name: String,
    tracks: Vec<String>,
}

struct MergeExpr {
    playlists: Vec<String>,
    into: String,
}
