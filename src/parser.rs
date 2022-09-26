use crate::ast::*;
use crate::token::Token;

pub struct Parser {
    pub token_stream: Vec<Token>,
    pub idx: usize,
}

impl Parser {
    fn at_end(&self) -> bool {
        self.idx >= self.token_stream.len()
    }

    fn current(&self) -> &Token {
        &self.token_stream[self.idx]
    }

    fn peek(&self) -> &Token {
        assert!(self.idx + 1 < self.token_stream.len());
        &self.token_stream[self.idx + 1]
    }

    pub fn parse_module(&mut self) -> ParsedModule {
        let mut token: &Token;
        // FIXME: Add different module names (current filename)
        let mut module = ParsedModule::new("main");

        while !self.at_end() {
            token = self.current();

            match token {
                Token::EOF => break,
                Token::EOL(_span) => { /* Just ignore EOLs */ },

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
                    self.idx += 1;
                    let ident = match self.current() {
                        Token::IdentName(_span, name) => name.clone(),
                        _ => panic!("Syntax Error! Expected identifier, but got something else!")
                    };

                    // Ensure the type operator (`:`) is present.
                    self.idx += 1;
                    match self.current() {
                        Token::Colon(_span) => { },
                        _ => panic!("Syntax Error! Expected a colon, but got something else!")
                    }

                    // Check what kind of complex type we're trying to declare.
                    self.idx += 1;
                    let complex_type = match self.current() {
                        Token::LParen(_span) => ComplexType::Procedure,
                        Token::KeywordStruct(_span) => ComplexType::Struct,
                        Token::KeywordEnum(_span) => ComplexType::Enum,
                        Token::IdentName(_span, _name) => ComplexType::Constant,
                        Token::BuiltinType(_span, _type_name) => ComplexType::Constant,
                        _ => panic!("Syntax Error! Expected complex type identifier ('struct', 'enum', '()'), but got something else!")
                    };

                    let (params, return_type) = match complex_type {
                        ComplexType::Procedure => {
                            let param_list: Vec<ParsedVarDecl>;
                            param_list = self.parse_param_list_decl();

                            self.eat_newlines();

                            let has_return_type = match self.current() {
                                Token::ThinArrow(_span) => true,
                                Token::LCurly(_span) => false,
                                _ => { panic!("Syntax Error! Unexpected Token in procedure definition! Expected '{{' or '->' but got: {:?}", self.current()); },
                            };


                            let return_type: ParsedType;
                            if has_return_type {
                                self.idx += 1;
                                return_type = self.parse_type_name();
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
                            body = self.parse_block();
                            let proc = ParsedProcDecl {
                                name: ident,
                                parameters: params.expect("Syntax Error! You defined a function without even an empty parameter list. How?!?!"),
                                parsed_return_type: return_type,
                                body
                            };
                            module.procs.push(proc);
                        },

                        ComplexType::Struct => {
                            self.idx += 1;

                            let mut struct_decl = ParsedStructDecl {
                                name: ident,
                                data_members: Vec::new()
                            };

                            let mut member_name: Option<String> = None;
                            let mut member_type: Option<ParsedType> = None;
                            let mut is_past_colon = false;
                            let mut in_struct_body = match self.current() {
                                Token::LCurly(_span) => true,
                                _ => { panic!("Syntax Error! Expected '{{' in struct declaration, got {:?}", self.current()); }
                            };

                            self.idx += 1;

                            while in_struct_body {
                                self.eat_newlines();
                                token = self.current();

                                match token {
                                    Token::RCurly(_span) => in_struct_body = false,
                                    Token::IdentName(_span, name) => {
                                        // First determine if we're looking at a member name or type name.
                                        if member_name.is_none() && member_type.is_none() {
                                            // We're looking at a member name
                                            member_name = Some(name.clone());
                                        } else if is_past_colon && (member_type.is_none() && member_name.is_some()) {
                                            // We're looking at a non-primitive type name
                                            member_type = Some(self.parse_type_name());
                                        } else {
                                            panic!("Syntax Error! Invalid Placement of identifier name in struct member declaration [{:?}]", token);
                                        }
                                    },
                                    Token::Colon(_span) => {
                                        if member_name.is_some() && member_type.is_none() {
                                            is_past_colon = true;
                                        } else {
                                            panic!("Syntax Error! Invalid Placement of ':' in struct member declaration [{:?}]", token);
                                        }
                                    },
                                    Token::BuiltinType(_span, _primitive) => {
                                        if is_past_colon && (member_type.is_none() && member_name.is_some()) {
                                            member_type = Some(self.parse_type_name());
                                            continue;
                                        } else {
                                            panic!("Syntax Error! Invalid Placement of type name in struct member declaration [{:?}]", token);
                                        }
                                    },
                                    Token::Comma(_span) => {
                                        assert!(member_name.is_some() && member_type.is_some(), "Unexpected ',' in struct member definition");

                                        struct_decl.data_members.push(ParsedVarDecl {
                                                                          parsed_type: member_type.unwrap(),
                                                                          name: member_name.unwrap(),
                                                                          defualt_value: ParsedExpression::Invalid
                                                                      });

                                        member_name = None;
                                        member_type = None;
                                        is_past_colon = false;
                                    },
                                    _ => {
                                        assert!(member_name.is_some() && member_type.is_some(), "Unexpected token in struct member definition: {:?}", self.current());
                                        member_name = None;
                                        member_type = None;
                                        is_past_colon = false;
                                    }
                                }

                                self.idx += 1;
                            }

                            module.structs.push(struct_decl);
                        },

                        ComplexType::Enum => { todo!("Implement Enums") },

                        ComplexType::Constant => { todo!("Implement constants") },
                    }
                },

                _ => {
                    // FIXME: Propagate this error rather than just freaking out...
                    panic!("Unexpected token: {:?}", token);
                }
            }

            self.idx += 1;
        }

        module
    }

