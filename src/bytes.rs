#![allow(clippy::cast_precision_loss)]
use std::fmt::Display;

#[must_use]
pub fn humanize(bytes: u64) -> String {
    Unit::from(bytes).to_string()
}

const BASE: u64 = 1_000;

enum Unit {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
    Terabytes(f64),
    Petabytes(f64),
    Exabytes(f64),
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Bytes(v) => write!(f, "{v} B"),
            Unit::Kilobytes(v) => write!(f, "{v:.0} KB"),
            Unit::Megabytes(v) => write!(f, "{v:.1} MB"),
            Unit::Gigabytes(v) => write!(f, "{v:.2} GB"),
            Unit::Terabytes(v) => write!(f, "{v:.2} TB"),
            Unit::Petabytes(v) => write!(f, "{v:.2} PB"),
            Unit::Exabytes(v) => write!(f, "{v:.2} EB"),
        }
    }
}

impl From<u64> for Unit {
    fn from(value: u64) -> Self {
        if value < BASE {
            return Unit::Bytes(value);
        }
        let exponent = value.ilog10() / BASE.ilog10();
        let value = value as f64 / BASE.pow(exponent) as f64;
        match exponent {
            1 => Unit::Kilobytes(value),
            2 => Unit::Megabytes(value),
            3 => Unit::Gigabytes(value),
            4 => Unit::Terabytes(value),
            5 => Unit::Petabytes(value),
            _ => Unit::Exabytes(value),
        }
    }
}

#[cfg(test)]
mod test {
    use super::humanize;

    #[test]
    fn humanize_returns_bytes_in_a_human_readable_format_using_si_units() {
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
        assert_eq!(humanize(100_421_000_000_000_000), "100.42 PB");
        assert_eq!(humanize(1_000_421_000_000_000_000), "1.00 EB");
        assert_eq!(humanize(6_500_421_000_000_000_000), "6.50 EB");
        assert_eq!(humanize(18_446_744_073_709_551_615), "18.45 EB");
    }
}
