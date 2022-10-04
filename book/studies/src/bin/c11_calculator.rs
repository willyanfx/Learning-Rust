pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}
pub fn divide(a: i32, b: i32) -> i32 {
    a / b
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sum_test() {
        assert_eq!(4, sum(2, 2));
        assert_eq!(10, sum(8, 2));
    }

    #[test]
    fn subtract_test() {
        assert_eq!(0, subtract(2, 2));
        assert_eq!(6, subtract(8, 2));
    }

    #[test]
    fn multiply_test() {
        assert_eq!(4, multiply(2, 2));
        assert_eq!(16, multiply(8, 2));
    }
    #[test]
    fn divide_test() {
        assert_eq!(1, divide(2, 2));
        assert_eq!(4, divide(8, 2));
    }
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
