fn extract_line(input: &str) -> (&str, &str) {
    match input.split_once('\n') {
        Some(tuple) => tuple,
        None => (input, ""),
    }
}

fn main() {
    println!("Hello, world!");
}
