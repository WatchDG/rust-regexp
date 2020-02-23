#[cfg(test)]
mod ipv6addr {
    use regexp;
    #[test]
    fn t1() {
        assert!(regexp::IP_V6_ADDR("::"));
    }
}
