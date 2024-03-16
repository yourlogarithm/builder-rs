
# Yet Another Rust Builder Pattern Derive Macro
### Usage
```rust
#[derive(Builder)]
struct Foo {
    pub a: i32,
    pub b: String,
    pub c: bool,
    pub d: Option<usize>,
}

let foo = Foo::builder()
    .a(-3)
    .b("Hello, world!".to_string())
    .c(true)
    .d(Some(2048))
    .build();

assert_eq!(-3, foo.a);
assert_eq!("Hello, world!".to_string(), foo.b);
assert_eq!(true, foo.c);
assert_eq!(Some(2048), foo.d);
```
### Note
The macro requires that fields implement the `Default`. When a value for the field is not provided it will assign the default value:
```rust
#[derive(Builder)]
struct DefaultFoo {
    pub a: i32,
    pub b: bool,
}

let foo = DefaultFoo::builder()
    .build();

assert_eq!(0, foo.a);
assert_eq!(false, foo.b);
``` 
Declaring fields that do not implement the `Default` trait will result in a compile error:
```rust
pub struct NonDefault {}

#[derive(Builder)]
pub struct Foo {
    d: NonDefault, // error[E0277]: the trait bound `NonDefault: Default` is not satisfied
}
```
