extern crate rust_euler;

mod tests {
    use rust_euler::rust_euler;
    #[test]
    fn function_1() {
        match rust_euler::problem_1() {
            Some (x) => assert_eq!(x, 233168),
            None     => assert!(false),
        }
    }
    #[test]
    fn function_2() {
        match rust_euler::problem_2() {
            Some (x) => assert_eq!(x, 4613732),
            None     => assert!(false),
        }
    }
    #[test]
    fn function_3() {
        match rust_euler::problem_3() {
            Some (x) => assert_eq!(x, 6857),
            None     => assert!(false),
        }
    }
}