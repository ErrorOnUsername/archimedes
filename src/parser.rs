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

    pub fn parse_token_stream(&mut self, token_stream: &Vec<Token>) {
        let token = &token_stream[self.token_idx];

        match token {
            Token::EOF => return,
            Token::EOL(_span) => { /* Just ignore EOLs */ },

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

    fn parse_procedure_decl(&mut self, _token_stream: &Vec<Token>) {
        unreachable!();
    }

    fn parse_struct_decl(&mut self, _token_stream: &Vec<Token>) {
        unreachable!();
    }

    fn parse_enum_decl(&mut self, _token_stream: &Vec<Token>) {
        unreachable!();
    }

    fn parse_var_decl(&mut self, _token_stream: &Vec<Token>) {
        unreachable!();
    }
}
