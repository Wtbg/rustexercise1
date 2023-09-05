#[allow(dead_code)]
pub fn increment(x: Vec<char>) -> Vec<char> {
    x.into_iter()
        .map(|c| std::char::from_u32((c as u32) + 1).unwrap())
        .collect()
}
