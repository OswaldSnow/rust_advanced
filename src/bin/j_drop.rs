fn main() {
    /*
    Drop
     */

    // Drop 释放资源

    let mut _my1 = MyDropStruct;
    // drop 获取了对 self 的可变引用
    // 调用后 drop 后实际上还可以使用 这是不对的
    // 所以下点面的调用方式 编译器会报错
    // _my1.drop();
    
    // 使用此方式比较安全 drop 函数拿走了变量的所有权
    // 所以调用后变量不再可用
    
    drop(_my1)
    
    // 变量生命周期结束后会自动调用 Drop drop
    // 大部分情况下不需要手动为类型实现 Drop
    // 除非需要 添加额外的清理逻辑 或者 控制释放顺序
    // 还有 文件描述符、网络 socket等
    
    // 一个类型无法同时实现 Copy 和 Drop 
    // rust 默认的实现了 Copy 类型的内存回收会自动完成
    // 对于我们自定义的类型 同时实现 Copy 和 Drop 就会出现问题
    // 换句话说 自定义的类型实现了 Copy 然后也不需要手动实现 Drop 了 
    

}

#[derive(Debug)]
struct MyDropStruct;

impl Drop for MyDropStruct{
    fn drop(&mut self) {
        println!("{:?} is Drop!!!",self)
    }
}
