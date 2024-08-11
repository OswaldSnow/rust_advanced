use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    /*
    引用计数
    Rc
    Arc
     */

    // rust 默认存在所有权 但是特殊情况下 多个变量可以拥有同一数据

    // Rc<T>

    // 一般情况下
    let s1 = String::from("hello Rc&Arc");
    let _b1 = Box::new(s1);
    // s1 所有权被转移 s1不再有效 编译器提示错误
    // println!("{s1}");

    // 使用 Rc<T>
    // 创建一个 Rc智能指针 指向数据 "hello Rc"
    let rc1 = Rc::new(String::from("hello Rc"));

    // 使用 Rc::clone clone 一个 Rc
    // 此种 clone 方式只 clone 了 Rc 而对于指向的数据 并没有发生 clone
    // 也就是说目前 rc1 和 rc2 虽然是两个变量 但是都指向了同一块内存数据
    let rc2 = Rc::clone(&rc1);

    // 再次 clone
    let rc3 = Rc::clone(&rc2);

    {
        let rc4 = Rc::clone(&rc3);
        // 获取计数 也就是当前内存数据被几个 Rc 所引用了
        // 4
        println!("ref count {}", Rc::strong_count(&rc4));
    }

    // 再次获取计数 rc4 已经被释放
    // 但是 Rc 的引用计数不为 0 那么 Rc 中的数据就不会被释放
    // 3
    println!("ref count {}", Rc::strong_count(&rc3));

    // Rc<T> 是智能指针 实现了 Deref 特征 可以自行进行隐式转换
    let rc1_val: &str = &rc1;
    println!("rec val is {}", rc1_val);

    // 示例
    // 不同主人
    // 使用 引用 的方式会比现在的实现更复杂
    let master_a_rc = Rc::new(Master {
        name: "Karima".to_string(),
    });

    let gadget_a = Slave {
        id: 1,
        name: "snow".to_string(),
        master: Rc::clone(&master_a_rc),
    };

    let _gadget_b = Slave {
        id: 1,
        name: "oswald".to_string(),
        master: Rc::clone(&master_a_rc),
    };

    let _a_master = gadget_a.get_master();

    // Rc<T> 获取的是不可变引用 且 不能应用在多线程场景

    // 多线程安全的 Rc =》 Arc =》 Atomic Rc（原子化RC）
    let s_thread = Arc::new(String::from("支持多线程的 Rc Arc"));
    for _ in 0..10 {
        let s_clone = Arc::clone(&s_thread);
        let _handle = thread::spawn(move || {
            // println!("s === {s_clone}");
            let _s_clone = s_clone;
        });

        println!("s_thread ref count === {}", Arc::strong_count(&s_thread))
    }

    // 以上两者对数据都是只读的
}

#[allow(unused)]
struct Master {
    name: String,
}

#[allow(unused)]
struct Slave {
    id: u32,
    name: String,
    master: Rc<Master>,
}

impl Slave {
    fn get_master(&self) -> Rc<Master> {
        Rc::clone(&self.master)
    }
}
