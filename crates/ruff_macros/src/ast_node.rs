use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{Attribute, Error, Fields, Ident, ItemEnum, Result, Type, Variant};

pub(crate) fn generate_ast_enum(input: ItemEnum) -> Result<TokenStream> {
    let ast_enum = AstEnum::new(input)?;
    let node_enum = generate_node_enum(&ast_enum);
    Ok(quote! {
        #node_enum
    })
}

fn snake_case(ident: &Ident) -> Ident {
    let string = ident.to_string().to_snake_case();
    Ident::new(&string, ident.span())
}

fn concat(prefix: &str, ident: &Ident, suffix: &str) -> Ident {
    let mut string = ident.to_string();
    string.insert_str(0, prefix);
    string.push_str(suffix);
    Ident::new(&string, ident.span())
}

struct AstEnum {
    enum_node_ty: Ident,
    variants: Vec<AstVariant>,
}

struct AstVariant {
    variant_name: Ident,
    variant_node_ty: Ident,
    attrs: Vec<Attribute>,
}

impl AstEnum {
    fn new(input: ItemEnum) -> Result<AstEnum> {
        let ItemEnum {
            ident: id_ident, ..
        } = input;
        let node_ident = trim_id(&id_ident)?;
        let variants: Result<Vec<_>> = input.variants.into_iter().map(AstVariant::new).collect();
        let variants = variants?;
        let storage_ty = concat("", &node_ident, "Storage");
        let storage_field = snake_case(&storage_ty);
        Ok(AstEnum {
            id_ident,
            node_ident,
            storage_field,
            storage_ty,
            variants,
        })
    }
}

impl AstVariant {
    fn new(variant: Variant) -> Result<AstVariant> {
        let Fields::Unnamed(fields) = &variant.fields else {
            return Err(Error::new(
                variant.fields.span(),
                "Each AstNode variant must have a single unnamed field",
            ));
        };
        let mut fields = fields.unnamed.iter();
        let field = fields.next().ok_or_else(|| {
            Error::new(
                variant.fields.span(),
                "Each AstNode variant must have a single unnamed field",
            )
        })?;
        if fields.next().is_some() {
            return Err(Error::new(
                variant.fields.span(),
                "Each AstNode variant must have a single unnamed field",
            ));
        }
        let Type::Path(field_ty) = &field.ty else {
            return Err(Error::new(
                field.ty.span(),
                "Each AstNode variant must wrap a simple Id type",
            ));
        };
        let id_ty = field_ty.path.require_ident()?.clone();
        let node_ty = trim_id(&id_ty)?;
        let vec_name = snake_case(&node_ty);
        Ok(AstVariant {
            variant_name: variant.ident.clone(),
            id_ty,
            node_ty,
            vec_name,
            attrs: variant.attrs,
        })
    }
}

fn generate_id_enum(ast_enum: &AstEnum) -> TokenStream {
    let AstEnum {
        id_ident, variants, ..
    } = ast_enum;
    let variants = variants.iter().map(|v| {
        let AstVariant {
            attrs,
            variant_name,
            id_ty,
            ..
        } = v;
        quote! {
            #( #attrs )*
            #variant_name(#id_ty)
        }
    });
    quote! {
        #[automatically_derived]
        #[derive(Copy, Clone, Debug, PartialEq, is_macro::Is)]
        pub enum #id_ident {
            #( #variants ),*
        }
    }
}

fn generate_node_enum(ast_enum: &AstEnum) -> TokenStream {
    let AstEnum {
        node_ident,
        variants,
        ..
    } = ast_enum;
    let variants = variants.iter().map(|v| {
        let AstVariant {
            attrs,
            variant_name,
            node_ty,
            ..
        } = v;
        quote! {
            #( #attrs )*
            #variant_name(crate::Node<'a, &'a #node_ty>)
        }
    });
    quote! {
        #[automatically_derived]
        #[derive(Copy, Clone, Debug, PartialEq, is_macro::Is)]
        pub enum #node_ident<'a> {
            #( #variants ),*
        }
    }
}

