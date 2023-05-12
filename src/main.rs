use std::println;

fn main() {
    /* scalar: bool char */
    // let type_bool = true; // bool 这个没什么疑问
    // println!("type_bool: {}", type_bool);
    // let type_char: char = 'c'; // 字符, 仅一个. 4Byte == 32bit
    //                            // Byte is B. bit is b.
    // println!("type_char: {}", type_char);

    /* conclusion: mut scalar, 修改的是内存地址内的数据. 值变, 但内存地址相同 */
    // let mut mut_num = 3;
    // println!("mut i32's addr: {:p}", &mut_num); // mut scalar's addr: 0x16f35ea24
    // mut_num = 5;
    // println!("i32 changed. Had addr changed?: {:p}", &mut_num); // i32 changed. Had addr changed?: 0x16f35ea24

    /* unsure: char &str String difference? */
    let type_str = "slice"; // 切片类型 slice. 数据也是在stack的
    let mut hello = "hello rust!"; // 也就是说, slice相当于不可变字符串
    println!("addr: {:p}", hello);
    hello = "hello rust cc";
    println!("changed addr: {:p}", hello);
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
