use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{
    parse::Parse,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon2, Comma},
    Error, Expr, LitStr, Path, PathSegment,
};

fn handle_generator(
    interrupt_export_name: LitStr,
    interrupt_path: Path,
    defs: TokenStream,
    pre_write: TokenStream,
    post_write: TokenStream,
    interrupt_impl: TokenStream,
    return_value: TokenStream,
) -> TokenStream {
    let segments = interrupt_path.segments.len();

    if segments <= 1 {
        abort!(
            interrupt_path,
            "You must specify the interrupt name with at least two path segments! Try using Interrupt::{} or interrupt::{} instead.",
            interrupt_path.segments.first().unwrap().ident.to_string(),
            interrupt_path.segments.first().unwrap().ident.to_string()
        );
    }

    let interrupt_path_type = interrupt_path.segments.iter().take(segments - 1);
    let path: Punctuated<PathSegment, Colon2> = interrupt_path_type.map(|s| s.clone()).collect();

    quote::quote! {
        {
            #defs

            static mut HANDLER: core::mem::MaybeUninit<fn()> = core::mem::MaybeUninit::uninit();

            #[export_name = #interrupt_export_name]
            pub unsafe extern "C" fn isr() {
                (HANDLER.assume_init())();
            }

            impl ::cortex_m_interrupt::EventHandle for Handle {
                fn register(self, f: fn()) {
                    #pre_write
                    unsafe {
                        HANDLER.write(f);
                    }

                    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);

                    #post_write
                }
            }

            impl ::cortex_m_interrupt::IrqHandle for Handle {
                type Number = #path;

                fn interrupt(&self) -> Self::Number {
                    #interrupt_impl
                }
            }

            #return_value
        }
    }
}

pub(crate) struct Take {
    interrupt_path: Path,
    priority: Expr,
}

impl Parse for Take {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let interrupt_ident = input
            .parse()
            .map_err(|e| Error::new(e.span(), "Expected path to interrupt as first argument."))?;

        input.parse::<Comma>()?;

        let priority = input.parse().map_err(|e| {
            Error::new(
                e.span(),
                "Expected an expression that represents the priority to assign to the interrupt.",
            )
        })?;

        Ok(Self {
            interrupt_path: interrupt_ident,
            priority,
        })
    }
}

impl Take {
    pub(crate) fn build(&self, use_logical_prio: bool) -> proc_macro::TokenStream {
        let Take {
            interrupt_path,
            priority,
        } = self;

        let interrupt_export_name = if let Some(last_seg) = &interrupt_path.segments.last() {
            &last_seg.ident
        } else {
            abort!(interrupt_path, "Could not find last segment of type path");
        };

        if super::is_exception(&interrupt_export_name.to_string()) {
            abort!(
                interrupt_path,
                "`{}` is an exception, which is not serviced by the NVIC. You may want to use `take_exception` instead",
                interrupt_export_name
            )
        }

        let interrupt_export_name =
            LitStr::new(&interrupt_export_name.to_string(), interrupt_path.span());

        let set_priority = if use_logical_prio {
            quote! {
                let prio_bits = ::cortex_m_interrupt::determine_prio_bits(&mut nvic, #interrupt_path.number());
                let priority = ::cortex_m_interrupt::logical2hw(self.priority, prio_bits);
                nvic.set_priority(#interrupt_path, priority);
            }
        } else {
            quote! {
                nvic.set_priority(#interrupt_path, self.priority);
            }
        };

        let defs = quote! {
            pub struct Handle {
                priority: u8,
            }
            const _ASSERT_NVIC_IRQ: () = ::cortex_m_interrupt::assert_is_nvic_interrupt(#interrupt_path);
        };

        let pre_write = quote! {
            use  ::cortex_m_interrupt::cortex_m::interrupt::InterruptNumber;
            ::cortex_m_interrupt::cortex_m::peripheral::NVIC::mask(#interrupt_path);
        };

        let post_write = quote! {
            ::cortex_m_interrupt::cortex_m::interrupt::free(|_| {
                unsafe {
                    let mut nvic: ::cortex_m_interrupt::cortex_m::peripheral::NVIC = core::mem::transmute(());
                    #set_priority
                    ::cortex_m_interrupt::cortex_m::peripheral::NVIC::unmask(#interrupt_path);
                }
            });
        };

        let return_value = quote! {
            Handle {
                priority: #priority,
            }
        };

        let interrupt_impl = quote! {
            #interrupt_path
        };

        handle_generator(
            interrupt_export_name,
            interrupt_path.clone(),
            defs,
            pre_write,
            post_write,
            interrupt_impl,
            return_value,
        )
        .into()
    }
}
