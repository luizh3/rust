use calculator_lib::*;

// Integration Test
#[test] 
fn test_subtraction() {
    assert_eq!( 4, operations::subtraction( 8, 4 ) );
}

#[test] 
#[should_panic(expected = "Divide by zero, error")]
fn test_subtraction_panic() {
    assert_eq!( 0, operations::subtraction( 8, 0 ) )
}