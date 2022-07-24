


pub trait CastToNum {
    fn to_i32(&self) -> i32;
}
impl CastToNum for i32 {
    fn to_i32(&self) -> i32 {
        *self
    }    
}
impl CastToNum for u8 {
    fn to_i32(&self) -> i32 {
        *self as i32
    }
}
impl CastToNum for usize {
    fn to_i32(&self) -> i32 {
        *self as i32
    }
}
