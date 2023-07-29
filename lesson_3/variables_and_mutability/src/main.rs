fn main() {
    // Int addition
    println!("1 + 2 = {}", 1u32 + 2);

    // Int subtraction
    println!("1 - 2  = {}", 1i32 - 2);

    // Define variables
    let a: i16 = 1;
    let b = 2i16;

    println!("{}", a + b);

    // Mutable
    let mut a_mut = 12;
    println!("OLD a_mut {}", a_mut);
    a_mut = 100;
    println!("Mutataed a_mut {}", a_mut);

    // mismatched types error
    // a_mut = "error";

    let a_mut = "i am shadowing";

    println!("{}", a_mut);

    // shadowing use case with inner and outer scopes
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");

    // Setting Constant using `const` 

    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

    println!("THREE HOURS IN SECOND: {}", THREE_HOURS_IN_SECONDS)
}