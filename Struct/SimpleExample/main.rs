use std::fmt;

struct Person {
    name: String,
    age: i32,
    cpf: String,
    has_children : bool,
    salary: f64,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Name:{}\nAge:{}\nCPF:{}\nHas Children:{}\nSalary:{}", self.name, self.age, self.cpf, self.has_children, self.salary )
    }
}

fn print_person( person: &Person ){
    println!( "{}", person );
}

fn main(){

    let person = Person {
        name: "Mike".to_string(),
        age: 32,
        cpf: String::from( "01234567890" ),
        has_children: true,
        salary: 1.421,
    };

    print_person( &person )

}