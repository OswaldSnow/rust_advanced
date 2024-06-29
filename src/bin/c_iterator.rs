fn main() {
    /*
    迭代器

    首先有必要复习 迭代器 这种设计模式

     Iterator
     IntoIterator
     */

    let values = vec![1,2,3];
    // 此段代码一开始没看明白 QAQ
    {
        let result = match IntoIterator::into_iter(values) {
             mut iter => loop {
                match iter.next() {
                    Some(x) => { println!("{}", x); },
                    None => break,
                }
            },
        };
        result
    }

    // mut 迭代器获取可变引用
    let mut v1 = vec![4,5,6];
    let mut v1_iter = v1.iter_mut();
    let v1_0_val = v1_iter.next();
    *(v1_0_val.unwrap()) = 10;
    println!("v1 {:?}",v1);

    // zip 收集
    let names = ["sunface", "sunfei"];
    let ages = [18, 18];
    let folks: Vec<(&str,i32)> = names.into_iter().zip(ages.into_iter()).collect();

    println!("{:#?}",folks);

    // enumerate

}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter{
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }


}