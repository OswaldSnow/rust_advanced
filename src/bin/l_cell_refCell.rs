#[allow(unused)]
use std::cell::{Cell, RefCell};
use std::rc::Rc;

fn main() {
    /*
    内部可变性
    Cell
    RefCell

    实现编译期可变、不可变引用共存
    借用检查推迟到了运行期
    但是并不是取消了此限制
    在运行期间可能发生共存或者不符合rust借用检查的情况 一旦发生 -> panic
    Cell 针对于内部值实现了 Copy 的情况
    RefCell 可以针对未实现 Copy 的情况

    用于你确信代码是正确的，而编译器却发生了误判时
     */

    // Cell 内部可变性
    let mut b = 123;
    let c = b;
    b = 345;
    println!("c {}, b {}", c, b);

    // 不需要将原始的 Cell 标志为 可变的
    let cell_i32 = Cell::new(123);
    let cell_i32_1 = cell_i32.get();
    cell_i32.set(456);
    println!("cell_i32 {:?}, cell_i32_1 {:?}", cell_i32, cell_i32_1);

    // RefCell
    // 编译器不会报错 运行期会 panic
    // let s = RefCell::new(String::from("hello RefCell"));
    // let _s1 = s.borrow();
    // let _s2 = s.borrow_mut();

    // Cell 只适用于 Copy 类型，用于提供值，而 RefCell 用于提供引用
    // Cell 不会 panic，而 RefCell 会

    // Cell 没有性能损耗
    // 内部可变性
    let x = Cell::new(1);
    let y = &x;
    let z = &x;
    x.set(2);
    y.set(3);
    z.set(4);
    println!("{}", x.get());

    // 内部可变性
    // 使用 RefCell 包裹

    // Rc + RefCell 配合使用
    // 实现数据存在多个所有者并且都可以进行修改
    let ss = Rc::new(RefCell::new(String::from("hello karima baby")));
    let ss1 = ss.clone();
    let ss2 = ss.clone();

    ss1.borrow_mut().push_str(", I love you!!!");
    ss1.borrow_mut()
        .push_str(", then karima say: I love you too oswald baby!!!");

    println!("{:?}", ss);
    println!("{:?}", ss1);
    println!("{:?}", ss2);

    // 以下方式就会报错
    // let mut sss = String::from("sss value");
    // let sss1 = &sss;
    // let sss2 = &sss;
    //
    // sss = String::from("sss new value");
    //
    // println!("sss {:?}",sss);
    // println!("sss1 {:?}",sss1);
    // println!("sss2 {:?}",sss2);

    // Cell::from_mut
    // 将引用通过 Cell 包装起来

    // 使用 RefCell 要在确保代码没有问题的情况下进行
    // 否则程序还是会 panic
}

#[allow(unused)]
#[derive(Debug)]
struct MyDemo {
    name: String,
}
