use crate::ast::*;
use crate::token::Token;

pub fn parse_type_name(token_stream: &[Token], start_index: usize) -> ParsedType {
    let mut idx = start_index;
    let mut last_was_ident = false;
    let mut at_type_end = false;

    let mut mod_path: Vec<String> = Vec::new();
    let type_name: String;

    // TODO: Add array types as well

    // '&' and '*' are only allowed to prefix a type name, not suffix
    loop {
        match &token_stream[idx] {
            Token::Ampersand(_span) => {
                mod_path.push(String::from("&"));
                last_was_ident = false;
            },
            Token::Star(_span) => {
                mod_path.push(String::from("*"));
                last_was_ident = false;
            },
            _ => {
                idx += 1;
                break;
            }
        }
        idx += 1;
    }

    while !at_type_end {
        match &token_stream[idx] {
            Token::IdentName(_span, name) => {
                last_was_ident = true;
                mod_path.push(name.clone());
            },
            Token::DoubleColon(_span) => last_was_ident = false,
            _ => at_type_end = true
        }
        idx += 1;
    }

    if last_was_ident {
        type_name = mod_path.pop().expect("Sytax Error! No type name given!");
    } else {
        panic!("Syntax Error! Incomplete type name. No type name folling '::'");
    }

    ParsedType::Name(mod_path, type_name)
}

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
                // If it's none of those, we also might be
                // declaring a constant.
                //
                //      decl SOME_CONST: i32 = 0x2a;
                //

                // Get the identifier name
                idx += 1;
                let ident = match &token_stream[idx] {
                    Token::IdentName(_span, name) => name.clone(),
                    _ => panic!("Syntax Error! Expected identifier, but got something else!")
                };

                // Ensure the type operator (`:`) is present.
                idx += 1;
                match &token_stream[idx] {
                    Token::Colon(_span) => { },
                    _ => panic!("Syntax Error! Expected a colon, but got something else!")
                }

                // See if a `$` comes just after the colon,
                // which would denote a compile-time procedure.
                //
                // i.e.:
                //     decl exec_at_compile_time : $() { }
                idx += 1;
                let is_macro = match &token_stream[idx] {
                    Token::Dollar(_span) => true,
                    _ => {
                        idx -= 1;
                        false
                    }
                };

                // Check what kind of complex type we're trying to declare.
                idx += 1;
                let complex_type = match &token_stream[idx] {
                    Token::LParen(_span) => ComplexType::Procedure,
                    Token::KeywordStruct(_span) => ComplexType::Struct,
                    Token::KeywordEnum(_span) => ComplexType::Enum,
                    Token::IdentName(_span, _name) => ComplexType::None,
                    Token::BuiltinType(_span, _type_name) => ComplexType::None,
                    _ => panic!("Syntax Error! Expected complex type identifier ('struct', 'enum', '()'), but got something else!")
                };

                let (params, return_type) = match complex_type {
                    ComplexType::Procedure => {
                        let param_list = parse_param_list_decl(token_stream, idx);

                        let has_return_type = match &token_stream[idx] {
                            Token::ThinArrow(_span) => true,
                            Token::LCurly(_span) => true,
                            _ => { panic!("Syntax Error! Unexpected Token in procedure definition! Expected '{{' or '->' but got: {:?}", token_stream[idx]); },
                        };

                        idx += 1;

                        let return_type: ParsedType;
                        if has_return_type {
                            return_type = parse_type_name(token_stream, idx);
                        } else {
                            return_type = ParsedType::Name(Vec::new(), String::from("nothing"));
                        }

                        (Some(param_list), return_type)
                    },
                    _ => (None, ParsedType::Name(Vec::new(), String::from("nothing")))
                };

                match complex_type {
                    ComplexType::Procedure => {
                        let body = parse_block(token_stream, idx);
                        let proc = ParsedProcDecl {
                            name: ident,
                            parameters: params.expect("Syntax Error! You defined a function without even an empty parameter list. How?!?!"),
                            parsed_return_type: return_type,
                            body
                        };
                        module.procs.push(proc);
                    },

                    ComplexType::Struct => {
                        let mut member_name: Option<String> = None;
                        let mut member_type: Option<ParsedType> = None;
                        let mut is_past_colon = false;
                        let mut in_struct_body = match &token_stream[idx] {
                            Token::LCurly(_span) => true,
                            _ => { panic!("Syntax Error! Missing '{{' in struct declaration"); }
                        };

                        while in_struct_body {
                            token = &token_stream[idx];

                            match token {
                                Token::RCurly(_span) => in_struct_body = false,
                                Token::IdentName(_span, _name) => {
                                    // First determine if we're looking at a member name or type name.
                                    if member_name.is_none() && member_type.is_none() {
                                        // We're looking at a member name
                                    } else if is_past_colon && (member_type.is_none() && member_name.is_some()) {
                                        // We're looking at a non-primitive type name
                                    } else {
                                        panic!("Syntax Error! Invalid Placement of identifier name in struct member declaration [{:?}]", token);
                                    }
                                },
                                Token::Colon(_span) => {
                                    if !(member_name.is_some() && member_type.is_none()) {
                                        panic!("Syntax Error! Invalid Placement of ':' in struct member declaration [{:?}]", token);
                                    }
                                    is_past_colon = true;
                                },
                                Token::BuiltinType(_span, _primitive) => {
                                    if is_past_colon && (member_type.is_none() && member_name.is_some()) {
                                    } else {
                                        panic!("Syntax Error! Invalid Placement of type name in struct member declaration [{:?}]", token);
                                    }
                                },
                                Token::Semicolon(_span) => {
                                    member_name = None;
                                    member_type = None;
                                    is_past_colon = false;
                                },
                                Token::EOL(_span) => {
                                    if !(!is_past_colon && member_name.is_none() && member_type.is_none()) {
                                        panic!("Syntax Error! Invalid placement of new line in struct member declaration [{:?}]", token);
                                    }
                                },

                                _ => { }
                            }

                            idx += 1;
                        }
                    },

                    ComplexType::Enum => { },

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
