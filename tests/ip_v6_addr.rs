#[cfg(test)]
mod ipv6addr {
    use regexp;
    #[test]
    fn fe767226_8ef4_4604_8b95_f0992150ffeb() {
        assert!(regexp::IP_V6_ADDR("::"));
    }
    #[test]
    fn e4ae8fcb_6a19_4e12_820e_8b727fcac61a() {
        assert!(regexp::IP_V6_ADDR(
            "FEDC:BA98:7654:3210:FEDC:BA98:7654:3210"
        ));
    }
    #[test]
    fn b6ad5796_3c91_4e60_a861_a219ddf73970() {
        assert!(regexp::IP_V6_ADDR("1080::8:800:200C:417A"));
    }
    #[test]
    #[should_panic]
    fn c1e0fbd0_affc_433a_a3e1_7c138fa4da90() {
        assert!(regexp::IP_V6_ADDR("::12345"));
    }
    #[test]
    #[should_panic]
    fn eecaf616_feb1_4d0c_9d51_279a88d5b4b5() {
        assert!(regexp::IP_V6_ADDR("12345::"));
    }
}
