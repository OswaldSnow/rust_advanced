use std::fmt::{Display, Formatter};
fn main() {
    /*
    new type
     */

    // 当需要自行为 i32 类型实现 Display 特征时
    // 默认是做不到的
    // 因为 rust 中存在 孤儿原则
    // 要为 A类型 实现 B trait
    // A B 至少要有一个在当前作用域定义
    // 很明显 i32 和 Display 不符合这个条件

    // 但是可以使用 new type 的方式
    let meter_1 = Meter(9);
    println!("{}",meter_1);

    // 别名
    type MA = Meter;
    let ma_1: MA = Meter(100);
    println!("{ma_1}");

    type U32ALIAS = u32;
    let a: U32ALIAS = 19;
    println!("{a}");

    // 永不返回的函数类型 -> !
    // 永不返回的发散函数 diverging function

}

// new type 定义一个 元组结构体 包含一个 i32 类型
// 这样就可以为 Meter 实现 Display 了
struct Meter(i32);

impl Display for Meter{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f , "the meter is {}",self.0)
    }
}