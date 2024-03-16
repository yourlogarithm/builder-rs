use builder_rs::Builder;

pub struct NonDefault {}

#[derive(Builder)]
pub struct Foo {
    d: NonDefault
}

fn main() {
    let _foo = Foo::builder()
        .build();
}