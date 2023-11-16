

fn main() {

    // Primitives 

    let is_something : bool = true;
    
    let float : f64 = 1.10;
    let integer : i8 = 10; // i8, i16, i32, i64, i128
    let unasigned_integer : u8 = 10; // u8, u16, u32, u64, u128
    let character : char =  'a';

    println!("{} {} {} {} {}", is_something, float, integer, unasigned_integer, character );
    
    let values_tuple = ( 10i8, 2u32, false );

    println!("{:?}", values_tuple );

    println!("{}", values_tuple.0 );

    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    println!("{:?}", tuple_of_tuples);

    let array_integer : [u32; 3] = [ 1, 2, 3];

    println!("{:?}", array_integer );

    println!("{}", array_integer[0] );

    // Custom 

    let str_string : &str = "Ola mundo";

    let string : String = "Ola mundo".to_string();

    println!("{}\n{}", str_string, string );

    let vec = vec![1, 2, 3, 4];

    for x in vec.iter().rev() {
        println!("{x:?}");
    }

    let mut new_vec = Vec::new();

    new_vec.push( 10 );
    new_vec.push( 15 );

    let value_vec : u8 = new_vec.pop().unwrap();

    println!("{:?} {} {}", new_vec, new_vec.len(), value_vec );

    if let Some( last_value_vec ) = new_vec.pop() {
        println!("{}", last_value_vec );
    }

    match new_vec.pop() {
        Some( value ) => {
            println!("{}", value );
        }
        None => {
            println!("Is Empty");
        }
    }



}