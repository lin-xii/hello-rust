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
/*
以下是 Rust 中摄氏度和华氏度互相转换的示例函数：
fn to_fahrenheit(celsius: f32) -> f32 {
    (celsius * 1.8) + 32.0
}
 fn to_celsius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) / 1.8
}
您可以将这些函数放在一个模块中，并在您的 Rust 移植性程序中使用：
mod temperature {
    pub fn to_fahrenheit(celsius: f32) -> f32 {
        (celsius * 1.8) + 32.0
    }
     pub fn to_celsius(fahrenheit: f32) -> f32 {
        (fahrenheit - 32.0) / 1.8
    }
}
 fn main() {
    let temp_in_celsius = 30.0;
    let temp_in_fahrenheit = temperature::to_fahrenheit(temp_in_celsius);
    println!("{}°C = {}°F", temp_in_celsius, temp_in_fahrenheit);
     let temp_in_fahrenheit = 86.0;
    let temp_in_celsius = temperature::to_celsius(temp_in_fahrenheit);
    println!("{}°F = {}°C", temp_in_fahrenheit, temp_in_celsius);
}
输出应如下所示：
30°C = 86°F
86°F = 30°C
请注意调用函数时传入的参数类型。在这个例子中，我们使用了  `f32`  类型，您可以根据需要进行修改。
 */
