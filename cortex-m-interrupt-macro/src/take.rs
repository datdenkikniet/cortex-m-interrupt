use proc_macro_error::abort;
use syn::{parse::Parse, spanned::Spanned, token::Comma, Error, LitStr, Path};

pub(crate) struct Take {
    event_path: Path,
}

impl Parse for Take {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let event_path = input.parse().map_err(|e| {
            Error::new(
                e.span(),
                "Expected path to event (interrupt or exception) as first argument.",
            )
        })?;

        Ok(Self { event_path })
    }
}

impl Take {
    pub(crate) fn build(&self) -> proc_macro::TokenStream {
        let Take { event_path } = self;

        let event_export_name = if let Some(last_seg) = &event_path.segments.last() {
            &last_seg.ident
        } else {
            abort!(event_path, "Could not find last segment of type path");
        };

        let interrupt_export_name = LitStr::new(&event_export_name.to_string(), event_path.span());

        quote::quote! {
            {
                struct Handle;

                static mut HANDLER: core::mem::MaybeUninit<fn()> = core::mem::MaybeUninit::uninit();

                #[export_name = #interrupt_export_name]
                pub unsafe extern "C" fn isr() {
                    (HANDLER.assume_init())();
                }

               impl ::cortex_m_interrupt::EventHandle for Handle {
                    fn register(self, f: fn()) {
                        unsafe {
                            HANDLER.write(f);
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
