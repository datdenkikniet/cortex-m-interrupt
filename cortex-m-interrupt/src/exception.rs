use crate::InterruptRegistration;

/// A handle that can be used to configure the occupation of an interrupt caused by an exception.
///
/// The proc-macro [`take_exception`] should be used to create an implementor of this trait.
///
/// [`take_exception`]: super::take_exception
pub trait ExceptionRegistration: InterruptRegistration {
    const EXCEPTION: crate::cortex_m::peripheral::scb::Exception;

    /// The [`Exception`] that this [`ExceptionRegistration`] is associated with.
    ///
    /// [`Exception`]: cortex_m::peripheral::scb::Exception
    fn exception(&self) -> crate::cortex_m::peripheral::scb::Exception {
        Self::EXCEPTION
    }
}
