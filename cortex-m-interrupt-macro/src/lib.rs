use proc_macro_error::proc_macro_error;

mod take;
use take::Take;
use take_nvic_interrupt::TakeNvicInterrupt;

mod take_nvic_interrupt;

#[proc_macro]
#[proc_macro_error]
pub fn take(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as Take).build()
}

/// Create an `NvicInterruptHandle` bound to the interrupt specified by `interrupt` with logical priority `priority`.
///
/// Usage:
///
/// ```rust,no_compile
/// use cortex_m_interrupt::take;
///
/// // The value returned by `take_nvic_interrupt` will always `impl cortex_m_interrupt::NvicInterruptHandle`.
/// let irq_handle = take_nvic_interrupt!(interrupt, priority);
///
///
/// // For example:
/// let handle = cortex_m_interrupt::take_nvic_interrupt!(stm32f1xx_hal::pac::interrupt::EXTI15_10, 7);
/// ```
///
/// A logical priority with a lower value has a lower priority level. This means that the logical priority
/// 0 has the lowest priority level, while logical priority `2^N` (where `N = available priority bits on platform`)
/// has the highest priority.
///
/// The macro calculates the amount of priority bits available on the platform at runtime.
#[proc_macro]
#[proc_macro_error]
pub fn take_nvic_interrupt(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    syn::parse_macro_input!(input as TakeNvicInterrupt).build(true)
}
