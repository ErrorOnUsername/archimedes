use std::fs::File;
use std::io::Read;

use crate::token::{
    Span,
    NumberLiteralFormat,
    NumericConstant,
    PrimitiveType,
    Token
};

const NUMBER_LITERAL_SEPERATOR: u8 = b'\'';

pub struct Tokenizer {
    current_file_contents: String,
    current_file_size: usize,
    cursor: usize,
    line: usize,
    in_block_comment: bool
}

fn is_valid_identifier_char(c: u8) -> bool {
    (c >= b'A' && c <= b'Z') || (c >= b'a' && c <= b'z')
                             || (c >= b'0' && c <= b'9')
                             || (c == b'_')
}

fn is_valid_number_literal_char(c: u8) -> bool {
    (c >= b'0' && c <= b'9') || (c >= b'a' && c <= b'f')
                             || (c >= b'A' && c >= b'F')
                             || (c == NUMBER_LITERAL_SEPERATOR)
}

impl Tokenizer {
    pub fn new(main_file_path: String) -> Self {
        let mut file = File::open(&main_file_path).expect("Could't open main file!!");
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).expect("Couldn't read main file!!");

        Self {
            current_file_contents: file_contents,
            current_file_size: file.metadata().expect("Couldn't get the main file's meta").len() as usize,
            cursor: 0,
            line: 0,
            in_block_comment: false
        }
    }

    fn byte_at(&self, idx: usize) -> u8 {
        self.current_file_contents.as_bytes()[idx]
    }

    fn at_eof(&self) -> bool {
        self.cursor >= self.current_file_size
    }

    fn is_eof(&self, idx: usize) -> bool {
        idx >= self.current_file_size
    }

    fn is_current_whitespace(&self) -> bool {
        !self.at_eof() && (self.byte_at(self.cursor) == b' '
                       || self.byte_at(self.cursor) == b'\t'
                       || self.byte_at(self.cursor) == b'\r')
    }

    fn is_comment_consumed(&mut self) -> bool {
        if !self.at_eof() && self.byte_at(self.cursor) == b'/' {
            if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'/' {
                // We're looking at a line comment rn

                loop {
                    self.cursor += 1;
                    if !self.at_eof() && self.byte_at(self.cursor) == b'\n' {
                        self.line += 1;
                        self.cursor += 1;
                        return true;
                    }
                }
            } else if self.in_block_comment || (!self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'*') {
                // We're looking at a block comment rn
                self.in_block_comment = true;

                // Increment once beforehand so we don't accidentally read
                // the first '*' as the closing '*'
                self.cursor += 1;
                loop {
                    self.cursor += 1;

                    if !self.at_eof() && self.byte_at(self.cursor) == b'\n' {
                        return false;
                    }

                    if !self.at_eof() && self.byte_at(self.cursor) == b'*' {
                        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'/' {
                            self.cursor += 2;
                            self.in_block_comment = false;
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
        while self.is_current_whitespace() && !self.is_comment_consumed() {
            self.cursor += 1;
        }
    }

    pub fn read_next_token(&mut self) -> Token {
        if self.at_eof() {
            return Token::EOF;
        }

        self.consume_useless_bytes();

        return match self.byte_at(self.cursor) {
            b'\n' => {
                self.cursor += 1;
                Token::EOL(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b'0' | b'1' | b'2' |
            b'3' | b'4' | b'5' |
            b'6' | b'7' | b'8' |
            b'9' => self.tokenize_number(),

            b'~' => self.tokenize_tilde_variations(),

            b'!' => self.tokenize_bang_variations(),

            b'#' => {
                self.cursor += 1;
                Token::Hash(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b'$' => {
                self.cursor += 1;
                Token::Dollar(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b'%' => self.tokenize_percent_variations(),

            b'^' => self.tokenize_caret_variations(),

            b'&' => self.tokenize_ampersand_variations(),

            b'*' => self.tokenize_star_variations(),

            b'(' => {
                self.cursor += 1;
                Token::LParen(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b')' => {
                self.cursor += 1;
                Token::RParen(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b'-' => self.tokenize_dash_variations(),
            b'+' => self.tokenize_plus_variations(),

            b'=' => self.tokenize_equals_variations(),

            b'[' => {
                self.cursor += 1;
                Token::LSquare(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b']' => {
                self.cursor += 1;
                Token::RSquare(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b'|' => self.tokenize_pipe_variations(),

            b'{' => {
                self.cursor += 1;
                Token::LCurly(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b'}' => {
                self.cursor += 1;
                Token::RCurly(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b';' => {
                self.cursor += 1;
                Token::Semicolon(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b':' => self.tokenize_colon_variations(),

            b'\'' => self.tokenize_char_literal(),
            b'"' => self.tokenize_string_literal(),

            b',' => {
                self.cursor += 1;
                Token::Comma(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            b'.' => self.tokenize_dot_variations(),

            b'/' => self.tokenize_slash_variations(),

            b'<' => self.tokenize_left_angle_variations(),
            b'>' => self.tokenize_right_angle_variations(),

            b'?' => {
                self.cursor += 1;
                Token::QuestionMark(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
            },

            _ => self.tokenize_names(),
        }
    }

    fn tokenize_tilde_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::TildeAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Tilde(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_bang_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::NEQ(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Bang(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_percent_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::PercentAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Percent(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_caret_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::CaretAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'^' {
            self.cursor += 2;

            return Token::DoubleCaret(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Caret(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_ampersand_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::AmpersandAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'&' {
            self.cursor += 2;

            return Token::DoubleAmpersand(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Ampersand(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_star_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::StarAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Star(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_dash_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::MinusAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'-' {
            self.cursor += 2;

            return Token::MinusMinus(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'>' {
            self.cursor += 2;

            return Token::ThinArrow(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Minus(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_plus_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::PlusAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'+' {
            self.cursor += 2;

            return Token::PlusPlus(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Plus(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_equals_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::EQ(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'>' {
            self.cursor += 2;

            return Token::ThiccArrow(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Assign(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_pipe_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::PipeAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'|' {
            self.cursor += 2;

            return Token::DoublePipe(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Pipe(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_colon_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::ColonAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b':' {
            self.cursor += 2;

            return Token::DoubleColon(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Colon(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_char_literal(&mut self) -> Token {
        Token::Trash(Span { file_id: 0, start: 0, end: 0})
    }

    fn tokenize_string_literal(&mut self) -> Token {
        Token::Trash(Span { file_id: 0, start: 0, end: 0})
    }

    fn tokenize_dot_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'.' {
            self.cursor += 2;

            return Token::DotDot(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Dot(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_slash_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::SlashAssign(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::Slash(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_left_angle_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::LEQ(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'<' {
            self.cursor += 2;

            if !self.at_eof() && self.byte_at(self.cursor) == b'=' {
                self.cursor += 1;

                return Token::LShiftAssign(Span { file_id: 0, start: self.cursor - 3, end: self.cursor });
            }

            return Token::LShift(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::LAngle(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_right_angle_variations(&mut self) -> Token {
        if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'=' {
            self.cursor += 2;

            return Token::GEQ(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'<' {
            self.cursor += 2;

            if !self.at_eof() && self.byte_at(self.cursor) == b'=' {
                self.cursor += 1;

                return Token::RShiftAssign(Span { file_id: 0, start: self.cursor - 3, end: self.cursor });
            }

            return Token::RShift(Span { file_id: 0, start: self.cursor - 2, end: self.cursor });
        }

        self.cursor += 1;

        Token::RAngle(Span { file_id: 0, start: self.cursor - 1, end: self.cursor })
    }

    fn tokenize_number(&mut self) -> Token {
        let mut base = NumberLiteralFormat::Decimal;
        let mut num_str = String::new();

        // Parse base prefix (if present)
        if self.byte_at(self.cursor) == b'0' {
            if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'x' {
                self.cursor += 2;
                base = NumberLiteralFormat::Hexadecimal;
            } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'o' {
                self.cursor += 2;
                base = NumberLiteralFormat::Octal;
            } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) == b'b' {
                self.cursor += 2;
                base = NumberLiteralFormat::Binary;
            } else if !self.is_eof(self.cursor + 1) && self.byte_at(self.cursor + 1) != b'.' {
                // FIXME: Propagate error rather that just freaking out
                panic!("Leading zero with no known base prefix at cursor: {}", self.cursor);
            }
        }

        while is_valid_number_literal_char(self.byte_at(self.cursor)) {
            if self.byte_at(self.cursor) == NUMBER_LITERAL_SEPERATOR {
                self.cursor += 1;
                continue;
            }

            if self.byte_at(self.cursor) == b'.' {
                panic!("We can't handle floating points yet!");
            }

            num_str.push(self.byte_at(self.cursor) as char);

            self.cursor += 1;
        }

        Token::Number(
            Span { file_id: 0, start: self.cursor - num_str.len(), end: self.cursor },
            NumericConstant::Integer(num_str, base)
        )
    }

    fn tokenize_names(&mut self) -> Token {
        let mut ident = String::new();

        while is_valid_identifier_char(self.byte_at(self.cursor)) {
            ident.push(self.byte_at(self.cursor) as char);
            self.cursor += 1;
        }

        if ident.len() == 0 {
            // FIXME: Propagate error rather that just freaking out
            panic!("Unknown token \"{}\" at cursor: {}", self.byte_at(self.cursor) as char, self.cursor);
        }

        let start = self.cursor - ident.len();
        let end = self.cursor;

        return match ident.as_str() {
            "import" => Token::KeywordImport(Span { file_id: 0, start, end }),

            "decl" => Token::KeywordDecl(Span { file_id: 0, start, end }),

            "let" => Token::KeywordLet(Span { file_id: 0, start, end }),

            "mut" => Token::KeywordMut(Span { file_id: 0, start, end }),

            "struct" => Token::KeywordStruct(Span { file_id: 0, start, end }),

            "enum" => Token::KeywordEnum(Span { file_id: 0, start, end }),

            "match" => Token::KeywordMatch(Span { file_id: 0, start, end }),

            "if" => Token::KeywordIf(Span { file_id: 0, start, end }),

            "else" => Token::KeywordElse(Span { file_id: 0, start, end }),

            "for" => Token::KeywordFor(Span { file_id: 0, start, end }),

            "while" => Token::KeywordWhile(Span { file_id: 0, start, end }),

            "continue" => Token::KeywordContinue(Span { file_id: 0, start, end }),

            "break" => Token::KeywordBreak(Span { file_id: 0, start, end }),

            "return" => Token::KeywordReturn(Span { file_id: 0, start, end }),

            "nothing" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::Nothing
                )
            },

            "bool" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::Bool
                )
            },

            "char" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::Char
                )
            },

            "u8" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::U8
                )
            },

            "i8" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::I8
                )
            },

            "u16" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::U16
                )
            },

            "i16" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::I16
                )
            },

            "u32" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::U32
                )
            },

            "i32" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::I32
                )
            },

            "u64" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end},
                    PrimitiveType::U64
                )
            },

            "i64" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::I64
                )
            },

            "f32" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::F32
                )
            },

            "f64" => {
                Token::BuiltinType(
                    Span { file_id: 0, start, end },
                    PrimitiveType::F64
                )
            },

            _ => {
                Token::IdentName(
                    Span { file_id: 0, start, end },
                    ident
                )
            }
        }
    }
}