fn generate_node_enum_node_method(ast_enum: &AstEnum) -> TokenStream {
    let AstEnum {
        id_ident,
        node_ident,
        variants,
        ..
    } = ast_enum;
    let variants = variants.iter().map(|v| {
        let AstVariant { variant_name, .. } = v;
        quote! { #id_ident::#variant_name(id) => #node_ident::#variant_name(self.ast.wrap(&self.ast[id])) }
    });
    quote! {
        #[automatically_derived]
        impl<'a> crate::Node<'a, #id_ident> {
            #[inline]
            pub fn node(&self) -> #node_ident<'a> {
                match self.node {
                    #( #variants ),*
                }
            }
        }
    }
}

fn generate_node_enum_ranged_impl(ast_enum: &AstEnum) -> TokenStream {
    let AstEnum {
        node_ident,
        variants,
        ..
    } = ast_enum;
    let variants = variants.iter().map(|v| {
        let AstVariant { variant_name, .. } = v;
        quote! { #node_ident::#variant_name(node) => node.range() }
    });
    quote! {
        #[automatically_derived]
        impl ruff_text_size::Ranged for #node_ident<'_> {
            fn range(&self) -> ruff_text_size::TextRange {
                match self {
                    #( #variants ),*
                }
            }
        }
    }
}

fn generate_variant_ids(ast_enum: &AstEnum) -> TokenStream {
    let AstEnum {
        variants,
        storage_field,
        ..
    } = ast_enum;
    let variants = variants.iter().map(|v| {
        let AstVariant {
            id_ty,
            node_ty,
            vec_name,
            ..
        } = v;
        quote! {
            #[automatically_derived]
            #[ruff_index::newtype_index]
            pub struct #id_ty;

            #[automatically_derived]
            impl std::ops::Index<#id_ty> for crate::Ast {
                type Output = #node_ty;
                #[inline]
                fn index(&self, id: #id_ty) -> &#node_ty {
                    &self.#storage_field.#vec_name[id]
                }
            }

            #[automatically_derived]
            impl std::ops::IndexMut<#id_ty> for crate::Ast {
                #[inline]
                fn index_mut(&mut self, id: #id_ty) -> &mut #node_ty {
                    &mut self.#storage_field.#vec_name[id]
                }
            }

            #[automatically_derived]
            impl<'a> crate::Node<'a, #id_ty> {
                #[inline]
                pub fn node(&self) -> crate::Node<'a, &'a #node_ty> {
                    self.ast.wrap(&self.ast[self.node])
                }
            }

            #[automatically_derived]
            impl<'a> ruff_text_size::Ranged for #node_ty {
                fn range(&self) -> TextRange {
                    self.range
                }
            }

            #[automatically_derived]
            impl<'a> ruff_text_size::Ranged for crate::Node<'a, &'a #node_ty> {
                fn range(&self) -> TextRange {
                    self.as_ref().range()
                }
            }
        }
    });
    quote! { #( #variants )* }
}

fn generate_storage(ast_enum: &AstEnum) -> TokenStream {
    let AstEnum {
        id_ident,
        variants,
        storage_field,
        storage_ty,
        ..
    } = ast_enum;
    let storage_fields = variants.iter().map(|v| {
        let AstVariant {
            id_ty,
            node_ty,
            vec_name,
            ..
        } = v;
        quote! { #vec_name: ruff_index::IndexVec<#id_ty, #node_ty> }
    });
    let add_methods = variants.iter().map(|v| {
        let AstVariant {
            variant_name,
            node_ty,
            vec_name,
            ..
        } = v;
        let method_name = concat("add_", vec_name, "");
        quote! {
            #[automatically_derived]
            impl crate::Ast {
                pub fn #method_name(&mut self, payload: #node_ty) -> #id_ident {
                    #id_ident::#variant_name(self.#storage_field.#vec_name.push(payload))
                }
            }
        }
    });
    quote! {
        #[automatically_derived]
        #[allow(clippy::derive_partial_eq_without_eq)]
        #[derive(Clone, Default, PartialEq)]
        pub(crate) struct #storage_ty {
            #( #storage_fields ),*
        }

        #( #add_methods )*
    }
}
