
pub struct Person {
    name : String,
    age : u8
}

impl Person {
    pub fn new( new_name: String, new_age : u8 ) -> Person {
        return Person {
            name: new_name,
            age: new_age,
        }
    }

    pub fn name( &self ) -> &String {
        return &self.name;
    }

    pub fn age( &self ) -> u8 {
        return self.age;
    }

    pub fn set_name( &mut self, name : String ) {
        self.name = name;
    }

    pub fn set_age( &mut self, age : u8 ){
        self.age = age;
    }
}