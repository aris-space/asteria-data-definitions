use darling::FromMeta;
use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use syn::{Attribute, LitStr};

#[derive(Clone)]
#[allow(dead_code)] // only used for mavlink, but don't want to feature gate
pub struct Unit(pub LitStr);

impl FromMeta for Unit {
    fn from_list(items: &[darling::ast::NestedMeta]) -> darling::Result<Self> {
        if items.len() != 1 {
            return Err(darling::Error::custom(
                "Expected exactly one argument, use `#[unit(\"...\")]`",
            ));
        }

        if let darling::ast::NestedMeta::Lit(syn::Lit::Str(s)) = &items[0] {
            Ok(Unit(s.clone()))
        } else {
            Err(darling::Error::custom(
                "Expected a string literal, use `#[unit(\"...\")]`",
            ))
        }
    }
}

impl ToTokens for Unit {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens);
    }
}

#[allow(dead_code)] // only used for mavlink, but don't want to feature gate
pub trait GetUnit {
    /// Get the value of the unit attribute
    fn get_unit(&self) -> darling::Result<Option<Unit>>;
}

impl GetUnit for Vec<Attribute> {
    fn get_unit(&self) -> darling::Result<Option<Unit>> {
        match self.iter().find(|attr| attr.path().is_ident("unit")) {
            Some(attr) => Unit::from_meta(&attr.meta).map(Some),
            None => Ok(None),
        }
    }
}

pub trait GetDocs {
    /// Get all doc comments collected as a single, newline
    /// separated string
    #[allow(dead_code)] // only used for mavlink, but don't want to feature gate
    fn get_docs_str(&self) -> String;

    /// Get all doc comments as an attribute tokenstream such that
    /// they can be applied to another item
    fn get_docs(&self) -> Option<TokenStream>;
}

impl GetDocs for Vec<syn::Attribute> {
    fn get_docs_str(&self) -> String {
        self.iter()
            .filter_map(|attr| {
                if attr.path().is_ident("doc")
                    && let syn::Meta::NameValue(meta_name_value) = &attr.meta
                    && let syn::Expr::Lit(expr_lit) = &meta_name_value.value
                    && let syn::Lit::Str(lit_str) = &expr_lit.lit
                {
                    Some(lit_str.value().trim().to_string())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn get_docs(&self) -> Option<TokenStream> {
        let attrs = self
            .iter()
            .filter(|attr| attr.path().is_ident("doc"))
            .collect::<Vec<_>>();
        if attrs.is_empty() {
            None
        } else {
            Some(quote! {
                #(#attrs)*
            })
        }
    }
}
