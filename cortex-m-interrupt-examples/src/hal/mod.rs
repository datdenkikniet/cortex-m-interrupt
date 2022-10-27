use core::task::Waker;

pub mod ssq;
use ssq::{Consumer, Producer, SingleSlotQueue};

#[cfg(any(
    feature = "stm32f1xx-hal",
    feature = "stm32f4xx-hal",
    feature = "stm32f7xx-hal"
))]
pub mod stm32;
#[cfg(any(
    feature = "stm32f1xx-hal",
    feature = "stm32f4xx-hal",
    feature = "stm32f7xx-hal"
))]
pub use stm32::*;

#[cfg(feature = "nrf52832-hal")]
pub mod nrf52832;
#[cfg(feature = "nrf52832-hal")]
pub use nrf52832::*;

pub type WakerQueue = SingleSlotQueue<Waker>;
pub type WakerProducer<'a> = Producer<'a, Waker>;
pub type WakerConsumer<'a> = Consumer<'a, Waker>;
