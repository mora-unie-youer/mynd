fn extract_line(input: &str) -> (&str, &str) {
    match input.split_once('\n') {
        Some(tuple) => tuple,
        None => (input, ""),
    }
}

fn parse_heading_level(input: &str) -> Option<(&str, (usize, &str))> {
    let (line, other) = extract_line(input);
    if !line.starts_with('*') {
        return None;
    }

    let (level, heading) = match line.split_once(char::is_whitespace) {
        Some((level, heading)) => {
            if level.chars().all(|c| c == '*') {
                (level.len(), heading.trim_start())
            } else {
                return None;
            }
        }
        None => return None,
    };

    Some((other, (level, heading)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_heading_level_works() {
        assert_eq!(parse_heading_level(""), None);
        assert_eq!(parse_heading_level("*"), None);
        assert_eq!(parse_heading_level("*a"), None);
        assert_eq!(parse_heading_level("* "), Some(("", (1, ""))));
        assert_eq!(parse_heading_level("** Test"), Some(("", (2, "Test"))));
        assert_eq!(parse_heading_level("**  Test"), Some(("", (2, "Test"))));
    }
}

fn main() {
    println!("Hello, world!");
}
