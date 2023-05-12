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

    /* memory addr allocate rule */
    // let num1: u8 = 1; // i32, 4byte
    // let num2: u8 = 2; // u8 1byte
    // println!("num1 & num2's addr: {:p} {:p}", &num1, &num2);
    /* conclusion: 从指针地址看, 0x16b326a06->0x16b326a07, 只增加了1. 看起来, 指针地址是以byte为最小增长单位的  */

    /* pointer地址, 是什么进制的? 16进制 */
    // TODO: 指针地址、内存16进制计算
    // let num_1: u8 = 2;
    // println!("num_1's addr: {:p}", &num_1);
    // let num_2: u8 = 3;
    // println!("num_2's addr: {:p}", &num_2);
    // let num_3: u8 = 4;
    // println!("num_3's addr: {:p}", &num_3);
    /*
    num_1's addr: 0x16d97297f
    num_2's addr: 0x16d9729cf
    num_3's addr: 0x16d972a1f
    看起来, 是4byte, 但16进制, 怎么计算来着???
     */

    /* unsure: char &str String difference? */
    // let type_str = "slice"; // 切片类型 slice. 数据也是在stack的
    //                         // slice 指向的指针, 在stack. 但实际的值, 在全局静态内存
    //                         // TODO: 相当于段内存吧?
    // let mut hello: &str = "h"; // 也就是说, slice相当于不可变字符串
    // println!("addr: {:p}", hello);
    // hello = "he";
    // println!("changed addr: {:p}", hello);
    // let hello = "hel";
    // println!("shadow hello's addr is: {:p}", hello);
    // console_debug(type_str);

    // let mut type_string = String::from("hello rust");
    // type_string.push_str("hello");
    // println!("this variable's type is what?: {}", type_str);
    // println!("this variable's type is what?: {}", type_string);

    /* &str, 因为都是immutable ref, 自身就不是值. 没有所有权, 所以, 不会报错 */
    // let a = "hello rust";
    // let b = a;
    // println!("a and b is available: {},{}", a, b); //ref没有ownership, 所以, 不会报错
}

fn console_debug(text: &str) {
    println!("console_debug: {}", text);
    println!("console_debug_addr: {:p}", text)
}
