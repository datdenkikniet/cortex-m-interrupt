#![cfg_attr(feature = "unstable-doc-cfg", feature(doc_cfg))]
#![cfg_attr(target = "arm", no_std)]

// Re-exports
pub use cortex_m;
pub use cortex_m_rt::DefaultHandler_;

/// Create an [`InterruptHandle`] bound to the interrupt named by `interrupt.
///
/// Usage:
/// ```rust,no_compile
/// let handle = cortex_m_interrupt::take!("EXTI15_10");
/// ```
pub use cortex_m_interrupt_macro::take;

/// Create an [`NvicInterruptHandle`] bound to the interrupt specified by `interrupt` with logical priority `priority`.
///
/// `interrupt` must be specified with _at least_ 2 path segments. For instance, `Interrupt::EXTI15_10` (where `Interrupt` implements
/// [`InterruptNumber`]) is allowed, but `EXTI15_10` by itself, even if imported using `use Interrupt::EXTI15_10`, is not.
///
/// A logical priority with a lower value has a lower priority level. This means that the logical priority
/// `1` has the lowest priority level, while logical priority `2^N` (where `N = available priority bits on platform`)
/// has the highest priority level. A logical priority of `0` is not allowed, and a logical priority greater than `2^N` panics
/// at runtime.
///
/// The macro adds code that calculates the amount of priority bits available on the platform at runtime.
///
/// Usage:
///
/// ```rust,no_compile
/// use cortex_m_interrupt::take;
///
/// // The value returned by `take_nvic_interrupt` will
/// // always `impl cortex_m_interrupt::NvicInterruptHandle`.
/// let irq_handle = take_nvic_interrupt!(interrupt, priority);
///
///
/// // For example:
/// let handle = cortex_m_interrupt::take_nvic_interrupt!(stm32f1xx_hal::pac::interrupt::EXTI15_10, 7);
/// ```
///
/// [`InterruptNumber`]: cortex_m::interrupt::InterruptNumber
#[doc_cfg::doc_cfg(feature = "unstable")]
pub use cortex_m_interrupt_macro::take_nvic_interrupt;

#[cfg(feature = "unstable")]
mod nvic;
#[cfg(feature = "unstable")]
pub use nvic::*;

/// A handle that can be used to register a handler for an interrupt.
///
/// Creating an implementor of [`InterruptHandle`] can be done using the [`take`] macro.
pub trait InterruptHandle {
    /// Register the interrupt handler for this [`InterruptHandle`]
    ///
    /// # Safety
    /// TODO: safety docs
    unsafe fn register(self, f: fn());
}
