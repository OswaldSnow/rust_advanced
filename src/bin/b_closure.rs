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
    let mut x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()});
    x = vec![4,5,6];
    println!("{:?}",x);

    // 强制闭包获取变量的所有权
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
    let mut update_string = |str| s.push_str(str) ;
    update_string("hello");
    s += " world";
    println!("{}",s);

    // 只要闭包捕获的类型都实现了Copy特征的话，这个闭包就会默认实现Copy特征。

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