/*
如果只使用简单的引用 &str
会出现所有权的问题
所以修改为 Option<&str>
 */
#[derive(Debug)]
struct WhatAboutThis<'a> {
    name: String,
    nickname: Option<&'a str>,
}

impl<'a> WhatAboutThis<'a> {
    fn tie_the_knot(&'a mut self) {
        self.nickname = Some(&self.name[..4]);
    }
}

fn main() {
    /*
    结构体的自引用
    “往往是实现特定的算法和数据结构时才需要，应用代码中几乎用不到。”
     */
    let mut tricky = WhatAboutThis {
        name: "Annabelle".to_string(),
        nickname: None,
    };

    tricky.tie_the_knot();

    // cannot borrow `tricky` as immutable because it is also borrowed as mutable
    // why?
    // println!("{:?}", tricky);
}
