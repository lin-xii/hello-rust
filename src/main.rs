fn main() {
    // addition
    let mut sum = 5 + 10;

    {
        sum = sum + 1
    }

    println!("{}", sum);

    let y = {
        let x = 3;
        x + 1; // 需要去掉分号, 才符合预期. 语法确实有点奇怪起来了...
    };

    println!("The value of y is: {y}");
}
