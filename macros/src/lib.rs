use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, AngleBracketedGenericArguments, AttributeArgs, Field, Fields, FieldsNamed,
    FieldsUnnamed, GenericArgument, Ident, ItemFn, ItemImpl, ItemStruct, Meta, MetaNameValue,
    NestedMeta, Path, PathArguments, PathSegment, Type, TypeArray, TypePath,
};

#[proc_macro_derive(Deref, attributes(default_field))]
pub fn deref(input: TokenStream) -> TokenStream {
    let ItemStruct {
        fields,
        ident,
        generics,
        ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, _) = generics.split_for_impl();

    if let Fields::Unnamed(FieldsUnnamed { unnamed, .. }) = fields {
        if unnamed.len() > 1 {
            panic!("unnamed fields must be 1");
        }
        let Field { ty, .. } = unnamed[0].clone();
        quote! {
            impl #impl_generics std::ops::Deref for #ident #ty_generics {
                type Target = #ty;

                fn deref(&self) -> &Self::Target {
                    &self.0
                }
            }

            impl #impl_generics std::ops::DerefMut for #ident #ty_generics {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    &mut self.0
                }
            }
        }
        .into()
    } else {
        panic!()
    }
}

#[proc_macro_derive(Iterator, attributes(default_field))]
pub fn iterator(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input);

    let ItemStruct {
        fields,
        ident,
        generics,
        ..
    } = &input;

    let (impl_generics, ty_generics, _) = generics.split_for_impl();
    let Fields::Named(FieldsNamed { named, .. }) = fields else {
        panic!()
    };

    let field_names = named
        .into_iter()
        .map(|field| field.ident.as_ref().unwrap())
        .map(|field| quote! {self.#field});

    let field_type = &named.into_iter().next().unwrap().ty;

    let ref_fields = field_names.clone().map(|t| quote! {&#t});
    let mut_ref_fields = field_names.clone().map(|t| quote! {&mut #t});
    let len = field_names.clone().count();

    quote! {
        impl #impl_generics IntoIterator for #ident #ty_generics {
            type Item = #field_type;
            type IntoIter = std::array::IntoIter<Self::Item, #len>;

            fn into_iter(self) -> Self::IntoIter {
                [#(#field_names),*].into_iter()
            }
        }
        impl #impl_generics #ident #ty_generics {
            pub fn iter(&self) -> impl Iterator<Item = &#field_type> {
                    [#(#ref_fields),*].into_iter()
            }
        }
        impl #impl_generics #ident #ty_generics {
            pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut #field_type> {
                    [#(#mut_ref_fields),*].into_iter()
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn extended_structure(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item);
    
    let ItemStruct {
        fields,
        ident,
        generics,
        ..
    } = &input;

    let idents: AttributeArgs = parse_macro_input!(attr);

    let (idents, named) = match fields {
        Fields::Named(FieldsNamed { named, .. }) => (
            named
                .into_iter()
                .map(|field| field.ident.as_ref().unwrap())
                .collect::<Vec<&Ident>>(),
            named,
        ),
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => (
            idents
                .iter()
                .map(|meta| {
                    let NestedMeta::Meta(Meta::Path(Path { segments, .. })) = meta else {
                        panic!()
                    };
                    &segments[0].ident
                })
                .into_iter()
                .collect(),
            unnamed,
        ),
        _ => panic!(),
    };
    let field_type = &named.into_iter().next().unwrap().ty;

    let structure = match fields {
        Fields::Unnamed(_) => {
            let body = idents
                .into_iter()
                .map(|curr| {
                    quote! {
                        pub #curr: #field_type,
                    }
                })
                .fold(quote! {}, |acc, curr| {
                    quote! {
                        #acc
                        #curr
                    }
                });

            quote! {
                struct #ident #generics {
                    #body
                }
            }
        }
        _ => quote! {#input},
    };

    quote! {
        use macros::Iterator;
        #[derive(Iterator)]
        #structure
    }
    .into()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Col {
    Base,
    Vec,
    Arr,
}

impl Col {
    pub fn quote(&self, t: &Type, type_: proc_macro2::TokenStream) -> proc_macro2::TokenStream {
        let clone_type = self.clone();

        match clone_type {
            Col::Base => quote! {#type_},
            Col::Arr => match t.clone() {
                Type::Array(TypeArray { len, .. }) => {
                    // let type_ = Ref::pure_type(t);
                    quote! {[#type_; #len]}
                }
                _ => {
                    panic!("Col quote")
                }
            },
            Col::Vec => quote! {Vec<#type_>},
        }
    }

    pub fn pure_type(type_: &Type) -> Type {
        match type_ {
            Type::Array(TypeArray { elem, .. }) => *elem.clone(),
            Type::Path(TypePath { path, .. }) => {
                let PathSegment {
                    ident, arguments, ..
                } = get_segment(&path);

                if ident.to_string() == "Vec" {
                    return tmp(arguments);
                }

                type_.clone()
            }
            _ => {
                panic!("Not Allow Type for arg")
            }
        }
    }
    pub fn from_type(type_: &Type) -> Self {
        match type_ {
            Type::Array(_) => Col::Arr,
            Type::Path(TypePath { path, .. }) => {
                let PathSegment { ident, .. } = get_segment(&path);

                if ident.to_string() == "Vec" {
                    return Col::Vec;
                }

                Col::Base
            }
            _ => {
                panic!("Not Allow Type for arg")
            }
        }
    }
    pub fn get_cols(&self, mode: &Mode) -> Vec<Col> {
        match mode {
            Mode::Base => match self {
                Col::Base => {
                    vec![Col::Base]
                }
                Col::Arr => {
                    vec![Col::Arr, Col::Vec]
                }
                Col::Vec => {
                    vec![Col::Vec]
                }
            },
            Mode::Col => vec![Col::Vec],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Mode {
    Base,
    Col,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Ref {
    Base,
    Rc,
    Ref,
    RefRc,
}

impl Ref {
    fn pure_type(type_: &Type) -> Type {
        match Ref::from_type(type_) {
            Ref::Rc => get_arg(&get_path(type_)),
            Ref::Base => type_.clone(),
            _ => panic!("Not Allow Type for pure_type"),
        }
    }
    pub fn quote(&self, type_: &Type) -> proc_macro2::TokenStream {
        let ref_type = self.clone();
        let type_ = Ref::pure_type(&Col::pure_type(type_));
        match ref_type {
            Ref::Base => {
                quote! {#type_}
            }
            Ref::Rc => {
                quote! {Rc<#type_>}
            }
            Ref::Ref => {
                quote! {&#type_}
            }
            Ref::RefRc => {
                quote! {&Rc<#type_>}
            }
        }
    }
    pub fn from_type(type_: &Type) -> Self {
        let PathSegment { ident, .. } = get_segment_from_type(&Col::pure_type(type_));

        if ident.to_string() == "Rc" {
            return Ref::Rc;
        }

        Ref::Base
    }
}

const TYPES: [Ref; 4] = [Ref::Base, Ref::Rc, Ref::Ref, Ref::RefRc];
// const TYPES: [Ref; 2] = [Ref::Base, Ref::Rc];

fn get_segment(path: &Path) -> PathSegment {
    let Path { segments, .. } = path;
    segments.into_iter().next().unwrap().clone()
}

fn get_path(elem: &Type) -> Path {
    let Type::Path(TypePath { path, .. }) = elem else {
        panic!()
    };
    path.clone()
}

fn get_segment_from_type(type_: &Type) -> PathSegment {
    get_segment(&get_path(type_))
}

fn tmp(arguments: PathArguments) -> Type {
    let PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. }) = arguments
    else {
        panic!()
    };
    let GenericArgument::Type(arg) = args.into_iter().next().unwrap() else {
        panic!()
    };
    arg
}

fn get_arg(path: &Path) -> Type {
    tmp(get_segment(&path).arguments)
}

#[proc_macro_attribute]
pub fn of_to(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemImpl = parse_macro_input!(item);
    let impls = {
        let ItemImpl {
            trait_,
            self_ty: to_type,
            generics,
            ..
        } = input.clone();

        let to_type = *to_type.clone();
        let of_type = {
            let Some((_, path, _)) = trait_ else { panic!() };
            get_arg(&path)
        };

        let [curr_of_ref, curr_to_ref] = [&of_type, &to_type].map(|type_| Ref::from_type(type_));
        let [mut curr_of_col, mut curr_to_col] =
            [&of_type, &to_type].map(|type_| Col::from_type(type_));

        let mut mode = Mode::Base;

        match (curr_of_col, curr_to_col) {
            (Col::Base, _) | (_, Col::Base) => {}
            // _ => mode = Mode::Col,
            _ => panic!("curr_of_col или curr_to_col должен быть Col::Base"),
        }
        // dbg!(&curr_of_col, &curr_to_col, &mode);
        if let Mode::Col = mode {
            [curr_of_col, curr_to_col] = [Col::Vec; 2];
        }

        let of_body = |of_ref, of_col| {
            let quote = match of_ref {
                Ref::Base => match curr_of_ref {
                    Ref::Base => quote! {arg},
                    Ref::Rc => quote! {Rc::new(arg)},
                    Ref::Ref => quote! {&arg},
                    Ref::RefRc => quote! {&Rc::new(arg)},
                },
                Ref::Rc => match curr_of_ref {
                    Ref::Base => quote! {((*arg).clone())},
                    Ref::Rc => quote! {arg},
                    Ref::Ref => quote! {&arg.clone()},
                    Ref::RefRc => quote! {&arg},
                },
                Ref::Ref => match curr_of_ref {
                    Ref::Base => quote! {arg.clone()},
                    Ref::Rc => quote! {Rc::new(arg.clone())},
                    Ref::Ref => quote! {arg},
                    Ref::RefRc => quote! {&Rc::new(arg.clone())},
                },
                Ref::RefRc => match curr_of_ref {
                    Ref::Base => quote! {((*arg).clone())},
                    Ref::Rc => quote! {arg.clone()},
                    Ref::Ref => quote! {&(*arg.clone())},
                    Ref::RefRc => quote! {arg},
                },
            };

            match curr_of_col {
                Col::Arr => match of_col {
                    Col::Vec => {
                        let y = Col::Arr.quote(&of_type, of_ref.quote(&of_type));
                        quote! {
                            let arg: #y = arg.try_into().unwrap();
                            arg
                        }
                    }
                    Col::Arr => {
                        quote! {
                            arg.map(|arg| #quote)
                        }
                    }
                    _ => panic!(),
                },
                Col::Base => quote! {#quote},
                _ => {
                    // panic!("to_body")
                    quote! {}
                }
            }
        };

        let to_body = |of_trans, to_ref, of_col, to_col| {
            let quote = match curr_to_col {
                Col::Arr => match curr_to_ref {
                    Ref::Base => match to_ref {
                        Ref::Base => quote! {#of_trans.to()},
                        Ref::Rc => quote! {#of_trans.to::<#to_type>().map(|v| Rc::new(v))},
                        Ref::Ref => quote! {#of_trans.map(|v| &v)},
                        Ref::RefRc => quote! {#of_trans.map(|v| &Rc::new(v))},
                    },
                    Ref::Rc => match curr_of_ref {
                        Ref::Base => {
                            quote! {#of_trans.to::<#to_type>().map(|v| (*v).clone())}
                        }
                        Ref::Rc => quote! {#of_trans.to()},
                        Ref::Ref => quote! {&#of_trans.clone()},
                        Ref::RefRc => quote! {&#of_trans},
                    },
                    _ => quote! {},
                },
                Col::Base => match curr_to_ref {
                    Ref::Base => match to_ref {
                        Ref::Base => quote! {#of_trans.to()},
                        Ref::Rc => quote! {Rc::new(#of_trans.to::<#to_type>())},
                        Ref::Ref => quote! {&#of_trans},
                        Ref::RefRc => quote! {&Rc::new(#of_trans)},
                    },
                    Ref::Rc => match to_ref {
                        Ref::Base => {
                            quote! {*#of_trans.to::<#to_type>().clone()}
                        }
                        Ref::Rc => quote! {#of_trans.to()},
                        Ref::Ref => quote! {&#of_trans.clone()},
                        Ref::RefRc => quote! {&#of_trans},
                    },
                    _ => quote! {},
                },
                _ => {
                    // panic!("of_body")
                    quote!()
                }
            };
            match of_col {
                Col::Vec => {
                    quote! {
                        #of_trans.to()
                    }
                }
                _ => match to_col {
                    Col::Vec => {
                        let y = Col::Arr.quote(&to_type, to_ref.quote(&to_type));
                        quote! {arg.to::<#y>().to_vec()}
                    }
                    _ => quote! {#quote},
                },
            }
        };

        TYPES
            .into_iter()
            .map(|of_ref| {
                curr_of_col
                    .get_cols(&mode)
                    .into_iter()
                    .map(|of_col| {
                        TYPES
                            .iter()
                            .filter(|&to_ref| ![Ref::Ref, Ref::RefRc].contains(to_ref))
                            .map(|&to_ref| {
                                curr_to_col
                                    .get_cols(&mode)
                                    .into_iter()
                                    .filter(|&to_col| {
                                        (of_ref != curr_of_ref || to_ref != curr_to_ref)
                                            || (to_col != curr_to_col || of_col != curr_of_col)
                                    })
                                    .map(|to_col| [(of_col, of_ref), (to_col, to_ref)])
                                    .collect::<Vec<_>>()
                            })
                            .flatten()
                            .collect::<Vec<_>>()
                    })
                    .flatten()
                    .collect::<Vec<_>>()
            })
            .flatten()
            .map(|[(of_col, of_ref), (to_col, to_ref)]| {
                // dbg!([(of_col, of_ref), (to_col, to_ref)]);
                (
                    [(of_col, of_ref, &of_type), (to_col, to_ref, &to_type)]
                        .map(|(col, ref_, type_)| col.quote(type_, ref_.quote(type_))),
                    to_body(of_body(of_ref, of_col), to_ref, of_col, to_col),
                )
            })
            .map(|([of_type, to_type], body)| {
                // dbg!([of_type.to_string(), to_type.to_string()]);
                quote! {
                    impl #generics Of<#of_type> for #to_type {
                        fn of(arg: #of_type) -> Self {
                            #body
                        }
                    }
                }
            })
            .fold(quote! {}, |acc, r| {
                quote! {
                    #acc

                    #r
                }
            })
    };

    quote! {
        #input

        #impls
    }
    .into()
}

#[proc_macro_attribute]
pub fn setter(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input: ItemStruct = parse_macro_input!(item);
    let ItemStruct { fields, ident, .. } = input.clone();
    let Fields::Named(FieldsNamed { named: fields, .. }) = fields else {
        panic!("Only works on structs with named fields");
    };
    let methods = fields.into_iter().map(|Field { ident, ty, .. }| {
        let setter_ident = format_ident!("set_{}", ident.clone().unwrap());
        quote! {
            pub fn #setter_ident(mut self, value: #ty) -> Self {
                self.#ident = value;
                self
            }
        }
    });
    quote! {
        #input

        impl #ident {
            #(#methods)*
        }
    }
    .into()
}

#[proc_macro_derive(Default, attributes(default_field))]
pub fn default_macro_derive(input: TokenStream) -> TokenStream {
    let ItemStruct { ident, fields, .. } = parse_macro_input!(input);
    dbg!(&fields);
    let field_defaults = {
        let Fields::Named(FieldsNamed { named: fields, .. }) = fields else {
            panic!("Only works on structs with named fields");
        };

        fields.into_iter().map(|field| {
            let Field { ident, ty, .. } = field;

            let default_value = field
                .attrs
                .iter()
                .find_map(|attr| {
                    if attr.path.is_ident("default_field") {
                        match attr.parse_meta() {
                            Ok(Meta::NameValue(MetaNameValue { lit, .. })) => {
                                return Some(quote! { #lit });
                            }
                            _ => {}
                        }
                    }
                    None
                })
                .unwrap_or_else(|| {
                    let PathSegment {
                        ident, arguments, ..
                    } = get_segment_from_type(&ty);
                    if ident == "Option" {
                        let type_ = tmp(arguments);
                        return quote! { Some(<#type_ as std::default::Default>::default()) };
                    }
                    quote! { <#ty as std::default::Default>::default() }
                });

            quote! {
                #ident: #default_value
            }
        })
    };

    let expanded = quote! {
        impl std::default::Default for #ident {
            fn default() -> Self {
                Self {
                    #(#field_defaults),*
                }
            }
        }
    };

    TokenStream::from(expanded)
}
#[proc_macro_attribute]
pub fn svg_test(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attrs = parse_macro_input!(attr as syn::AttributeArgs);

    let mut resolution = ["1080".to_string(), "1080".to_string()];
    let mut view_box = [0, 0, 12, 12];

    for arg in attrs {
        if let NestedMeta::Meta(Meta::NameValue(nv)) = arg {
            if nv.path.is_ident("resolution") {
                if let syn::Lit::Str(lit) = nv.lit {
                    resolution = lit
                        .value()
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .collect::<Vec<String>>()
                        .try_into()
                        .unwrap();
                }
            } else if nv.path.is_ident("viewBox") {
                if let syn::Lit::Str(lit) = nv.lit {
                    view_box = lit
                        .value()
                        .split(',')
                        .map(|s| s.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                        .try_into()
                        .unwrap();
                }
            }
        }
    }

    let [width, height] = resolution;
    let [x1, y1, x2, y2] = view_box;

    let (name, block, attrs) = {
        let input = parse_macro_input!(item as ItemFn);
        (input.sig.ident, input.block, input.attrs)
    };

    let output = quote! {
        #[test]
        #(#attrs)*
        fn #name() {
            use svg::Document;
            use std::{env, path::{Path, PathBuf}};
            use crate::planet::shared::traits::Svg;
            use svg::node::element::Polygon;

            let mut document = Document::new()
                .set("viewBox", (#x1, #y1, #x2, #y2))
                .set("width", #width)
                .set("height", #height);

            #block.to_svg(&mut document);
            let path = {
                let module_path_str = module_path!().to_string().replace("::", "\\");
                let path: PathBuf = Path::new(&module_path_str).components().skip(1).collect();
                let path = env::current_dir().unwrap().join("src").join(&path).join("svg_tests");
                std::fs::create_dir_all(&path).unwrap();
                path.join(format!("{}", stringify!(#name)) + ".svg")
            };
            svg::save(path.to_str().unwrap(), &document).unwrap();
        }
    };

    output.into()
}
