use std::ops::{Deref, DerefMut};

fn main() {
    /*
    Deref 特征
     */

    // 可以对一个类型使用 *some
    // 实际上这个类型就实现了 Deref 特征
    // *some 就是调用了 *(some.deref())
    let x = Box::new(1);
    let _y = *x + 1;

    // 实现一个自己的 “智能指针”
    let mybox_1 = MyBox(2);
    // *(mybox_1.deref())
    let box_1 = *mybox_1;
    println!("box_1 {box_1}");

    // Deref 的隐式转换
    let s1 = String::from("hello deref");
    // 此处 s1 是一个对 String 的引用 而 display_str 的参数需要 &str
    // &String 为什么可以作为参数呢
    // 因为 String 实现了 Deref 特征 且 Target 定义为了 str
    // 编译器可以自动解引用为 &str
    // 且直接传入 s1 是不行的 ==> Deref(仅引用类型的实参才会触发自动解引用)
    display_str(&s1);

    // Deref 连续隐式转换
    let s2 = MyBox::new(String::from("hello rust"));
    display_str(&s2);
    let _s3 = &s2;
    let _s4 = s2.to_string();

    // 引用归一
    // 对于上面的 “智能指针” 类型 正确实现 Deref 的情况下会编译器会自动解引用 指向内部类型
    // 而对于 &&&&t 类型 最终会被归一成 &t
    // 由于标准库源码中存在以下代码
    // impl<T: ?Sized> Deref for &T {
    //     type Target = T;
    //
    //     fn deref(&self) -> &T {
    //         *self
    //     }
    // }
    // 就会会发生 &&&&t -> &&&t -> &&t -> &t

    // Deref 转换
    // T: Deref<Target=U> 可以将 &T 转换成 &U
    // T: DerefMut<Target=U> 可以将 &mut T 转换成 &mut U
    // T: Deref<Target=U> 可以将 &mut T 转换成 &U
}

fn display_str(s: &str) {
    println!("display_str {}", s);
}

// 定义元组结构体
struct MyBox<T>(T);

// 实现 Deref 以及 DerefMut
// 你只管实现 其他的交给编译器 QAQ
// 以下代码的基础上就能实现上方的 三种转换规则
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// 稍做修改就能实现 &mut T -> &U 的转变
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> MyBox<T> {
    fn new(t: T) -> Self {
        MyBox(t)
    }
}
