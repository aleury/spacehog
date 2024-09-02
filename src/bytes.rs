#![allow(clippy::cast_precision_loss)]

use std::fmt::Display;

pub fn humanize(bytes: u64) -> String {
    Unit::from(bytes).to_string()
}

enum Unit {
    B(u64),
    KB(f64),
    MB(f64),
    GB(f64),
    TB(f64),
    PB(f64),
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::B(v) => write!(f, "{v} B"),
            Unit::KB(v) => write!(f, "{v:.0} KB"),
            Unit::MB(v) => write!(f, "{v:.1} MB"),
            Unit::GB(v) => write!(f, "{v:.2} GB"),
            Unit::TB(v) => write!(f, "{v:.2} TB"),
            Unit::PB(v) => write!(f, "{v:.2} PB"),
        }
    }
}

impl From<u64> for Unit {
    fn from(value: u64) -> Self {
        const BASE: u64 = 1000;
        if value < BASE {
            Unit::B(value)
        } else {
            let exponent = value.ilog10() / BASE.ilog10();
            let value = value as f64 / BASE.pow(exponent) as f64;
            match exponent {
                1 => Unit::KB(value),
                2 => Unit::MB(value),
                3 => Unit::GB(value),
                4 => Unit::TB(value),
                _ => Unit::PB(value),
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::humanize;

    #[test]
    fn test_humanize() {
        assert_eq!(humanize(0), "0 B");
        assert_eq!(humanize(256), "256 B");
        assert_eq!(humanize(512), "512 B");
        assert_eq!(humanize(1_000), "1 KB");
        assert_eq!(humanize(2_650), "3 KB");
        assert_eq!(humanize(737_525), "738 KB");
        assert_eq!(humanize(1_000_000), "1.0 MB");
        assert_eq!(humanize(1_240_000), "1.2 MB");
        assert_eq!(humanize(1_250_000), "1.2 MB");
        assert_eq!(humanize(1_260_000), "1.3 MB");
        assert_eq!(humanize(10_525_000), "10.5 MB");
        assert_eq!(humanize(2_886_000_000), "2.89 GB");
        assert_eq!(humanize(200_500_150_001), "200.50 GB");
        assert_eq!(humanize(50_000_000_000_000), "50.00 TB");
        assert_eq!(humanize(1_421_000_000_000_000), "1.42 PB");
    }
}
