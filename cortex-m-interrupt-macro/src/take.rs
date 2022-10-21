use syn::{parse::Parse, Error, Ident, LitStr};

pub struct Take {
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
    pub fn new(irq: Ident) -> Self {
        Self { irq }
    }

    pub fn build(&self) -> proc_macro2::TokenStream {
        let Take { irq } = self;

        let interrupt_export_name = LitStr::new(&irq.to_string(), irq.span());

        quote::quote! {
            {
                struct Handle;

                static REGISTERED: core::sync::atomic::AtomicBool = core::sync::atomic::AtomicBool::new(false);

                static mut HANDLER: fn() = || { unsafe { ::cortex_m_interrupt::DefaultHandler_()  } };

                #[export_name = #interrupt_export_name]
                #[allow(non_snake_case)]
                pub unsafe extern "C" fn #irq() {
                    (HANDLER)();
                }

               impl ::cortex_m_interrupt::InterruptHandle for Handle {
                    #[inline(always)]
                    fn register(self, f: fn()) {
                        if REGISTERED.swap(true, core::sync::atomic::Ordering::Acquire) {
                            panic!(stringify!(Attempted to register already-registered interrupt #irq))
                        }

                        unsafe {
                            HANDLER = f;
                        }

                        core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);
                    }
                }

                Handle
            }
        }
    }
}
