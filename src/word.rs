use std::io::{Error};
use stdweb::unstable::{TryInto};

mod word_list;

pub struct Word {
    text: String,
    capitalise: bool,
    // lock: bool
}

impl Word {
    pub fn random_word() -> Result<Word, Error> {

        let line_number = js!{
            return Math.floor(Math.random() * 370103 );
        };
        let line_number: f64 = line_number.try_into().unwrap();
        let line_number: usize = line_number as usize;

        let word = word_list::get_word(line_number).to_string();

        Ok(
            Word {
                text: word,
                capitalise: true,
                // lock: false
            }
        )
    }

    pub fn to_string(&self) -> String {
        let output: String;
        if self.capitalise {
            let mut c = self.text.chars();
            output = match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        } else {
            output = self.text.clone();
        }
        output
    }
}