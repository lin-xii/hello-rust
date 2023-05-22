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
    // self可以是ref, 也可以是variable, move ownership
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    if rect1.width() {
        println!("方法:method、字段:field可以同名");
    }

    // Output
    println!("rectangle's area is:{:#?}", rect1.area());
    dbg!(rect1);
}
