use quote::quote;
use syn::{parse::Parse, Ident};

pub struct TakeException {
    exception: Ident,
}

impl Parse for TakeException {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exception = input.parse()?;

        Ok(Self { exception })
    }
}

impl TakeException {
    pub fn build(&self) -> proc_macro::TokenStream {
        let take = crate::Take::new(self.exception.clone()).build();

        match self.exception.to_string().as_str() {
            "DefaultHandler" | "HardFault" => {
                proc_macro_error::abort!(self.exception, "Registering a handle for the DefaultHandler or HardFault exceptions is not supported.");
            }
            _ => {}
        }

        let exception = &self.exception;

        let int_handle = quote! { ::cortex_m_interrupt::InterruptHandle };
        let exception_path = quote! { ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception };

        quote! {{
            struct ExceptionHandle<T: #int_handle> {
                exception: ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception,
                handle: T,
            }

            impl<T: #int_handle> ::cortex_m_interrupt::InterruptHandle for ExceptionHandle<T> {
                fn register(&mut self, f: fn()) {
                    self.handle.register(f);
                }

                unsafe fn reset(&mut self) {
                    self.handle.reset();
                }
            }

            impl<T: #int_handle> ::cortex_m_interrupt::ExceptionHandle for ExceptionHandle<T> {
                const EXCEPTION: #exception_path = #exception_path::#exception;
            }

            ExceptionHandle {
                exception: #exception_path::#exception,
                handle: #take,
            }
        }}
        .into()
    }
}
