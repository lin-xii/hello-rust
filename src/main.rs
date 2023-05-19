use std::println;

fn main() {
    let result = celsius_to_fahrenheit(&28);
    println!("result: {}", result)
}

fn celsius_to_fahrenheit(celsius: &i32) -> i32 {
    (celsius + 40) * 9 / 5
}

fn fahrenheit_to_celsius(fahrenheit: &i32) -> i32 {
    fahrenheit * 5 / 9
}
