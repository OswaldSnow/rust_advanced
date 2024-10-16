#[cfg(test)]
mod tests {

    #[test]
    fn tests() {
        println!("this is tests function");
    }

    #[test]
    #[ignore]
    fn test_0() {
        println!("this is test+0 function");
        assert!(true);
    }

    #[test]
    fn test_1() {
        println!("this is test_1 function");
        assert_eq!(1, 1);
    }
}
