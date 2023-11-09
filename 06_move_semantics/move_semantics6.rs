// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.

fn main() {
    let data = "Rust is great!".to_string();

    let last_char = get_char(data.clone()); // Clone to preserve ownership
    println!("Last char: {}", last_char);

    let uppercase_data = string_uppercase(&data); // Pass a reference to the string

    // Printing the modified string
    println!("{}", uppercase_data);
}

// Should not take ownership
fn get_char(data: String) -> char {
    data.chars().last().unwrap()
}

// Should take ownership
fn string_uppercase(data: &String) -> String {
    data.to_uppercase() // Return a new modified string
}