    fn parse_type_name(&mut self) -> ParsedType {
        let mut last_was_ident = false;
        let mut at_type_end = false;

        let mut mod_path: Vec<String> = Vec::new();

        // Short-circuit for builtins
        if let Token::BuiltinType(_span, primitive) = self.current() {
            let ty = ParsedType::Name(Vec::new(), String::from(primitive.as_str()));

            self.idx += 1;
            return ty;
        }

        assert!(matches!(self.current(), Token::LSquare(_span)), "Add array types");

        // '*' is only allowed to prefix a type name, not suffix
        loop {
            match self.current() {
                Token::Star(_span) => {
                    mod_path.push(String::from("*"));
                    last_was_ident = false;
                },
                _ => {
                    self.idx += 1;
                    break;
                }
            }
            self.idx += 1;
        }

        while !at_type_end {
            match self.current() {
                Token::IdentName(_span, name) => {
                    last_was_ident = true;
                    mod_path.push(name.clone());
                },
                Token::DoubleColon(_span) => last_was_ident = false,
                _ => at_type_end = true
            }
            self.idx += 1;
        }

        let type_name: String = if last_was_ident {
            mod_path.pop().expect("Sytax Error! No type name given!")
        } else {
            println!("{:?}", self.current());
            panic!("Syntax Error! Incomplete type name. No type name following '::'");
        };

        ParsedType::Name(mod_path, type_name)
    }

