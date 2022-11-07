#![doc = include_str!("../README.md")]
#![cfg_attr(feature = "unstable-doc-cfg", feature(doc_cfg))]
#![cfg_attr(not(test), no_std)]

// /// Return an instance of an unnameable struct that implements [`NvicInterruptRegistration`], which
// /// is bound to the interrupt specified by `interrupt` with logical priority `priority`.
// ///
// /// `interrupt` must name an enum variant of an enum that implements [`InterruptNumber`] with _at least_ 2 path segments.
// ///
// /// For instance, `Interrupt::EXTI15_10` (where `Interrupt` implements [`InterruptNumber`]) is allowed,
// /// but `EXTI15_10` by itself, even if imported using `use Interrupt::EXTI15_10`, is not.
// ///
// /// The returned struct has the following features:
// /// * Calling `register` more than once for the same `Interrupt` panics.
// /// * The bound interrupt will be masked in the NVIC before configuring the occupation of the registration, and
// /// unmasked after.
// /// * The the amount of available NVIC priority bits is determined runtime.
// ///
// /// # Logical priority
// ///
// /// A logical priority with a lower value has a lower priority level. This means that the logical priority
// /// `1` has the lowest priority level, while logical priority `2^N` (where `N = <available priority bits on platform>`)
// /// has the highest priority level. A logical priority of `0` is not allowed, and a logical priority greater than `2^N` panics
// /// at runtime.
// ///
// /// # Usage
// ///
// /// ```rust,ignore
// /// use cortex_m_interrupt::take_nvic_interrupt;
// ///
// /// // The value returned by `take_nvic_interrupt` will
// /// // always `impl cortex_m_interrupt::NvicInterruptRegistration`.
// /// let registration = take_nvic_interrupt!(interrupt, priority);
// ///
// /// ```
// ///
// /// ```rust,no_run
// /// // For example, using stm32f1xx hal:
// /// use stm32f1xx_hal::pac::interrupt;
// /// let registration = cortex_m_interrupt::take_nvic_interrupt!(interrupt::EXTI15_10, 7);
// /// ```
// ///
// /// [`InterruptNumber`]: cortex_m::interrupt::InterruptNumber
// /// [`Interrupt::EXTI15_10`]: stm32f1xx_hal::pac::interrupt::EXTI15_10
// pub use cortex_m_interrupt_macro::take_nvic_interrupt;

/// Return an instance of an unnameable struct that implements [`ExceptionRegistration`], which
/// is bound to the exception specified by `exception`.
///
/// `exception` may be any of the variants of [`Exception`] (from [`cortex_m`]), except
/// for [`Exception::HardFault`]
///
/// The returned struct has the following features:
/// * Calling `register` more than once for the same [`Exception`] panics.
///
/// # Usage
///
/// ```rust,ignore
/// use cortex_m_interrupt::take_exception;
///
/// // The value returned by `take_exception` will
/// // always `impl cortex_m_interrupt::ExceptionRegistration`.
/// let registration = take_exception!(exception);
/// ```
///
/// ```rust,no_run
/// // For example:
/// let registration = cortex_m_interrupt::take_exception!(SysTick);
/// ```
///
/// [`Exception`]: cortex_m::peripheral::scb::Exception
/// [`Exception::HardFault`]: cortex_m::peripheral::scb::Exception::HardFault
pub use cortex_m_interrupt_macro::register_interrupt;

// mod exception;
// pub use exception::ExceptionRegistration;
//
// mod nvic;
// pub use nvic::{determine_prio_bits, logical2hw, NvicInterruptRegistration};

/// This trait is implemented by the HAL.
pub trait InterruptRegistration<Vector> {
    const VECTOR: Vector; // Holds vector name for compiletime errors

    fn on_interrupt();
}

/// This trait is implemented by the proc-macro.
pub unsafe trait InterruptToken<Periperhal> {}
