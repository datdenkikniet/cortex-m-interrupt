#![no_std]

// Re-export this path
pub use cortex_m_rt::DefaultHandler_;

pub use cortex_m_interrupt_macro::take;

/// A handle that can be used to register a handler for an interrupt.
///
/// Creating an implementor of [`IrqHandle`] can be done using the [`take`] and
/// [`take_raw_prio`] macros. [`take`] is only available with the feature `rtic-priority`.
pub trait InterruptHandle {
    /// Register the interrupt handler for this [`IrqHandle`]
    ///
    /// # Safety
    /// TODO: safety docs
    unsafe fn register(self, f: fn());
}
