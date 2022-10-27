pub mod gpiote;

use cortex_m_interrupt::NvicInterruptRegistration;

use gpiote::{Channel, Configured};

use core::{
    future::Future,
    task::{self, Poll},
};

use crate::hal::{WakerConsumer, WakerProducer, WakerQueue};

use nrf52832_hal::{
    gpio::{Floating, Input, Pin},
    gpiote::{EventPolarity, Gpiote, GpioteInputPin},
};

pub use nrf52832_hal::pac::Interrupt;

pub fn channel<T: GpioteInputPin>(pin: &T) -> Channel<4, Configured> {
    let p = unsafe { nrf52832_hal::pac::Peripherals::steal() };

    let gpiote = Gpiote::new(p.GPIOTE);
    let channel = gpiote::new(gpiote).4.configure(pin, EventPolarity::LoToHi);

    channel
}

pub fn hw_setup() -> Pin<Input<Floating>> {
    let p = unsafe { nrf52832_hal::pac::Peripherals::steal() };

    let p0 = nrf52832_hal::gpio::p0::Parts::new(p.P0);

    let pin = p0.p0_04.into_floating_input().degrade();
    pin
}

pub struct AsyncIrqPin<const N: usize> {
    send_waker: WakerProducer<'static>,
    irq: Channel<N, Configured>,
    pin: Pin<Input<Floating>>,
}

impl<const N: usize> AsyncIrqPin<N> {
    pub fn new<T: NvicInterruptRegistration<nrf52832_hal::pac::Interrupt>>(
        waker_queue: &'static mut WakerQueue,
        registration: T,
        irq: Channel<N, Configured>,
        pin: Pin<Input<Floating>>,
    ) -> Self {
        assert_eq!(registration.number(), Interrupt::GPIOTE);

        use core::mem::MaybeUninit;
        static mut WAKER: MaybeUninit<WakerConsumer<'static>> = MaybeUninit::uninit();

        let (r, w) = waker_queue.split();

        unsafe { WAKER = MaybeUninit::new(r) };

        registration.occupy(|| {
            unsafe {
                let mut gpiote_channel = Channel::<N, Configured>::conjure();
                gpiote_channel.disable_interrupt();
            }
            if let Some(waker) = unsafe { WAKER.assume_init_mut().dequeue() } {
                waker.wake();
            } else {
                // This error can occur after a `poll` that returned
                // `Poll::Ready`, in which case it is not a problem. More interrupts
                // is OK, fewer would not be.
            }
        });

        Self {
            send_waker: w,
            irq,
            pin,
        }
    }
}

impl<const N: usize> core::ops::Deref for AsyncIrqPin<N> {
    type Target = Pin<Input<Floating>>;

    fn deref(&self) -> &Self::Target {
        &self.pin
    }
}

impl<const N: usize> Future for AsyncIrqPin<N> {
    type Output = ();

    fn poll(mut self: core::pin::Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        self.irq.reset_events();

        if self.irq.is_event_triggered() {
            // Disable interrupt
            self.irq.disable_interrupt();

            Poll::Ready(())
        } else {
            self.send_waker.enqueue(cx.waker().clone());

            // Enable the interrupt
            self.irq.enable_interrupt();

            Poll::Pending
        }
    }
}
