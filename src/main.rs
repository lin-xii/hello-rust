use std::println;

#[derive(Debug)]
struct User {
    // 用不了&str, 需要配合生命周期
    // 需要看的还很多
    name: String,
    // 实例只要调用过field, 就不会warning了.
    // 只能说rustc, 是真的强
    age: i32,
}

fn main() {
    let user1 = User {
        name: String::from("user_one"),
        age: 1,
    };
    // 相比println, dbg才是真正调试信息的工具!!!
    dbg!(user1.name);
}
