use crate::InterruptHandle;

/// A handle that can be used to register a handler for an interrupt caused by an exception.
///
/// The proc-macro [`take_exception`] should be used to create an implementor of this trait.
///
/// [`take_exception`]: super::take_exception
pub trait ExceptionHandle: InterruptHandle {
    const EXCEPTION: crate::cortex_m::peripheral::scb::Exception;

    /// The [`Exception`] that this `ExceptionHandle` registers.
    ///
    /// [`Exception`]: cortex_m::peripheral::scb::Exception
    fn exception(&self) -> crate::cortex_m::peripheral::scb::Exception {
        Self::EXCEPTION
    }
}
