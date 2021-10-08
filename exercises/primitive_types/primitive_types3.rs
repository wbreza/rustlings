// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

// I AM DONE

fn main() {
    let a = [false; 200];

    if a.len() >= 100 {
        println!("Wow, that's a big array! {}", a.len());
    } else {
        println!("Meh, I eat arrays like that for breakfast. {}", a.len());
    }
}
