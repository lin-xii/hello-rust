/// 长方形
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

    let area_rectangle = area(rect1);

    // Output
    dbg!(area_rectangle);
}

fn area(rectangle: Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
