#![no_std]

// Re-exports
pub use cortex_m;
pub use cortex_m_rt::DefaultHandler_;

use core::task::Waker;

mod ssq;
use ssq::{Consumer, Producer, SingleSlotQueue};

pub use cortex_m_interrupt_macro::{take, take_nvic_interrupt};

mod nvic;
pub use nvic::*;

pub type WakerQueue = SingleSlotQueue<Waker>;
pub type WakerProducer<'a> = Producer<'a, Waker>;
pub type WakerConsumer<'a> = Consumer<'a, Waker>;

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
