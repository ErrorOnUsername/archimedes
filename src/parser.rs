use crate::ast::*;
use crate::token::{
    Token,
    PrimitiveType,
};

pub fn parse_module(token_stream: &[Token], start_index: usize) -> ParsedModule {
    let mut idx = start_index;
    let mut token: &Token;
    // FIXME: Add different module names (current filename)
    let mut module = ParsedModule::new("main");

    while idx < token_stream.len() {
        token = &token_stream[idx];

        match token {
            Token::EOF => break,
            Token::EOL(_span) => {
                /* Just ignore EOLs */
                continue
            },

            Token::Hash(span) => {
                panic!("Handle directives here: {:?}", span);
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
                        let param_list: Vec<ParsedVarDecl>;
                        param_list = parse_param_list_decl(token_stream, &mut idx);

                        let has_return_type = match &token_stream[idx] {
                            Token::ThinArrow(_span) => true,
                            Token::LCurly(_span) => true,
                            _ => { panic!("Syntax Error! Unexpected Token in procedure definition! Expected '{{' or '->' but got: {:?}", token_stream[idx]); },
                        };

                        idx += 1;

                        let return_type: ParsedType;
                        if has_return_type {
                            return_type = parse_type_name(token_stream, &mut idx);
                        } else {
                            return_type = ParsedType::Name(Vec::new(), String::from("nothing"));
                        }

                        (Some(param_list), return_type)
                    },
                    _ => (None, ParsedType::Name(Vec::new(), String::from("nothing")))
                };

                match complex_type {
                    ComplexType::Procedure => {
                        let body: ParsedBlock;
                        body = parse_block(token_stream, &mut idx);
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

fn parse_type_name(token_stream: &[Token], idx: &mut usize) -> ParsedType {
    let mut last_was_ident = false;
    let mut at_type_end = false;

    let mut mod_path: Vec<String> = Vec::new();
    let type_name: String;

    // Short-circuit for builtins
    match &token_stream[*idx] {
        Token::BuiltinType(_span, primitive) => {
            return match primitive {
                PrimitiveType::Nothing => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("nothing"))
                },
                PrimitiveType::Bool => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("bool"))
                },
                PrimitiveType::Char => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("char"))
                },
                PrimitiveType::String => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("string"))
                },
                PrimitiveType::U8 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("u8"))
                },
                PrimitiveType::I8 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("i8"))
                },
                PrimitiveType::U16 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("u16"))
                },
                PrimitiveType::I16 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("i16"))
                },
                PrimitiveType::U32 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("u32"))
                },
                PrimitiveType::I32 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("i32"))
                },
                PrimitiveType::U64 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("u64"))
                },
                PrimitiveType::I64 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("i64"))
                },
                PrimitiveType::F32 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("f32"))
                },
                PrimitiveType::F64 => {
                    *idx += 1;
                    ParsedType::Name(Vec::new(), String::from("f64"))
                }
            }
        },
        _ => { }
    }

    // TODO: Add array types as well

    // '*' is only allowed to prefix a type name, not suffix
    loop {
        match &token_stream[*idx] {
            Token::Star(_span) => {
                mod_path.push(String::from("*"));
                last_was_ident = false;
            },
            _ => {
                *idx += 1;
                break;
            }
        }
        *idx += 1;
    }

    while !at_type_end {
        match &token_stream[*idx] {
            Token::IdentName(_span, name) => {
                last_was_ident = true;
                mod_path.push(name.clone());
            },
            Token::DoubleColon(_span) => last_was_ident = false,
            _ => at_type_end = true
        }
        *idx += 1;
    }

    if last_was_ident {
        type_name = mod_path.pop().expect("Sytax Error! No type name given!");
    } else {
        panic!("Syntax Error! Incomplete type name. No type name folling '::'");
    }

    ParsedType::Name(mod_path, type_name)
}

