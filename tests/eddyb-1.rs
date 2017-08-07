#[macro_use]
extern crate checked_builder;

bld! {
    {
        a: String,
        b: i32,
        c: [u8; 3]
    }
    fn print(self) {
        println!("{} {} {:?}", self.a, self.b, &self.c[..]);
    }
}

#[test]
fn main() {
    Builder::default().a("foo").b(5).c([1, 2, 3]).print();
}
