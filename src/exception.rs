use crate::InterruptHandle;

/// A handle that can be used to register a handler for an interrupt caused by an exception.
///
/// Creating an implementor of [`ExceptionHandle`] can be done using the [`take_exception`] macro.
pub trait ExceptionHandle: InterruptHandle {
    /// The [`Exception`] that this `ExceptionHandle` registers.
    ///
    /// [`Exception`]: cortex_m::peripheral::scb::Exception
    fn exception(&self) -> crate::cortex_m::peripheral::scb::Exception;
}
