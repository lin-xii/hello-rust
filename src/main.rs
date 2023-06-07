fn main() {
    // doc: 验证patch
    let example_closure = |x| x;
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}

// 改点代码, 试试rebase
// 搞点代码, 再弄一次rebase
// 改点代码, 试试rebase one
// 改点代码, 试试rebase twice
// 改点代码, 试试rebase
// 搞点代码, 再弄一次rebase
