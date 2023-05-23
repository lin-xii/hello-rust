fn main() {
    let config_max = Some(3);
    match config_max {
        Some(max) => println!("some:{}", max),
        _ => (),
    }
    // if let Some(max) = config_max {
    //     println!("The maximum is configured to be {}", max);
    // }
    dbg!(config_max);
}
