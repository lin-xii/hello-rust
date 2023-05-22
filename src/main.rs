/// 长方形
#[derive(Debug)]
struct Rectangle {
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };
    let area_rectangle = area(&rect1);

    // Output
    println!("rectangle is:{:#?}", rect1);
    dbg!(area_rectangle);
    // dbg! 会拿走ownership, 打印的时候, 需要考虑这个问题
    dbg!(rect1);
    println!("{:#?}", rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
