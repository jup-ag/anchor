use anchor_lang_idl::types::Idl;
use quote::quote;

use super::common::convert_idl_type_def_to_ts_with_depth;

pub fn gen_types_mod(idl: &Idl) -> proc_macro2::TokenStream {
    gen_types_mod_with_depth(idl, 10) // Default max depth of 10
}

pub fn gen_types_mod_with_depth(idl: &Idl, max_depth: usize) -> proc_macro2::TokenStream {
    let types = idl
        .types
        .iter()
        .filter(|ty| {
            // Skip accounts and events
            !(idl.accounts.iter().any(|acc| acc.name == ty.name)
                || idl.events.iter().any(|ev| ev.name == ty.name))
        })
        .map(|ty| convert_idl_type_def_to_ts_with_depth(ty, &idl.types, 0, max_depth));

    quote! {
        /// Program type definitions.
        ///
        /// Note that account and event type definitions are not included in this module, as they
        /// have their own dedicated modules.
        pub mod types {
            use super::*;

            #(#types)*
        }
    }
}
