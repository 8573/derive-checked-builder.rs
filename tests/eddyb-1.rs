#[macro_use]
extern crate checked_builder;

bld! {
    Foo: required {
        a: String,
        b: i32,
        c: [u8; 3]
    } optional {
        d: String
    }
    fn print(self) {
        println!("{} {} {:?} {:?}", self.a, self.b, &self.c[..], self.d);
    }
}

#[test]
fn main() {
    Foo::default().a("foo").b(5).c([1, 2, 3]).print();
    Foo::default().a("bar").b(42).c([0, 0, 0]).d("quux").print();
}
