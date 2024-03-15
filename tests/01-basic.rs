use builder_rs::Builder;

#[derive(Builder)]
pub struct Foo {
    pub a: i32,
    pub b: String,
    pub c: Option<bool>,
}

fn main() {
    let _foo = Foo::builder()
        .a(42)
        .b("hello".to_string())
        .c(Some(true))
        .build();
}