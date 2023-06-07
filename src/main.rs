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
    // doc: 验证patch
    let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
    let n = example_closure(5);
}

// 改点代码, 试试rebase
// 搞点代码, 再弄一次rebase
// 改点代码, 试试rebase one
// 改点代码, 试试rebase twice
