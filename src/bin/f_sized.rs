#[allow(unused)]
fn main() {
    /*
    不定长类型 DST
    和 Sized
     */

    // 为什么有 &str 类型 而没有 str 类型
    // error
    // let s0: str = "hello world";
    // rust 需要在编译期知道类型的大小
    // 那所有的 str 就需要有一样的大小

    // 而 &str 类型类似于指针 是有固定大小的
    let s1 = "hello world";

    // 既然 rust 要求在编译器获取所有类型的大小
    // 那么对于泛型参数 T 如何在编译期间获取大小呢
    fn generic<T>(t: T) {
        // todo
    }
    // 泛型函数只能用于一切实现了 Sized 特征的类型上
    // 而编译器会自动为泛型实现 Sized 特征
    // fn generic<T: Sized>(t: T){}
    // 每一个在编译期能获取大小的类型都会实现 Sized 特征

    // Box 智能指针
    let b_str = Box::new("hello world");
    // Box 存放了一个指针 指向了堆上的内存数据
    // 这种方式可以存放动态数据了 因为 Box 本身的大小是固定的

    // Box<str>
    // 源类型是 &'static str（字符串字面量）
    // 目标类型是 Box<str>（明确指定的）
    // 编译器找到了从 &str 到 Box<str> 的 Into 实现
    // 因此，.into() 调用成功地执行了这个转换
    let s1: Box<str> = "Hello there!".into();
    let s2 = &*s1;
    println!("s2 is {}", s2);
}
