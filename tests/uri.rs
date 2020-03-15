#[cfg(test)]
mod uri {
    mod authority {
        use regexp::uri::authority;

        #[test]
        fn test() {
            let result = authority::parse::RE.captures("localhost:1234").unwrap();
            assert_eq!(result.name("host").unwrap().as_str(), "localhost");
            assert_eq!(result.name("port").unwrap().as_str(), "1234");
        }
    }

    mod path {
        use regexp::PATH_EMPTY;

        #[test]
        fn test() {
            assert!(PATH_EMPTY(""))
        }
    }
}
