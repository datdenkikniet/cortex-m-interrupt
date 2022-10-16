#![no_std]

// Re-exports
pub use cortex_m;
pub use cortex_m_rt::DefaultHandler_;

pub use cortex_m_interrupt_macro::{take, take_nvic_interrupt};

mod nvic;
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
