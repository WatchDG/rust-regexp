#[cfg(test)]
mod ipv4addr {
    use regexp;
    #[test]
    fn t1() {
        assert!(regexp::IP_V4_ADDR("0.0.0.0"));
    }
}
