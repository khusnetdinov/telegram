#[derive(Debug)]
pub struct StructVisitor {
    ident: proc_macro2::Ident,
    structs: Vec<String>,
}

impl StructVisitor {
    fn angle_bracketed_visitor(angle_bracketed: &syn::PathArguments) -> Vec<String> {
        let mut types: Vec<String> = Vec::new();

        if let syn::PathArguments::AngleBracketed(ref angle_bracketed) = angle_bracketed {
            if let syn::GenericArgument::Type(syn::Type::Path(ref ty_path)) =
                angle_bracketed.args[0]
            {
                types.push(ty_path.path.segments[0].ident.to_string());
                types.append(&mut Self::angle_bracketed_visitor(
                    &ty_path.path.segments[0].arguments,
                ));
            }
        }

        types
    }

    fn visitor(field: &syn::Field) -> Vec<String> {
        let mut types: Vec<String> = Vec::new();

        if let syn::Type::Path(ref ty_path) = field.ty {
            types.push(ty_path.path.segments[0].ident.to_string());
            types.append(&mut Self::angle_bracketed_visitor(
                &ty_path.path.segments[0].arguments,
            ));
        }

        types
    }

    pub fn new(field: &syn::Field) -> Self {
        Self {
            ident: field.clone().ident.unwrap(),
            structs: Self::visitor(field),
        }
    }

    fn is_build_in(&self) -> bool {
        let build_in: [&str; 11] = [
            "String", "&str", "i64", "i32", "i16", "i8", "u64", "u32", "u16", "u8", "bool",
        ];

        build_in.contains(&self.structs.last().unwrap().as_str())
    }

    fn is_boxed(&self) -> bool {
        self.structs.contains(&String::from("Box"))
    }

    pub fn quoted(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;

        if self.is_build_in() {
            quote::quote! { #ident: value.#ident }
        } else if self.structs.len() == 2 && self.is_boxed() {
            quote::quote! { #ident: Box::new((*value.#ident).into()) }
        } else if self.structs.len() == 3
            && self.structs.iter().filter(|ty| ty == &"Vec").count() == 2
        {
            quote::quote! { #ident: value.#ident.iter().map(|coll| coll.iter().map(|inner| inner.to_owned().into()).collect()).collect() }
        } else {
            let quoted = self
                .structs
                .iter()
                .rev()
                .fold(quote::quote! {}, |quoted, ty| match self.structs.len() {
                    1 => quote::quote! { into() },
                    2 => match ty.as_str() {
                        "Option" => quote::quote! { map(|inner| #quoted) },
                        "Vec" => quote::quote! { iter().map(|inner| #quoted ).collect() },
                        "Box" => unreachable!(),
                        _ => {
                            quote::quote! { inner.to_owned().into() }
                        }
                    },
                    _ => match ty.as_str() {
                        "Option" => {
                            if self.is_boxed() {
                                quote::quote! { map(|inner| #quoted) }
                            } else {
                                quote::quote! { map(|option| option.#quoted) }
                            }
                        }
                        "Vec" => quote::quote! { iter().map(|inner| #quoted ).collect() },
                        "Box" => quote::quote! { Box::new((*inner).into()) },
                        _ => {
                            if ident == ty {
                                quote::quote! {}
                            } else {
                                quote::quote! { inner.to_owned().into() }
                            }
                        }
                    },
                });

            quote::quote! {
                #ident: value.#ident.#quoted
            }
        }
    }
}
