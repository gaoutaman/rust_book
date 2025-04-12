fn main() {
    // let s = String::from("hello");
    let s = "hello";

    takes_ownwership(s);

    println!("{s} after ownership");

    let x = 5;

    makes_copy(x);

    println!("{x} after copy");
}

fn takes_ownwership(some_string: &str) {
    println!("{some_string}");
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}
