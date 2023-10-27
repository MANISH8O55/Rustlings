// primitive_types6.rs
// Use a tuple index to access the second element of `numbers`.
// You can put the expression for the second element where ??? is so that the test passes.
// Execute `rustlings hint primitive_types6` or use the `hint` watch subcommand for a hint.



fn main() {
    // Define a tuple
    let numbers = (1, 2, 3);

    // Use tuple indexing to access the second element (which is 2)
    let second_element = numbers.1;

    // Assert that the second_element is equal to 2
    assert_eq!(2, second_element, "This is not the 2nd number in the tuple!");
}
