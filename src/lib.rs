#![no_std]

// Re-export this path
pub use cortex_m;
use cortex_m::{interrupt::InterruptNumber, peripheral::NVIC};

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

/// A handle that can be used to register a handler for an interrupt.
///
/// Creating an implementor of [`IrqHandle`] can be done using the [`take`] and
/// [`take_raw_prio`] macros. [`take`] is only available with the feature `rtic-priority`.
pub trait IrqHandle {
    /// Register the interrupt handler for this [`IrqHandle`]
    fn register(self, f: fn());
}

/// Determine the amount of priority bits available on the current MCU.
///
/// This function determines the amount of priority bits available on a Cortex-M MCU by
/// setting the priority of an interrupt to the maximum value `0xFF`, and reading the resulting
/// priority.
///
/// The count of leading ones in the resulting value indicates the amount of
/// priority-level bits available, allowing us to calculate the amount of priority
/// levels supported by this NVIC.
///
/// After performing this calculation, the priority of the placeholder interrupt is restored.
///
/// It is guaranteed that all non-implemented priority bits will be read back as zero for any
/// NVIC implementation that conforms to the [GIC] (see section 3.5.1), which includes at least all
/// [armv7m] (see section B1.5.4) and [armv6m] (see section B3.4) cores.
///
/// # Safety
/// This function should only be called from a critical section.
///
/// [GIC]: https://documentation-service.arm.com/static/5f8ff196f86e16515cdbf969
/// [armv7m]: https://documentation-service.arm.com/static/606dc36485368c4c2b1bf62f
/// [armv6m]: https://documentation-service.arm.com/static/5f8ff05ef86e16515cdbf826
pub unsafe fn determine_prio_bits(nvic: &mut NVIC, placeholder_interrupt: u16) -> u8 {
    #[derive(Clone, Copy)]
    struct RawInterrupt(u16);
    unsafe impl InterruptNumber for RawInterrupt {
        fn number(self) -> u16 {
            self.0
        }
    }

    let interrupt = RawInterrupt(placeholder_interrupt);

    let current_prio = NVIC::get_priority(interrupt);

    nvic.set_priority(interrupt, 0xFF);
    let written_prio = NVIC::get_priority(interrupt);
    nvic.set_priority(interrupt, current_prio);

    let prio_bits = written_prio.leading_ones();

    written_prio >> (8 - prio_bits)
}
