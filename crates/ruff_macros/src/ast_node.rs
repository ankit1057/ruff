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
            ident: enum_node_ty,
            ..
        } = input;
        let variants: Result<Vec<_>> = input.variants.into_iter().map(AstVariant::new).collect();
        let variants = variants?;
        Ok(AstEnum {
            enum_node_ty,
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
        let variant_node_ty = field_ty.path.require_ident()?.clone();
        Ok(AstVariant {
            variant_name: variant.ident.clone(),
            variant_node_ty,
            attrs: variant.attrs,
        })
    }
}

fn generate_node_enum(ast_enum: &AstEnum) -> TokenStream {
    let AstEnum {
        enum_node_ty,
        variants,
        ..
    } = ast_enum;
    let enum_ref_ty = concat("", enum_node_ty, "Ref");
    let any_variant = concat("Any", enum_node_ty, "");

    let enum_node_variants = variants.iter().map(|v| {
        let AstVariant {
            attrs,
            variant_name,
            variant_node_ty,
            ..
        } = v;
        quote! {
            #( #attrs )*
            #variant_name(#variant_node_ty)
        }
    });

    let enum_node_from_impls = variants.iter().map(|v| {
        let AstVariant {
            variant_name,
            variant_node_ty,
            ..
        } = v;
        quote! {
            #[automatically_derived]
            impl From<#variant_node_ty> for #enum_node_ty {
                fn from(node: #variant_node_ty) -> #enum_node_ty {
                    #enum_node_ty::#variant_name(node)
                }
            }
        }
    });

    let enum_node_ranged_impls = variants.iter().map(|v| {
        let AstVariant {
            variant_node_ty, ..
        } = v;
        quote! {
            #[automatically_derived]
            impl ruff_text_size::Ranged for #variant_node_ty {
                fn range(&self) -> ruff_text_size::TextRange {
                    self.range
                }
            }
        }
    });

    let enum_ref_variants = variants.iter().map(|v| {
        let AstVariant {
            attrs,
            variant_name,
            variant_node_ty,
            ..
        } = v;
        quote! {
            #( #attrs )*
            #variant_name(&'a #variant_node_ty)
        }
    });

    let enum_ref_from_impls = variants.iter().map(|v| {
        let AstVariant {
            variant_name,
            variant_node_ty,
            ..
        } = v;
        quote! {
            #[automatically_derived]
            impl<'a> From<&'a #variant_node_ty> for #enum_ref_ty<'a> {
                fn from(payload: &'a #variant_node_ty) -> #enum_ref_ty<'a> {
                    #enum_ref_ty::#variant_name(payload)
                }
            }
        }
    });

    let enum_node_as_ref_variants = variants.iter().map(|v| {
        let AstVariant { variant_name, .. } = v;
        quote! { #enum_node_ty::#variant_name(node) => #enum_ref_ty::#variant_name(node), }
    });

    let enum_ref_as_ptr_variants = variants.iter().map(|v| {
        let AstVariant { variant_name, .. } = v;
        quote! { #enum_ref_ty::#variant_name(node) => std::ptr::NonNull::from(*node).cast(), }
    });

    let enum_ref_kind_variants = variants.iter().map(|v| {
        let AstVariant {
            variant_name,
            variant_node_ty,
            ..
        } = v;
        quote! { #enum_ref_ty::#variant_name(node) => crate::NodeKind::#variant_node_ty, }
    });

    let enum_ref_visit_preorder_variants = variants.iter().map(|v| {
        let AstVariant { variant_name, .. } = v;
        quote! { #enum_ref_ty::#variant_name(node) => node.visit_source_order(visitor), }
    });

    let enum_ref_ranged_variants = variants.iter().map(|v| {
        let AstVariant { variant_name, .. } = v;
        quote! { #enum_ref_ty::#variant_name(node) => node.range(), }
    });

    let any_node_from_impls = variants.iter().map(|v| {
        let AstVariant {
            variant_name,
            variant_node_ty,
            ..
        } = v;
        quote! {
            #[automatically_derived]
            impl From<#variant_node_ty> for crate::AnyNode {
                fn from(node: #variant_node_ty) -> crate::AnyNode {
                    crate::AnyNode::#any_variant(#enum_node_ty::#variant_name(node))
                }
            }

            #[automatically_derived]
            impl<'a> From<&'a #variant_node_ty> for crate::AnyNodeRef<'a> {
                fn from(node: &'a #variant_node_ty) -> crate::AnyNodeRef<'a> {
                    crate::AnyNodeRef::#any_variant(#enum_ref_ty::#variant_name(node))
                }
            }
        }
    });

    let any_node_is_impls = variants.iter().map(|v| {
        let AstVariant {
            variant_name,
            variant_node_ty,
            ..
        } = v;
        let any_method_name = concat("is_", &snake_case(variant_node_ty), "");
        let method_name = concat("is_", &snake_case(variant_name), "");
        quote! {
            pub fn #any_method_name(&self) -> bool {
                match self {
                    crate::AnyNodeRef::#any_variant(node) => node.#method_name(),
                    _ => false,
                }
            }
        }
    });

    quote! {
        #[automatically_derived]
        #[derive(Clone, Debug, PartialEq, is_macro::Is)]
        pub enum #enum_node_ty {
            #( #enum_node_variants ),*
        }

        #( #enum_node_from_impls )*

        #( #enum_node_ranged_impls )*

        #[automatically_derived]
        #[derive(Clone, Copy, Debug, PartialEq, is_macro::Is)]
        pub enum #enum_ref_ty<'a> {
            #( #enum_ref_variants ),*
        }

        #( #enum_ref_from_impls )*

        impl #enum_node_ty {
            pub const fn as_ref(&self) -> #enum_ref_ty {
                match self {
                    #( #enum_node_as_ref_variants )*
                }
            }
        }

        impl<'a> From<&'a #enum_node_ty> for #enum_ref_ty<'a> {
            fn from(node: &'a #enum_node_ty) -> #enum_ref_ty<'a> {
                node.as_ref()
            }
        }

        impl<'a> #enum_ref_ty<'a> {
            pub fn as_ptr(&self) -> std::ptr::NonNull<()> {
                match self {
                    #( #enum_ref_as_ptr_variants )*
                }
            }

            pub const fn kind(self) -> crate::NodeKind {
                match self {
                    #( #enum_ref_kind_variants )*
                }
            }

            pub fn visit_preorder<'b, V>(self, visitor: &mut V)
            where
                V: crate::visitor::source_order::SourceOrderVisitor<'b> + ?Sized,
                'a: 'b,
            {
                match self {
                    #( #enum_ref_visit_preorder_variants )*
                }
            }
        }

        #[automatically_derived]
        impl ruff_text_size::Ranged for #enum_ref_ty<'_> {
            fn range(&self) -> ruff_text_size::TextRange {
                match self {
                    #( #enum_ref_ranged_variants )*
                }
            }
        }

        #[automatically_derived]
        impl From<#enum_node_ty> for crate::AnyNode {
            fn from(node: #enum_node_ty) -> crate::AnyNode {
                crate::AnyNode::#any_variant(node)
            }
        }

        #[automatically_derived]
        impl<'a> From<&'a #enum_node_ty> for crate::AnyNodeRef<'a> {
            fn from(node: &'a #enum_node_ty) -> crate::AnyNodeRef<'a> {
                crate::AnyNodeRef::#any_variant(#enum_ref_ty::from(node))
            }
        }

        #[automatically_derived]
        impl<'a> From<#enum_ref_ty<'a>> for crate::AnyNodeRef<'a> {
            fn from(node: #enum_ref_ty<'a>) -> crate::AnyNodeRef<'a> {
                crate::AnyNodeRef::#any_variant(node)
            }
        }

        #( #any_node_from_impls )*

        #[automatically_derived]
        impl crate::AnyNodeRef<'_> {
            #( #any_node_is_impls )*
        }
    }
}
