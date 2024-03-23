use proc_macro::TokenStream;
use proc_macro2::token_stream::IntoIter;
use proc_macro2::Literal;
use proc_macro2::Punct;
use proc_macro2::Spacing;
use proc_macro2::TokenTree;

use quote::quote;
use quote::ToTokens;

use syn::Expr;
use syn::Ident;
use syn::ImplItem;
use syn::ImplItemFn;
use syn::ItemImpl;
use syn::ReturnType;
use syn::Type;

pub(crate) mod attributes;

#[derive(Default, Debug)]
struct Attributes {
    name: Option<Ident>,
    args: Vec<(String, Expr, Option<Expr>)>,
    returns: Option<Expr>,
}

trait VecExt<T> {
    fn split(self, value: T) -> Vec<Vec<T>>;
}

impl VecExt<TokenTree> for IntoIter {
    fn split(self, value: TokenTree) -> Vec<Vec<TokenTree>> {
        let mut data = vec![];

        for item in self {
            let last = data.last_mut();

            let equals = match (&item, &value) {
                (TokenTree::Group(a), TokenTree::Group(b)) => a.to_string() == b.to_string(),
                (TokenTree::Ident(a), TokenTree::Ident(b)) => a == b,
                (TokenTree::Punct(a), TokenTree::Punct(b)) => a.as_char() == b.as_char(),
                (TokenTree::Literal(a), TokenTree::Literal(b)) => a.to_string() == b.to_string(),
                _ => false,
            };

            if equals {
                data.push(vec![]);
            } else if let Some(last) = last {
                last.push(item);
            } else {
                data.push(vec![item]);
            }
        }

        data
    }
}

fn generate_member(func: ImplItemFn, is_module: bool) -> [proc_macro2::TokenStream; 2] {
    // let prop_attr = attributes::get_list(&func.attrs, "prop");
    let func_attr = attributes::get_list(&func.attrs, "func");

    if let Some((_, list)) = func_attr {
        let mut attributes = Attributes::default();

        let tokens = list
            .tokens
            .into_iter()
            .split(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

        for tokens in tokens {
            if let TokenTree::Ident(name) = &tokens[0] {
                match (name.to_string().as_str(), &tokens[2..]) {
                    ("name", [TokenTree::Literal(literal)]) => {
                        let literal = literal.to_string();
                        let literal = literal
                            .strip_prefix('"')
                            .map(str::to_string)
                            .unwrap_or(literal.clone())
                            .strip_suffix('"')
                            .map(str::to_string)
                            .unwrap_or(literal);

                        attributes.name = Some(syn::parse_str::<Ident>(&literal).unwrap())
                    }
                    ("args", [TokenTree::Group(group)]) => {
                        attributes.args = group
                            .stream()
                            .into_iter()
                            .filter_map(|token| {
                                if let TokenTree::Group(group) = token {
                                    let tokens = group
                                        .stream()
                                        .into_iter()
                                        .split(TokenTree::Punct(Punct::new(',', Spacing::Alone)));

                                    if let TokenTree::Literal(literal) = &tokens[0][0] {
                                        let literal = literal.to_string();
                                        let literal = literal
                                            .strip_prefix('"')
                                            .map(str::to_string)
                                            .unwrap_or(literal.clone())
                                            .strip_suffix('"')
                                            .map(str::to_string)
                                            .unwrap_or(literal);

                                        Some((
                                            literal,
                                            syn::parse::<Expr>(
                                                proc_macro2::TokenStream::from_iter(
                                                    tokens[1].clone(),
                                                )
                                                .into(),
                                            )
                                            .unwrap(),
                                            tokens.get(2).map(|tokens| {
                                                syn::parse::<Expr>(
                                                    proc_macro2::TokenStream::from_iter(
                                                        tokens.clone(),
                                                    )
                                                    .into(),
                                                )
                                                .unwrap()
                                            }),
                                        ))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            })
                            .collect();
                    }
                    ("returns", tokens) => {
                        attributes.returns = Some(
                            syn::parse::<Expr>(
                                proc_macro2::TokenStream::from_iter(tokens.to_vec()).into(),
                            )
                            .unwrap(),
                        );
                    }
                    _ => {}
                }
            }
        }

        let original_name = func.sig.ident.clone();
        let name = attributes
            .name
            .unwrap_or(func.sig.ident.clone())
            .to_string();
        let args = attributes
            .args
            .into_iter()
            .map(|(name, ty, default)| {
                if let Some(default) = default {
                    quote!(.param_default(#name, #ty, #default))
                } else {
                    quote!(.param(#name, #ty))
                }
            })
            .collect::<Vec<_>>();
        let returns = if let Some(returns) = attributes.returns {
            quote!(.returns(#returns))
        } else {
            quote!()
        };

        let optional_ty = syn::parse_str::<Type>("Option<impl Into<Value>>").unwrap();
        let default_ty = syn::parse_str::<Type>("impl Into<Value>").unwrap();

        let returning = if let ReturnType::Type(_, ty) = func.sig.output.clone() {
            if *ty == optional_ty {
                quote! {
                    if let Some(result) = result {
                        args.returns(result);
                    }
                }
            } else if *ty == default_ty {
                quote!(args.returns(result))
            } else {
                quote!()
            }
        } else {
            quote!()
        };

        let object_func = if is_module {
            quote! {
                module.export(#name, FunctionBuilder::new(#name)
                    #(#args)*
                    #returns
                    .build(move |args| {
                        let result = self.#original_name(args);

                        #returning
                    })
                );
            }
        } else {
            quote! {
                .prop(#name, FunctionBuilder::new(#name)
                    #(#args)*
                    #returns
                    .build(move |args| {
                        let result = self.#original_name(args);

                        #returning
                    })
                )
            }
        };

        let mut func = func;

        func.attrs.clear();

        return [func.into_token_stream(), object_func];
    }

    [quote!(), quote!()]
}

#[proc_macro_attribute]
pub fn native_object(_attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = syn::parse::<ItemImpl>(item).unwrap();

    let name = item_impl.self_ty.to_token_stream();

    let mut members = vec![];
    let mut object_members = vec![];

    for item in item_impl.items {
        if let ImplItem::Fn(func) = item {
            let [member, object_member] = generate_member(func, false);

            members.push(member);
            object_members.push(object_member);
        }
    }

    let result = quote! {
        impl #name {
            #(#members)*
        }

        impl NativeObject for #name {
            fn build_object(&'static self) -> Value {
                ObjectBuilder::default()
                    #(#object_members)*
                    .build()
            }
        }
    };

    TokenStream::from(result)
}

#[proc_macro_attribute]
pub fn native_module(attrs: TokenStream, item: TokenStream) -> TokenStream {
    let item_impl = syn::parse::<ItemImpl>(item).unwrap();

    let module_name = syn::parse::<Literal>(attrs).unwrap();

    let name = item_impl.self_ty.to_token_stream();

    let mut members = vec![];
    let mut module_members = vec![];

    for item in item_impl.items {
        if let ImplItem::Fn(func) = item {
            let [member, module_member] = generate_member(func, true);

            members.push(member);
            module_members.push(module_member);
        }
    }

    let result = quote! {
        impl #name {
            #(#members)*
        }

        impl NativeModule for #name {
            fn build_module(&'static self) -> Module {
                let mut module = Module::new(#module_name);

                #(#module_members)*

                module
            }
        }
    };

    TokenStream::from(result)
}
