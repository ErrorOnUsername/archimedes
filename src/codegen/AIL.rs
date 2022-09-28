#[allow(non_snake_case)]

use std::collections::HashMap;
use std::ops::Deref;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::ast::{BinaryOperator, ParsedExpression, ParsedModule, ParsedProcDecl, ParsedStatement, ParsedType, UnaryOperator};
use crate::codegen::AIL::Instruction::{GETMEM, PUSH32, PUSH8, PUSHS, READ};
use crate::token::{FloatingPointLiteralFormat, IntegerLiteralFormat, NumericConstant};

fn CreateIL(ast: ParsedModule)
{
    let mut buffer:Vec<u8> = Vec::new();

    buffer.append(&mut "AIL".as_bytes().to_vec());

    let mut mainloc: u64 = 0;

    let mut procmap = HashMap::new();

    for proc in ast.procs {
        procmap.insert(proc.name.clone(), proc.clone());
        let mut parmap = HashMap::new();
        for param in proc.parameters {
            parmap.insert(param.name.clone(), param.parsed_type.clone());

        }

        let mut varmap = HashMap::new();
        let mut vars = 0;
        //function body parse
        for statement in proc.body.stmts {
            match statement {
                ParsedStatement::Expr(ex) => {}
                ParsedStatement::VarDecl(var) =>
                    {
                        varmap.insert(var.name.clone(), (vars, var.parsed_type.clone()));
                        match var.parsed_type {
                            ParsedType::Name(idk, typ) =>
                                {
                                    match typ.as_str() {
                                        "u8" => {buffer.append(&mut GetInstructionBytes(PUSH8(1, false)))}
                                        "u16" => {buffer.append(&mut GetInstructionBytes(PUSH8(2, false)))}
                                        "u32" => {buffer.append(&mut GetInstructionBytes(PUSH8(4, false)))}
                                        "u64" => {buffer.append(&mut GetInstructionBytes(PUSH8(8, false)))}
                                        "i8" => {buffer.append(&mut GetInstructionBytes(PUSH8(1, false)))}
                                        "i16" => {buffer.append(&mut GetInstructionBytes(PUSH8(2, false)))}
                                        "i32" => {buffer.append(&mut GetInstructionBytes(PUSH8(4, false)))}
                                        "i64" => {buffer.append(&mut GetInstructionBytes(PUSH8(8, false)))}
                                        "f32" => {buffer.append(&mut GetInstructionBytes(PUSH8(4, false)))}
                                        "f64" => {buffer.append(&mut GetInstructionBytes(PUSH8(8, false)))}

                                        _ => {panic!("invalid type! {:?}", typ)}
                                    }
                                    buffer.append(&mut GetInstructionBytes(GETMEM(vars)))
                                }
                            ParsedType::Array(_, _) => {panic!("array type not implemented")}
                        }
                        vars+=1;

                    }
                ParsedStatement::VarAssign(var, op, val) =>
                    {
                        match op {
                            BinaryOperator::Assign => {}
                            BinaryOperator::AddAssign => {}
                            BinaryOperator::SubtractAssign => {}
                            BinaryOperator::MultiplyAssign => {}
                            BinaryOperator::DivideAssign => {}
                            BinaryOperator::ModuloAssign => {}
                            BinaryOperator::AndAssign => {}
                            BinaryOperator::OrAssign => {}
                            BinaryOperator::XORAssign => {}
                            BinaryOperator::LeftShiftAssign => {}
                            BinaryOperator::RightShiftAssign => {}
                            _ => {panic!("unknown")}
                        }
                    }
                ParsedStatement::If(_, _, _) => {}
                ParsedStatement::Block(_) => {}
                ParsedStatement::ForLoop(_, _, _) => {}
                ParsedStatement::WhileLoop(_, _) => {}
                ParsedStatement::InfiniteLoop(_) => {}
                ParsedStatement::Continue => {}
                ParsedStatement::Break => {}
                ParsedStatement::Return(_) => {}
            }
        }

    }
}

