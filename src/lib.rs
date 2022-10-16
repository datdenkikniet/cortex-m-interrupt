#![no_std]

// Re-export this path
pub use cortex_m_rt::DefaultHandler_;

use core::task::Waker;

mod ssq;
use ssq::{Consumer, Producer, SingleSlotQueue};

pub use cortex_m_interrupt_macro::take;

pub type WakerQueue = SingleSlotQueue<Waker>;
pub type WakerProducer<'a> = Producer<'a, Waker>;
pub type WakerConsumer<'a> = Consumer<'a, Waker>;

/// A handle that can be used to register a handler for an interrupt.
///
/// Creating an implementor of [`IrqHandle`] can be done using the [`take`] and
/// [`take_raw_prio`] macros. [`take`] is only available with the feature `rtic-priority`.
pub trait InterruptHandle {
    /// Register the interrupt handler for this [`IrqHandle`]
    fn register(self, f: fn());
}
