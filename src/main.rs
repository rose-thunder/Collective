fn main() {
    println!("Hello, world!");
    let result = add_numbers(2, 3);
    println!("{}", result)
}

fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y;
    result
}
