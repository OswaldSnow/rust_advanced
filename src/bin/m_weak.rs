use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
#[allow(unused)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            List::Cons(_, item) => {
                // println!("current val is {}",val);
                Some(item)
            }
            List::Nil => None,
        }
    }
}

#[derive(Debug)]
#[allow(unused)]
enum ListByWeak {
    Cons(i32, RefCell<Weak<ListByWeak>>),
    Nil,
}

impl ListByWeak {
    fn tail(&self) -> Option<&RefCell<Weak<ListByWeak>>> {
        match self {
            ListByWeak::Cons(_, item) => Some(item),
            ListByWeak::Nil => None,
        }
    }

    fn current_value(&self) -> Option<i32> {
        match self {
            ListByWeak::Cons(val, _) => Some(*val),
            ListByWeak::Nil => None,
        }
    }
}

type LBW = ListByWeak;

fn main() {
    /*
    关于 Rc Weak 的循环引用
     */

    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a的初始化rc计数 = {}", Rc::strong_count(&a));
    println!("a指向的节点 = {:?}", a.tail());

    // 创建`b`到`a`的引用
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("在b创建后，a的rc计数 = {}", Rc::strong_count(&a));
    println!("b的初始化rc计数 = {}", Rc::strong_count(&b));
    println!("b指向的节点 = {:?}", b.tail());

    // 利用RefCell的可变性，创建了`a`到`b`的引用
    if let Some(link) = a.tail() {
        println!("link val is {:?}", link);
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("在更改a后，b的rc计数 = {}", Rc::strong_count(&b));
    println!("在更改a后，a的rc计数 = {}", Rc::strong_count(&a));

    // 下面一行println!将导致循环引用
    // 我们可怜的8MB大小的main线程栈空间将被它冲垮，最终造成栈溢出
    // println!("a next item = {:?}", a.tail());

    println!("=========");

    // 使用 Weak 解决上方的 循环引用问题
    // Weak 返回 Option，不对引用计数，同时也允许指向的数据不存在
    // Weak 使用示例
    let five = Rc::new(5);

    let weak_five = Rc::downgrade(&five);

    let weak_val = weak_five.upgrade();
    if let Some(val) = weak_val {
        println!("weak_val is {}", val)
    }

    drop(five);

    let weak_val = weak_five.upgrade();
    if let None = weak_val {
        println!("weak_val is Option::None")
    }

    println!("=========");

    // 使用 Weak 解决前面循环依赖的问题
    let lbw_nil = Rc::new(LBW::Nil);

    let lbw_one = Rc::new(LBW::Cons(10, RefCell::new(Rc::downgrade(&lbw_nil))));

    let lbw_two = Rc::new(LBW::Cons(20, RefCell::new(Rc::downgrade(&lbw_one))));

    println!(
        "one value is {:?},one Weak val is {:?}",
        lbw_one.current_value().unwrap(),
        lbw_one.tail().unwrap().borrow().upgrade().unwrap()
    );
    println!(
        "two value is {:?},two Weak val is {:?}",
        lbw_two.current_value().unwrap(),
        lbw_two.tail().unwrap().borrow().upgrade().unwrap()
    );

    if let Some(item) = lbw_one.tail() {
        *item.borrow_mut() = Rc::downgrade(&lbw_two);
    }

    println!("loop ref done");

    // 此时不会发生之前 Rc 导致的循环依赖问题
    println!(
        "one value is {:?},one Weak val is {:?}",
        lbw_one.current_value().unwrap(),
        lbw_one.tail().unwrap().borrow().upgrade().unwrap()
    );
    println!(
        "two value is {:?},two Weak val is {:?}",
        lbw_two.current_value().unwrap(),
        lbw_two.tail().unwrap().borrow().upgrade().unwrap()
    );

    // 由于 Rc 需要进行引用计数，当出现循环依赖时，引用计数永远不为0，就会导致 OOM
    // 而 Weak 不进行引用计数，且由于使用了 Option，允许引用的数据不存在
}
