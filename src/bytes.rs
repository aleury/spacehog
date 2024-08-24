#![allow(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation
)]

const UNIT_SIZE: f64 = 1000.0;

const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

pub fn prettify(bytes: u64) -> String {
    if bytes == 0 {
        return "0 B".to_string();
    }

    let exp = (bytes as f64).ln() / UNIT_SIZE.ln();
    let exp_floor = exp.floor() as usize;

    let (value, suffix) = if exp_floor >= UNITS.len() {
        (
            bytes as f64 / UNIT_SIZE.powi((UNITS.len() - 1) as i32),
            UNITS[UNITS.len() - 1],
        )
    } else {
        (
            bytes as f64 / UNIT_SIZE.powi(exp_floor as i32),
            UNITS[exp_floor],
        )
    };

    format!("{value:.0} {suffix}")
}

#[cfg(test)]
mod test {
    use super::prettify;
    #[test]
    fn prettify_formats_a_number_of_bytes_as_human_readable_text() {
        struct Case {
            size: u64,
            want: &'static str,
        }
        let cases = vec![
            Case {
                size: 0,
                want: "0 B",
            },
            Case {
                size: 525,
                want: "525 B",
            },
            Case {
                size: 1000,
                want: "1 KB",
            },
            Case {
                size: 1024,
                want: "1 KB",
            },
            Case {
                size: 2024,
                want: "2 KB",
            },
            Case {
                size: 2_000_000,
                want: "2 MB",
            },
            Case {
                size: 5_500_250,
                want: "6 MB",
            },
            Case {
                size: 1_000_000_000,
                want: "1 GB",
            },
            Case {
                size: 25_000_000_000,
                want: "25 GB",
            },
            Case {
                size: 3_000_000_000_000,
                want: "3 TB",
            },
            Case {
                size: 1_500_000_000_000_000,
                want: "1500 TB",
            },
        ];
        for case in cases {
            let got = prettify(case.size);
            assert_eq!(case.want.to_string(), got);
        }
    }
}
