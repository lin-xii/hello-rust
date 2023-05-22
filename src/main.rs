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

    fn can_hold(&self, another: &Rectangle) -> bool {
        self.width > another.width && self.height > another.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };
    let rect2 = Rectangle {
        width: 100,
        height: 5,
    };
    let square1 = Rectangle::square(33);

    // Output
    println!("rectangle's area is:{:#?}", rect1.area());
    // dbg!(rect1);
    dbg!(rect1.can_hold(&rect2));
    dbg!(square1);
}
