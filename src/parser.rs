use crate::ast::*;
use crate::token::Token;

pub struct Parser {
    ast: Vec<ParsedStatement>,
    token_idx: usize
}

impl Parser {
    pub fn new() -> Self {
        Self {
            ast: Vec::new(),
            token_idx: 0
        }
    }

    pub fn parse_token_stream(&mut self, token_stream: &[Token]) {
        let token = &token_stream[self.token_idx];

        match token {
            Token::EOF => return,
            Token::EOL(_span) => {
                /* Just ignore EOLs */
                self.token_idx += 1;
            },

            Token::KeywordImport(span) => {
                panic!("Handle imports in parser! {:?}", span);
            },

            Token::KeywordDecl(_span) => {
                // First determine what complex type we're using.
                // This could be one of the following:
                //
                //  - procedure:
                //      decl something : () { }
                //  - struct:
                //      decl SomeType : struct { }
                //  - enum:
                //      decl SomeEnum : enum { }
                //
                // If it's none of those, we'll just freak out.
                self.token_idx += 1;
                let _ident = match &token_stream[self.token_idx] {
                    Token::IdentName(_span, name) => name.clone(),
                    _ => panic!("Syntax Error! Expected identifier, but got something else!")
                };

                self.token_idx += 1;
                match &token_stream[self.token_idx] {
                    Token::Colon(_span) => { },
                    _ => panic!("Syntax Error! Expected a colon, but got something else!")
                }

                self.token_idx += 1;
                let complex_type = match &token_stream[self.token_idx] {
                    Token::LParen(_span) => 0,
                    Token::KeywordStruct(_span) => 1,
                    Token::KeywordEnum(_span) => 2,
                    _ => panic!("Syntax Error! Expected complex type identifier ('struct', 'enum', '()'), but got something else!")
                };

                self.token_idx += 1;
                if complex_type == 0 {
                    // FIXME: Actually parse parameters
                    match &token_stream[self.token_idx] {
                        Token::RParen(_span) => { },
                        _ => panic!("Syntax Error! Unterminated '(' in procedure definition")
                    }
                }

                let _body = self.parse_block(token_stream);

                match complex_type {
                    0 => {
                        // Procedure
                    },

                    1 => {
                        // Struct
                    },

                    2 => {
                        // Enum
                    },

                    _ => { }
                }

                unreachable!();
            }

            Token::KeywordLet(_span) => {
                // This will always produce a ParsedVarDecl
                unreachable!();
            }

            _ => {
                // FIXME: Propagate this error rather than just freaking out...
                panic!("Unexpected token: {:?}", token);
            }
        }
    }

    fn parse_block(&mut self, _token_stream: &[Token]) -> ParsedBlock { panic!("parse_block"); }

    fn parse_expression(&mut self, _token_stream: &[Token]) -> ParsedExpression { panic!("parse_block"); }
}
