extern crate core;

mod ast;
mod codegen;
mod parser;
mod token;
mod tokenizer;
mod typechecker;

use std::borrow::Borrow;
use token::Token;
use tokenizer::Tokenizer;
use typechecker::Typechecker;
use parser::Parser;

fn main() {
    let mut tokenizer = Tokenizer::new(String::from("test_files/test.amds"));
    tokenizer.dump_file_contents();

    let mut current_token = tokenizer.read_next_token();
    let mut token_stream = Vec::new();

    while current_token != Token::EOF {
        token_stream.push(current_token);
        current_token = tokenizer.read_next_token();
    }

    let mut parser = Parser {
        token_stream,
        idx: 0,
        typechecker: Typechecker::new()
    };

    let main_module = parser.parse_module();

    println!("main_module:");
    println!("    name: {}", main_module.name);
    println!("    structs:");
    for struct_decl in main_module.structs.clone() {
        println!("    |-->{}", struct_decl.name);
        for member in struct_decl.data_members {
            println!("        |-->{}: {}", member.name, match &member.parsed_type { ast::ParsedType::Name(_vec, name) => name, _ => panic!() });
        }
    }
    println!("    enums:");
    for struct_decl in main_module.enums.clone() {
        println!("    |-->{}", struct_decl.name);
    }
    println!("    procs:");
    for struct_decl in main_module.procs.clone() {
        println!("    |-->{}", struct_decl.name);
    }

    println!("\n\nAttempting to generate IL, this will probably panic");
    codegen::AIL::CreateIL(main_module);
    println!("Done!");

}
