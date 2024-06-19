use crate::func::func1::Color;
use crate::func::func1::A;
use crate::func::func1::sum;
mod func;

macro_rules! cal {
    ($e:expr) => {
        {
            let val: usize = $e;
            println!("{} = {}", stringify!{$e}, val);
        }
    };
}

fn main() {
    println!("Hello, world!");
    cal!(10+4*3);
    assert_eq!(sum(1,2), 3);
    let a = A(1,2);
    println!("{}", a.sum());
    println!("{}", a.sum2(3,4));
    println!("{:?}",std::mem::size_of_val(&Color::R));
}
