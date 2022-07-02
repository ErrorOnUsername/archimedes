use crate::ast::*;
use crate::types::*;
use crate::token::Token;

pub fn parse_module(token_stream: &[Token], start_index: usize) -> ParsedModule {
    let mut idx = start_index;
    let mut token: &Token;
    // FIXME: Add different module names
    let mut module = ParsedModule::new("main");

    while idx < token_stream.len() {
        token = &token_stream[idx];

        match token {
            Token::EOF => break,
            Token::EOL(_span) => {
                /* Just ignore EOLs */
                continue
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
                idx += 1;
                let ident = match &token_stream[idx] {
                    Token::IdentName(_span, name) => name.clone(),
                    _ => panic!("Syntax Error! Expected identifier, but got something else!")
                };

                idx += 1;
                let is_macro = match &token_stream[idx] {
                    Token::Dollar(_span) => true,
                    _ => {
                        idx -= 1;
                        false
                    }
                };

                idx += 1;
                match &token_stream[idx] {
                    Token::Colon(_span) => { },
                    _ => panic!("Syntax Error! Expected a colon, but got something else!")
                }

                idx += 1;
                let complex_type = match &token_stream[idx] {
                    Token::LParen(_span) => ComplexType::Procedure,
                    Token::KeywordStruct(_span) => ComplexType::Struct,
                    Token::KeywordEnum(_span) => ComplexType::None,
                    _ => {
                        eprintln!("Syntax Error! Expected complex type identifier ('struct', 'enum', '()'), but got something else!");
                        ComplexType::None
                    }
                };

                let params = match complex_type {
                    ComplexType::Procedure => Some(parse_param_list_decl(token_stream, idx)),
                    _ => None
                };

                idx += 1;
                let return_type_id = match &token_stream[idx] {
                    Token::ThinArrow(_span) => { 0 },
                    _ => 0
                };

                match complex_type {
                    ComplexType::Procedure => {
                        let body = parse_block(token_stream, idx);
                        let proc = ParsedProcDecl {
                            name: ident,
                            is_macro,
                            parameters: params.expect("Syntax Error! You defined a function without even an empty parameter list. How?!?!"),
                            return_type_id,
                            body
                        };
                        module.procs.push(proc);
                    },

                    ComplexType::Struct => {
                    },

                    ComplexType::Enum => {
                    },

                    ComplexType::None => { panic!("The type you tried to define was not a struct, enum, or procedure, which are the only supported complex types!"); }
                }
            },

            _ => {
                // FIXME: Propagate this error rather than just freaking out...
                panic!("Unexpected token: {:?}", token);
            }
        }

        idx += 1;
    }

    module
}

pub fn parse_param_list_decl(_token_stream: &[Token], _start_index: usize) -> Vec<ParsedVarDecl> { panic!("parse_param_list_decl"); }

pub fn parse_block(_token_stream: &[Token], _start_index: usize) -> ParsedBlock { panic!("parse_block"); }

pub fn parse_expression(_token_stream: &[Token], _start_index: usize) -> ParsedExpression { panic!("parse_block"); }