    fn parse_param_list_decl(&mut self) -> Vec<ParsedVarDecl> {
        let mut is_at_list_end = false;
        let mut found_name = false;
        let mut passed_colon = false;

        let mut name = String::new();
        let mut parsed_type = ParsedType::Name(Vec::new(), String::new());
        let mut params = Vec::new();

        match self.current() {
            Token::LParen(_span) => self.idx += 1,
            _ => panic!("Syntax Error! Parameter list did not start with '('")
        }

        while !is_at_list_end {
            match self.current() {
                Token::IdentName(_span, ident_name) => {
                    if !passed_colon {
                        name = ident_name.clone();
                        found_name = true;
                    } else if passed_colon && found_name {
                        parsed_type = self.parse_type_name();
                        // We increment at that end of this loop, so we need to
                        // decrement here so we don't accidentally skip a token.
                        self.idx -= 1;
                    } else { panic!("Unknown state. I had no brain power to find what this means so figure it out future me :)"); }
                },
                Token::Colon(_span) => {
                    passed_colon = true;
                }
                Token::BuiltinType(_span, _primitive) => {
                    assert!(passed_colon && found_name, "Syntax Error! No name on parameter declaration or missing ':' operator!");
                    parsed_type = self.parse_type_name();
                    // We increment at that end of this loop, so we need to
                    // decrement here so we don't accidentally skip a token.
                    self.idx -= 1;
                },
                Token::Star(_span) => {
                    assert!(passed_colon && found_name, "Syntax Error! No name on parameter declaration or missing ':' operator!");
                    parsed_type = self.parse_type_name();
                    // We increment at that end of this loop, so we need to
                    // decrement here so we don't accidentally skip a token.
                    self.idx -= 1;
                }, Token::LSquare(_span) => { assert!(passed_colon && found_name, "Syntax Error! No name on parameter declaration or missing ':' operator!");
                    parsed_type = self.parse_type_name();
                    // We increment at that end of this loop, so we need to
                    // decrement here so we don't accidentally skip a token.
                    self.idx -= 1;
                },
                Token::Comma(_span) => {
                    params.push(ParsedVarDecl {
                        parsed_type: parsed_type.clone(),
                        name: name.clone(),
                        defualt_value: ParsedExpression::Invalid
                    });

                    found_name = false;
                    passed_colon = false;
                },
                Token::RParen(_span) => {
                    assert!(!(found_name ^ passed_colon), "Syntax Error! No name or type on parameter declaration!");

                    if found_name && passed_colon {
                        params.push(ParsedVarDecl {
                            parsed_type: parsed_type.clone(),
                            name: name.clone(),
                            defualt_value: ParsedExpression::Invalid
                        });
                    }

                    is_at_list_end = true;
                },
                _ => panic!("Syntax Error! Unexpected token in parameter list declaration: {:?}", self.current())
            }

            self.idx += 1;
        }

        params
    }

    fn parse_param_list_usage(&mut self) -> Vec<ParsedVarDecl> {
        let mut is_at_list_end = false;
        let mut found_name = false;
        let mut passed_colon = false;

        let parsed_type = ParsedType::Name(Vec::new(), String::new());
        let mut name = String::new();
        let mut default_value = ParsedExpression::Invalid;
        let mut params = Vec::new();

        match self.current() {
            Token::LParen(_span) => self.idx += 1,
            _ => panic!("Syntax Error! Parameter list did not start with '('")
        }

        while !is_at_list_end {
            match self.current() {
                Token::IdentName(_span, ident_name) => {
                    if !passed_colon {
                        name = ident_name.clone();
                        found_name = true;
                    } else {
                        default_value = self.parse_expression(false, true);
                        continue;
                    }
                },
                Token::Colon(_span) => {
                    passed_colon = true;
                }
                Token::Comma(_span) => {
                    params.push(ParsedVarDecl {
                        parsed_type: parsed_type.clone(),
                        name: name.clone(),
                        defualt_value: default_value.clone()
                    });

                    found_name = false;
                    passed_colon = false;
                },
                Token::RParen(_span) => {
                    assert!(!(found_name ^ passed_colon), "Syntax Error! No label on parameter name");
                    if found_name && passed_colon {
                        params.push(ParsedVarDecl {
                            parsed_type: parsed_type.clone(),
                            name: name.clone(),
                            defualt_value: default_value.clone()
                        });
                    }
                    is_at_list_end = true;
                },
                _ => {
                    assert!(found_name && passed_colon, "No label on parameter name, got {:?}", self.current());
                    default_value = self.parse_expression(false, true);
                }
            }

            self.idx += 1;
        }

        params
    }

