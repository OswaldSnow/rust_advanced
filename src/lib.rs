#[derive(Debug)]
pub struct Foo;

impl Foo{
    pub fn mutate_and_share(&mut self) -> &mut Self {
        self
    }
    pub fn share(&mut self) {

    }

}
