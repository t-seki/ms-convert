const MILLISECOND: f64 = 1.0;
const SECOND: f64 = 1_000.0;
const MINUTE: f64 = SECOND * 60.0;
const HOUR: f64 = MINUTE * 60.0;
const DAY: f64 = HOUR * 24.0;
const WEEK: f64 = DAY * 7.0;
const MONTH: f64 = DAY * 30.0;
const YEAR: f64 = DAY * 365.25;

pub fn parse(input: &str) -> Result<f64, ParseError> {
    // 1. 入力の長さバリデーション (1..=99)
    // 2. 正規表現 or 手動パースで数値と単位を分離
    // 3. 単位に応じて乗数を決定 (match 式)
    // 4. 計算結果を返す

    if input.is_empty() {
        return Err(ParseError::EmptyInput);
    }

    let input = input.trim();

    if input.len() > 100 {
        return Err(ParseError::TooLong);
    }

    // 数値部分の終端を探す
    let split = input
        .find(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
        .ok_or(ParseError::InvalidFormat)?;

    let (num_part, unit_part) = input.split_at(split);
    let num: f64 = num_part.parse().map_err(|_| ParseError::InvalidFormat)?;
    let unit = unit_part.trim();

    let multiplier = match unit {
        "ms" | "millisecond" | "milliseconds" => MILLISECOND,
        "s"| "sec" | "second" | "seconds" => SECOND,
        "m" | "min" | "minute" | "minutes" => MINUTE,
        "h" | "hr" | "hour" | "hours" => HOUR,
        "d" | "day" | "days" => DAY,
        "w" | "week" | "weeks" => WEEK,
        "mo" | "month" | "months" => MONTH,
        "y" | "year" | "years" => YEAR,
        _ => return Err(ParseError::UnknownUnit(unit.to_string()))
    };

    Ok(num * multiplier)
}

pub fn format(ms: f64, long: bool) -> String {
    let abs = ms.abs();

    let (val, short, singular, plural) = if abs >= YEAR {
        (ms / YEAR, "y", "year", "years")
    } else if abs >= MONTH {
        (ms / MONTH, "mo", "month", "months")
    } else if abs >= WEEK {
        (ms / WEEK, "w", "week", "weeks")
    } else if abs >= DAY {
        (ms / DAY, "d", "day", "days")
    } else if abs >= HOUR {
        (ms / HOUR, "h", "hour", "hours")
    } else if abs >= MINUTE {
        (ms / MINUTE, "m", "minute", "minutes")
    } else if abs >= SECOND {
        (ms / SECOND, "s", "second", "seconds")
    } else {
        (ms, "ms", "millisecond", "milliseconds")
    };

    let rounded = val.round() as i64;

    if long {
        let unit = if rounded.abs() == 1 { singular } else { plural };
        format!("{rounded} {unit}")
    } else {
        format!("{rounded}{short}")
    }
}

#[derive(Debug, PartialEq)]
pub enum ParseError {
    EmptyInput,
    TooLong,
    InvalidFormat,
    UnknownUnit(String),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::EmptyInput => write!(f, "input must not be empty"),
            ParseError::TooLong => write!(f, "input exceeds 100 characters"),
            ParseError::InvalidFormat => write!(f, "invalid time format"),
            ParseError::UnknownUnit(u) => write!(f, "unknown unit: {u}"),
        }
    }
}

