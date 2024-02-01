use rand;
use rand::{thread_rng, Rng};
use std::char::from_u32;

fn main() {
    let password_length: i32 = 15;
    let mut result: String = String::new();

    for _ in 0..password_length {
        let number: u32 = thread_rng().gen_range(48..122);
        let ch: char = from_u32(number).unwrap();
        result.push(ch);
    }

    println!("\nPassword Generated:\n{result}");
}
