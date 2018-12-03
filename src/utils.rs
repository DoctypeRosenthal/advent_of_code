pub fn split_by_newline(x: &str) -> Vec<&str> {
    x.split('\n').collect()
}

pub fn get_first(s: &str) -> char {
    s.chars().next().unwrap()
}