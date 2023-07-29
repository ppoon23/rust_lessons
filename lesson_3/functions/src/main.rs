fn main() {
    let x = plus_one(5);

    println!("The value of x + 1 is: {x}");
}

fn plus_one(x: i32) -> i32 {
    return x + 1
}