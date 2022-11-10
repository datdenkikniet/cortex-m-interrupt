use proc_macro_error::proc_macro_error;

mod register_interrupt;

#[proc_macro]
#[proc_macro_error]
pub fn register_interrupt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as register_interrupt::RegisterInterrupt)
        .codegen()
        .into()
}
