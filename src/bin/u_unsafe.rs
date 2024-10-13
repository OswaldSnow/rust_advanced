fn main() {
    let num = 5;

    let r1 = &num as *const i32;

    // 目前版本 1.79.0 以下代码已不再需要 unsafe
    // unsafe {
    println!("r1 is: {:?}", r1);
    // }
}
