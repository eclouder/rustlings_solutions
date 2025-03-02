#![allow(clippy::ptr_arg)]

// Shouldn't take ownership
fn get_char(data: &String) -> char { // Change to reference
    data.chars().last().unwrap() // Use the reference
}

// Should take ownership
fn string_uppercase(data: &mut String) {
    *data = data.to_uppercase();
    println!("{}", data);
}

fn main() {
    let mut data = "Rust is great!".to_string(); // Make data mutable

    get_char(&data); // Pass a reference to the function

    string_uppercase(&mut data); // Pass a mutable reference to the function
}