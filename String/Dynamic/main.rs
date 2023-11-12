use std::io;

fn main() {

    let mut name = String::new();
    println!("{:-^40}", "Input you name");

    io::stdin().read_line( &mut name ).expect("Error reading console");

    println!( "Name: {name}" );

}