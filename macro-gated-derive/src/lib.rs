//! # Gated Derive
//!
//! `gated_derive` is a procedural macro utility designed to reduce boilerplate when managing
//! optional dependencies and their associated derive macros and attributes.
//!
//! It automatically transforms a simple list of desired derives into a set of `#[cfg_attr(...)]`
//! gates. Furthermore, it scans the struct or enum for helper attributes associated with those
//! derives and gates them behind the same feature flags.
//!
//! ## Usage
//!
//! Apply the attribute to a struct or enum:
//!
//! ```rust,ignore
//! #[gated_derive(DerivePath, AnotherDerive(managed_attr))]
//! ```
//!
//! ### Semantic Rules
//!
//! 1.  **Feature Naming**: The feature name is derived from the **first segment** of the path.
//!     *   `serde::Serialize` $\rightarrow$ `feature = "derive_serde"`
//!     *   `MyTrait` $\rightarrow$ `feature = "derive_MyTrait"`
//!
//! 2.  **Attribute Association**:
//!     *   **Explicit**: `Path::To::Derive(attr_name)` will gate usages of `#[attr_name(...)]`
//!         behind the feature.
//!     *   **Implicit**: `Path::To::Derive` (no parens) defaults to gating attributes matching
//!         the first path segment (e.g., `serde` for `serde::Serialize`).
//!
//! 3.  **Shared Attributes**: If multiple derives claim the same attribute (e.g., `Serialize`
//!     and `Deserialize` both use `#[serde]`), the macro generates an `any(...)` condition,
//!     ensuring the attribute remains active if *at least one* of the features is enabled.
//!
//! ---
//!
//! ## Examples
//!
//! ### Example 1: CLI Configuration
//!
//! A configuration struct that supports command-line parsing (via `clap`) and file loading
//! (via `serde`), but both are optional features.
//!
//! ```rust,ignore
//! // Cargo.toml:
//! // [features]
//! // derive_clap = ["dep:clap"]
//! // derive_serde = ["dep:serde"]
//!
//! #[gated_derive(clap::Parser, serde::Deserialize)]
//! #[command(author, version)] // Gated by `derive_clap` (default inference)
//! struct AppConfig {
//!     #[arg(long)] // Gated by `derive_clap` (clap uses `arg` and `command` attributes?
//!                  // *See caveats below regarding multiple attributes per derive*)
//!     #[serde(default)] // Gated by `derive_serde`
//!     port: u16,
//! }
//! ```
//! *Note: To handle `clap` correctly which uses multiple attribute names (`clap`, `command`, `arg`),
//! you would pass them explicitly: `clap::Parser(clap, command, arg)`.*
//!
//! ### Example 2: Database and API Models
//!
//! A model that might be used with `sqlx` for database interaction or `schemars` for generating
//! OpenAPI specs.
//!
//! ```rust,ignore
//! #[gated_derive(sqlx::FromRow, schemars::JsonSchema(schemars))]
//! struct User {
//!     #[sqlx(rename = "user_id")] // Gated by feature "derive_sqlx"
//!     id: i32,
//!
//!     #[schemars(skip)] // Gated by feature "derive_schemars"
//!     internal_flag: bool,
//! }
//! ```
//!
//! ---
//!
//! ## Details and Caveats
//!
//! ### 1. Cargo.toml Requirements
//! This macro **does not** create features for you. You must define the corresponding features
//! in your `Cargo.toml`.
//!
//! ```toml
//! [features]
//! derive_serde = ["dep:serde"]
//! derive_defmt = ["dep:defmt"]
//! ```
//!
//! ### 2. Multiple Attributes
//! If a single derive uses multiple helper attributes (like `clap` uses `arg`, `command`, and `value`),
//! you must list them all in parentheses if you want them gated:
//!
//! ```rust,ignore
//! #[gated_derive(clap::Parser(clap, command, arg, value))]
//! ```
//!
//! ### 3. Hygiene
//! The macro parses the syntax tree of your item. It blindly gates attributes based on the identifier.
//! If you have a local macro or attribute named `#[serde]` that has nothing to do with `serde::Serialize`,
//! it will still be gated if you use `gated_derive(serde::Serialize)`.
//!
//! ### 4. Order of Operations
//! `gated_derive` runs before the actual derive macros it injects. However, it effectively replaces
//! itself with `#[cfg_attr(..., derive(...))]`.

use proc_macro::TokenStream;
use quote::quote;
use std::collections::{HashMap, HashSet};
use syn::{
    Attribute, DeriveInput, Ident, Path, Token,
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote,
    punctuated::Punctuated,
    visit_mut::{self, VisitMut},
};

