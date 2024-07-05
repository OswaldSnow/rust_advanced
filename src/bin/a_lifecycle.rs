use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    /*
    生命周期进阶
     */

    // Rust 编译器有三条生命周期省略规则，适用于大多数简单的引用场景：
    // 1、每个引用参数都有自己的生命周期参数。
    // 2、如果只有一个输入生命周期参数，该生命周期将被赋给所有输出生命周期参数。
    // 3、如果有多个输入生命周期参数并且其中一个是 &self 或 &mut self
    //      那么 self 的生命周期将被赋给所有输出生命周期参数。

    // 对于函数来说
    // 1、函数的输入参数中包含引用
    // 2、返回值中也包含与输入引用相关联的引用时
    // 需要显式地标注生命周期 以确保引用的生命周期关系被正确地描述和跟踪
    // Rust 编译器能够自动推断某些情况下的生命周期
    // 当输入引用和返回值之间不存在依赖关系时 显式的生命周期标识可以省略

    // 以下函数的生命周期参数 m 可以省略
    // 原因是满足了生命周期省略的第2条规则
    // 以下代码会编译报错
    // 编译器认为 对于参数 map 发生了两次可变借用
    // 解决思路是在函数体内不要发生两次可变借用
    #[allow(unused)]
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        // // 相关原始代码
        // // 这里一次 map 的可变借用
        // match map.get_mut(&key) {
        //     Some(value) => value,
        //     None => {
        //         map.insert(key.clone(), V::default());
        //         // 这里又发生了一次 map 的可变借用
        //         map.get_mut(&key).unwrap()
        //     }
        // }
        // // 综上 编译器认为 在同一个作用域内进行了多次可变借用


        // 解决方法 1
        // 直接使用 rust 提供的 entry 方法
        // map.entry(key.clone()).or_insert_with(V::default)

        // 解决方法 2
        // 使用分支
        // rust 清楚以下代码的 map.get_mut 不会同时发生
        if map.contains_key(&key) {
            map.get_mut(&key).unwrap()
        } else {
            map.insert(key.clone(), V::default());
            map.get_mut(&key).unwrap()
        }

    }


    // 生命周期约束
    // b >= a 写法
    #[allow(unused)]
    struct DoubleRef<'a,'b:'a, T> {
        r: &'a T,
        s: &'b T
    }


    // 符合上方生命周期消除第2条规则的普通函数
    #[allow(unused)]
    fn fn_elision(x: &i32) -> &i32 {
        x
    }

    // 但是闭包中这种写法会报错 加上 'static 可以解决
    let closure_slision = |x: &'static i32| -> &'static i32 { x };
    let x: &'static i32 = &19;
    closure_slision(&x);

    // 或者使用此种方式
    let _closure_slision = fun(|x: &i32| -> &i32 { x });
    fn fun<T, F: Fn(&T) -> &T>(f: F) -> F {
        f
    }


    // 引用的生命周期从借用处开始 一直持续到最后一次使用的地方
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // 新编译器中，r1,r2作用域在这里结束

    let r3 = &mut s;
    println!("{}", r3);


    // 生命周期消除规则的补充

    // impl 消除
    // impl<'a> Reader for BufReader<'a> {
    //     // methods go here
    //     // impl内部实际上没有用到'a
    // }
    // 可以写成以下形式
    // 但是 <'_> 依然需要保留
    // 因为 BufReader<'a> 是一个完整的类型
    // impl Reader for BufReader<'_> {
    //     // methods go here
    // }

    // 约束消除
    // Rust 2015
    // struct Ref<'a, T: 'a> {
    //     field: &'a T
    // }
    //
    // // Rust 2018
    // struct Ref<'a, T> {
    //     field: &'a T
    // }


    // 复杂的例子
    // 会提示错误
    let mut list = List {
        manager: Manager {
            text: "hello"
        }
    };

    // get_interface 方法中获取了list的可变引用
    // 参数 &'a mut self
    list.get_interface().noop();

    println!("Interface should be dropped here and the borrow released");

    // 下面的调用会失败，因为同时有不可变/可变借用
    // 但是Interface在之前调用完成后就应该被释放了
    // use_list(&list);
}

#[allow(unused)]
// 复杂的例子
struct Interface<'a> {
    manager: &'a mut Manager<'a>
}

impl<'a> Interface<'a> {
    pub fn noop(self) {
        println!("interface consumed");
    }
}

#[allow(unused)]
struct Manager<'a> {
    text: &'a str
}

struct List<'a> {
    manager: Manager<'a>,
}

impl<'a> List<'a> {
    pub fn get_interface(&'a mut self) -> Interface {
        Interface {
            manager: &mut self.manager
        }
    }
}

#[allow(unused)]
fn use_list(list: &List) {
    println!("{}", list.manager.text);
}