use std::sync::Arc;

fn main() {
    /*
    类型转换
     */

    // 一个复杂类型派生 Clone 需要其内部的所有类型都能派生 Clone

    // 阅读一遍 to be continued
}

#[allow(unused)]
#[derive(Clone)]
struct Container<T>(Arc<T>);

#[allow(unused)]
fn clone_containers<T>(foo: &Container<i32>, bar: &Container<T>) {
    let foo_cloned = foo.clone();
    // let bar_cloned = bar.clone();
}
