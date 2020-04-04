#[cfg(test)]
mod ipv6addr {
    use regexp::is_match::ip::V6;
    #[test]
    fn ae73404e_8421_4191_bd96_9e207a247ec7() {
        assert!(V6("FEDC:BA98:7654:3210:FEDC:BA98:7654:3210"));
        assert!(V6("::BA98:7654:3210:FEDC:BA98:7654:3210"));
        assert!(V6("FEDC::7654:3210:FEDC:BA98:7654:3210"));
        assert!(V6("FEDC:BA98::3210:FEDC:BA98:7654:3210"));
        assert!(V6("FEDC:BA98:7654::FEDC:BA98:7654:3210"));
        assert!(V6("FEDC:BA98:7654:3210::BA98:7654:3210"));
        assert!(V6("FEDC:BA98:7654:3210:FEDC::7654:3210"));
        assert!(V6("FEDC:BA98:7654:3210:FEDC:BA98::3210"));
        assert!(V6("FEDC:BA98:7654:3210:FEDC:BA98:7654::"));
        assert!(V6("::7654:3210:FEDC:BA98:7654:3210"));
        assert!(V6("FEDC::3210:FEDC:BA98:7654:3210"));
        assert!(V6("FEDC:BA98::FEDC:BA98:7654:3210"));
        assert!(V6("FEDC:BA98:7654::BA98:7654:3210"));
        assert!(V6("FEDC:BA98:7654:3210::7654:3210"));
        assert!(V6("FEDC:BA98:7654:3210:FEDC::3210"));
        assert!(V6("FEDC:BA98:7654:3210:FEDC:BA98::"));
        assert!(V6("::3210:FEDC:BA98:7654:3210"));
        assert!(V6("FEDC::FEDC:BA98:7654:3210"));
        assert!(V6("FEDC:BA98::BA98:7654:3210"));
        assert!(V6("FEDC:BA98:7654::7654:3210"));
        assert!(V6("FEDC:BA98:7654:3210::3210"));
        assert!(V6("FEDC:BA98:7654:3210:FEDC::"));
        assert!(V6("::FEDC:BA98:7654:3210"));
        assert!(V6("FEDC::BA98:7654:3210"));
        assert!(V6("FEDC:BA98::7654:3210"));
        assert!(V6("FEDC:BA98:7654::3210"));
        assert!(V6("FEDC:BA98:7654:3210::"));
        assert!(V6("::BA98:7654:3210"));
        assert!(V6("FEDC::7654:3210"));
        assert!(V6("FEDC:BA98::3210"));
        assert!(V6("FEDC:BA98:7654::"));
        assert!(V6("::7654:3210"));
        assert!(V6("FEDC::3210"));
        assert!(V6("FEDC:BA98::"));
        assert!(V6("FEDC::"));
        assert!(V6("::3210"));
        assert!(V6("::"));
    }
    #[test]
    #[should_panic]
    fn c1e0fbd0_affc_433a_a3e1_7c138fa4da90() {
        assert!(V6("::12345"));
    }
    #[test]
    #[should_panic]
    fn eecaf616_feb1_4d0c_9d51_279a88d5b4b5() {
        assert!(V6("12345::"));
    }
}
