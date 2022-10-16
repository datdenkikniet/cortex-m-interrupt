use syn::{parse::Parse, Error, Ident, LitStr};

pub(crate) struct Take {
    irq: Ident,
}

impl Parse for Take {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let irq = input.parse().map_err(|e| {
            Error::new(
                e.span(),
                "Expected path to event (interrupt or exception) as first argument.",
            )
        })?;

        Ok(Self { irq })
    }
}

impl Take {
    pub(crate) fn build(&self) -> proc_macro::TokenStream {
        let Take { irq } = self;

        let interrupt_export_name = LitStr::new(&irq.to_string(), irq.span());

        quote::quote! {
            {
                struct Handle;

                static mut HANDLER: fn() = || { unsafe { ::cortex_m_interrupt::DefaultHandler_()  } };

                #[export_name = #interrupt_export_name]
                pub unsafe extern "C" fn #irq() {
                    (HANDLER)();
                }

               impl ::cortex_m_interrupt::InterruptHandle for Handle {
                    fn register(self, f: fn()) {
                        unsafe {
                            HANDLER = f;
                        }

                        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
                    }
                }

                Handle
            }
        }
        .into()
    }
}
