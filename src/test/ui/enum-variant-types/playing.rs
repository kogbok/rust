// run-pass

#[allow(dead_code)]
enum Foo {
    Variant1,
    Variant2,
}


pub fn main() {
    let _x = @Foo::Variant1;
}