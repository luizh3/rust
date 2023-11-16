use model::*;
use person::*;

pub fn is_valid(person: Person) -> bool {
    return person.age() > 0 && !person.name().is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_without_name() {
        let person = Person::new("".to_string(), 90);
        assert_eq!(is_valid(person), false);
    }

    #[test]
    fn test_is_valid_without_age() {
        let person = Person::new("Eren Yeager".to_string(), 0);
        assert_eq!(is_valid(person), false);
    }

    #[test]
    fn test_is_valid_with_name_and_age() {
        let person = Person::new("Eren Yeager".to_string(), 19);
        assert_eq!(is_valid(person), true);
    }
}
