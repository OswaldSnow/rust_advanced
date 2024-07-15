use rust_advanced::Foo;

fn main() {
    /*
    rust 进阶
     */

    let mut f1 = Foo;

    let f2 = f1.mutate_and_share();

    f2.share();
}
