use proc_macro2::TokenStream;
use proc_macro_error::proc_macro_error;

mod take;
use syn::LitStr;
use take::Take;

fn is_exception(name: &str) -> bool {
    match name {
        "HardFault" | "NonMaskableInt" | "MemoryManagement" | "BusFault" | "UsageFault"
        | "SecureFault" | "SVCall" | "DebugMonitor" | "PendSV" | "SysTick" => true,
        _ => false,
    }
}

/// Register an `IrqHandle` to the interrupt specified by `interrupt` with logical priority `priority`.
///
/// Usage:
///
/// ```rust,no_compile
/// use cortex_m_interrupt::take;
///
/// // The value returned by `take` will always `impl cortex_m_interrupt::IrqHandle`.
/// let irq_handle = take!(interrupt, priority);
///
///
/// // For example:
/// let handle = cortex_m_interrupt::take!(stm32f1xx_hal::pac::interrupt::EXTI15_10, 7);
/// ```
///
/// A logical priority with a lower value has a lower priority level. This means that the logical priority
/// 0 has the lowest priority level, while logical priority `2^N` (where `N = available priority bits on platform`)
/// has the highest priority.
///
/// The macro calculates the amount of priority bits available on the platform at runtime.
///
/// If you wish to use a raw priority value, and/or want to avoid the runtiem calculation of the amount
/// of available priority bits, the `take_raw_prio` proc-macro can be used instead.
#[proc_macro]
#[proc_macro_error]
pub fn take(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as Take).build(true)
}

/// Register an `IrqHandle` to the interrupt specified by `interrupt` with raw priority `priority`.
///
/// Usage:
///
/// ```rust,no_compile
/// use cortex_m_interrupt::take;
/// // The value returned by `take_raw_prio` will always `impl cortex_m_interrupt::IrqHandle`.
/// let irq_handle = take_raw_prio!(interrupt, priority);
///
/// // For example
/// let handle = cortex_m_interrupt::take_raw_prio!(stm32f1xx_hal::pac::interrupt::EXTI15_10, 254);
/// ```
///
/// The `priority` is not interpreted and written directly to the NVIC priority register.
#[proc_macro]
#[proc_macro_error]
pub fn take_raw_prio(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as Take).build(false)
}

pub(crate) fn handle_generator(
    interrupt_export_name: LitStr,
    defs: TokenStream,
    pre_write: TokenStream,
    post_write: TokenStream,
    return_value: TokenStream,
) -> TokenStream {
    quote::quote! {
        {
            #defs

            static mut HANDLER: core::mem::MaybeUninit<fn()> = core::mem::MaybeUninit::uninit();

            #[export_name = #interrupt_export_name]
            pub unsafe extern "C" fn isr() {
                (HANDLER.assume_init())();
            }

           impl ::cortex_m_interrupt::IrqHandle for Handle {
                fn register(self, f: fn()) {
                    #pre_write
                    unsafe {
                        HANDLER.write(f);
                    }

                    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);

                    #post_write
                }
            }

            #return_value
        }
    }
}
