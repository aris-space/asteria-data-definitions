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
//!     // 1. Simple Case: Uses registry defaults (if any) or gates only the derive.
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
//!     // 4. Opt-Out of Attributes:
//!     // Gates the derive, but leaves attributes ungated.
//!     other::Macro(gated_attrs())
//! )]
//! struct MyStruct { ... }
//! ```
//!
//! ## Semantic Rules
//!
//! ### 1. Feature Naming
//! Unless overridden via `feature = "..."`, the feature name is derived from the **first segment**
//! of the path.
//! *   `serde::Serialize` $\rightarrow$ `derive_serde`
//! *   `my_lib::Macro` $\rightarrow$ `derive_my_lib`
//!
//! *Note: This means `json::Error` and `toml::Error` would both trigger `derive_json` and `derive_toml` respectively, preventing collision.*
//!
//! ### 2. Hardcoded Registry
//! The macro contains an internal registry for common crates to provide sensible defaults.
//!
//! | Path | Default Gated Attributes |
//! |------|--------------------------|
//! | `serde::Serialize` | `serde` |
//! | `serde::Deserialize` | `serde` |
//! | `schemars::JsonSchema` | `schemars`, `serde` |
//!
//! *Note: Path matching is exact. `Serialize` (without `serde::`) will **not** trigger the registry.*
//!
//! ### 3. Attribute Resolution
//! 1.  **Explicit**: If `gated_attrs(...)` is provided, those attributes are used (replacing defaults).
//! 2.  **Registry**: If no explicit attrs are given, the registry is checked.
//! 3.  **Fallback**: If neither, no helper attributes are gated; only the `derive` itself is gated.
//!
//! ### 4. Shared Attributes (Union Logic)
//! If multiple derives claim the same attribute, an `any(...)` condition is generated.
//!
//! **Example**: `schemars` generates JSON schemas but respects `serde` attributes.
//!
//! ```rust,ignore
//! // Cargo.toml:
//! // [features]
//! // derive_serde = ["dep:serde"]
//! // derive_schemars = ["dep:schemars"]
//!
//! #[gated_derive(
//!     serde::Serialize,      // Defaults: feature="derive_serde", attrs=[serde]
//!     schemars::JsonSchema   // Defaults: feature="derive_schemars", attrs=[schemars, serde]
//! )]
//! struct User {
//!     // This attribute is active if EITHER feature is enabled.
//!     // Generates: #[cfg_attr(any(feature = "derive_serde", feature = "derive_schemars"), serde(rename = "id"))]
//!     #[serde(rename = "id")]
//!     user_id: i32,
//! }
//! ```

use darling::{FromMeta, ast::NestedMeta, util::PathList};
use proc_macro::TokenStream;
use quote::{ToTokens, quote};
use std::collections::{HashMap, HashSet};
use syn::{
    Attribute, AttributeArgs, DeriveInput, Ident, Meta, Path, parse_macro_input, parse_quote,
    spanned::Spanned,
    visit_mut::{self, VisitMut},
};

/// Configuration options for a single derive entry.
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
                // Construct the condition
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

/// Returns the hardcoded default attributes for a given path.
fn get_registry_defaults(path: &Path) -> Option<Vec<Ident>> {
    // Convert path to string "path::to::Item" for comparison
    let path_str = path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect::<Vec<_>>()
        .join("::");

    let attrs = match path_str.as_str() {
        "serde::Serialize" | "serde::Deserialize" => vec!["serde"],
        // schemars uses its own attribute, but also respects serde attributes
        "schemars::JsonSchema" => vec!["schemars", "serde"],
        _ => return None,
    };

    Some(
        attrs
            .into_iter()
            .map(|s| Ident::new(s, path.span()))
            .collect(),
    )
}

#[proc_macro_attribute]
pub fn gated_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let mut item = parse_macro_input!(input as DeriveInput);

    let mut new_derives = Vec::new();
    let mut attr_feature_map: HashMap<String, HashSet<String>> = HashMap::new();

    for arg in args {
        // 1. Parse outer arguments (Path vs List)
        let (derive_path, config) = match arg {
            // Case: `serde::Serialize`
            NestedMeta::Meta(Meta::Path(path)) => (path, DeriveConfig::default()),

            // Case: `serde::Serialize(...)`
            NestedMeta::Meta(Meta::List(list)) => {
                let path = list.path;
                let cfg = DeriveConfig::from_list(&list.nested)
                    .expect("Failed to parse gated_derive configuration");
                (path, cfg)
            }

            _ => panic!("Unsupported syntax in gated_derive. Expected Path or Path(...)"),
        };

        // 2. Determine Feature Name
        let feature_name = if let Some(feat) = config.feature {
            feat
        } else {
            // Default: "derive_" + first segment
            let first = derive_path
                .segments
                .first()
                .expect("Derive path must have at least one segment")
                .ident
                .to_string();
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

        // 4. Register Dependencies
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
