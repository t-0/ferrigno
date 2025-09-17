extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn make_cstring(input: TokenStream) -> TokenStream {
    format!("b\"{}\0\" as *const u8 as *const i8", input.to_string().trim_matches('"')).parse().unwrap()
}
