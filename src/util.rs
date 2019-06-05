// 2019-05-07

use std::fmt;
use crate::alphabet::char_to_struct;

pub struct Letter {
    pub line1: &'static str,
    pub line2: &'static str,
    pub line3: &'static str,
    pub line4: &'static str,
    pub line5: &'static str,
    pub line6: &'static str,
}

pub struct BundleOfStrings {
    pub line1: String,
    pub line2: String,
    pub line3: String,
    pub line4: String,
    pub line5: String,
    pub line6: String,
}


impl BundleOfStrings {
    pub fn new() -> Self {
        Self {
            line1: String::new(),
            line2: String::new(),
            line3: String::new(),
            line4: String::new(),
            line5: String::new(),
            line6: String::new(),
        }
    }
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\n{}\n{}\n{}\n{}\n{}",
            &self.line1,
            &self.line2,
            &self.line3,
            &self.line4,
            &self.line5,
            &self.line6,
        )
    }
}

impl fmt::Display for BundleOfStrings {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}\n{}\n{}\n{}\n{}\n{}",
            &self.line1,
            &self.line2,
            &self.line3,
            &self.line4,
            &self.line5,
            &self.line6,
        )
    }
}

// Converts the input into a vector of characters
pub fn string_to_char_vector(input: String) -> Vec<char> {

    let iterateur = input.chars();
    let mut vector_of_chars: Vec<char> = Vec::new();
    for character in iterateur {
        vector_of_chars.push(character)
    }
    vector_of_chars
}

// Splits the vector of characters into several ones, of max. 20 chars each
pub fn split_into_lines(vector_of_chars: Vec<char>) -> Vec<Vec<char>> {

    let mut vector_to_be_split: Vec<char> = vector_of_chars.clone();
    let mut lines: Vec<Vec<char>> = Vec::new();

    // drain the vector of its content, 20 chars at a time
    while vector_to_be_split.len() > 20 {
        let line: Vec<char> = vector_to_be_split.drain(0..20).collect();
        lines.push(line);
    }

    // add the rest
    lines.push(vector_to_be_split);

    lines
}

// This function contains two loops, one nested in the other
// 1. Iterates over the lines (vectors of characters)
    // 2. for each line, a loop iterates over each char to push its content
    // onto the "bundle", an instance of BundleOfStrings
// 1. Push the bundle to a vector of bundles, the 'paragraph', go to next line
pub fn decorate(lines: Vec<Vec<char>>) -> Vec<BundleOfStrings> {

    let mut paragraph: Vec<BundleOfStrings> = Vec::new();
    let iterate_over_lines = lines.iter();

    for line in iterate_over_lines {

        let mut bundle = BundleOfStrings::new();

        let iterate_over_letters = line.iter();

        for letter in iterate_over_letters {
            bundle.line1.push_str(char_to_struct(*letter).line1);
            bundle.line2.push_str(char_to_struct(*letter).line2);
            bundle.line3.push_str(char_to_struct(*letter).line3);
            bundle.line4.push_str(char_to_struct(*letter).line4);
            bundle.line5.push_str(char_to_struct(*letter).line5);
            bundle.line6.push_str(char_to_struct(*letter).line6);
        }

        paragraph.push(bundle);
    }

    paragraph
}

// this aggregates the six lines of the result into one
pub fn concatenate(paragraph: Vec<BundleOfStrings>) -> String {
    let mut end_result = String::new();
    let iterator = paragraph.iter();
    for ligne in iterator {
        end_result.push_str(&ligne.line1); end_result.push('\n');
        end_result.push_str(&ligne.line2); end_result.push('\n');
        end_result.push_str(&ligne.line3); end_result.push('\n');
        end_result.push_str(&ligne.line4); end_result.push('\n');
        end_result.push_str(&ligne.line5); end_result.push('\n');
        end_result.push_str(&ligne.line6); end_result.push('\n');
    }
    end_result
}
