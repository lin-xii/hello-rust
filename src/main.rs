use std::println;

fn main() {
    anthoer_fn(5)
}

fn anthoer_fn(x: i32) {
    println!("{:p}", &x)
}
