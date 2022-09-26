mod ast;
mod codegen;
mod parser;
mod token;
mod tokenizer;

use token::Token;
use tokenizer::Tokenizer;
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
        idx: 0
    };

    let main_module = parser.parse_module();

    println!("main_module:");
    println!("    name: {}", main_module.name);
    println!("    structs:");
    for struct_decl in main_module.structs {
        println!("    |-->{}", struct_decl.name);
        for member in struct_decl.data_members {
            println!("        |-->{}: {}", member.name, match &member.parsed_type { ast::ParsedType::Name(_vec, name) => name, _ => panic!() });
        }
    }
    println!("    enums:");
    for struct_decl in main_module.enums {
        println!("    |-->{}", struct_decl.name);
    }
    println!("    procs:");
    for struct_decl in main_module.procs {
        println!("    |-->{}", struct_decl.name);
    }
}
