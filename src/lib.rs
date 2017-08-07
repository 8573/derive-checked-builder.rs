pub struct Unset;

#[macro_export]
macro_rules! builder {
    (struct $($token:tt)*) => {
        builder!(@main [struct] $($token)*);
    };

    (pub struct $($token:tt)*) => {
        builder!(@main [pub struct] $($token)*);
    };

    (@main [$($struct_keyword:tt)*] $builder_type_name:ident;

     required {
         $($req_field_name:ident: $req_field_ty:ty,)*
     }

     optional {
         $($opt_field_name:ident: $opt_field_ty:ty,)*
     }

     impl {
         $($full_methods:tt)*
     }
    ) => {
        #[allow(non_camel_case_types)]
        $($struct_keyword)* $builder_type_name<$($req_field_name = $crate::Unset),*> {
            $($req_field_name: $req_field_name,)*
            $($opt_field_name: Option<$opt_field_ty>,)*
        }

        impl Default for $builder_type_name {
            fn default() -> Self {
                $builder_type_name {
                    $($req_field_name: $crate::Unset,)*
                    $($opt_field_name: None,)*
                }
            }
        }

        builder!(@setters req
            $builder_type_name:
            optional { $($opt_field_name)* }
            $(($req_field_name: $req_field_ty))*;
        );

        builder!(@setters opt
            $builder_type_name:
            required { $($req_field_name)* }
            $(($opt_field_name: $opt_field_ty))*;
        );

        impl $builder_type_name<$($req_field_ty),*> {
            $($full_methods)*
        }
    };

    (@setters req
     $builder_type_name:ident:
     optional { $($opt_field_name:ident)* }
     ; $($rest:tt)*
    ) => {};

    (@setters req
     $builder_type_name:ident:
     optional { $($opt_field_name:ident)* }
     ($name:ident: $ty:ty)
     $(($next_field_name:ident: $next_field_ty:ty))*
     ; $(($prev_field_name:ident: $prev_field_ty:ty))*
    ) => {
        #[allow(non_camel_case_types)]
        impl<$($prev_field_name,)* $name, $($next_field_name,)*>
            $builder_type_name<$($prev_field_name,)* $name, $($next_field_name,)*>
        {
            builder!(@setter req
                $builder_type_name:
                ($builder_type_name<$($prev_field_name,)* $ty, $($next_field_name,)*>)
                ($name: $ty)
                $($prev_field_name)* $($next_field_name)* $($opt_field_name)*
            );
        }

        builder!(@setters req
            $builder_type_name:
            optional { $($opt_field_name)* }
            $(($next_field_name: $next_field_ty))*
            ; $(($prev_field_name: $prev_field_ty))* ($name: $ty)
        );
    };

    (@setters opt
     $builder_type_name:ident:
     required { $($req_field_name:ident)* }
     ; $($rest:tt)*
    ) => {};

    (@setters opt
     $builder_type_name:ident:
     required { $($req_field_name:ident)* }
     ($name:ident: $ty:ty)
     $(($next_field_name:ident: $next_field_ty:ty))*
     ; $(($prev_field_name:ident: $prev_field_ty:ty))*
    ) => {
        #[allow(non_camel_case_types)]
        impl<$($req_field_name,)*>
            $builder_type_name<$($req_field_name,)*>
        {
            builder!(@setter opt
                $builder_type_name:
                ($name: $ty)
            );
        }

        builder!(@setters opt
            $builder_type_name:
            required { $($req_field_name)* }
            $(($next_field_name: $next_field_ty))*
            ; $(($prev_field_name: $prev_field_ty))* ($name: $ty)
        );
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: u8)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: u8) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: u16)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: u16) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: u32)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: u32) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: u64)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: u64) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: usize)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: usize) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: i8)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: i8) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: i16)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: i16) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: i32)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: i32) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: i64)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: i64) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: isize)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: isize) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: f32)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: f32) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: f64)
     $($other_field_name:ident)*
    ) => {
        fn $name(self, value: f64) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter req
     $builder_type_name:ident:
     ($ret_ty:ty)
     ($name:ident: $ty:ty)
     $($other_field_name:ident)*
    ) => {
        fn $name<T: Into<$ty>>(self, value: T) -> $ret_ty {
            $builder_type_name {
                $name: value.into(),
                $($other_field_name: self.$other_field_name,)*
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: u8)
    ) => {
        fn $name(self, value: u8) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: u16)
    ) => {
        fn $name(self, value: u16) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: u32)
    ) => {
        fn $name(self, value: u32) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: u64)
    ) => {
        fn $name(self, value: u64) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: usize)
    ) => {
        fn $name(self, value: usize) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: i8)
    ) => {
        fn $name(self, value: i8) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: i16)
    ) => {
        fn $name(self, value: i16) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: i32)
    ) => {
        fn $name(self, value: i32) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: i64)
    ) => {
        fn $name(self, value: i64) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: isize)
    ) => {
        fn $name(self, value: isize) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: f32)
    ) => {
        fn $name(self, value: f32) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: f64)
    ) => {
        fn $name(self, value: f64) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };

    (@setter opt
     $builder_type_name:ident:
     ($name:ident: $ty:ty)
    ) => {
        fn $name<T: Into<$ty>>(self, value: T) -> Self {
            $builder_type_name {
                $name: Some(value.into()),
                ..self
            }
        }
    };
}
