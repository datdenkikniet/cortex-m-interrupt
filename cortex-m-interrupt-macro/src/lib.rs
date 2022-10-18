use proc_macro_error::proc_macro_error;

mod take;
use take::Take;

mod take_exception;

mod take_nvic_interrupt;
use take_exception::TakeException;
use take_nvic_interrupt::TakeNvicInterrupt;

#[proc_macro]
#[proc_macro_error]
pub fn take_nvic_interrupt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as TakeNvicInterrupt).build(true)
}

#[proc_macro]
#[proc_macro_error]
pub fn take_exception(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as TakeException).build()
}
