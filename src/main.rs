struct User {
    name: String,
    age: i32,
    is_student: bool,
}

fn main() {
    let user1 = User {
        name: String::from("rust"),
        age: 5,
        is_student: false,
    };
    println!("main: {}", user1.name);
}
