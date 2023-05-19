use std::println;

fn main() {
    let y = {
        let x = 1 + 1;
        println!("x addr: {:p}", &x); // x addr: 0x16d982b1c
        x
    };
    println!("y addr: {:p}", &y); // y addr: 0x16d982b18
                                  /*
                                  地址不一样??? 不应该是所有权么?
                                  看着应该一样.
                                  但因为scalar在stack, ownership的行为是copy, 所以地址不相同
                                   */
    let b = {
        let a = Box::new(44);
        println!("a addr: {:p}", &a); // a addr: 0x16bba69e8
        a
    };
    println!("b addr: {:p}", &b); // b addr: 0x16bba69e0
                                  /*
                                  已经是heap内存了, 但仍旧不符合预期.
                                  目前的猜想, 是因为定义方式的问题, 导致allocate了2块内存地址
                                  所以地址不一致
                                                                */

    let data = Box::new(42); // 在堆上分配一个整数
    println!("data addr: {}", data);
    let new_data = data;
    println!("new_data addr: {:p}", new_data);
    /*
    data、new_data地址一致. 复合预期
    因为顺序allocate内存, 所以直接move ownership时, memory address不变化
     */

    /*
    char 4byte
    bool 1byte
    float 8byte
    integer 根据长度变化
     */
    let pointer_size = true;
}
