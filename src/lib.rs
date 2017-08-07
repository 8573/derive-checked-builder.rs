#[macro_export]
macro_rules! bld {
    ({ $($name:ident: $ty:ty),* } $($full_methods:tt)*) => {
        #[allow(non_camel_case_types)]
        struct Builder<$($name=()),*> {
            $($name: $name),*
        }

        impl Default for Builder {
            fn default() -> Builder {
                Builder { $($name: (),)* }
            }
        }

        bld!(@setters $(($name: $ty))*;);

        impl Builder<$($ty),*> {
            $($full_methods)*
        }
    };
    (@setters ;$($rest:tt)*) => {};
    (@setters ($name:ident: $ty:ty) $(($na:ident: $ta:ty))*;$(($nb:ident: $tb:ty))*) => {
        #[allow(non_camel_case_types)]
        impl<$($nb,)* $($na,)*> Builder<$($nb,)* (), $($na,)*> {
            fn $name<$name: Into<$ty>>(self, $name: $name) -> Builder<$($nb,)* $ty, $($na,)*> {
                Builder {
                    $($nb: self.$nb,)*
                    $name: $name.into(),
                    $($na: self.$na,)*
                }
            }
        }

        bld!(@setters $(($na: $ta))*; $(($nb: $tb))* ($name: $ty));
    }
}
