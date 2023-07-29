use std::io;

fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';  // cool

    println!("x, y: {x}, {y}");
    println!("c, z: {c}, {z}");
    println!("heart_eyed_cat: {heart_eyed_cat}");

    // Tuples
    // 0-indexing
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
    println!("{five_hundred}, {six_point_four}, {one}");

    // let u = ();  // unit

    // Array - stack, array with fixed length
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // signed 32 integers with length of 5
    let zeros = [0; 10]; // 10 zeros
    
    let zeros_1 = zeros[1];
    println!("{}, {}", a[0], zeros_1);


    println!("Running an exercise");
    
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}