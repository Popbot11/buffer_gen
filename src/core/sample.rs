use std::fmt;

#[derive(Debug, Clone, Copy)]
pub struct Sample{
    pub i: usize,
    pub val: f32,
}
impl Sample {
    pub fn new(i: usize, val: f32) -> Self{
        Self {
            i: i,
            val: val
        }
    }
}
impl fmt::Display for Sample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i: {}, val: {}", self.i, self.val)
    }
}