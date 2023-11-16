
use std::fmt;

enum Color {
    BLACK,
    BLUE,
    RED,
}

impl fmt::Display for Color {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        match self {
            Color::BLACK => write!( f, "Black"),
            Color::BLUE => write!( f, "Blue"),
            Color::RED => write!( f, "Red"),
        }
    }
}

struct Vehicle {
    color: Color,
    name: String,
    year: u16,
}

impl fmt::Display for Vehicle {
    fn fmt( &self, f: &mut fmt::Formatter ) -> fmt::Result {
        write!(f, "Name:{}\nYear:{}\nColor:{}", self.name, self.year, self.color )
    }
}

impl Vehicle {

    // Static method, not receive self
    fn new() -> Vehicle {

        let vehicle : Vehicle = Vehicle {
            color: Color::RED,
            name: "A Car".to_string(),
            year: 1997,
        };

        return vehicle;
    }

    // Instantiate method, receive self( current instantiate )
    fn set_color( &mut self, color : Color ){
        self.color = color;
    }

    fn set_name( &mut self, name : String ){
        self.name = name;
    }

    fn set_year( &mut self, year : u16 ){
        self.year = year;
    }

}

fn main(){

    let mut vehicle : Vehicle = Vehicle {
        color: Color::BLACK,
        name: String::from( "Marea turbo" ),
        year: 1805,
    };

    println!( "{}", vehicle );

    println!( "{:-^40}", "" );

    vehicle.set_color( Color::BLUE );
    vehicle.set_name( "Marea turbo V2".to_string() );
    vehicle.set_year( 1804 );

    println!( "{}", vehicle );

    let default_vehicle = Vehicle::new();

    println!( "Default Vehicle\n {}", default_vehicle );


}