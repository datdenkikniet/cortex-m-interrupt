use proc_macro_error::abort;
use quote::quote;
use syn::{parse::Parse, spanned::Spanned, token::Comma, Error, Expr, LitStr, TypePath};

pub(crate) struct Take {
    interrupt_path: TypePath,
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

        let interrupt_export_name = if let Some(last_seg) = &interrupt_path.path.segments.last() {
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

        super::handle_generator(
            interrupt_export_name,
            defs,
            pre_write,
            post_write,
            return_value,
        )
        .into()
    }
}
