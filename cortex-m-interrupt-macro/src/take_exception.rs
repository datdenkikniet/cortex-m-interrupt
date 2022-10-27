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
            struct ExceptionRegistration {
                exception: ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception,
            }

            impl ::cortex_m_interrupt::InterruptRegistration for ExceptionRegistration {
                fn occupy(self, f: fn()) {
                    let handle = #take;
                    handle.occupy(f);
                }
            }

            impl ::cortex_m_interrupt::ExceptionRegistration for ExceptionRegistration {
                const EXCEPTION: ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception = ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception::#exception;
            }

            ExceptionRegistration {
                exception: ::cortex_m_interrupt::cortex_m::peripheral::scb::Exception::#exception,
            }
        }}
        .into()
    }
}
