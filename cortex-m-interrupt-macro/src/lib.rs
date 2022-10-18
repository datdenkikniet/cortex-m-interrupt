use proc_macro_error::proc_macro_error;

mod take;
use take::Take;
use take_nvic_interrupt::TakeNvicInterrupt;

mod take_nvic_interrupt;

#[proc_macro]
#[proc_macro_error]
pub fn take_nvic_interrupt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as TakeNvicInterrupt).build(true)
}
