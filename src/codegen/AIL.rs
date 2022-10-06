#[allow(non_snake_case)]

use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::ops::Deref;
use std::path::Path;
use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::ast::{BinaryOperator, ParsedExpression, ParsedModule, ParsedProcDecl, ParsedStatement, ParsedType, UnaryOperator};
use crate::codegen::AIL::Instruction::*;
use crate::token::{FloatingPointLiteralFormat, IntegerLiteralFormat, NumericConstant};

pub fn CreateIL(ast: ParsedModule)
{

}

//fn InstructionsFromExpression(expr: ParsedExpression, varmap: HashMap<String,(u64, ParsedType)>) -> Vec<Instruction>
//{

//}

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
    MALLOC(u64), //size in bytes
    WRITE(u64, u64), //addr, offset
    READ(u64, u64 ,u8), //addr, offset, size <= 8
    FREEMEM(u64),
    CLONE,
    SWRITE(u64), //uses address on stack, arg is offset
    MEMCPY(u64, u64, u64, u64, u64), // addr, offset, addr, offset, size
    EQ,  //comparison ops
    GT,
    LT,
    GE,
    LE,
    NE,
    JIT(u64), //jump if true
    JMP(u64), //uncond jump
    RJP(i64), //relative uncond jmp
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
        /*Instruction::GETMEM(id) => {
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
        }*/
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
        _ => {}
    }
    return buffer;
}
