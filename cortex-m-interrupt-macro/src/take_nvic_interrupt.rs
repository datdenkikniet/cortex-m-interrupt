use proc_macro_error::{abort, ResultExt};
use quote::quote;
use syn::{
    parse::Parse,
    punctuated::Punctuated,
    token::{Colon2, Comma},
    LitInt, Path, PathSegment,
};

pub struct TakeNvicInterrupt {
    interrupt_path: Path,
    priority: LitInt,
}

impl Parse for TakeNvicInterrupt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let interrupt_path = input.parse()?;
        let _ = input.parse::<Comma>()?;
        let priority = input.parse()?;
        Ok(Self {
            interrupt_path,
            priority,
        })
    }
}

impl TakeNvicInterrupt {
    pub fn build(&self, use_logical_priority: bool) -> proc_macro::TokenStream {
        let Self {
            interrupt_path,
            priority,
        } = self;

        let int_path_len = interrupt_path.segments.len();
        let interrupt_ident = interrupt_path.segments.last().unwrap();

        if int_path_len <= 1 {
            abort!(
                interrupt_path,
                "The interrupt must be specified with a least 2 path segments. For example: `Interrupt::{}`",
                interrupt_ident.ident.to_string()
            )
        }

        let interrupt_type = self.interrupt_path.segments.iter().take(int_path_len - 1);
        let interrupt_type: Punctuated<PathSegment, Colon2> =
            interrupt_type.map(|s| s.clone()).collect();

        let prio_value: u32 = priority.base10_parse().unwrap_or_abort();
        if prio_value == 0 {
            abort!(priority, "Priority must be 1 or greater.");
        }

        let set_priority = if use_logical_priority {
            quote! {
                 let prio_bits = ::cortex_m_interrupt::determine_prio_bits(&mut nvic, #interrupt_path);
                 let priority = ::cortex_m_interrupt::logical2hw(self.priority, prio_bits);

                 if let Some(priority) = priority {
                    nvic.set_priority(#interrupt_path, priority);
                 } else {
                    panic!(stringify!(Unsupported priority level #priority was used for interrupt #interrupt_path));
                 }
            }
        } else {
            quote! {
                nvic.set_priority(#interrupt_path, self.priority);
            }
        };

        let take_interrupt = crate::Take::new(interrupt_ident.ident.clone()).build();

        quote! {{
            struct NvicInterruptRegistration {
                priority: core::num::NonZeroU8,
            }

            impl ::cortex_m_interrupt::InterruptRegistration for NvicInterruptRegistration {
                #[inline(always)]
                fn occupy(self, f: fn()) {
                    use ::cortex_m_interrupt::InterruptRegistration;

                    ::cortex_m_interrupt::cortex_m::interrupt::free(|_| unsafe {
                        let registration = #take_interrupt;

                        ::cortex_m_interrupt::cortex_m::peripheral::NVIC::mask(#interrupt_path);

                        registration.occupy(f);

                        let mut nvic: ::cortex_m_interrupt::cortex_m::peripheral::NVIC = unsafe { core::mem::transmute(()) };
                        #set_priority
                        ::cortex_m_interrupt::cortex_m::peripheral::NVIC::unmask(#interrupt_path);
                    })

                }
            }

            impl ::cortex_m_interrupt::NvicInterruptRegistration<#interrupt_type> for NvicInterruptRegistration {
                const INTERRUPT_NUMBER: #interrupt_type = #interrupt_path;
            }

            NvicInterruptRegistration {
                // Note(unwrap): the macro verifies that `#priority` is not 0.
                priority: core::num::NonZeroU8::new(#priority).unwrap(),
            }
        }}
        .into()
    }
}
