fn main() {
    /*
    闭包
     */

    let x = 1;
    let sum = |y| {
        x + y
    };
    assert_eq!(3,sum(2));

    // 闭包与所有权
    // 1、FnOnce 转移所有权
    // FnOnce 会将捕获到的变量的所有权转移到闭包
    // 所以 FnOnce 类型的闭包变量在使用一次后就不可以再使用了
    let mut x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()});
    x = vec![4,5,6];
    println!("{:?}",x);

    // 使用 move 强制闭包获取变量的所有权
    // 常用于 闭包的生命周期大于捕获变量的生命周期时
    // 例如将闭包转移进其他线程
    use::std::thread;
    let v = vec![1,2,3];
    let handle = thread::spawn(move || {
        println!("Here is vector: {0:?}",v);
    });
    handle.join().unwrap();

    // 2、FnMut 可变借用的方式捕获了环境中的值
    let mut s = String::new();
    // 同时 闭包也要标记为 mut QAQ
    let mut update_string = |str| s.push_str(str) ;
    update_string("hello");
    s += " world";
    println!("{}",s);

    // 只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征。
    // 上方的 FnOnce 处，如果 FnOnce 实现了 Copy 特征那么就会被 Copy

    // 一个闭包实现了哪种 Fn 特征取决于该闭包如何使用被捕获的变量
    // 而不是取决于闭包如何捕获它们。

    // 一个闭包并不仅仅实现某一种 Fn 特征，规则如下：
    // 所有的闭包都自动实现了 FnOnce 特征，因此任何一个闭包都至少可以被调用一次
    // 没有移出所捕获变量的所有权的闭包自动实现了 FnMut 特征
    // 不需要对捕获变量进行改变的闭包自动实现了 Fn 特征

    let s = String::new();
    let update_string =  || println!("{}",s);
    // 闭包 update_string 三种特征都符合
    exec(update_string);
    exec1(update_string);
    exec2(update_string);

    // 闭包作为函数返回值
    // 不可以直接使用 Fn(i32) -> i32 作为返回值
    // 但是使用 impl Fn(i32) -> i32 只能返回同样的类型
    // |x| x + 1 和 ｜x| x -1 是不同的类型
    // 可以使用 特征对象的方式 Box<dyn Fn(i32) -> i32>
    fn factory(x:i32) -> Box<dyn Fn(i32) -> i32> {
        let num = 5;

        if x > 1{
            Box::new(move |x| x + num)
        } else {
            Box::new(move |x| x - num)
        }
    }

    let fn1 = factory(8);
    println!("result = {}",fn1(8));

}

// FnOnce 只能运行一次的闭包
fn fn_once<F>(func: F)
where
    F: FnOnce(usize) -> bool,
{
    // 闭包的所有权被转移
    println!("{}", func(3));
    // func 是一个 FnOnce 的闭包
    // 在这里第二次调用 报错
    // println!("{}", func(4));
}


fn exec<F: FnOnce()>(f: F)  {
    f()
}

fn exec1<F: FnMut()>(mut f: F)  {
    f()
}

fn exec2<F: Fn()>(f: F)  {
    f()
}