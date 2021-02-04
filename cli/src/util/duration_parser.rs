/// Parse the time taken in seconds from the given string in the format
/// "Xh Xm Xs" where X must be a number and h, m or s are optional.
pub(crate) fn parse_duration(src: &str) -> Result<i32, String> {
    let mut hours = 0;
    let mut minutes = 0;
    let mut seconds = 0;

    let mut number_buffer: String = String::new();
    for char in src.chars() {
        let is_hours = char == 'h';
        let is_minutes = char == 'm';
        let is_seconds = char == 's';

        let consume_buffer = is_hours || is_minutes || is_seconds;
        if consume_buffer {
            // Parse number from buffer and clear it
            let num: i32 = match number_buffer.trim().parse() {
                Ok(res) => Ok(res),
                Err(_) => Err(format!(
                    "Could not parse time taken from given string '{}'",
                    src
                )),
            }?;

            if is_hours {
                hours += num;
            } else if is_minutes {
                minutes += num;
            } else if is_seconds {
                seconds += num;
            }

            number_buffer.clear();
        } else {
            // Add to buffer
            number_buffer.push(char);
        }
    }

    return Ok(hours * 60 * 60 + minutes * 60 + seconds);
}

/// Format a duration given in seconds in the form "Xh Xm Xs".
pub(crate) fn format_duration(mut seconds: u32) -> String {
    let hours = seconds / 60 / 60;
    seconds -= hours * 60 * 60;

    let minutes = seconds / 60;
    seconds -= minutes * 60;

    let mut result = Vec::new();
    if hours > 0 {
        result.push(format!("{}h", hours));
    }
    if minutes > 0 {
        result.push(format!("{}m", minutes));
    }
    if seconds > 0 {
        result.push(format!("{}s", seconds));
    }

    result.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatting_1s() {
        assert_eq!("1s", format_duration(1));
    }

    #[test]
    fn test_formatting_1m() {
        assert_eq!("1m", format_duration(60));
    }

    #[test]
    fn test_formatting_1h() {
        assert_eq!("1h", format_duration(60 * 60));
    }

    #[test]
    fn test_formatting_complex_1() {
        assert_eq!("651h 43m 55s", format_duration(2346235));
    }

    #[test]
    fn test_formatting_complex_2() {
        assert_eq!("46m", format_duration(2760));
    }

    #[test]
    fn test_formatting_complex_3() {
        assert_eq!("2h 13m", format_duration(7980));
    }

    #[test]
    fn test_formatting_complex_4() {
        assert_eq!("2h 30s", format_duration(7230));
    }
}
