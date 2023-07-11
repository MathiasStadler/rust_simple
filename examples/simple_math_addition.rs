pub fn main() {
    // inside main
    let a = 1 + 1;
    println!("result of a ; {}", a);

    // inside function
    let _a: i32 = 2;
    let _b: i32 = 1;
    let _sum_addition: i32 = addition(_a, _b);
    println!("result of function addition    => {}", _sum_addition);
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}
