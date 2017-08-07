#[macro_use]
extern crate derive_checked_builder;

#[derive(CheckedBuilder)]
struct Foo {
    a: String,

    b: i32,

    c: [u8; 3],

    #[builder(default = "\"def\"")]
    d: String,

    #[builder(default(8))]
    e: u16,
}

impl FooBuilder<String, i32, [u8; 3]> {
    fn format(self) -> String {
        format!(
            "{} {} {:?} {:?} {}",
            self.a,
            self.b,
            &self.c[..],
            self.d,
            self.e
        )
    }
}

#[test]
fn main() {
    assert_eq!(
        Foo::default().a("foo").b(5).c([1, 2, 3]).b(7).format(),
        "foo 7 [1, 2, 3] None 8"
    );

    assert_eq!(
        Foo::default()
            .a("bar")
            .b(42)
            .c([0, 0, 0])
            .d("quux")
            .format(),
        "bar 42 [0, 0, 0] Some(\"quux\") 8"
    );
}
