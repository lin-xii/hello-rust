fn main() {
    let mut x = 5;
    x = 10;
    println!("is had x ?   {x}");
    println!("x addr {:p}", &x);
    do_something(x);
    println!("after do_something {x}");
    println!("after do_something addr {:p}", &x); // 地址没变
}

fn do_something(mut s: i32) {
    println!("this is doSomething's scope addr {:p}", &s);
    println!("this is doSomething's scope  {s}");
    s = 6;
    println!("this is doSomething's scope  {s}");
    println!("this is doSomething's scope  addr{:p}", &s);
}
