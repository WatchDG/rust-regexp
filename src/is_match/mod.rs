use regex::Regex;

pub mod ip {
    use super::Regex;

    /// IPv4Address.
    /// # Example:
    /// ```
    /// use regexp::is_match::ip::V4;
    /// assert!(V4("0.0.0.0"));
    /// ```
    pub static V4: fn(&str) -> bool = |data: &str| -> bool {
        lazy_static! {
            static ref IP_V4_ADDR_RE: Regex = Regex::new(
                r"(?x)
            ^
            (?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)
            \.
            (?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)
            \.
            (?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)
            \.
            (?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)
            $
            "
            )
            .unwrap();
        }
        IP_V4_ADDR_RE.is_match(data)
    };

    /// IPv6Address.
    /// # Example:
    /// ```
    /// use regexp::is_match::ip::V6;
    /// assert!(V6("::"));
    /// ```
    pub static V6: fn(&str) -> bool = |data: &str| -> bool {
        lazy_static! {
            static ref IP_V6_ADDR_RE: Regex = Regex::new(
                r"(?x)
            ^(?:[\dA-Fa-f]{1,4}:){6}(?:[\dA-Fa-f]{1,4}:[\dA-Fa-f]{1,4}|(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d))$
            |
            ^::(?:[\dA-Fa-f]{1,4}:){5}(?:[\dA-Fa-f]{1,4}:[\dA-Fa-f]{1,4}|(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d))$
            |
            ^(?:[\dA-Fa-f]{1,4})?::(?:[\dA-Fa-f]{1,4}:){4}(?:[\dA-Fa-f]{1,4}:[\dA-Fa-f]{1,4}|(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d))$
            |
            ^(?:(?:[\dA-Fa-f]{1,4}:)?[\dA-Fa-f]{1,4})?::(?:[\dA-Fa-f]{1,4}:){3}(?:[\dA-Fa-f]{1,4}:[\dA-Fa-f]{1,4}|(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d))$
            |
            ^(?:(?:[\dA-Fa-f]{1,4}:){0,2}[\dA-Fa-f]{1,4})?::(?:[\dA-Fa-f]{1,4}:){2}(?:[\dA-Fa-f]{1,4}:[\dA-Fa-f]{1,4}|(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d))$
            |
            ^(?:(?:[\dA-Fa-f]{1,4}:){0,3}[\dA-Fa-f]{1,4})?::[\dA-Fa-f]{1,4}:(?:[\dA-Fa-f]{1,4}:[\dA-Fa-f]{1,4}|(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d))$
            |
            ^(?:(?:[\dA-Fa-f]{1,4}:){0,4}[\dA-Fa-f]{1,4})?::(?:[\dA-Fa-f]{1,4}:[\dA-Fa-f]{1,4}|(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d)\.(?:25[0-5]|2[0-4]\d|1\d\d|[1-9]\d|\d))$
            |
            ^(?:(?:[\dA-Fa-f]{1,4}:){0,5}[\dA-Fa-f]{1,4})?::[\dA-Fa-f]{1,4}$
            |
            ^(?:(?:[\dA-Fa-f]{1,4}:){0,6}[\dA-Fa-f]{1,4})?::$
            "
            )
            .unwrap();
        }
        IP_V6_ADDR_RE.is_match(data)
    };
}

pub mod uri {
    use super::Regex;

    /// Scheme.
    /// # Example:
    /// ```
    /// use regexp::is_match::uri::SCHEME;
    /// assert!(SCHEME("https"));
    /// ```
    pub static SCHEME: fn(&str) -> bool = |data: &str| -> bool {
        lazy_static! {
            static ref URI_SCHEME_RE: Regex = Regex::new(r"^[:alpha:][[:alpha:]\d+-\.]*$").unwrap();
        }
        URI_SCHEME_RE.is_match(data)
    };

    pub mod authority {
        use super::Regex;

        /// Scheme.
        /// # Example:
        /// ```
        /// use regexp::is_match::uri::authority::PORT;
        /// assert!(PORT("80"));
        /// ```
        pub static PORT: fn(&str) -> bool = |data: &str| -> bool {
            lazy_static! {
                static ref URI_AUTHORITY_PORT_RE: Regex = Regex::new(r"^\d*$").unwrap();
            }
            URI_AUTHORITY_PORT_RE.is_match(data)
        };
    }
}