fn parse_param_list_decl(token_stream: &[Token], idx: &mut usize) -> Vec<ParsedVarDecl> {
    let mut is_at_list_end = false;
    let mut found_name = false;
    let mut passed_colon = false;

    let mut name = String::new();
    let mut parsed_type = ParsedType::Name(Vec::new(), String::new());
    let mut params = Vec::new();

    match &token_stream[*idx] {
        Token::LParen(_span) => *idx += 1,
        _ => panic!("Syntax Error! Parameter list did not start with '('")
    }

    while !is_at_list_end {
        match &token_stream[*idx] {
            Token::IdentName(_span, ident_name) => {
                if !passed_colon {
                    name = ident_name.clone();
                    found_name = true;
                } else if passed_colon && found_name {
                    parsed_type = parse_type_name(token_stream, idx);
                    // We increment at that end of this loop, so we need to
                    // decrement here so we don't accidentally skip a token.
                    *idx -= 1;
                } else { panic!("Unknown state. I had no brain power to find what this means so figure it out future me :)"); }
            },
            Token::Colon(_span) => {
                passed_colon = true;
            }
            Token::BuiltinType(_span, _primitive) => {
                assert!(passed_colon && found_name, "Syntax Error! No name on parameter declaration or missing ':' operator!");
                parsed_type = parse_type_name(token_stream, idx);
                // We increment at that end of this loop, so we need to
                // decrement here so we don't accidentally skip a token.
                *idx -= 1;
            },
            Token::Star(_span) => {
                assert!(passed_colon && found_name, "Syntax Error! No name on parameter declaration or missing ':' operator!");
                parsed_type = parse_type_name(token_stream, idx);
                // We increment at that end of this loop, so we need to
                // decrement here so we don't accidentally skip a token.
                *idx -= 1;
            }, Token::LSquare(_span) => { assert!(passed_colon && found_name, "Syntax Error! No name on parameter declaration or missing ':' operator!");
                parsed_type = parse_type_name(token_stream, idx);
                // We increment at that end of this loop, so we need to
                // decrement here so we don't accidentally skip a token.
                *idx -= 1;
            },
            Token::Comma(_span) => {
                params.push(ParsedVarDecl { parsed_type: parsed_type.clone(), name: name.clone() });

                found_name = false;
                passed_colon = false;
            },
            Token::RParen(_span) => {
                assert!(found_name && passed_colon, "Syntax Error! No name or type on parameter declaration!");
                params.push(ParsedVarDecl { parsed_type: parsed_type.clone(), name: name.clone() });
                is_at_list_end = true;
            },
            _ => panic!("Syntax Error! Unexpected token in parameter list declaration: {:?}", token_stream[*idx])
        }

        *idx += 1;
    }

    params
}

fn parse_block(token_stream: &[Token], idx: &mut usize) -> ParsedBlock {
    let mut block = ParsedBlock::new();

    eat_newlines(token_stream, idx);

    match &token_stream[*idx] {
        Token::LCurly(_span) => *idx += 1,
        _ => panic!("Syntax Error! Block does not start with '{{'")
    }

    loop {
        match &token_stream[*idx] {
            Token::RCurly(_span) => {
                *idx += 1;
                break;
            }
            Token::EOL(_span) => {
                *idx += 1;
                continue;
            },
            _ => { }
        }

        let stmt: ParsedStatement;
        stmt = parse_statement(token_stream, idx);
        block.stmts.push(stmt);
    }

    block
}

