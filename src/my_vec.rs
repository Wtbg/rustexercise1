#[allow(dead_code)]
pub struct Buffer<T: std::ops::Add<Output = T> + Clone> {
    pub data: Vec<T>,
}
impl<T: std::ops::Add<Output = T> + Clone> Buffer<T> {
    #[allow(dead_code)]
    pub fn create(v: &[T]) -> Self {
        Buffer {
            data: v.to_owned(),
        }
    }
    #[allow(dead_code)]
    pub fn sum(&self) -> Option<T> {
        let ret = self.data.iter().fold(None, |acc, x| {
            match acc {
                None => Some(x.clone()),
                Some(y) => Some(y + x.clone()),
            }
        });
        ret.clone()
    }
}
