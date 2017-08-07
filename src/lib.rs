//! This library provides a procedural (custom `#[derive]`) macro for generating builder `struct`s.
//!
//! This macro can be used as follows:

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::Tokens;
use std::borrow::Cow;
use syn::*;

#[proc_macro_derive(CheckedBuilder)]
pub fn checked_builder(input: TokenStream) -> TokenStream {
    let src = input.to_string();
    let ast = syn::parse_derive_input(&src).expect("Failed to parse input.");
    let gen = checked_builder_impl(ast);
    gen.parse().expect("Failed to parse output.")
}

fn checked_builder_impl(mut ty: DeriveInput) -> Tokens {
    let fields = struct_fields(&ty.body)
        .iter()
        .cloned()
        .collect::<Vec<Field>>();

    ty.attrs.push(Attribute {
        style: AttrStyle::Outer,
        value: MetaItem::List(
            "allow".into(),
            vec![
                NestedMetaItem::MetaItem(MetaItem::Word("non_camel_case_types".into())),
            ],
        ),
        is_sugared_doc: false,
    });

    let our_attrs = ty.attrs
        .iter()
        .filter_map(|attr| match attr.value {
            MetaItem::List(ref id, ref contents) if id == "builder" => Some(contents),
            _ => None,
        })
        .flat_map(|contents| {
            contents.iter().filter_map(|attr| match attr {
                &NestedMetaItem::MetaItem(ref item) => Some(item),
                _ => None,
            })
        })
        .collect::<Vec<_>>();

    let builder_ty_name = {
        let name_attr = our_attrs
            .iter()
            .filter_map(|a| match a {
                &&MetaItem::List(ref id, ref contents) if id == "name" => {
                    Some(
                        contents
                            .iter()
                            .filter_map(|a| match a {
                                &NestedMetaItem::MetaItem(
                        MetaItem::NameValue(ref id, Lit::Str(ref content, _))
                    ) if ["prefix", "suffix"].contains(&id.as_ref()) => Some((id, content)),
                                _ => None,
                            })
                            .collect::<Vec<_>>(),
                    )
                }
                _ => None,
            })
            .flat_map(|x| x)
            .collect::<Vec<_>>();

        let prefix = name_attr
            .iter()
            .filter(|&&(name, _)| name == "prefix")
            .map(|&(_, value)| value.as_ref())
            .next()
            .unwrap_or("");

        let suffix = name_attr
            .iter()
            .filter(|&&(name, _)| name == "suffix")
            .map(|&(_, value)| value.as_ref())
            .next()
            .unwrap_or("Builder");

        Ident::new(format!("{}{}{}", prefix, ty.ident, suffix))
    };

    let mut setters_generics = ty.generics.clone();

    setters_generics.ty_params.extend(
        fields.iter().map(|&Field {
             ref ident, ..
         }| {
            ident.clone().unwrap().into()
        }),
    );

    let mut setters = Tokens::new();

    for (field_idx, &Field {
        ref ident,
        vis: _,
        ref attrs,
        ref ty,
    } )in fields.iter().enumerate()
    {
        let (ty_param, param_ty) = if is_prim_num_ty(ty) {
            (None, Ty::Path(None, ident.clone().unwrap().into()))
        } else {
            (Some(quote!(<T: Into<#ty>>)), Ty::Path(None, "T".into()))
        };

        let setter_generics = setters_generics.clone();

        let idx_in_ty_params = setter_generics.ty_params.len() - fields.len() + field_idx;

        setter_generics.ty_params[idx_in_ty_params] =

        let ctor_fields = Tokens::new();

        for &Field { ident: ref ident_2, .. } in fields.iter() {
            ctor_fields.append(if ident_2 == ident {
                quote!(#ident_2: value,)
            } else {
                quote!(#ident_2: self.#ident_2,)
            });
        }

        setters.append(quote!(
            fn #ident #ty_param (self, value: #param_ty) -> #builder_ty_name #setter_generics {
                #builder_ty_name {
                    #ctor_fields
                }
            }
        ))
    }

    ty.ident.clone_from(&builder_ty_name);

    ty.generics.ty_params.extend(fields.iter().map(|&Field {
         ref ident, ..
     }| {
        TyParam {
            attrs: vec![],
            ident: ident.clone().unwrap(),
            bounds: vec![],
            default: Some(Ty::Tup(vec![])),
        }
    }));

    for &mut Field {
        ref ident,
        ref mut ty,
        ..
    } in struct_fields_mut(&mut ty.body)
    {
        *ty = Ty::Path(None, ident.clone().unwrap().into())
    }

    println!(
        "{:#?}",
        quote! {
            #ty

            impl #setters_generics #builder_ty_name #setters_generics {
                #setters
            }
        }
    );

    panic!("boo")
}

fn struct_fields(body: &Body) -> &Vec<Field> {
    match body {
        &Body::Struct(VariantData::Struct(ref fs)) => fs,
        &Body::Enum(_) |
        &Body::Struct(VariantData::Tuple(_)) |
        &Body::Struct(VariantData::Unit) => {
            panic!("`derive-checked-builder` only supports `struct`s with named fields.")
        }
    }
}

fn struct_fields_mut(body: &mut Body) -> &mut Vec<Field> {
    match body {
        &mut Body::Struct(VariantData::Struct(ref mut fs)) => fs,
        &mut Body::Enum(_) |
        &mut Body::Struct(VariantData::Tuple(_)) |
        &mut Body::Struct(VariantData::Unit) => {
            panic!("`derive-checked-builder` only supports `struct`s with named fields.")
        }
    }
}

fn is_prim_num_ty(_ty: &Ty) -> bool {
    // TODO
    false
}
