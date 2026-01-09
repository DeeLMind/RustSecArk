use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput,ItemFn};

#[proc_macro_derive(Hello)]
pub fn hello_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;

    let expanded = quote! {
        impl #name {
            pub fn hello() {
                println!("Hello from {}", stringify!(#name));
            }
        }
    };

    expanded.into()
}


#[proc_macro_attribute]
pub fn log(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let name = &input.sig.ident;
    let block = &input.block;
    let vis = &input.vis;
    let sig = &input.sig;

    let expanded = quote! {
        #vis #sig {
            println!("[ENTER] {}", stringify!(#name));
            let __ret = (|| #block)();
            println!("[EXIT] {}", stringify!(#name));
            __ret
        }
    };

    expanded.into()
}