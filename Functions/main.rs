
fn say_hello() {
    println!("Hello");
}

fn say_something( ds_something : &str ) {
    println!( "{ds_something}" );
}

fn addition( first : i32, second : i32 ) -> i32 {
    return first + second;
}

fn subtraction( first : i32, second : i32 ) -> i32 {
    first - second
}

fn multiplication( first : i32, second : i32 ) -> i32 {
    let result = {
        first * second
    };

    result
}

fn main(){

    say_hello();

    say_something( "1-2-3" );

    // Debug mode = :?
    println!("{:?}", addition( 5, 5 ) );
    println!("{:?}", subtraction( 5, 5 ) );
    println!("{:?}", multiplication( 5, 5 ) );


}