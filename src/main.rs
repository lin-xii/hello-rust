fn main() {
    // doc: 验证patch
    let example_closure = |x| x;
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

// 改点代码, 试试rebase
// 搞点代码, 再弄一次rebase
// 改点代码, 试试rebase one
// 改点代码, 试试rebase twice
// 改点代码, 试试rebase
