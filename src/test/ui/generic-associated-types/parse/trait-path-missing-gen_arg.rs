#![feature(generic_associated_types)]
//~^ WARNING: the feature `generic_associated_types` is incomplete

trait X {
    type Y<'a>;
}

const _: () = {
  fn f1<'a>(arg : Box<dyn X< : 32 >>) {}
      //~^ ERROR: expected one of `>`, const, lifetime, or type, found `:`
      //~| ERROR: expected parameter name, found `>`
      //~| ERROR: expected one of `!`, `)`, `+`, `,`, or `::`, found `>`
      //~| ERROR: constant provided when a type was expected
};

const _: () = {
  fn f1<'a>(arg : Box<dyn X< = 32 >>) {}
      //~^ ERROR: expected one of `>`, const, lifetime, or type, found `=`
};

fn main() {}
