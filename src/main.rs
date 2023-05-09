fn main() {
    let mut a = 1;
    println!("mut a's addr is {:p}", &a);
    a = 2;
    println!("a reAssign's addr is {:p}", &a); // mut variable. 修改的是stack的数据, 不会重新入栈

    let b = 3;
    println!("b's addr is {:p}", &b);
    let b = 4;
    println!("shadow b's addr is {:p}", &b); // shadow. 会重新入栈

    let c = 5;
    println!("c's addr is {:p}", &c);
    let d = c;
    println!("d's addr is {:p} and c is live {:p}", &d, &c); // re assgin. 所有权move. 但这块属于copy了

    let e = 6;
    println!("e's addr is {:p}", &e);
    do_something(e);
    println!("after fn. e's addr is {:p}", &e); // scalar fn 传參, 所有权会变动, 但属于copy. 原scope的值, 可以继续使用

    // TODO: compound的所有权, 是个什么样子. 目测和scalar会有区别. 因为compound涉及heap数据了
    // 所以, 其实reference, 解决的是compound的问题? 待验证. 看了官方文档对String做🌰的描述, 更加确信了. reference解决的就是复杂类型(heap数据)的多对一关系
}

fn do_something(n: i32) {
    println!("this is do_sometiong. n's addr is {:p}", &n);
}
