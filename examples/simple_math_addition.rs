pub fn main() {
    // inside main
    let a = 2 + 1;
    println!("result of a => {}", a);

    // inside function
    let _a: i32 = 2;
    let _b: i32 = 1;
    let _sum_addition: i32 = addition(_a, _b);
    println!("result of addition inside function    => {}", _sum_addition);
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn first_simple_easy_successful_test() {
        assert_eq!(1, 1);
    }

    #[test]
    #[should_panic]
    fn first_simple_easy_failed_test() {
        assert_eq!(1, 0);
    }

    #[test]
    fn test_function() {
        assert_eq!(addition(2, 1), 3);
    }
}
