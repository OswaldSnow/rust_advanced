#[derive(Debug)]
pub struct Foo;

impl Foo {
    pub fn mutate_and_share(&mut self) -> &mut Self {
        self
    }
    pub fn share(&mut self) {}

    pub fn for_example_fn() {
        println!("This method is called for example");
    }
}
