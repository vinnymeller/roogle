use std::path::Path;

use crate::CallableInfo;
use syn::{Visibility, __private::ToTokens};

pub fn handle_item_fn(f: &syn::ItemFn, src_file_path: &Path) -> CallableInfo {
    CallableInfo {
        file: src_file_path.to_path_buf(),
        struct_name: None,
        sig: f.sig.clone(),
        // generic_params: f
        //     .sig
        //     .generics
        //     .params
        //     .iter()
        //     .map(|p| p.to_token_stream().to_string())
        //     .collect(),
        // identifier: f.sig.ident.to_string(),
        // args: f
        //     .sig
        //     .inputs
        //     .iter()
        //     .map(|arg| arg.into_token_stream().to_string())
        //     .collect(),
        // return_type: f.sig.output.to_token_stream().to_string(),
    }
}

fn handle_item_impl_fn(
    f: &syn::ImplItemFn,
    struct_name: String,
    src_file_path: &Path,
) -> CallableInfo {
    CallableInfo {
        file: src_file_path.to_path_buf(),
        struct_name: Some(struct_name),
        sig: f.sig.clone(),
        // generic_params: f
        //     .sig
        //     .generics
        //     .params
        //     .iter()
        //     .map(|p| p.to_token_stream().to_string())
        //     .collect(),
        // identifier: f.sig.ident.to_string(),
        // args: f
        //     .sig
        //     .inputs
        //     .iter()
        //     .map(|arg| arg.into_token_stream().to_string())
        //     .collect(),
        // return_type: f.sig.output.clone().into_token_stream().to_string(),
    }
}

pub fn handle_item_impl(i: &syn::ItemImpl, src_file_path: &Path) -> Vec<CallableInfo> {
    let mut functions = Vec::new();
    let struct_name = i.self_ty.clone().into_token_stream().to_string();
    i.items.iter().for_each(|impl_item| {
        if let syn::ImplItem::Fn(f) = impl_item {
            if let Visibility::Public(_) = f.vis {
                functions.push(handle_item_impl_fn(f, struct_name.clone(), src_file_path));
            }
        };
    });
    functions
}

pub fn handle_parsed_items(items: &[syn::Item], src_file_path: &Path) -> Vec<CallableInfo> {
    let mut functions = Vec::new();
    items.iter().for_each(|item| match item {
        syn::Item::Fn(f) => {
            // only add public functions, not private ones because whats the point ?
            if let Visibility::Public(_) = f.vis {
                functions.push(handle_item_fn(f, src_file_path))
            }
        }
        syn::Item::Impl(i) => functions.extend(handle_item_impl(i, src_file_path)),
        _ => (),
    });

    functions
}
