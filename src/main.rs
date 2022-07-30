mod ast;
mod parser;
mod token;
mod tokenizer;

use token::Token;
use tokenizer::Tokenizer;

fn main() {
    let thing: u32 = 0xdeadbeef;
    let mut tokenizer = Tokenizer::new(String::from("test_files/test.amds"));

    //tokenizer.dump_file_contents();

    let mut current_token = tokenizer.read_next_token();
    let mut token_stream = Vec::new();

    while current_token != Token::EOF {
        token_stream.push(current_token);
        current_token = tokenizer.read_next_token();
    }

    let main_module = parser::parse_module(&token_stream, 0);
    println!("main_module:");
    println!("    name: {}", main_module.name);
    println!("    structs:");
    for struct_decl in main_module.structs {
        println!("    |-->{}", struct_decl.name);
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
