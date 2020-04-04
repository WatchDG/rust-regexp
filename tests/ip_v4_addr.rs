#[cfg(test)]
mod ipv4addr {
    use regexp::is_match::ip::V4;
    #[test]
    fn d7f7920f_c255_4fb1_939d_51ce1ab27bd2() {
        assert!(V4("0.0.0.0"));
    }
    #[test]
    #[should_panic]
    fn ea4bf8f4_9bcb_4325_828e_ba895424cac5() {
        assert!(V4("localhost"));
    }
    #[test]
    #[should_panic]
    fn a323fa60_69b0_4dd9_8a15_fd6afb37f81a() {
        assert!(V4("300.0.0.0"));
    }
    #[test]
    #[should_panic]
    fn ad19e394_897e_4a82_9ca4_beb4d3573abf() {
        assert!(V4("256.0.0.0"));
    }
    #[test]
    #[should_panic]
    fn b3592514_5ff1_45f1_8207_c0ce4ccc0757() {
        assert!(V4("260.0.0.0"));
    }
}
