#[cfg(test)]
mod uri {
    mod path {
        use regexp::PATH_EMPTY;
        #[test]
        fn test() {
            assert!(PATH_EMPTY(""))
        }
    }
}
