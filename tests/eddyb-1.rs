#[macro_use]
extern crate checked_builder;

builder! {
    struct Foo;

    required {
        a: String,
        b: i32,
        c: [u8; 3],
    }

    optional {
        d: String,
    }

    impl {
        fn format(self) -> String {
            format!("{} {} {:?} {:?}", self.a, self.b, &self.c[..], self.d)
        }
    }
}

#[test]
fn main() {
    assert_eq!(
        Foo::default().a("foo").b(5).c([1, 2, 3]).b(7).format(),
        "foo 7 [1, 2, 3] None"
    );
    assert_eq!(
        Foo::default()
            .a("bar")
            .b(42)
            .c([0, 0, 0])
            .d("quux")
            .format(),
        "bar 42 [0, 0, 0] Some(\"quux\")"
    );
}
