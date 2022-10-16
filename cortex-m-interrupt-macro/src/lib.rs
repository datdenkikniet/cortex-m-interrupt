use proc_macro_error::proc_macro_error;

mod take;
use take::Take;

/// Register an `EventHandle` to the interrupt specified by `interrupt`.
/// 
/// This works for any enum that implements `InterruptNumber`
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
    syn::parse_macro_input!(input as Take).build()
}
