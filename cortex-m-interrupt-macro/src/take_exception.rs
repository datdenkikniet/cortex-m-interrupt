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

        quote! {{
            struct ExceptionHandle {
                exception: ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception,
            }

            impl ::cortex_m_interrupt::InterruptHandle for ExceptionHandle {
                fn register(self, f: fn()) {
                    let handle = #take;
                    handle.register(f);
                }
            }

            impl ::cortex_m_interrupt::ExceptionHandle for ExceptionHandle {
                fn exception(&self) -> ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception {
                    ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception::#exception
                }
            }

            ExceptionHandle {
                exception: ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception::#exception,
            }
        }}
        .into()
    }
}
