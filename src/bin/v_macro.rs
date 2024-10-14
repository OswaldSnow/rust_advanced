use a_macro_derive::HelloMacro;
use std::fmt::Formatter;

// 声明式宏
macro_rules! greet {
    () => {};
    ($($name:expr),*) => {
        $(
            println!("Hello, {}!", $name);
        )*

    };
}

fn main() {
    let p1 = Person;
    greet! {p1};
    greet!["hello macro", "world"];
    greet!();

    // vec 宏
    let _v1 = vec![1, 2, 3];
    let _v2 = vec![1, 2, 3];

    println!("this is println macro");

    /*
    以上为声明式宏
    还存在另一种宏 过程宏
    1、derive 派生宏 类似 #[derive(Debug)] 等

    2、属性宏 类似以下
        #[route(GET, "/")]
        fn index() {
            // some code
        }

    3、函数宏 类似以下
    let sql = sql!(SELECT * FROM posts WHERE id=1);


    根据
    https://www.reddit.com/r/rust/comments/t1oa1e/what_are_the_complex_technical_reasons_why/
    过程宏放入独立包的原因在于它必须先被编译后才能使用，
    如果过程宏和使用它的代码在一个包，就必须先单独对过程宏的代码进行编译，
    然后再对我们的代码进行编译，但悲剧的是 Rust 的编译单元是包，因此你无法做到这一点。
     */

    Person::hello_macro();
}

pub trait Hello {
    fn hello_macro();
}

// 自定义派生宏
#[derive(HelloMacro)]
struct Person;

impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Person")
    }
}
