use std::cell::Cell;

fn main() {
    /*
    内部可变形
    拥有不可变引用的同时可以修改目标数据
    Cell
    RefCell
     */

    // Cell 和 RefCell 在功能上没有区别，区别在于 Cell<T> 适用于 T 实现 Copy 的情况

    let cell_1 = Cell::new("karima");
    let one = cell_1.get();
    cell_1.set("karima and oswald");
    let two = cell_1.get();

    println!("one === {}, two === {}", one, two);
}