    fn parse_block(&mut self) -> ParsedBlock {
        let mut block = ParsedBlock::new();

        self.eat_newlines();

        match self.current() {
            Token::LCurly(_span) => self.idx += 1,
            _ => panic!("Syntax Error! Block does not start with '{{'")
        }

        loop {
            match self.current() {
                Token::RCurly(_span) => {
                    self.idx += 1;
                    break;
                }
                Token::EOL(_span) => {
                    self.idx += 1;
                    continue;
                },
                _ => { }
            }

            let stmt: ParsedStatement;
            stmt = self.parse_statement();
            block.stmts.push(stmt);
        }

        block
    }

    fn parse_statement(&mut self) -> ParsedStatement {
        let stmt = match self.current() {
            Token::KeywordLet(_span) => {
                self.idx += 1;
                let mut var_decl: ParsedVarDecl = ParsedVarDecl {
                    parsed_type: ParsedType::Name(Vec::new(), String::from("")),
                    name: String::new(),
                    defualt_value: ParsedExpression::Invalid
                };

                match self.current() {
                    Token::IdentName(_span, name) => var_decl.name = name.clone(),
                    _ => panic!("Syntax Error! No name in varable declaration. Got: {:?}", self.current())
                }

                self.idx += 1;

                let iterpret_type = match self.current() {
                    Token::ColonAssign(_span) => {
                        self.idx += 1;
                        true
                    },
                    Token::Colon(_span) => {
                        self.idx += 1;
                        var_decl.parsed_type = self.parse_type_name();
                        false
                    },
                    _ => panic!("Syntax Error! Unexpected token in variable declaration: {:?}", self.current())
                };

                if !iterpret_type {
                    match self.current() {
                        Token::Assign(_span) => self.idx += 1,
                        _ => panic!("Syntax Error! Expected '=', but got: {:?}", self.current())
                    }
                }

                let default_value = self.parse_expression(false, true);
                var_decl.defualt_value = default_value;

                assert!(matches!(self.current(), Token::Semicolon(_span)), "Syntax Error! Expected ';' at end of variable declaration, got {:?}", self.current());
                self.idx += 1;

                ParsedStatement::VarDecl(var_decl)
            },

            Token::KeywordIf(_span) => {
                self.idx += 1;

                let if_cond = self.parse_expression(false, true);
                let if_body = self.parse_block();

                // FIXME: Add if/else chaining
                ParsedStatement::If(if_cond, if_body, None)
            },

            Token::KeywordFor(_span) => {
                self.idx += 1;

                let mut it_decl: ParsedVarDecl = ParsedVarDecl {
                    parsed_type: ParsedType::Name(Vec::new(), String::from("")),
                    name: String::new(),
                    defualt_value: ParsedExpression::Invalid
                };

                match self.current() {
                    Token::IdentName(_span, name) => it_decl.name = name.clone(),
                    _ => panic!("Syntax Error! No name in varable declaration. Got: {:?}", self.current())
                }

                self.idx += 1;

                match self.current() {
                    Token::KeywordIn(_span) => self.idx += 1,
                    _ => panic!("Syntax Error! Expected 'in' after for loop iterator definition, got: {:?}", self.current())
                }

                let range_expr = self.parse_range_expression();
                let body = self.parse_block();

                ParsedStatement::ForLoop(it_decl, range_expr, body)
            },

            Token::KeywordWhile(_span) => {
                self.idx += 1;

                let cond = self.parse_expression(false, true);
                let body = self.parse_block();

                ParsedStatement::WhileLoop(cond, body)
            },

            Token::KeywordLoop(_span) => {
                self.idx += 1;

                let body = self.parse_block();

                ParsedStatement::InfiniteLoop(body)
            },

            Token::KeywordContinue(_span) => {
                self.idx += 1;

                match self.current() {
                    Token::Semicolon(_span) => self.idx += 1,
                    _ => panic!("Syntax Error! Expected ';', got: {:?}", self.current())
                }

                ParsedStatement::Continue
            },

            Token::KeywordBreak(_span) => {
                self.idx += 1;

                match self.current() {
                    Token::Semicolon(_span) => self.idx += 1,
                    _ => panic!("Syntax Error! Expected ';', got: {:?}", self.current())
                }

                ParsedStatement::Break
            },

            Token::KeywordReturn(_span) => {
                self.idx += 1;
                let expr = self.parse_expression(false, true);

                assert!(matches!(self.current(), Token::Semicolon(_span)), "Syntax Error! Expected ';' at end of return expression, got {:?}", self.current());
                self.idx += 1;

                ParsedStatement::Return(expr)
            },

            _ => {
                let expr = self.parse_expression(true, true);

                assert!(matches!(self.current(), Token::Semicolon(_span)), "Syntax Error! Expected ';' at end of expression, got {:?}", self.current());
                self.idx += 1;

                ParsedStatement::Expr(expr)
            }
        };

        stmt
    }

