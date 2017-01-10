#[macro_use]
extern crate hello_world_derive;

trait THelloWorld {
    fn hello();
}

#[derive(HelloWorld)]
struct FrenchToast;

#[derive(HelloWorld)]
struct Waffles;

fn main() {
    FrenchToast::hello();
    Waffles::hello();
}