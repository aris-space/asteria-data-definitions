//! # Gated Derive
//!
//! `gated_derive` reduces boilerplate when managing optional dependencies, their associated
//! derive macros, and the helper attributes those macros consume.
//!
//! It transforms a list of derives into `#[cfg_attr(feature = "...", derive(...))]` gates and
//! automatically gates the associated helper attributes (like `#[serde]`) on the struct fields.
//!
//! ## Usage
//!
//! ```rust,ignore
//! #[gated_derive(
//!     // 1. Simple Case: Uses registry defaults (if any) to gate attributes or gates only the derive.
//!     serde::Serialize,
//!
//!     // 2. Override Feature:
//!     // Default feature would be "derive_my_crate".
//!     my_crate::MyMacro(feature = "custom_feat"),
//!
//!     // 3. Explicit Attributes (Total Replacement):
//!     // Gates `#[clap]`, `#[arg]`, `#[command]`.
//!     clap::Parser(gated_attrs(clap, arg, command)),
//!
//!     // 4. Opt-Out of gating attributes:
//!     // Gates the derive, but leaves known associated attributes ungated.
//!     Default(gated_attrs())
//! )]
//! struct MyStruct { ... }
//! ```
//!
//! ## Semantic Rules
//!
//! ### 1. Feature Naming
//! Unless overridden via `feature = "..."`, the feature name is derived from the **first segment**
//! of the path, turned into ascii lowercase, and prefixed with `derive_`.
//! *   `serde::Serialize` -> `derive_serde`
//! *   `my_lib::Macro` -> `derive_my_lib`
//! *   `Deserialize` -> `derive_deserialize`
//!
//! ### 2. Default Gated Attribute Registry
//! The macro contains an internal hardcoded registry for common crates to provide sensible known values of associated attributes.
//! It can be overriden with `gated_attrs(attr1, attr2)`.
//!
//! | Path | Default Gated Attributes |
//! |------|--------------------------|
//! | `serde::Serialize` | `serde` |
//! | `serde::Deserialize` | `serde` |
//! | `schemars::JsonSchema` | `schemars`, `serde` |
//! | `utoipa::ToSchema` | `schema`, `serde` |
//! | `Default` | `default` |
//!
//! ### 3. Shared Attributes (Union Logic)
//! If multiple derives claim the same attribute, an `any(...)` condition is generated.
//!
//! **Example**: `schemars` generates JSON schemas but respects `serde` attributes.
//!
//! ```rust,ignore
//! #[gated_derive(serde::Serialize, schemars::JsonSchema)]
//! struct User {
//!     // Active if EITHER feature is enabled.
//!     // Generates: #[cfg_attr(any(feature = "derive_serde", feature = "derive_schemars"), serde(rename = "id"))]
//!     #[serde(rename = "id")]
//!     user_id: i32,
//! }
//! ```

use darling::{FromMeta, ast::NestedMeta, util::PathList};
use proc_macro::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{
    Attribute, DeriveInput, Ident, Meta, Path, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    spanned::Spanned as _,
    visit_mut::{self, VisitMut},
};

// -----------------------------------------------------------------------------
// Configuration Constants
// -----------------------------------------------------------------------------

/// Registry mapping Derive Paths to their standard helper attributes.
/// Matching is exact string equality on the path provided by the user.
const REGISTRY: &[(&str, &[&str])] = &[
    ("serde::Serialize", &["serde"]),
    ("serde::Deserialize", &["serde"]),
    ("schemars::JsonSchema", &["schemars", "serde"]),
    ("utoipa::ToSchema", &["schema", "serde"]),
    ("Default", &["default"]),
    ("std::default::Default", &["default"]),
    ("can::Message", &["can"]),
    ("can::Encode", &["can"]),
    ("can::Decode", &["can"]),
    ("mavlink::Message", &["mavlink"]),
    ("mavlink::FieldSet", &["mavlink", "unit"]),
    ("mavlink::Encode", &["mavlink"]),
    ("mavlink::Decode", &["mavlink"]),
    ("mavlink::Enum", &["mavlink"]),
    ("mavlink::Dialect", &["dialect"]),
];

// -----------------------------------------------------------------------------
// Data Structures
// -----------------------------------------------------------------------------

/// Configuration options for a single derive entry.
///
/// This struct parses the content *inside* the parentheses of a configured derive.
/// e.g. `path(feature = "...", gated_attrs(...))`
#[derive(Debug, Default, FromMeta)]
struct DeriveConfig {
    /// Override the default feature name.
    #[darling(default)]
    feature: Option<String>,

    /// Explicit list of attributes to gate.
    /// - `None` (missing): Use registry defaults.
    /// - `Some([])` (empty): Gate nothing.
    /// - `Some([a, b])`: Gate specific attributes.
    #[darling(default, rename = "gated_attrs")]
    gated_attrs: Option<PathList>,
}

/// A wrapper to parse the comma-separated list of arguments to the macro.
///
/// In Syn 2, `AttributeArgs` is removed, so we parse `Punctuated<Meta, Comma>`.
struct MacroArgs {
    items: Punctuated<Meta, Token![,]>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(MacroArgs {
            items: Punctuated::parse_terminated(input)?,
        })
    }
}