    fn parse_expression(&mut self, can_assign: bool, allow_newlines: bool) -> ParsedExpression {
        let mut expr_stack: Vec<ParsedExpression> = Vec::new();
        let mut last_op_priority = 1_000_000;

        let lhs = self.parse_operand();
        expr_stack.push(lhs);

        loop {
            if allow_newlines {
                if self.is_eof() || matches!(self.current(), Token::LCurly(_span))
                { break; }

                self.eat_newlines();
            } else {
                if self.is_eol() {
                    break;
                }
            }

            let op = self.parse_operator(can_assign);
            let op_priority = op.priority();

            if let ParsedExpression::Operator(b_op) = &op {
                if matches!(&b_op, BinaryOperator::Invalid) {
                    break
                }
            }

            self.eat_newlines();

            let rhs = self.parse_operand();

            while op_priority <= last_op_priority && expr_stack.len() > 1 {
                let pop_rhs = expr_stack.pop().unwrap();
                let pop_op = expr_stack.pop().unwrap();

                last_op_priority = pop_op.priority();

                // This might look backwards, but remember we're using a stack,
                // so we pop it off in reverse, so the high priority ones end
                // up at the top of the stack.
                if last_op_priority < op_priority {
                    expr_stack.push(pop_op);
                    expr_stack.push(pop_rhs);
                    break;
                }

                let pop_lhs = expr_stack.pop().unwrap();

                match &pop_op {
                    ParsedExpression::Operator(bin_op) => {
                        expr_stack.push(ParsedExpression::BinaryOperation(Box::new(pop_lhs), bin_op.clone(), Box::new(pop_rhs)));
                    },
                    _ => panic!("WHAT?!?! Operator is not an operator")
                }
            }

            expr_stack.push(op);
            expr_stack.push(rhs);

            last_op_priority = op_priority;
        }

        while expr_stack.len() > 1 {
            let pop_rhs = expr_stack.pop().unwrap();
            let pop_op = expr_stack.pop().unwrap();
            let pop_lhs = expr_stack.pop().unwrap();

            match pop_op {
                ParsedExpression::Operator(bin_op) => {
                    expr_stack.push(ParsedExpression::BinaryOperation(Box::new(pop_lhs), bin_op, Box::new(pop_rhs)))
                },
                _ => panic!("WHAT?!?! Operator is not an operator")
            }
        }

        expr_stack[0].clone()
    }

    fn parse_operand(&mut self) -> ParsedExpression {
        self.eat_newlines();
        let expr = self.parse_operand_prefix();
        self.parse_operand_postfix(expr)
    }

