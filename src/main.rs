/// 长方形
#[derive(Debug)]
struct Rectangle {
    /// 宽度
    width: u32,
    /// 高度
    height: u32,
}

impl Rectangle {
    /// 计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    // Output
    println!("rectangle is:{:#?}", rect1);
    dbg!(rect1.area());
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }
