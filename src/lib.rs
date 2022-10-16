#![no_std]

// Re-export this path
pub use cortex_m;
use cortex_m::{interrupt::InterruptNumber, peripheral::NVIC};

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
pub trait EventHandle {
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
/// priority-level bits available.
///
/// After performing this calculation, the priority of the placeholder interrupt is **not** restored.
///
/// It is guaranteed that all non-implemented priority bits will be read back as zero for any
/// NVIC implementation that conforms to the [GIC] (see section 3.5.1), which includes at least all
/// [armv7m] (see section B1.5.4) and [armv6m] (see section B3.4) cores.
///
/// # Safety
/// This function should only be called from a critical section, as it alters the priority
/// of an interrupt.
///
/// The caller must restore the priority of `placeholder_interrupt` to a known-and-valid priority after
/// calling this function, as [`determine_prio_bits`] does not restore the overwritten priority.
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

    nvic.set_priority(interrupt, 0xFF);
    let written_prio = NVIC::get_priority(interrupt);

    let prio_bits = written_prio.leading_ones();

    prio_bits as u8
}

/// Convert a logical priority (where higher priority number = higher priority level) to
/// a hardware priority level (where lower priority number = higher priority level).
///
/// Taken from [`cortex_m_rtic`]
///
/// See RTIC-LICENSE-MIT for the license.
///
/// [`cortex_m_rtic`]: https://crates.io/crates/cortex-m-rtic
#[inline]
#[must_use]
pub fn logical2hw(logical: u8, nvic_prio_bits: u8) -> u8 {
    ((1 << nvic_prio_bits) - logical) << (8 - nvic_prio_bits)
}

/// Assert that `ISR` is an NVIC-servicable interrupt
#[inline(always)]
pub const fn assert_is_nvic_interrupt<ISR>(_: ISR)
where
    ISR: InterruptNumber,
{
}