    fn parse_operand_prefix(&mut self) -> ParsedExpression {
        match self.current() {
            Token::LAngle(_span) => {
                self.idx += 1;
                let expr = self.parse_operand();
                ParsedExpression::UnaryOperation(Box::new(expr), UnaryOperator::Dereference)
            },
            Token::Ampersand(_span) => {
                self.idx += 1;
                let expr = self.parse_operand();
                ParsedExpression::UnaryOperation(Box::new(expr), UnaryOperator::AddressOf)
            },
            Token::Bang(_span) => {
                self.idx += 1;
                let expr = self.parse_operand();
                ParsedExpression::UnaryOperation(Box::new(expr), UnaryOperator::LogicalNot)
            },
            Token::Tilde(_span) => {
                self.idx += 1;
                let expr = self.parse_operand();
                ParsedExpression::UnaryOperation(Box::new(expr), UnaryOperator::BitwiseNot)
            },
            Token::PlusPlus(_span) => {
                self.idx += 1;
                let expr = self.parse_operand();
                ParsedExpression::UnaryOperation(Box::new(expr), UnaryOperator::PreIncrement)
            },
            Token::MinusMinus(_span) => {
                self.idx += 1;
                let expr = self.parse_operand();
                ParsedExpression::UnaryOperation(Box::new(expr), UnaryOperator::PreDecrement)
            },
            Token::StringLiteral(_span, lit) => {
                let string = lit.clone();
                self.idx += 1;
                ParsedExpression::StringLiteral(string)
            },
            Token::CharLiteral(_span, lit) => {
                let ch = *lit;
                self.idx += 1;
                ParsedExpression::CharLiteral(ch)
            },
            Token::BooleanLiteral(_span, val) => {
                let b = *val;
                self.idx += 1;
                ParsedExpression::Bool(b)
            },
            Token::Number(_span, constant) => {
                let n = constant.clone();
                self.idx += 1;
                ParsedExpression::NumericConstant(n)
            },
            Token::IdentName(_span, name) => {
                let ident = name.clone();

                match self.peek() {
                    Token::LParen(_span) => {
                        let call = self.parse_proc_call();
                        ParsedExpression::ProcCall(call)
                    }
                    _ => {
                        self.idx += 1;
                        ParsedExpression::Var(ident)
                    }
                }
            },
            _ => panic!("Unexpected token: {:?}", self.current())
        }
    }

    fn parse_operand_postfix(&mut self, base: ParsedExpression) -> ParsedExpression {
        match self.current() {
            Token::PlusPlus(_span) => {
                self.idx += 1;
                ParsedExpression::UnaryOperation(Box::new(base), UnaryOperator::PostIncrement)
            },
            Token::MinusMinus(_span) => {
                self.idx += 1;
                ParsedExpression::UnaryOperation(Box::new(base), UnaryOperator::PostDecrement)
            },
            Token::KeywordAs(_span) => {
                self.idx += 1;
                let to_type = self.parse_type_name();
                ParsedExpression::UnaryOperation(Box::new(base), UnaryOperator::TypeCast(Box::new(to_type)))
            }
            Token::LAngle(_span) => panic!("Add array indexing"),
            _ => base
        }
    }

    fn parse_range_expression(&mut self) -> ParsedExpression {
        self.eat_newlines();

        let left_bound = match self.current() {
            Token::LSquare(_span) => RangeExprBound::Inclusive,
            Token::LParen(_span) => RangeExprBound::Exclusive,
            _ => panic!("Syntax Error! Expected '[' or '(' to specify lower range inclusivity, but got: {:?}", self.current())
        };
        let start = self.parse_expression(false, true);

        match self.current() {
            Token::DotDot(_span) => self.idx += 1,
            _ => panic!("Syntax Error! Expected '..' in range expression, got: {:?}", self.current())
        }

        let end = self.parse_expression(false, true);
        let right_bound = match self.current() {
            Token::RSquare(_span) => RangeExprBound::Inclusive,
            Token::RParen(_span) => RangeExprBound::Exclusive,
            _ => panic!("Syntax Error! Expected ']' or ')' to specify lower range inclusivity, but got: {:?}", self.current())
        };

        self.idx += 1;

        ParsedExpression::Range(left_bound, Box::new(start), Box::new(end), right_bound)
    }

