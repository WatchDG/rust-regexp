#[cfg(test)]
mod ipv6addr {
    use regexp;

    #[test]
    fn fe767226_8ef4_4604_8b95_f0992150ffeb() {
        assert!(regexp::IP_V6_ADDR("::"));
    }
    // TODO: fix
    //    #[test]
    //    #[should_panic]
    //    fn c1e0fbd0_affc_433a_a3e1_7c138fa4da90() {
    //        assert!(regexp::IP_V6_ADDR("::12345"));
    //    }
}
