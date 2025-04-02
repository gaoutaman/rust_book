const DAYS_IN_FOUR_WEEKS: u8 = 7 * 4;

fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    println!("Days in 4 weeks is {DAYS_IN_FOUR_WEEKS}");

    let y = 5;
    let y = 5 + 1; //shadowing
    {
        let y = y * 2;
        println!("The value of y in inner scope is {y}");
    }
    println!("The value of y in outer scope is {y}");

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // use pattern matching to destructure a tuple
    let (x, y, z) = tup;
    println!("Values of tuple are {x}, {y}, {z}.");

    let arr = [1, 2, 3, 4];

    // type and number of elements
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    let same_array = [0; 10]; // 10 zeros.
    // access araray with [] e.g. same_array[3]
}
