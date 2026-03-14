//#[macro_export]
//macro_rules! create_string_variable {
//    ($name:ident; $value:expr) => {
//        pub const $name: *const i8 = make_cstring! ($value);
//    };
//}
// macro_rules! c_string {
//     ($value:tt) => {
//          concat!("b\"", $value, "\"\0").as_bytes().as_ptr() as *const u8 as *const i8
//     };
// }
// #[proc_macro]
// pub fn sql(input: TokenStream) -> TokenStream {
//     let input = input.to_string();
//     let output = format!("b\"{}\\0\" as *const u8 as *const i8", input);
//     output.parse().unwrap()
// }
// pub(crate) use {};
