use std::println;

fn main() {
    let mut a = 1;
    println!("before: {:p}", &a);
    plus_one(&mut a);
    println!("after: {}", a)
}

// &mut ref能修改值么?
// stack的不行? heap的可以?
// 光知道String是可以的
// scalar需要配合deref * 才能修改值
fn plus_one(num: &mut i32) {
    *num = *num + 1;
    println!("plus_one: {:p}", num);
}
