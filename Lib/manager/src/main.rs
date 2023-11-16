use std::io;

use model::*;
use person::*;

fn main() {
    let mut person = Person::new( "".to_string(), 0 );

    println!("{:-^40}", "Input you name");

    let mut ds_name = String::new();

    io::stdin()
        .read_line(&mut ds_name)
        .expect("Error reading console");

    person.set_name(ds_name);

    println!("{:-^40}", "Input age");

    let mut ds_age = String::new();

    io::stdin()
        .read_line(&mut ds_age)
        .expect("Error reading console");

    let age: Result<u8, _> = ds_age.trim().parse();

    match age {
        Ok(value) => {
            person.set_age(value);
        }
        Err(_) => {
            println!("Invalid Input");
        }
    }

    println!("{:-^40}", "");

    println!("Is Valid: {}", service::is_valid(person));
}
