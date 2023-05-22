fn main() {
    let width = 30;
    let height = 50;

    let area_rectangle = area(width, height);
    dbg!(area_rectangle);
}

fn area(witdh: u32, height: u32) -> u32 {
    witdh * height
}
