fn extract_line(input: &str) -> (&str, &str) {
    match input.split_once('\n') {
        Some(tuple) => tuple,
        None => (input, ""),
    }
}

fn parse_heading_level(input: &str) -> Option<(&str, usize, &str)> {
    let (line, other) = extract_line(input);
    if !line.starts_with('*') {
        return None;
    }

    let (level, heading) = match line.split_once(' ') {
        Some((level, heading)) => {
            if level.chars().all(|c| c == '*') {
                (level.len(), heading.trim_start())
            } else {
                return None;
            }
        }
        None => return None,
    };

    Some((other, level, heading))
}

fn main() {
    println!("Hello, world!");
}
