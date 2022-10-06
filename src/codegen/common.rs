use std::collections::HashMap;
#[allow(non_snake_case)]

use crate::token::Token;
use crate::tokenizer::Tokenizer;
use crate::parser::Parser;
use crate::ast::{BinaryOperator, ParsedExpression, ParsedModule, ParsedProcDecl, ParsedStatement, ParsedType, UnaryOperator};
use crate::token::{FloatingPointLiteralFormat, IntegerLiteralFormat, NumericConstant};

pub fn ParseAST(module: ParsedModule)
{


}

fn Pass1(module: ParsedModule)
{
    for x in module.structs {
        let mut vars = HashMap::new();
        for y in x.data_members {
            vars.insert(y.name, y.parsed_type);
        }

    }
}