fn parse_statement(token_stream: &[Token], idx: &mut usize) -> ParsedStatement {
    let stmt = match &token_stream[*idx] {
        Token::KeywordLet(_span) => {
            let mut var_decl: ParsedVarDecl = ParsedVarDecl {
                parsed_type: ParsedType::Name(Vec::new(), String::from("")),
                name: String::new()
            };

            *idx += 1;

            match &token_stream[*idx] {
                Token::IdentName(_span, name) => var_decl.name = name.clone(),
                _ => panic!("Syntax Error! No name in varable declaration. Got: {:?}", token_stream[*idx])
            }

            *idx += 1;

            let iterpret_type = match &token_stream[*idx] {
                Token::ColonAssign(_span) => {
                    *idx += 1;
                    true
                },
                Token::Colon(_span) => {
                    *idx += 1;
                    var_decl.parsed_type = parse_type_name(token_stream, idx);
                    false
                },
                _ => panic!("Syntax Error! Unexpected token in variable declaration: {:?}", token_stream[*idx])
            };

            if !iterpret_type {
                match &token_stream[*idx] {
                    Token::Assign(_span) => *idx += 1,
                    _ => panic!("Syntax Error! Expected '=', but got: {:?}", token_stream[*idx])
                }
            }

            let default_value = parse_expression(token_stream, idx);

            ParsedStatement::VarDecl(var_decl, default_value)
        },
        Token::IdentName(_span, _name) => {
            let expr = parse_expression(token_stream, idx);

            ParsedStatement::Expr(expr)
        },
        Token::LAngle(_span) => { unreachable!(); },

        Token::KeywordMatch(_span) => {
            let expr = parse_expression(token_stream, idx);

            ParsedStatement::Expr(expr)
        },
        Token::KeywordIf(_span) => {
            *idx += 1;

            let if_cond = parse_expression(token_stream, idx);

            eat_newlines(token_stream, idx);

            match &token_stream[*idx] {
                Token::LCurly(_span) => { },
                // FIXME: Add support for single-expression if statements
                _ => panic!("Syntax Error! Didn't find '{{' after if condition, got {:?} instead", token_stream[*idx])
            }

            let if_body = parse_block(token_stream, idx);

            // FIXME: Add if/else chaining
            ParsedStatement::If(if_cond, if_body, None)
        },
        Token::KeywordFor(_span) => { unreachable!(); },
        Token::KeywordWhile(_span) => {
            *idx += 1;

            let cond = parse_expression(token_stream, idx);

            eat_newlines(token_stream, idx);

            match &token_stream[*idx] {
                Token::LCurly(_span) => { },
                _ => panic!("Syntax Error! Expected '{{' after while condition, but got: {:?}", token_stream[*idx])
            }

            let body = parse_block(token_stream, idx);

            ParsedStatement::WhileLoop(cond, body)
        },
        Token::KeywordLoop(_span) => {
            *idx += 1;

            eat_newlines(token_stream, idx);

            match &token_stream[*idx] {
                Token::LCurly(_span) => { },
                _ => panic!("Syntax Error! Expected '{{' after loop statement, but got: {:?}", token_stream[*idx])
            }

            let body = parse_block(token_stream, idx);

            ParsedStatement::InfiniteLoop(body)
        },

        Token::KeywordContinue(_span) => {
            *idx += 1;

            match &token_stream[*idx] {
                Token::Semicolon(_span) => *idx += 1,
                _ => panic!("Syntax Error! Expected ';', got: {:?}", token_stream[*idx])
            }

            ParsedStatement::Continue
        },
        Token::KeywordBreak(_span) => {
            *idx += 1;

            match &token_stream[*idx] {
                Token::Semicolon(_span) => *idx += 1,
                _ => panic!("Syntax Error! Expected ';', got: {:?}", token_stream[*idx])
            }

            ParsedStatement::Break
        },
        Token::KeywordReturn(_span) => {
            *idx += 1;

            let expr = parse_expression(token_stream, idx);

            match &token_stream[*idx] {
                Token::Semicolon(_span) => *idx += 1,
                _ => panic!("Syntax Error! Expected ';', got: {:?}", token_stream[*idx])
            }

            ParsedStatement::Return(expr)
        },
        _ => panic!("Syntax Error! Unexpected token: {:?}", token_stream[*idx])
    };

    stmt
}

fn parse_expression(_token_stream: &[Token], _idx: &mut usize) -> ParsedExpression { panic!("parse_expression"); }

fn eat_newlines(token_stream: &[Token], idx: &mut usize) {
    loop {
        match &token_stream[*idx] {
            Token::EOL(_span) => *idx += 1,
            _ => break
        }
    }
}
