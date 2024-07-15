#[allow(unused)]
fn main() {
    /*
    枚举和整数
     */

    let e1 = MyEnum::B;
    // 枚举转换为整数
    let i1 = e1 as i32;
    println!("i1 is {i1}");
    // 整数转换为枚举
    // match i1 {
    //     MY_ENUM::A => {},
    //     MY_ENUM::B => {},
    //     MY_ENUM::C => {},
    //     _ => {}
    // }
    // 编译器会提示类型不匹配错误
    // 一个是 i32 类型 一个是 MY_ENUM 枚举类型 无法 match

    match i1.try_into().unwrap() {
        MyEnum::A => {
            println!("get A")
        }
        MyEnum::B => {
            println!("get B")
        }
        MyEnum::C => {
            println!("get C")
        }
        _ => {
            println!("get NONE")
        }
    }

    // std::mem::transmute
    // unsafe 暂时不看
}

// A = 1
// B C 会自动 +1
enum MyEnum {
    A = 1,
    B,
    C,
}

// i32 转换为 MY_ENUM
// 理解以下代码需要结合
// impl<T, U> TryInto<U> for T
// where
//     U: TryFrom<T>,
// {
//     type Error = U::Error;
//
//     #[inline]
//     fn try_into(self) -> Result<U, U::Error> {
//         U::try_from(self)
//     }
// }
impl TryFrom<i32> for MyEnum {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == MyEnum::A as i32 => Ok(MyEnum::A),
            x if x == MyEnum::B as i32 => Ok(MyEnum::B),
            x if x == MyEnum::C as i32 => Ok(MyEnum::C),
            _ => Err(()),
        }
    }
}
