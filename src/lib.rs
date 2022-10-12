#![no_std]

// Re-export this path
pub use cortex_m;

use core::task::Waker;

mod ssq;
use ssq::{Consumer, Producer, SingleSlotQueue};

pub use cortex_m_interrupt_macro::take_raw_prio;

#[cfg(feature = "rtic-priority")]
pub use cortex_m_interrupt_macro::take;
#[cfg(feature = "rtic-priority")]
pub use rtic::export::logical2hw;

pub type WakerQueue = SingleSlotQueue<Waker>;
pub type WakerProducer<'a> = Producer<'a, Waker>;
pub type WakerConsumer<'a> = Consumer<'a, Waker>;

pub trait IrqHandle {
    fn register(self, f: fn());
}
