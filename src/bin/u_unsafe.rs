fn main() {
    // 创建并使用裸指针
    // 不可变裸指针 *const
    let mut num = 5;
    let r1 = &num as *const i32;
    let _r2 = &mut num as *mut i32;
    // 可以同时存在 不可变指针 和 可变指针

    // 目前版本 1.79.0 以下代码已不再需要 unsafe
    // unsafe {
    println!("r1 is {:?}", r1);
    // }

    // 可变裸指针 *mut
    let mut address = "American";
    let p_address = &mut address as *mut &str;
    unsafe {
        *p_address = "new American";
    }
    println!("address is {:#?}", p_address);

    // 从智能指针创建裸指针
    let mut b1 = Box::new(123);
    let _mut_p_b1 = &mut *b1 as *mut i32;
    let _immut_p_b1 = &*b1 as *const i32;

    // unsafe 函数和方法
    unsafe { some_unsafe_fn() };

    // 与其他语言交互
}

unsafe fn some_unsafe_fn() {
    println!("imitate unsafe action")
}
