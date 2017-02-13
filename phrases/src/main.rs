extern crate phrases;
use phrases::chinese::{greetings, farewells};
use phrases::english as english_sayings;

fn main() {
    println!("Hello in Chinese: {}", greetings::hello());
    println!("Goodbye in Chinese: {}", farewells::goodbye());
    println!("Hello in English: {}", english_sayings::greetings::hello());
    println!("Goodbye in English: {}", english_sayings::farewells::goodbye());
    println!("Hello in Japanese: {}", phrases::japanese::hello());
    println!("Goodbye in Japanese: {}", phrases::japanese::goodbye());
}
