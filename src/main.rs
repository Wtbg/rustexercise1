mod my_conpare;
mod my_itmap;
mod my_vec;
fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_vec() {
        let v_i32 = vec![1, 2, 3, 4, 5];
        let v_i64 = vec![1, 2, 3, 4, 5];
        let v_f32 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let v_f64 = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let v_single = vec![1];
        assert_eq!(my_vec::Buffer::create(&v_i32).sum(), Some(15));
        assert_eq!(my_vec::Buffer::create(&v_i64).sum(), Some(15));
        assert_eq!(my_vec::Buffer::create(&v_f32).sum(), Some(15.0));
        assert_eq!(my_vec::Buffer::create(&v_f64).sum(), Some(15.0));
        assert_eq!(my_vec::Buffer::create(&v_single).sum(), Some(1));
    }
    #[test]
    fn test_my_compare() {
        assert_eq!(my_conpare::compareString("abc", "abc"), false);
        assert_eq!(my_conpare::compareString("abc", "abd"), false);
        assert_eq!(my_conpare::compareString("abc", "abb"), true);
        assert_eq!(my_conpare::compareString("abc", "ab"), true);
        assert_eq!(my_conpare::compareString("abc", "abcd"), false);
    }
    #[test]
    fn test_my_itmap() {
        assert_eq!(my_itmap::increment(vec!['a', 'b', 'c']), vec!['b', 'c', 'd']);
        assert_eq!(my_itmap::increment(vec!['c', 'b', 'a']), vec!['d', 'c', 'b']);
        assert_eq!(my_itmap::increment(vec!['c']), vec!['d']);
    }
}