fn InstructionsFromExpression(expr: ParsedExpression, varmap: HashMap<String,(u64, ParsedType)>) -> Vec<Instruction>
{
    let mut ret = Vec::new();
    match expr {
        ParsedExpression::Bool(val) => {
            ret.push(PUSH8(val.into(), false));
        }
        ParsedExpression::NumericConstant(val) => {
            match val {
                NumericConstant::Integer(val, format) => {
                    match format {
                        IntegerLiteralFormat::Binary => {
                            let result = i32::from_str_radix(&*val, 2);
                            ret.push(PUSH32(result.expect("error parsing int") as u32, true));
                        }
                        IntegerLiteralFormat::Octal => {
                            let result = i32::from_str_radix(&*val, 8);
                            ret.push(PUSH32(result.expect("error parsing int") as u32, true));
                        }
                        IntegerLiteralFormat::Decimal => {
                            let result = i32::from_str_radix(&*val, 10);
                            ret.push(PUSH32(result.expect("error parsing int") as u32, true));
                        }
                        IntegerLiteralFormat::Hexadecimal => {
                            let result = i32::from_str_radix(&*val, 16);
                            ret.push(PUSH32(result.expect("error parsing int") as u32, true));
                        }
                    }
                }
                NumericConstant::FloatingPoint(val, format) => {
                    match format {
                        FloatingPointLiteralFormat::Standard => {
                            let num = str::parse::<f32>(&*val).unwrap();
                            ret.push(PUSHS(num));
                        }
                        FloatingPointLiteralFormat::ENotation => {panic!("e notation floats not supported");}
                    }
                }
            }
        }
        ParsedExpression::StringLiteral(_) => {panic!("string literal not implemented");}
        ParsedExpression::CharLiteral(val) => {
            ret.push(PUSH8(val, false));
        }
        ParsedExpression::Var(val) => {
            if varmap.contains_key(&*val) {
                let var = varmap.get(&*val).unwrap();

                match (var.clone()).1 {
                    ParsedType::Name(path, val) => {
                        match val.as_str() {
                            "u8"  => {
                                ret.push(READ((*var).0, 0))
                            }
                            "u16" => {
                                ret.push(READ((*var).0, 1))
                            }
                            "u32" => {
                                ret.push(READ((*var).0, 2))
                            }
                            "u64" => {
                                ret.push(READ((*var).0, 3))
                            }
                            "i8"  => {
                                ret.push(READ((*var).0, 4))
                            }
                            "i16" => {
                                ret.push(READ((*var).0, 5))
                            }
                            "i32" => {
                                ret.push(READ((*var).0, 6))
                            }
                            "i64" => {
                                ret.push(READ((*var).0, 7))
                            }
                            "f32" => {
                                ret.push(READ((*var).0, 8))
                            }
                            "f64" => {
                                ret.push(READ((*var).0, 9))
                            }
                            _ => {panic!("unknown type")}
                        }
                    }
                    ParsedType::Array(_, _) => {panic!("no array type");}
                }


            }

        }
        ParsedExpression::NamespacedVar(_, _) => {panic!("namespacing not implemented")}
        ParsedExpression::Range(_, _, _, _) => {panic!("range not implemented")}
        ParsedExpression::Match(_, _) => {panic!("match not implemented")}
        ParsedExpression::Operator(op) => {
            panic!("im honestly not sure how an expression evals to just an operator");
        }
        ParsedExpression::UnaryOperation(expr, op) => {
            match expr.deref() {
                ParsedExpression::Bool(val) => {
                    match op {
                        UnaryOperator::PreIncrement => {}
                        UnaryOperator::PostIncrement => {}
                        UnaryOperator::PreDecrement => {}
                        UnaryOperator::PostDecrement => {}
                        UnaryOperator::LogicalNot => {}
                        UnaryOperator::BitwiseNot => {}
                        UnaryOperator::AddressOf => {}
                        UnaryOperator::Dereference => {}
                        UnaryOperator::TypeCast(_) => {}
                    }
                }
                ParsedExpression::NumericConstant(_) => {}
                ParsedExpression::StringLiteral(_) => {}
                ParsedExpression::CharLiteral(_) => {}
                ParsedExpression::Var(_) => {}
                ParsedExpression::NamespacedVar(_, _) => {}
                ParsedExpression::Range(_, _, _, _) => {}
                ParsedExpression::Match(_, _) => {}
                ParsedExpression::Operator(_) => {}
                ParsedExpression::UnaryOperation(_, _) => {}
                ParsedExpression::BinaryOperation(_, _, _) => {}
                ParsedExpression::ProcCall(_) => {}
                ParsedExpression::Invalid => {}
            }
        }
        ParsedExpression::BinaryOperation(_, _, _) => {}
        ParsedExpression::ProcCall(_) => {}
        ParsedExpression::Invalid => {}
    }
    return ret;
}

enum Instruction
{
    NOP,
    PUSH8(u8, bool),
    PUSH16(u16, bool),
    PUSH32(u32, bool),
    PUSH64(u64, bool),
    PUSHS(f32),
    PUSHD(f64),
    POP,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    TO8(bool),
    TO16(bool),
    TO32(bool),
    TO64(bool),
    TOS(),
    TOD(),
    CALL(u8, u64),
    RET,
    GETMEM(u64),
    WRITE(u64),
    READ(u64, u8),
    FREEMEM(u64),
    CLONE
}

