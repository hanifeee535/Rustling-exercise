// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let vec0 = vec![22, 44, 66];

    let vec1 = fill_vec(); // Call the function without any arguments

    assert_eq!(vec1, vec![22, 44, 66, 88]);
}

// `fill_vec()` no longer takes `vec: Vec<i32>` as argument - don't change this!
fn fill_vec() -> Vec<i32> {
    let mut vec = vec![22, 44, 66]; // Create a new Vec<i32> and initialize it with values

    vec.push(88); // Add an element to the Vec

    vec // Return the modified vector
}