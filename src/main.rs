use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

use person::Person;
use person::Genders;
mod person;

fn main() -> Result<(), Error> {
    let file_path = "C:\\Rust\\Personenliste\\textfiles\\test.txt";
    
    concatenate_strings("This is".to_string());
    let write_result = write_to_file(file_path);
    match write_result {
        Ok(file) => file,
        Err(error) => panic!("Problem writing in the file: {:?}", error),
    }

    let read_result = read_from_file(file_path);
    match read_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    }

    let person1 = Person{name: String::from("Albert Test"), age: 19, gender: Genders::Male};
    let mut person_list: Vec<Person> = Vec::new();
    person_list.push(person1);

    Ok(())
}

fn write_to_file(file_path: &str) -> Result<(), Error> {
    let mut output = File::create(file_path)?;
    write!(output, "File Output")?;

    Ok(())
}

fn read_from_file(file_path: &str) -> Result<(), Error> {
    let input = File::open(file_path)?;
    let buffered = BufReader::new(input);

    for line in buffered.lines() {
        println!("{}", line?);
    }

    Ok(())
}

fn concatenate_strings(foo: String) {
    let mut owned_string = foo.to_owned();
    let borrowed_string: &str = " concatenated";
    
    owned_string.push_str(borrowed_string);
    println!("{owned_string}");
}