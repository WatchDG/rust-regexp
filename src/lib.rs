#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

/// Regular expression for IPv4Address.
/// # Example:
/// ```
/// use regexp;
/// println!("{}", regexp::IP_V4_ADDR("0.0.0.0"));
/// ```
pub static IP_V4_ADDR: fn(&str) -> bool = |data: &str| -> bool {
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

/// Regular expression for IPv6Address.
/// # Example:
/// ```
/// use regexp;
/// println!("{}", regexp::IP_V6_ADDR("::"));
/// ```
pub static IP_V6_ADDR: fn(&str) -> bool = |data: &str| -> bool {
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

pub static PATH_EMPTY: fn(&str) -> bool = |data: &str| -> bool {
    lazy_static! {
        static ref PATH_EMPTY_RE: Regex = Regex::new(r"^$").unwrap();
    }
    PATH_EMPTY_RE.is_match(data)
};