fn GetInstructionBytes(instruction: Instruction) -> Vec<u8>
{
    let mut buffer = Vec::new();
    match instruction {
        Instruction::NOP => {
            for x in 0..12 {
                buffer.push(0);
            }}
        Instruction::PUSH8(val, sign) =>
            {
                buffer.push(1);
                buffer.push(0);
                buffer.push(sign.into());
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
                buffer.push(0);
                buffer.push(val);
            }
        Instruction::PUSH16(val, sign) => {
            buffer.push(2);
            buffer.push(0);
            buffer.push(sign.into());
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.append(&mut val.to_be_bytes().to_vec())
        }
        Instruction::PUSH32(val, sign) => {
            buffer.push(3);
            buffer.push(0);
            buffer.push(sign.into());
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.append(&mut val.to_be_bytes().to_vec())
        }
        Instruction::PUSH64(val, sign) => {
            buffer.push(4);
            buffer.push(0);
            buffer.push(sign.into());
            buffer.push(0);
            buffer.append(&mut val.to_be_bytes().to_vec())
        }
        Instruction::PUSHS(val) => {
            buffer.push(5);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.append(&mut val.to_be_bytes().to_vec())
        }
        Instruction::PUSHD(val) => {
            buffer.push(6);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.append(&mut val.to_be_bytes().to_vec())
        }
        Instruction::POP => {
            buffer.push(7);
            for x in 0..11 {
                buffer.push(0);
            }
        }
        Instruction::ADD => {
            buffer.push(8);
            for x in 0..11 {
                buffer.push(0);
            }
        }
        Instruction::SUB => {
            buffer.push(9);
            for x in 0..11 {
                buffer.push(0);
            }
        }
        Instruction::MUL => {
            buffer.push(0xa);
            for x in 0..11 {
                buffer.push(0);
            }
        }
        Instruction::DIV => {
            buffer.push(0xb);
            for x in 0..11 {
                buffer.push(0);
            }
        }
        Instruction::MOD => {
            buffer.push(0xc);
            for x in 0..11 {
                buffer.push(0);
            }
        }
        Instruction::TO8(sign) => {
            buffer.push(0xd);
            buffer.push(0);
            buffer.push(sign.into());
            for x in 0..9 {
                buffer.push(0);
            }
        }
        Instruction::TO16(sign) => {
            buffer.push(0xe);
            buffer.push(0);
            buffer.push(sign.into());
            for x in 0..9 {
                buffer.push(0);
            }
        }
        Instruction::TO32(sign) => {
            buffer.push(0xf);
            buffer.push(0);
            buffer.push(sign.into());
            for x in 0..9 {
                buffer.push(0);
            }
        }
        Instruction::TO64(sign) => {
            buffer.push(0x10);
            buffer.push(0);
            buffer.push(sign.into());
            for x in 0..9 {
                buffer.push(0);
            }
        }
        Instruction::TOS() => {
            buffer.push(0x11);
            buffer.push(0);
            buffer.push(0);
            for x in 0..9 {
                buffer.push(0);
            }
        }
        Instruction::TOD() => {
            buffer.push(0x12);
            buffer.push(0);
            buffer.push(0);
            for x in 0..9 {
                buffer.push(0);
            }
        }
        Instruction::CALL(bank, addr) => {
            buffer.push(0x13);
            buffer.push(0);
            buffer.push(bank);
            buffer.push(0);
            buffer.append(&mut addr.to_be_bytes().to_vec());
        }
        Instruction::RET => {
            buffer.push(0x14);
            buffer.push(0);
            buffer.push(0);
            for x in 0..9 {
                buffer.push(0);
            }
        }
        Instruction::GETMEM(id) => {
            buffer.push(0x15);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.append(&mut id.to_be_bytes().to_vec());
        }
        Instruction::WRITE(id) => {
            buffer.push(0x16);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.append(&mut id.to_be_bytes().to_vec());
        }
        Instruction::READ(id, typ) => {
            buffer.push(0x17);
            buffer.push(0);
            buffer.push(typ);
            buffer.push(0);
            buffer.append(&mut id.to_be_bytes().to_vec());
        }
        Instruction::FREEMEM(id) => {
            buffer.push(0x18);
            buffer.push(0);
            buffer.push(0);
            buffer.push(0);
            buffer.append(&mut id.to_be_bytes().to_vec());
        }
        Instruction::CLONE => {
            buffer.push(0x19);
            buffer.push(0);
            buffer.push(0);
            for x in 0..9 {
                buffer.push(0);
            }
        }
    }
    return buffer;
}