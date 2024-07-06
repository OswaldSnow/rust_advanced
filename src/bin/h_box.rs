fn main(){
    /*
    智能指针
     */

    // Box 智能指针
    // 将数据存放在堆上
    let n = Box::new(90);
    // Box 实现了 Deref trait 不需要显式取值
    println!("n val is {}",n);

    // 避免栈上的数据拷贝
    // 将动态大小类型变为固定大小类型
    // 特征对象

    // Box::leak

}
