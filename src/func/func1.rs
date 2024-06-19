// 常规函数 自由函数 关联函数 函数项类型
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub struct A (pub i32,pub i32);

impl A {
    // 方法
    pub fn sum(&self) -> i32 {
        self.0 + self.1
    }
    // 关联函数
    pub(crate) fn sum2(&self, a: i32, b: i32) -> i32 {
        a + b
    }



}

pub enum Color {
    R(i16),
    G(i16),
    B(i16),

}
// 函数项类型是0
// 函数指针是8

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sum() {
        assert_eq!(sum(1,2), 3);
    }
    #[test]
    fn test_sum2() {
        let a = A(1,2);
        assert_eq!(a.sum(), 3);
    }
    #[test]
    fn test_sum3() {
        let a = A(1,2);
        assert_eq!(a.sum2(3,4), 7);
    }
    #[test]
    fn test_color() {
        assert_eq!(std::mem::size_of_val(&Color::R), 2);
    }
}