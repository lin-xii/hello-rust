use std::println;

#[derive(Debug)]
struct User {
    // 用不了&str, 需要配合生命周期
    // 需要看的还很多
    name: String,
    age: i32,
}

fn main() {
    let user1 = User {
        name: String::from("user_one"),
        age: 1,
    };
    println!("{:?}", user1);
    // 相比println, dbg才是真正调试信息的工具!!!
    dbg!(user1);
}
