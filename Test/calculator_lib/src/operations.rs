
#[allow(dead_code)]
fn sum( first: i16, second: i16 ) -> i16 {
    return first + second;
}

pub fn subtraction( first: i16, second: i16 ) -> i16 {
    return first - second;
}

pub fn division( first: i16, second: i16 ) -> i16 {

    if second == 0 {
        panic!("Division by 0");
    }

    return first / second;

}

// Unit Test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum(){
        assert_eq!( 4, sum( 2, 2 ) );
    }
}
