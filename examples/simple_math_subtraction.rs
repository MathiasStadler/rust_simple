pub fn main() {
    let s = 2 - 1;
    println!("result of s ; {}", s);

    // inside function
    let _a: i32 = 2;
    let _b: i32 = 1;
    let _difference_subtraction = subtraction(_a, _b);
    println!(
        "result of subtraction inside function => {}",
        _difference_subtraction
    );
}

fn subtraction(a: i32, b: i32) -> i32 {
    a - b
}
