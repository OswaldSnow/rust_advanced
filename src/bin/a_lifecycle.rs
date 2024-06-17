fn main() {
    /*
    生命周期进阶
     */

    use std::collections::HashMap;
    use std::hash::Hash;

    // 对于函数来说
    // 1、函数的输入参数中包含引用
    // 2、返回值中也包含与输入引用相关联的引用时
    // 需要显式地标注生命周期 以确保引用的生命周期关系被正确地描述和跟踪
    // Rust 编译器能够自动推断某些情况下的生命周期
    // 当输入引用和返回值之间不存在依赖关系时 显式的生命周期标识可以省略

    // Rust 编译器有三条生命周期省略规则，适用于大多数简单的引用场景：
    // 1、每个引用参数都有自己的生命周期参数。
    // 2、如果只有一个输入生命周期参数，该生命周期将被赋给所有输出生命周期参数。
    // 3、如果有多个输入生命周期参数并且其中一个是 &self 或 &mut self
    //      那么 self 的生命周期将被赋给所有输出生命周期参数。
    // 以下函数不需要标注生命周期 m 标注 原因是 满足了第2条规则
    fn get_default<'m, K, V>(map: &'m mut HashMap<K, V>, key: K) -> &'m mut V
    where
        K: Clone + Eq + Hash,
        V: Default,
    {
        match map.get_mut(&key) {
            Some(value) => value,
            None => {
                map.insert(key.clone(), V::default());
                map.get_mut(&key).unwrap()
            }
        }
    }

}