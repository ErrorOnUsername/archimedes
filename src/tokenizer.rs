use std::fs::File;
use std::io::Read;

use crate::token::{
    Span,
    NumericConstant,
    Token
};

pub struct Tokenizer {
    main_file_path: String,
    current_file_contents: String,
    current_file_size: usize,
    cursor: usize,
    line: usize,
}

impl Tokenizer {
    pub fn new(main_file_path: String) -> Self {
        let mut file = File::open(&main_file_path).expect("Could't open main file!!");
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).expect("Couldn't read main file!!");

        Self {
            main_file_path,
            current_file_contents: file_contents,
            current_file_size: file.metadata().expect("Couldn't get the main file's meta").len() as usize,
            cursor: 0,
            line: 0,
        }
    }

    fn byte_at(&self, idx: usize) -> u8 {
        self.current_file_contents.as_bytes()[idx]
    }

    fn at_eof(&self) -> bool {
        self.cursor >= self.current_file_size
    }

    fn is_current_whitespace(&self) -> bool {
        !self.at_eof() || self.byte_at(self.cursor) == b' '
                       || self.byte_at(self.cursor) == b'\t'
                       || self.byte_at(self.cursor) == b'\r'
    }

    fn is_comment(&mut self) -> bool {
        if !self.at_eof() && self.byte_at(self.cursor) == b'/' {
            if self.cursor + 1 < self.current_file_size && self.byte_at(self.cursor + 1) == b'/' {
                // We're looking at a line comment rn

                loop {
                    self.cursor += 1;
                    if !self.at_eof() && self.byte_at(self.cursor) == b'\n' {
                        self.line += 1;
                        self.cursor += 1;
                        return true;
                    }
                }
            } else if self.cursor + 1 < self.current_file_size && self.byte_at(self.cursor + 1) == b'*' {
                // We're looking at a block comment rn

                // Increment once beforehand so we don't accidentally read
                // the first '*' as the closing '*'
                self.cursor += 1;
                loop {
                    self.cursor += 1;

                    if !self.at_eof() && self.byte_at(self.cursor) == b'\n' {
                        self.line += 1;
                    }

                    if !self.at_eof() && self.byte_at(self.cursor) == b'*' {
                        if self.cursor + 1 < self.current_file_size && self.byte_at(self.cursor + 1) == b'/' {
                            self.cursor += 2;
                            return true;
                        }

                        self.cursor += 1;
                    }
                }
            }
        }

        false
    }

    fn consume_useless_bytes(&mut self) {
        loop {
            if !self.is_current_whitespace() && !self.is_comment() {
                break;
            }
            self.cursor += 1;
        }
    }

    fn tokenize_number(&self) -> Token {
        Token::Number(
            Span { file_id: 0, start: 0, end: 0 },
            NumericConstant::Integer(0)
        )
    }

    fn tokenize_names(&self) -> Token {
        Token::IdentName(
            Span { file_id: 0, start: 0, end: 0 },
            String::new()
        )
    }

    pub fn read_next_token(&mut self) -> Token {
        if self.at_eof() {
            return Token::EOF(
                Span {
                    file_id: 0,
                    start: self.cursor,
                    end: self.cursor + 1 
                }
            );
        }

        self.consume_useless_bytes();

        return match self.byte_at(self.cursor) {
            b'\n' => Token::EOL(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'0' | b'1' | b'2' |
            b'3' | b'4' | b'5' |
            b'6' | b'7' | b'8' |
            b'9' => self.tokenize_number(),

            b'~' => Token::Tilde(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'!' => self.tokenize_bang_variations(),

            b'#' => Token::Hash(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'$' => Token::Dollar(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'%' => self.tokenize_percent_variations(),

            b'^' => self.tokenize_caret_variations(),

            b'&' => self.tokenize_ampersand_variations(),

            b'*' => self.tokenize_star_variations(),

            b'(' => Token::LParam(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),
            b')' => Token::RParam(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'-' => self.tokenize_minus_variations(),
            b'+' => self.tokenize_plus_variations(),

            b'=' => self.tokenize_equals_variations(),

            b'[' => Token::LSquare(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),
            b'[' => Token::RSquare(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'|' => self.tokenize_pipe_variations(),

            b'{' => Token::LCurly(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),
            b'}' => Token::RCurly(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b';' => Token::Semicolon(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b':' => self.tokenize_colon_variations(),

            b'\'' => self.try_tokenize_char_literal(),
            b'"' => self.try_tokenize_string_literal(),

            b',' => Token::Comma(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),
            b'.' => Token::Dot(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'/' => self.tokenize_slash_variations(),

            b'<' => Token::LAngle(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),
            b'>' => Token::RAngle(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            b'?' => Token::QuestionMark(Span { file_id: 0, start: self.cursor, end: self.cursor + 1 }),

            _ => self.tokenize_names(),
        }
    }
}
