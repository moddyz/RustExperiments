mod test {
    #[test]
    fn test_assert() {
        assert!(2 + 2 == 4, "2 + 2 should equal 4");
        assert_eq!(2 + 2, 4);
        assert_ne!(2 + 2, 3);
    }

    #[test]
    #[should_panic]
    fn test_panic() {
        panic!("Breaking...");
    }

    #[test]
    fn test_result() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("2 + 2 did not equal 4!"))
        }
    }
}