/// Represents a single entry in the gated_derive list.
/// Example: `arbitrary::Arbitrary(serde)` or `serde::Serialize`
struct GatedDeriveEntry {
    derive_path: Path,
    /// Attributes that should be gated by this derive's feature.
    /// If empty, defaults to the first segment of the derive_path.
    managed_attrs: Vec<Ident>,
}

impl Parse for GatedDeriveEntry {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let derive_path: Path = input.parse()?;
        let mut managed_attrs = Vec::new();

        // Check for optional parenthesized attributes: `(attr1, attr2)`
        if input.peek(syn::token::Paren) {
            let content;
            syn::parenthesized!(content in input);
            let attrs = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
            managed_attrs.extend(attrs);
        } else {
            // Default: use the first segment of the path (e.g., "serde" from "serde::Serialize")
            if let Some(first_segment) = derive_path.segments.first() {
                managed_attrs.push(first_segment.ident.clone());
            }
        }

        Ok(GatedDeriveEntry {
            derive_path,
            managed_attrs,
        })
    }
}

/// Represents the comma-separated list of arguments passed to `gated_derive`.
struct GatedDeriveArgs {
    entries: Punctuated<GatedDeriveEntry, Token![,]>,
}

impl Parse for GatedDeriveArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(GatedDeriveArgs {
            entries: Punctuated::parse_terminated(input)?,
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
        // Standard recursion for the attribute's meta if needed,
        // but we mainly care about replacing the attribute itself.

        // We only care about outer attributes (#[...]) generally used for derives/serde
        self.try_gate(attr);

        // Continue visiting recursively (though for attributes, usually not deep)
        visit_mut::visit_attribute_mut(self, attr);
    }
}

impl GatedVisitor {
    fn try_gate(&self, attr: &mut Attribute) {
        // Check if the attribute name (e.g., "serde", "can_id") is in our map
        if let Some(ident) = attr.meta.path().get_ident() {
            let ident_str = ident.to_string();
            if let Some(features) = self.attr_map.get(&ident_str) {
                // Determine the condition
                let condition = if features.len() == 1 {
                    let feat = features
                        .iter()
                        .next()
                        .expect("if condition ensures exactly one element");
                    quote! { feature = #feat }
                } else {
                    // Sort for deterministic output
                    let mut sorted_feats: Vec<_> = features.iter().collect();
                    sorted_feats.sort();
                    quote! { any( #( feature = #sorted_feats ),* ) }
                };

                // Replace the attribute with #[cfg_attr(condition, original_attr)]
                // We use parse_quote to reconstruct it cleanly.
                let original_meta = &attr.meta;
                *attr = parse_quote!(#[cfg_attr(#condition, #original_meta)]);
            }
        }
    }
}

/// The main procedural macro.
///
/// Usage:
/// ```ignore
/// #[gated_derive(Path::To::Derive(optional_attr_name), Other::Derive)]
/// ```
#[proc_macro_attribute]
pub fn gated_derive(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as GatedDeriveArgs);
    let mut item = parse_macro_input!(input as DeriveInput);

    // 1. Analyze arguments to build:
    //    - A list of new `cfg_attr` derives to apply to the item.
    //    - A mapping of which attributes need to be gated by which features.
    let mut new_derives = Vec::new();
    let mut attr_feature_map: HashMap<String, HashSet<String>> = HashMap::new();

    for entry in args.entries {
        // Extract feature name: "derive_" + first path segment
        // e.g., serde::Serialize -> derive_serde
        let first_segment = match entry.derive_path.segments.first() {
            Some(seg) => &seg.ident,
            None => continue, // Should not happen for valid paths
        };
        let feature_name = format!("derive_{first_segment}");
        let path = &entry.derive_path;

        // Create the `#[cfg_attr(feature = "...", derive(...))]`
        new_derives.push(quote! {
            #[cfg_attr(feature = #feature_name, derive(#path))]
        });

        // Register associated attributes to this feature
        for attr_ident in entry.managed_attrs {
            attr_feature_map
                .entry(attr_ident.to_string())
                .or_default()
                .insert(feature_name.clone());
        }
    }

    // 2. Visit the Item (Struct/Enum) recursively and gate specific attributes
    let mut visitor = GatedVisitor {
        attr_map: attr_feature_map,
    };
    visitor.visit_derive_input_mut(&mut item);

    // 3. Inject the generated derive macros at the top of the item
    // We reverse insert so they appear in order at the top
    for derive_attr in new_derives.into_iter().rev() {
        let attr: Attribute = parse_quote!(#derive_attr);
        item.attrs.insert(0, attr);
    }

    // 4. Output the result
    TokenStream::from(quote!(#item))
}
