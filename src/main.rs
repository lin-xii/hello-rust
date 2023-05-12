use std::println;

fn main() {
    // scalar
    let type_bool = true; // bool 这个没什么疑问
    println!("type_bool: {}", type_bool);

    let type_char: char = 'c'; // 字符, 仅一个. 4Byte == 32bit
                               // Byte is B. bit is b.
    println!("type_char: {}", type_char);

    // unsure: char &str String 区别?
    let type_str = "slice"; // 切片类型 slice. 数据也是在stack的
    let hello = "hello rust!"; // 也就是说, slice相当于不可变字符串
    console_debug(type_str);

    let mut type_string = String::from("hello rust");
    type_string.push_str("hello");
    println!("this variable's type is what?: {}", type_str);
    println!("this variable's type is what?: {}", type_string);
}

fn console_debug(text: &str) {
    println!("console_debug: {}", text);
    println!("console_debug_addr: {:p}", text)
}
