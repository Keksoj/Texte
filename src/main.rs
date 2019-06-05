// 2019-05-07

// The goal of this program is to mimic the functioning of figlet / toilet
// The program takes a string from standard input and outputs a string

// The real challenge there is to handle the data types

mod alphabet;
mod util;
use util::string_to_char_vector;
use util::split_into_lines;
use util::decorate;
use util::concatenate;
use std::io;

fn main() {

    let mut input: String = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => { }
        Err(error) => println!("error: {}", error),
    }

    let vector_of_chars = string_to_char_vector(input);

    let vector_of_chars_vectors = split_into_lines(vector_of_chars);

    let end_result = decorate(vector_of_chars_vectors);

    println!("{}", concatenate(end_result));

}
