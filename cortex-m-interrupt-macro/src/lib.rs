use proc_macro2::Literal;
use proc_macro_error::{abort, proc_macro_error};
use quote::quote;
use syn::{parse::Parse, token::Comma, Error, Expr, Lit, TypePath};

struct TakeInput {
    interrupt_path: TypePath,
    priority: Expr,
}

impl Parse for TakeInput {
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

fn build(input: proc_macro::TokenStream, use_rtic_prio: bool) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as TakeInput);

    let TakeInput {
        interrupt_path,
        priority,
    } = input;

    let interrupt_export_name = if let Some(last_seg) = interrupt_path.path.segments.last() {
        last_seg.ident.to_string()
    } else {
        abort!(interrupt_path, "Could not find last segment of type path.");
    };

    let interrupt_export_name = Lit::new(Literal::string(&interrupt_export_name));

    let set_priority = if use_rtic_prio {
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

    quote! {
        {
            static mut HANDLER: core::mem::MaybeUninit<fn()> = core::mem::MaybeUninit::uninit();

            #[export_name = #interrupt_export_name]
            pub unsafe extern "C" fn isr() {
                (HANDLER.assume_init())();
            }

            pub struct Handle {
                priority: u8,
            }

            impl ::cortex_m_interrupt::IrqHandle for Handle {
                fn register(self, f: fn()) {
                    use  ::cortex_m_interrupt::cortex_m::interrupt::InterruptNumber;

                    ::cortex_m_interrupt::cortex_m::peripheral::NVIC::mask(#interrupt_path);

                    unsafe {
                        HANDLER.write(f);
                    }

                    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);

                    ::cortex_m_interrupt::cortex_m::interrupt::free(|_| {
                        unsafe {
                            let mut nvic: ::cortex_m_interrupt::cortex_m::peripheral::NVIC = core::mem::transmute(());
                            #set_priority
                            ::cortex_m_interrupt::cortex_m::peripheral::NVIC::unmask(#interrupt_path);
                        }
                    });
                }
            }

            Handle {
                priority: #priority,
            }
        }
    }.into()
}

/// Register an `IrqHandle` to the interrupt specified by `interrupt` with RTIC priority `priority`.
///
/// Usage:
///
/// ```rust,no_compile
/// use cortex_m_interrupt::{take, IrqHandle};
/// let irq_handle: IrqHandle = take!(interrupt, priority);
///
/// // For example:
/// let handle = cortex_m_interrupt::take!(stm32f1xx_hal::pac::interrupt::EXTI15_10, 7);
/// ```
///
/// Important to note is that the `priority` is interpreted as RTIC priority, where
/// a higher priority indicates a higher priority level. If you wish to configure your
/// interrupt with a raw priority, see [`take_raw_prio!`].
#[proc_macro]
#[proc_macro_error]
pub fn take(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    build(input, true)
}

/// Register an `IrqHandle` to the interrupt specified by `interrupt` with raw priority `priority`.
///
/// Usage:
///
/// ```rust,no_compile
/// use cortex_m_interrupt::{take, IrqHandle};
/// let irq_handle: IrqHandle = take_raw_prio!(interrupt, priority);
///
/// // For example
/// let handle = cortex_m_interrupt::take_raw_prio!(stm32f1xx_hal::pac::interrupt::EXTI15_10, 254);
/// ```
///
/// The `priority` is not interpreted and written directly to the NVIC priority register.
#[proc_macro]
#[proc_macro_error]
pub fn take_raw_prio(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    build(input, false)
}
