use builder_rs::Builder;

pub struct NonDefault {
    pub a: i32,
    pub b: String,
    pub c: Option<bool>,
}

#[derive(Builder)]
pub struct Foo {
    pub a: i32,
    pub b: String,
    pub c: Option<bool>,
}

fn main() {
    let foo = Foo::builder()
        .a(42)
        .build();
    assert_eq!(42, foo.a);
    assert_eq!("".to_string(), foo.b);
    assert_eq!(None, foo.c);
}