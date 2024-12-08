use proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse_macro_input;

#[proc_macro_attribute]
pub fn derive_days(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the attribute input "1-24"
    let input = parse_macro_input!(attr);
    let range_str = input.value();

    // Split the input into start and end of the range
    let parts: Vec<&str> = range_str.split('-').collect();
    let start: usize = parts[0].parse().expect("Failed to parse start of range");
    let end: usize = parts[1].parse().expect("Failed to parse end of range");

    // Parse the original item (captures `hello world days`)
    let original_item = parse_macro_input!(item);
    let original_ident = original_item.ident;

    // Generate new modules
    let mut generated_modules = Vec::new();
    for i in start..=end {
        let mod_name = format_ident!("day{}", i);
        generated_modules.push(quote! {
            #original_ident #mod_name;
        });
    }

    let expanded = quote! {
        #(#generated_modules)*
    };

    expanded.into()
}
