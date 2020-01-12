extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, Lit, Meta, NestedMeta, Type};

struct Param {
    ident: Ident,
    name: Option<String>,
}

struct Params {
    ident: Ident,
    ty: Type,
}

enum Element {
    Param(Param),
    Params(Params),
}

fn get_name(meta: &Meta) -> Option<String> {
    match meta {
        Meta::NameValue(name_value) => {
            if name_value.path.is_ident("name") {
                match &name_value.lit {
                    Lit::Str(lit_str) => Some(lit_str.value()),
                    _ => None,
                }
            } else {
                None
            }
        }
        Meta::List(meta_list) => meta_list
            .nested
            .iter()
            .filter_map(|nested_meta| match nested_meta {
                NestedMeta::Meta(meta) => get_name(meta),
                _ => None,
            })
            .next(),
        _ => None,
    }
}

#[proc_macro_derive(NumPluginParameters, attributes(param, params))]
pub fn num_parameters_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let struct_name = ast.ident;

    let elements: Vec<Element> = match ast.data {
        Data::Struct(data_struct) => data_struct
            .fields
            .into_iter()
            .filter_map(|field| {
                if let Some(attr) = field.attrs.iter().find(|attr| attr.path.is_ident("param")) {
                    let meta: Meta = attr.parse_meta().unwrap();
                    let name = get_name(&meta);

                    Some(Element::Param({
                        Param {
                            ident: field.ident.unwrap(),
                            name,
                        }
                    }))
                } else if field.attrs.iter().any(|attr| attr.path.is_ident("params")) {
                    Some(Element::Params(Params {
                        ident: field.ident.unwrap(),
                        ty: field.ty,
                    }))
                } else {
                    None
                }
            })
            .collect(),
        _ => unimplemented!(),
    };

    let num_parameters = {
        let mut n_param = 0;
        let mut params_expr = quote! {};

        for element in &elements {
            match element {
                Element::Param(_) => {
                    n_param += 1;
                }
                Element::Params(params) => {
                    let ty = &params.ty;
                    params_expr = quote! {
                        #params_expr + #ty::num_parameters()
                    };
                }
            }
        }

        quote! {
            #n_param #params_expr
        }
    };

    let gen = quote! {
       impl NumPluginParameters for #struct_name {
           fn num_parameters() -> i32 {
               #num_parameters
           }
       }
    };

    gen.into()
}

#[proc_macro_derive(PluginParameters, attributes(param, params))]
pub fn plugin_parameters_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let struct_name = ast.ident;

    let elements: Vec<Element> = match ast.data {
        Data::Struct(data_struct) => data_struct
            .fields
            .into_iter()
            .filter_map(|field| {
                if let Some(attr) = field.attrs.iter().find(|attr| attr.path.is_ident("param")) {
                    let meta: Meta = attr.parse_meta().unwrap();
                    let name = get_name(&meta);

                    Some(Element::Param({
                        Param {
                            ident: field.ident.unwrap(),
                            name,
                        }
                    }))
                } else if field.attrs.iter().any(|attr| attr.path.is_ident("params")) {
                    Some(Element::Params(Params {
                        ident: field.ident.unwrap(),
                        ty: field.ty,
                    }))
                } else {
                    None
                }
            })
            .collect(),
        _ => unimplemented!(),
    };

    // TODO Use macros to create impls.
    let get_parameters_name_impl = {
        let mut index = quote!{ 0 };
        let mut match_inner = quote! {};

        for element in &elements {
            match element {
                Element::Param(param) => {
                    let name = param.name.clone().unwrap_or_default();
                    match_inner = quote! {
                        #match_inner
                        x if x == (#index) => #name.to_string(),
                    };
                    index = quote! { #index + 1 };
                }
                Element::Params(params) => {
                    let ident = &params.ident;
                    let ty = &params.ty;
                    match_inner = quote! {
                        #match_inner
                        x if (#index .. #index + #ty::num_parameters()).contains(&x) => self.#ident.get_parameter_name(index - (#index)),
                    };
                    index = quote! { #index + #ty::num_parameters() };
                }
            }
        }

        quote! {
            match index {
                #match_inner
                _ => String::new(),
            }
        }
    };

    let get_parameter_impl = {
        let mut index = quote!{ 0 };
        let mut match_inner = quote! {};

        for element in &elements {
            match element {
                Element::Param(param) => {
                    let ident = &param.ident;
                    match_inner = quote! {
                        #match_inner
                        x if x == (#index) => self.#ident.get(),
                    };
                    index = quote! { #index + 1 };
                }
                Element::Params(params) => {
                    let ident = &params.ident;
                    let ty = &params.ty;
                    match_inner = quote! {
                        #match_inner
                        x if (#index .. #index + #ty::num_parameters()).contains(&x) => self.#ident.get_parameter(index - (#index)),
                    };
                    index = quote! { #index + #ty::num_parameters() };
                }
            }
        }

        quote! {
            match index {
                #match_inner
                _ => 0.0,
            }
        }
    };

    let set_parameter_impl = {
        let mut index = quote!{ 0 };
        let mut match_inner = quote! {};

        for element in &elements {
            match element {
                Element::Param(param) => {
                    let ident = &param.ident;
                    match_inner = quote! {
                        #match_inner
                        x if x == (#index) => self.#ident.set(value),
                    };
                    index = quote! { #index + 1 };
                }
                Element::Params(params) => {
                    let ident = &params.ident;
                    let ty = &params.ty;
                    match_inner = quote! {
                        #match_inner
                        x if (#index .. #index + #ty::num_parameters()).contains(&x) => self.#ident.set_parameter(index - (#index), value),
                    };
                    index = quote! { #index + #ty::num_parameters() };
                }
            }
        }

        quote! {
            match index {
                #match_inner
                _ => {}
            }
        }
    };

    let gen = quote! {
       impl PluginParameters for #struct_name {
           fn get_parameter_name(&self, index: i32) -> String {
               #get_parameters_name_impl
           }

           fn get_parameter(&self, index: i32) -> f32 {
               #get_parameter_impl
           }

           fn set_parameter(&self, index: i32, value: f32) {
               #set_parameter_impl
           }
       }
    };

    // eprintln!("{}", &gen);
    gen.into()
}