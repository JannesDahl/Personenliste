use std::env;
use std::fs::File;
use std::io;
use std::io::{Write, BufReader, BufRead, Error};

use person::Person;
use person::Genders;
mod person;

fn main() -> Result<(), Error> {
    let file_path = env::current_dir()?.display().to_string() + "\\textfiles\\test.txt";

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let write_result = write_to_file(&file_path, input);
    match write_result {
        Ok(file) => file,
        Err(error) => panic!("Problem writing in the file: {:?}", error),
    }

    let read_result = read_from_file(&file_path);
    match read_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }

    let person1 = Person{name: String::from("Albert Test"), age: 19, gender: Genders::Male};
    let mut person_list: Vec<Person> = Vec::new();
    person_list.push(person1);

    Ok(())
}

fn write_to_file(file_path: &String, file_content: String) -> Result<(), Error> {
    let mut output = File::create(file_path)?;
    write!(output, "{}", file_content)?;

    Ok(())
}

fn read_from_file(file_path: &String) -> Result<(), Error> {
    let input = File::open(file_path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}