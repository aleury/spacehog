#![allow(clippy::cast_precision_loss)]

const UNITS: [&str; 6] = ["B", "KB", "MB", "GB", "TB", "PB"];

const BASE: u64 = 1000;

pub fn humanize(bytes: u64) -> String {
    if bytes < BASE {
        return format!("{bytes} B");
    }
    let exponent = bytes.ilog10() / BASE.ilog10();
    let unit = UNITS[exponent as usize];
    let value = bytes as f64 / BASE.pow(exponent) as f64;
    let precision = match unit {
        "KB" => 0,
        "MB" => 1,
        _ => 2,
    };
    format!("{value:.precision$} {unit}")
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
