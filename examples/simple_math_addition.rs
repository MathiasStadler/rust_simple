pub fn main() {
    // inside main
    let a = 1 + 1;
    println!("result of a ; {}", a);

    let s = 2 - 1;
    println!("result of s ; {}", s);
    
    // inside method
    let _a: i32 = 2;
    let _b: i32 = 1;
    let _sum_addition: i32 = addition(_a, _b);
    println!("result of function addition    => {}", _sum_addition);
    let _difference_subtraction = subtraction(_a, _b);
    println!("result of function subtraction => {}", _difference_subtraction);
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

fn subtraction(a: i32, b: i32) -> i32 {
    a + b
}