/// Visitor that traverses the AST and wraps target attributes in `cfg_attr`.
struct GatedVisitor {
    /// Map of Attribute Name -> Set of Feature Names required
    attr_map: HashMap<String, HashSet<String>>,
}

impl VisitMut for GatedVisitor {
    fn visit_attribute_mut(&mut self, attr: &mut Attribute) {
        self.try_gate(attr);
        // Continue recursion
        visit_mut::visit_attribute_mut(self, attr);
    }
}

impl GatedVisitor {
    fn try_gate(&self, attr: &mut Attribute) {
        if let Some(ident) = attr.meta.path().get_ident() {
            let ident_str = ident.to_string();
            if let Some(features) = self.attr_map.get(&ident_str) {
                // Construct the `cfg` condition
                let condition = if features.len() == 1 {
                    let feat = features.iter().next().expect("Set known to be non-empty");
                    quote! { feature = #feat }
                } else {
                    // Sort for deterministic output
                    let mut sorted_feats: Vec<_> = features.iter().collect();
                    sorted_feats.sort();
                    quote! { any( #( feature = #sorted_feats ),* ) }
                };

                // Replace #[attr(...)] with #[cfg_attr(condition, attr(...))]
                let original_meta = &attr.meta;
                *attr = parse_quote!(#[cfg_attr(#condition, #original_meta)]);
            }
        }
    }
}

// -----------------------------------------------------------------------------
// Logic
// -----------------------------------------------------------------------------

/// Returns the hardcoded default attributes for a given path from the REGISTRY.
fn get_registry_defaults(path: &Path) -> Option<Vec<Ident>> {
    // Convert path to string "path::to::Item" for comparison
    let path_str = path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");

    for (reg_path, attrs) in REGISTRY {
        if *reg_path == path_str {
            return Some(attrs.iter().map(|s| Ident::new(s, path.span())).collect());
        }
    }
    None
}

/// The main proc macro function, see the module docs [`self`].
#[proc_macro_attribute]
pub fn gated_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let args_parser = parse_macro_input!(args as MacroArgs);
    let mut item = parse_macro_input!(input as DeriveInput);

    let mut new_derives = Vec::new();
    let mut attr_feature_map: HashMap<String, HashSet<String>> = HashMap::new();

    // Iterate over the arguments: `path` or `path(...)`
    for arg in args_parser.items {
        // 1. Parse outer arguments and inner config
        let (derive_path, config) = match arg {
            // Case: `serde::Serialize`
            Meta::Path(path) => (path, DeriveConfig::default()),

            // Case: `serde::Serialize(...)`
            Meta::List(list) => {
                let path = &list.path;
                // Parse the tokens inside the parentheses as a list of NestedMeta
                // Syn 2 removed `nested`, so we parse `tokens` using darling's helper
                let nested_meta = match list
                    .parse_args_with(Punctuated::<NestedMeta, Token![,]>::parse_terminated)
                {
                    Ok(nm) => nm,
                    Err(e) => return e.into_compile_error().into(),
                };

                // Convert list to Struct using darling
                let cfg =
                    match DeriveConfig::from_list(&nested_meta.into_iter().collect::<Vec<_>>()) {
                        Ok(c) => c,
                        Err(e) => return e.write_errors().into(),
                    };

                (path.clone(), cfg)
            }

            // NameValue (`key = "val"`) is not supported at the top level
            Meta::NameValue(nv) => {
                return syn::Error::new_spanned(
                    nv,
                    "Name-Value arguments are not supported here. Use `Path` or `Path(config)`.",
                )
                .into_compile_error()
                .into();
            }
        };

        // 2. Determine Feature Name
        let feature_name = if let Some(feat) = config.feature {
            feat
        } else {
            // Default: "derive_" + first segment
            let mut first = derive_path
                .segments
                .first()
                .expect("Derive path must have at least one segment")
                .ident
                .to_string();
            first.make_ascii_lowercase();
            format!("derive_{first}")
        };

        // 3. Determine Attributes to Gate
        let attrs_to_gate: Vec<Ident> = if let Some(explicit_list) = config.gated_attrs {
            // User override (can be empty to opt-out)
            explicit_list
                .iter()
                .map(|p| {
                    p.get_ident()
                        .expect("gated_attrs entries must be identifiers")
                        .clone()
                })
                .collect()
        } else {
            // Registry defaults
            get_registry_defaults(&derive_path).unwrap_or_default()
        };

        // 4. Register Attribute Dependencies
        for attr in attrs_to_gate {
            attr_feature_map
                .entry(attr.to_string())
                .or_default()
                .insert(feature_name.clone());
        }

        // 5. Create the Derive Injection
        new_derives.push(quote! {
            #[cfg_attr(feature = #feature_name, derive(#derive_path))]
        });
    }

    // 6. Visit Item and Gate Attributes
    let mut visitor = GatedVisitor {
        attr_map: attr_feature_map,
    };
    visitor.visit_derive_input_mut(&mut item);

    // 7. Inject Generated Derives
    for derive_attr in new_derives.into_iter().rev() {
        let attr: Attribute = parse_quote!(#derive_attr);
        item.attrs.insert(0, attr);
    }

    TokenStream::from(quote!(#item))
}