impl std::error::Error for ParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    // --- parse: 各単位 (short) ---

    #[test]
    fn parse_milliseconds() {
        assert_eq!(parse("100ms").unwrap(), 100.0);
    }

    #[test]
    fn parse_seconds() {
        assert_eq!(parse("1s").unwrap(), 1_000.0);
    }

    #[test]
    fn parse_minutes() {
        assert_eq!(parse("1m").unwrap(), 60_000.0);
    }

    #[test]
    fn parse_hours() {
        assert_eq!(parse("1h").unwrap(), 3_600_000.0);
    }

    #[test]
    fn parse_days() {
        assert_eq!(parse("2d").unwrap(), 172_800_000.0);
    }

    #[test]
    fn parse_weeks() {
        assert_eq!(parse("3w").unwrap(), 1_814_400_000.0);
    }

    #[test]
    fn parse_months() {
        assert_eq!(parse("1mo").unwrap(), 2_592_000_000.0);
    }

    #[test]
    fn parse_years() {
        assert_eq!(parse("1y").unwrap(), 31_557_600_000.0);
    }

    // --- parse: 各単位 (long) ---

    #[test]
    fn parse_long_milliseconds() {
        assert_eq!(parse("1 millisecond").unwrap(), 1.0);
        assert_eq!(parse("53 milliseconds").unwrap(), 53.0);
    }

    #[test]
    fn parse_long_seconds() {
        assert_eq!(parse("1 sec").unwrap(), 1_000.0);
        assert_eq!(parse("1 second").unwrap(), 1_000.0);
        assert_eq!(parse("2 seconds").unwrap(), 2_000.0);
    }

    #[test]
    fn parse_long_minutes() {
        assert_eq!(parse("1 min").unwrap(), 60_000.0);
        assert_eq!(parse("1 minute").unwrap(), 60_000.0);
        assert_eq!(parse("2 minutes").unwrap(), 120_000.0);
    }

    #[test]
    fn parse_long_hours() {
        assert_eq!(parse("1 hr").unwrap(), 3_600_000.0);
        assert_eq!(parse("1 hour").unwrap(), 3_600_000.0);
        assert_eq!(parse("2 hours").unwrap(), 7_200_000.0);
    }

    #[test]
    fn parse_long_days() {
        assert_eq!(parse("1 day").unwrap(), 86_400_000.0);
        assert_eq!(parse("2 days").unwrap(), 172_800_000.0);
    }

    #[test]
    fn parse_long_weeks() {
        assert_eq!(parse("1 week").unwrap(), 604_800_000.0);
        assert_eq!(parse("2 weeks").unwrap(), 1_209_600_000.0);
    }

    #[test]
    fn parse_long_months() {
        assert_eq!(parse("1 month").unwrap(), 2_592_000_000.0);
        assert_eq!(parse("2 months").unwrap(), 5_184_000_000.0);
    }

    #[test]
    fn parse_long_years() {
        assert_eq!(parse("1 year").unwrap(), 31_557_600_000.0);
        assert_eq!(parse("2 years").unwrap(), 63_115_200_000.0);
    }

    // --- parse: ゼロ値 ---

    #[test]
    fn parse_zero() {
        assert_eq!(parse("0ms").unwrap(), 0.0);
    }

    // --- parse: 小数 ---

    #[test]
    fn parse_decimal() {
        assert_eq!(parse("1.5h").unwrap(), 5_400_000.0);
    }

    #[test]
    fn parse_leading_dot() {
        assert_eq!(parse(".5ms").unwrap(), 0.5);
    }

    #[test]
    fn parse_negative() {
        assert_eq!(parse("-3 days").unwrap(), -259_200_000.0);
        assert_eq!(parse("-100ms").unwrap(), -100.0);
        assert_eq!(parse("-1.5h").unwrap(), -5_400_000.0);
    }

    #[test]
    fn parse_negative_leading_dot() {
        assert_eq!(parse("-.5h").unwrap(), -1_800_000.0);
    }

    // --- parse: 空白 ---

    #[test]
    fn parse_extra_whitespace() {
        assert_eq!(parse("1   s").unwrap(), 1_000.0);
    }

    // --- parse: 前後空白 ---

    #[test]
    fn parse_leading_trailing_whitespace() {
        assert_eq!(parse(" 1s ").unwrap(), 1_000.0);
    }

    // --- parse: エラー ---

    #[test]
    fn parse_empty() {
        assert_eq!(parse("").unwrap_err(), ParseError::EmptyInput);
    }

    #[test]
    fn parse_whitespace_only() {
        assert_eq!(parse("   ").unwrap_err(), ParseError::InvalidFormat);
    }

    #[test]
    fn parse_number_only() {
        assert_eq!(parse("100").unwrap_err(), ParseError::InvalidFormat);
    }

    #[test]
    fn parse_no_unit() {
        assert_eq!(parse("abc").unwrap_err(), ParseError::InvalidFormat);
    }

    #[test]
    fn parse_unknown_unit() {
        assert_eq!(
            parse("1xyz").unwrap_err(),
            ParseError::UnknownUnit("xyz".to_string())
        );
    }

    #[test]
    fn parse_boundary_100_chars() {
        let input = format!("1{}", " ".repeat(97) + "s");
        assert_eq!(parse(&input).unwrap(), 1_000.0);
    }

    #[test]
    fn parse_too_long() {
        let long_input = format!("1{}", "a".repeat(101));
        assert_eq!(parse(&long_input).unwrap_err(), ParseError::TooLong);
    }

    // --- format: short ---

    #[test]
    fn format_short_ms() {
        assert_eq!(format(500.0, false), "500ms");
    }

    #[test]
    fn format_short_seconds() {
        assert_eq!(format(1_000.0, false), "1s");
    }

    #[test]
    fn format_short_minutes() {
        assert_eq!(format(60_000.0, false), "1m");
    }

    #[test]
    fn format_short_hours() {
        assert_eq!(format(3_600_000.0, false), "1h");
    }

    #[test]
    fn format_short_days() {
        assert_eq!(format(86_400_000.0, false), "1d");
    }

    #[test]
    fn format_short_weeks() {
        assert_eq!(format(604_800_000.0, false), "1w");
    }

    #[test]
    fn format_short_months() {
        assert_eq!(format(2_628_000_000.0, false), "1mo");
    }

    #[test]
    fn format_short_years() {
        assert_eq!(format(31_557_600_000.0, false), "1y");
    }

    #[test]
    fn format_short_rounding() {
        assert_eq!(format(234_234_234.0, false), "3d");
    }

    // --- format: long ---

    #[test]
    fn format_long_ms() {
        assert_eq!(format(500.0, true), "500 milliseconds");
    }

    #[test]
    fn format_long_negative_ms() {
        assert_eq!(format(-500.0, true), "-500 milliseconds");
    }

    #[test]
    fn format_long_second() {
        assert_eq!(format(1_000.0, true), "1 second");
    }

    #[test]
    fn format_long_seconds() {
        assert_eq!(format(10_000.0, true), "10 seconds");
    }

    #[test]
    fn format_long_minute() {
        assert_eq!(format(60_000.0, true), "1 minute");
    }

    #[test]
    fn format_long_minutes() {
        assert_eq!(format(120_000.0, true), "2 minutes");
        assert_eq!(format(600_000.0, true), "10 minutes");
    }

    #[test]
    fn format_long_hour() {
        assert_eq!(format(3_600_000.0, true), "1 hour");
    }

    #[test]
    fn format_long_hours() {
        assert_eq!(format(36_000_000.0, true), "10 hours");
    }

    #[test]
    fn format_long_day() {
        assert_eq!(format(86_400_000.0, true), "1 day");
    }

    #[test]
    fn format_long_week() {
        assert_eq!(format(604_800_000.0, true), "1 week");
    }

    #[test]
    fn format_long_month() {
        assert_eq!(format(2_628_000_000.0, true), "1 month");
    }

    #[test]
    fn format_long_year() {
        assert_eq!(format(31_557_600_000.0, true), "1 year");
    }

    #[test]
    fn format_long_rounding() {
        assert_eq!(format(234_234_234.0, true), "3 days");
    }
}
