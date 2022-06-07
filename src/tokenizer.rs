use std::fs::File;
use std::io::Read;

use crate::token::{
    ArchimedesSpan,
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

    fn read_next_token(&self) -> Token {
        self.consume_useless_bytes();

        return match self.byte_at(cursor) {
            b'0' | b'1' | b'2' |
            b'3' | b'4' | b'5' |
            b'6' | b'7' | b'8' |
            b'9' | => return tokenize_number(),

            _ => Token::Trash(ArchimedesSpan {
                file_id: 0,
                start: self.cursor,
                end: self.cursor + 1
            })
        }
    }
}
