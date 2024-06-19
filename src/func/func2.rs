// rust 闭包 实现原理/分类

// 闭包

pub fn closure() {
    // 未捕获环境变量 所有权 trait FnOnce
    let c1 = || println!("hello");
    c1();

    // 可修改环境变量 可变借用 trait Fnmut
    let mut arr = [1,2,3];
    let mut c2 = |i| {
        arr[i] += 1;
        println!("{:?}", arr);
    };
    c2(1);

    // 未修改环境变量 不可变借用 train Fn
    let ans = 42;
    let c3 = || {
        println!("{}", ans);
    };
    c3();
}

pub fn closure2() {
    fn c_mut()-> impl FnMut(i32) ->[i32;3] {

        let mut arr = [1,2,3];
        move |i| {
            arr[0] += i;
            arr
        }
    }

    let i = 42;
    let mut c = c_mut();
    println!("{:?}", c(i));
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_closure() {
        closure();
    }
    #[test]
    fn test_closure2() {
        closure2();
    }
}