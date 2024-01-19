use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

use person::Person;
use person::Genders;
mod person;

fn main() -> Result<(), Error> {
    let path = "C:\\Rust\\Personenliste\\textfiles\\test.txt";

    let person1 = Person{name: String::from("Albert Test"), age: 19, gender: Genders::Male};

    let insertPerson = "Name: {} Age: {} Gender: {}" + person1.name + person1.age + person1.gender.to_string();

    let mut output = File::create(path)?;
    write!(output, "{}", insertPerson)?;

    let input = File::open(path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}