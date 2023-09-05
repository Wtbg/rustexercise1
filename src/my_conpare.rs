#[allow(dead_code)]
#[allow(non_snake_case)]
pub fn compareString(x: &str, y: &str) -> bool {
    let x_vec: Vec<char> = x.chars().collect();
    let y_vec: Vec<char> = y.chars().collect();
    let x_len = x_vec.len();
    let y_len = y_vec.len();
    for i in 0..x_len {
        if i >= y_len {
            return true;
        }
        match x_vec[i].cmp(&y_vec[i]) {
            std::cmp::Ordering::Equal => {}
            std::cmp::Ordering::Less => {
                return true;
            }
            std::cmp::Ordering::Greater => {
                return false;
            }
        }
    }
    false
}
