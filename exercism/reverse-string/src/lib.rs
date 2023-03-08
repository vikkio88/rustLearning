pub fn reverse(input: &str) -> String {
    String::from(input.chars().rev().fold("".to_string(), |mut r, c: char| {
        r.push_str(&c.to_string());
        r
    }))
}
