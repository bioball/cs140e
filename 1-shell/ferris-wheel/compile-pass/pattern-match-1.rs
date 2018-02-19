// FIXME: Make me compile! Diff budget: 2 lines.
#![feature(match_default_bindings)]

// Do not change this definition.
enum MyEnum {
    A(String),
    B(String)
}

fn matcher<'a>(val: &'a MyEnum) -> &str {
    match val {
        MyEnum::A(string) => string.as_str(),
        MyEnum::B(string) => string.as_str()
    }
}

fn main() { }
