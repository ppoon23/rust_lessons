fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2);
    println!("{s3}");
    println!("{s1}");

    let (s4, s4_len) = calculate_length(s1);
    println!("{s4}, {s4_len}");
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// Returning ownership of parameters
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
