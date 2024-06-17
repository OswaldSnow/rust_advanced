#[derive(Debug)]
pub struct Foo;

impl Foo{
    pub fn mutate_and_share(&mut self) -> &Self {
        self
    }
    pub fn share(&self) {

    }

}
