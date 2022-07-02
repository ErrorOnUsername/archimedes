mod ast;
mod parser;
mod types;
mod token;
mod tokenizer;

use token::Token;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new(String::from("test_files/test.amds"));

    tokenizer.dump_file_contents();

    let mut token = tokenizer.read_next_token();

    while token != Token::EOF {
        println!("{:?}", token);

        token = tokenizer.read_next_token();
    }
}
