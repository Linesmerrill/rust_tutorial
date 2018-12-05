const MAX_POINTS: u32 = 100_000;

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let y = 5;
    let y = y + 1;
    let y = y + 2;

    println!("The value of y is {}", y);

    let val = plus_one(5);

    println!("The value of val is: {}", val);
}

fn plus_one(x: i32) -> i32 {
    x + 2;
    0
}
