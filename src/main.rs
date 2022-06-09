mod token;
mod tokenizer;

use token::Token;
use tokenizer::Tokenizer;

fn main() {
    let mut tokenizer = Tokenizer::new(String::from("test_files/test.amds"));

    let mut token = tokenizer.read_next_token();

    while token != Token::EOF {
        println!("{:?}", token);

        token = tokenizer.read_next_token();
    }
}
