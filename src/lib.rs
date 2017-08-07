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
         $($req_field_name:ident: $req_field_ty:ty),*
     }

     optional {
         $($opt_field_name:ident: $opt_field_ty:ty),*
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

        builder!(@setters
            $builder_type_name:
            optional { $($opt_field_name)* }
            $(($req_field_name: $req_field_ty))*;
        );

        #[allow(non_camel_case_types)]
        impl<$($req_field_name),*> $builder_type_name<$($req_field_name,)*> {
            $(fn $opt_field_name<T: Into<$opt_field_ty>>(self, $opt_field_name: T) -> Self {
                $builder_type_name { $opt_field_name: Some($opt_field_name.into()), ..self }
            })*
        }

        impl $builder_type_name<$($req_field_ty),*> {
            $($full_methods)*
        }
    };

    (@setters
     $builder_type_name:ident:
     optional { $($opt_field_name:ident)* }
     ; $($rest:tt)*
    ) => {};

    (@setters
     $builder_type_name:ident:
     optional { $($opt_field_name:ident)* }
     ($name:ident: $ty:ty)
     $(($next_field_name:ident: $next_field_ty:ty))*
     ; $(($prev_field_name:ident: $prev_field_ty:ty))*
    ) => {
        #[allow(non_camel_case_types)]
        impl<T, $($prev_field_name,)* $($next_field_name,)*>
            $builder_type_name<$($prev_field_name,)* T, $($next_field_name,)*>
        {
            fn $name<U: Into<$ty>>(
                self, $name: U
            ) -> $builder_type_name<$($prev_field_name,)* $ty, $($next_field_name,)*> {
                $builder_type_name {
                    $($prev_field_name: self.$prev_field_name,)*
                    $name: $name.into(),
                    $($next_field_name: self.$next_field_name,)*
                    $($opt_field_name: self.$opt_field_name,)*
                }
            }
        }

        builder!(@setters
            $builder_type_name:
            optional { $($opt_field_name)* }
            $(($next_field_name: $next_field_ty))*
            ; $(($prev_field_name: $prev_field_ty))* ($name: $ty)
        );
    };
}
