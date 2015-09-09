extern crate rust_euler;

mod tests {
    use rust_euler::rust_euler;
    #[test]
    fn function_1() {
        assert_eq!(233168, rust_euler::problem_1().0);
    }
    #[test]
    fn function_2() {
        assert_eq!(4613732, rust_euler::problem_2().0);
    }
    #[test]
    fn function_3() {
        assert_eq!(6857, rust_euler::problem_3().0);
    }
}