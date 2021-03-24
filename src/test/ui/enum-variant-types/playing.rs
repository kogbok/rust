// run-pass

#[allow(dead_code)]
enum Foo {
    Variant1,
    Variant2,
}

impl Foo {
    fn which(&self) -> u32 {
      match self { Self::Variant1 => 1, Self::Variant2 => 2 }
    }
  }

pub fn main() {
    let x = @Foo::Variant1;
    assert_eq!((x as Foo).which(), 1);
}