    fn parse_operator(&mut self, can_assign: bool) -> ParsedExpression {
        self.eat_newlines();

        let ret = match self.current() {
            Token::Plus(_span) => BinaryOperator::Add,
            Token::Minus(_span) => BinaryOperator::Subtract,
            Token::Star(_span) => BinaryOperator::Multiply,
            Token::Slash(_span) => BinaryOperator::Divide,
            Token::Percent(_span) => BinaryOperator::Modulo,

            Token::DoubleAmpersand(_span) => BinaryOperator::LogicalAnd,
            Token::DoublePipe(_span) => BinaryOperator::LogicalOr,
            Token::DoubleCaret(_span) => BinaryOperator::LogicalXOR,

            Token::NEQ(_span) => BinaryOperator::NEQ,
            Token::EQ(_span) => BinaryOperator::EQ,
            Token::LAngle(_span) => BinaryOperator::LT,
            Token::RAngle(_span) => BinaryOperator::GT,
            Token::LEQ(_span) => BinaryOperator::LEQ,
            Token::GEQ(_span) => BinaryOperator::GEQ,

            Token::Ampersand(_span) => BinaryOperator::BitwiseAnd,
            Token::Pipe(_span) => BinaryOperator::BitwiseOr,
            Token::Caret(_span) => BinaryOperator::BitwiseXOR,
            Token::LShift(_span) => BinaryOperator::BitwiseLeftShift,
            Token::RShift(_span) => BinaryOperator::BitwiseRightShift,

            Token::Assign(_span) => BinaryOperator::Assign,

            Token::PlusAssign(_span) => BinaryOperator::AddAssign,
            Token::MinusAssign(_span) => BinaryOperator::SubtractAssign,
            Token::StarAssign(_span) => BinaryOperator::MultiplyAssign,
            Token::SlashAssign(_span) => BinaryOperator::DivideAssign,
            Token::PercentAssign(_span) => BinaryOperator::ModuloAssign,

            Token::AmpersandAssign(_span) => BinaryOperator::AndAssign,
            Token::PipeAssign(_span) => BinaryOperator::OrAssign,
            Token::CaretAssign(_span) => BinaryOperator::XORAssign,
            Token::LShiftAssign(_span) => BinaryOperator::LeftShiftAssign,
            Token::RShiftAssign(_span) => BinaryOperator::RightShiftAssign,
            _ => {
                self.idx -= 1;
                BinaryOperator::Invalid
            }
        };

        if ret.is_assignment() && !can_assign {
            panic!("Syntax Error! Got an unexpected assignment operator [{:?}]", self.current());
        }

        self.idx += 1;

        ParsedExpression::Operator(ret)
    }

    fn parse_proc_call(&mut self) -> ParsedProcCall {
        let mut ret = ParsedProcCall {
            name: String::new(),
            passed_parameters: Vec::new(),
        };

        ret.name = match self.current() {
            Token::IdentName(_span, name) => name.clone(),
            _ => panic!("Expected identifier")
        };

        self.idx += 1;

        ret.passed_parameters = match self.current() {
            Token::LParen(_span) => self.parse_param_list_usage(),
            _ => panic!("Syntax Error! Expected '(' in procedure call.")
        };

        ret
    }

    fn eat_newlines(&mut self) {
        loop {
            match self.current() {
                Token::EOL(_span) => self.idx += 1,
                _ => break
            }
        }
    }

    fn is_eof(&self) -> bool {
        matches!(self.current(), Token::EOF)
    }

    fn is_eol(&self) -> bool {
        matches!(self.current(), Token::EOL(_span))
    }
}
