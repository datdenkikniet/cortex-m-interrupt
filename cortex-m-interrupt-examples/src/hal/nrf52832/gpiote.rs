//! A GPIOTE driver with owning semantics.
//! 
//! Written by korken89

use core::marker::PhantomData;

use nrf52832_hal::{
    gpiote::{EventPolarity, Gpiote, GpioteInputPin},
    pac::GPIOTE,
};

/// Uninit state for a gpiote channel
pub struct Uninit;
/// Configured state for a gpiote channel
pub struct Configured;

/// Convert the Gpiote to its channels
pub fn new(
    _gpiote: Gpiote,
) -> (
    Channel<0, Uninit>,
    Channel<1, Uninit>,
    Channel<2, Uninit>,
    Channel<3, Uninit>,
    Channel<4, Uninit>,
    Channel<5, Uninit>,
    Channel<6, Uninit>,
    Channel<7, Uninit>,
) {
    (
        Channel { _pd: PhantomData },
        Channel { _pd: PhantomData },
        Channel { _pd: PhantomData },
        Channel { _pd: PhantomData },
        Channel { _pd: PhantomData },
        Channel { _pd: PhantomData },
        Channel { _pd: PhantomData },
        Channel { _pd: PhantomData },
    )
}

/// A Gpiote channel
#[derive(Clone)]
pub struct Channel<const CH: usize, T> {
    _pd: PhantomData<T>,
}

impl<const CH: usize> Channel<CH, Uninit> {
    /// Configure a channel
    pub fn configure<P>(self, input_pin: &P, trigger_mode: EventPolarity) -> Channel<CH, Configured>
    where
        P: GpioteInputPin,
    {
        let gpiote = unsafe { &*GPIOTE::ptr() };

        gpiote.config[CH].write(|w| {
            match trigger_mode {
                EventPolarity::HiToLo => w.mode().event().polarity().hi_to_lo(),
                EventPolarity::LoToHi => w.mode().event().polarity().lo_to_hi(),
                EventPolarity::None => w.mode().event().polarity().none(),
                EventPolarity::Toggle => w.mode().event().polarity().toggle(),
            };

            unsafe { w.psel().bits(input_pin.pin()) }
        });

        // TODO: Is port event needed?
        // gpiote.intenset.write(|w| w.port().set());

        Channel { _pd: PhantomData }
    }
}

impl<const CH: usize> Channel<CH, Configured> {
    /// Conjure an configured GPIOTE channel from nothing.
    /// Assumes that the port is configured.
    pub unsafe fn conjure() -> Self {
        Self { _pd: PhantomData }
    }

    /// Enable interrupts from this channel.
    pub fn enable_interrupt(&mut self) {
        unsafe {
            let gpiote = &*GPIOTE::ptr();
            gpiote.intenset.write(|w| w.bits(1 << CH));
        }
    }

    /// Disable interrupts from this channel.
    pub fn disable_interrupt(&mut self) {
        unsafe {
            let gpiote = &*GPIOTE::ptr();
            gpiote.intenclr.write(|w| w.bits(1 << CH));
        }
    }

    /// Checks if the event is triggered on this channel.
    pub fn is_event_triggered(&self) -> bool {
        unsafe {
            let gpiote = &*GPIOTE::ptr();
            gpiote.events_in[CH].read().bits() != 0
        }
    }

    /// Resets the event on this channel.
    pub fn reset_events(&mut self) {
        unsafe {
            let gpiote = &*GPIOTE::ptr();
            gpiote.events_in[CH].write(|w| w);
        }
    }
}
