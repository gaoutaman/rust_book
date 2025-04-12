fn main() {
    println!("Hello, world!");

    another_function(543, '$');

    let g = times_two(20);
    println!("20 times 2 is {g}");
}

fn another_function(x: i32, y: char) {
    let a = {
        let b = 4;
        b + 5
    };
    println!("This is another function with arguments - {y}{x}");
    println!("{a}");
}

fn times_two(x: i32) -> i32 {
    x * 2
}
