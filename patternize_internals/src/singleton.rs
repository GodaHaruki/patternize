use proc_macro2::TokenStream;
use quote::quote;
use syn::DeriveInput;

pub fn generate_singleton_derive(attr: TokenStream, input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;
    let vis = &input.vis;

    let screaming_snake_case: TokenStream = {
        let uppercase_position = struct_name
            .to_string()
            .chars()
            .enumerate()
            .filter(|(_, c)| c.is_uppercase())
            .map(|(i, _)| i)
            .skip(1) // struct name is upper-camel-case so first uppercase char is head
            .collect::<Vec<usize>>();

        let s = struct_name.to_string();
        let mut s = s.as_str();

        let mut before_split = 0;

        let mut screaming_snake_case = uppercase_position
            .into_iter()
            .map(|i| {
                let (name, left) = s.split_at(i - before_split);
                s = left;
                before_split = i;
                name.to_uppercase()
            })
            .collect::<Vec<String>>();

        screaming_snake_case.push(s.to_uppercase());
        screaming_snake_case.join("_").parse().unwrap()
    };
    if attr.is_empty() {
        quote! {
            #input

            #vis static #screaming_snake_case: std::sync::OnceLock<#struct_name> = std::sync::OnceLock::new();
        }
    } else {
        quote! {
            #input

            #vis static #screaming_snake_case: std::sync::LazyLock<#struct_name> = std::sync::LazyLock::new(|| #attr);
        }
    }
}
