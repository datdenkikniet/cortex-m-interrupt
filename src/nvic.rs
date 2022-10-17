use cortex_m::{interrupt::InterruptNumber, peripheral::NVIC};

use crate::InterruptHandle;

/// An interrupt handle bound to an [`NVIC`] interrupt.
///
/// The proc-macro [`crate::take_nvic_interrupt`] can be used to create
/// an implementor of this trait.
pub trait NvicInterruptHandle: InterruptHandle {
    type InterruptNumber: InterruptNumber;

    fn number(&self) -> Self::InterruptNumber;
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
#[doc_cfg::doc_cfg(feature = "unstable")]
pub unsafe fn determine_prio_bits<T: InterruptNumber>(
    nvic: &mut NVIC,
    placeholder_interrupt: T,
) -> u8 {
    nvic.set_priority(placeholder_interrupt, 0xFF);
    let written_prio = NVIC::get_priority(placeholder_interrupt);

    let prio_bits = written_prio.leading_ones();

    prio_bits as u8
}

/// Convert a logical priority (where higher priority number = higher priority level) to
/// a hardware priority level (where lower priority number = higher priority level).
///
/// `None` is returned if the priority `logical` is greater than the amount of priority
/// levels supported by an NVIC with `nvic_prio_bits`, i.e. `logical > (1 << nvic_prio_bits)`.
///
/// Taken from [`cortex_m_rtic`]
///
/// See RTIC-LICENSE-MIT for the license.
///
/// [`cortex_m_rtic`]: https://crates.io/crates/cortex-m-rtic
#[doc_cfg::doc_cfg(feature = "unstable")]
#[inline]
#[must_use]
pub fn logical2hw(logical: core::num::NonZeroU8, nvic_prio_bits: u8) -> Option<u8> {
    if logical.get() <= 1 << nvic_prio_bits {
        Some(((1u8 << nvic_prio_bits) - logical.get()) << (8 - nvic_prio_bits))
    } else {
        None
    }
}

#[cfg(test)]
#[test]
fn test() {
    for i in 1..=24 {
        if i <= 16 {
            // Verify that we compute the correct priority
            // for all "valid" values.
            assert_eq!(
                logical2hw(core::num::NonZeroU8::new(i).unwrap(), 4),
                Some(((1u8 << 4) - i) << (8 - 4))
            );
        } else {
            // Verify that no priority is returned if it is outside of the
            // priority range
            assert_eq!(logical2hw(core::num::NonZeroU8::new(i).unwrap(), 4), None);
        }
    }
}
