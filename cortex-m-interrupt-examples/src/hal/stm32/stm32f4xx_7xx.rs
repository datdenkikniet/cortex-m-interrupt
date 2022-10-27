use core::{
    future::Future,
    pin::Pin,
    task::{self, Poll},
};

use crate::hal::{WakerProducer, WakerQueue};

use crate::hal::WakerConsumer;

use cortex_m_interrupt::NvicInterruptRegistration;

#[cfg(feature = "stm32f4xx-hal")]
use stm32f4xx_hal as hal;

#[cfg(feature = "stm32f7xx-hal")]
use stm32f7xx_hal as hal;

use hal::{
    gpio::{self, ExtiPin, GpioExt, Input},
    pac::EXTI,
    rcc::RccExt,
};

pub use hal::pac::Interrupt;

/// Sets up clocks and PC1 in Floating Input mode
pub fn hw_setup() -> gpio::Pin<'C', 1, Input> {
    let p = hal::pac::Peripherals::take().unwrap();
    let _clocks = p.RCC.constrain().cfgr.freeze();

    let gpioc = p.GPIOC.split();

    gpioc.pc1.into_floating_input()
}

pub struct AsyncExtiPin<const P: char, const N: u8> {
    send_waker: WakerProducer<'static>,
    irq: gpio::Pin<P, N, Input>,
}

impl<const P: char, const N: u8> AsyncExtiPin<P, N> {
    pub fn new<T: NvicInterruptRegistration<Interrupt>>(
        waker_queue: &'static mut WakerQueue,
        registration: T,
        irq: gpio::Pin<P, N, Input>,
    ) -> Self {
        macro_rules! num_int_map {
            ($($start:literal..=$end:literal => $int:ident$(,)?)*) => {
                match N {
                    $(
                        $start..=$end => assert_eq!(registration.number(), crate::Interrupt::$int),
                    )*
                    _ => unreachable!(),
                }
            };
        }

        num_int_map!(
            0..=0 => EXTI0,
            1..=1 => EXTI1,
            2..=2 => EXTI2,
            3..=3 => EXTI3,
            4..=4 => EXTI4,
            5..=9 => EXTI9_5,
            10..=15 => EXTI15_10,
        );

        use core::mem::MaybeUninit;
        static mut WAKER: MaybeUninit<WakerConsumer<'static>> = MaybeUninit::uninit();

        let (r, w) = waker_queue.split();

        unsafe { WAKER = MaybeUninit::new(r) };

        registration.occupy(|| {
            cortex_m::interrupt::free(|_| {
                let exti = unsafe { &*EXTI::ptr() };

                // Disable interrupt
                exti.imr
                    .modify(|r, w| unsafe { w.bits(r.bits() & !(1 << N)) });
            });

            if let Some(waker) = unsafe { WAKER.assume_init_mut().dequeue() } {
                waker.wake();
            }
        });

        Self { irq, send_waker: w }
    }
}

impl<const P: char, const N: u8> Future for AsyncExtiPin<P, N> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        let exti = unsafe { &*EXTI::ptr() };

        if self.irq.check_interrupt() {
            self.irq.clear_interrupt_pending_bit();

            Poll::Ready(())
        } else {
            self.send_waker.enqueue(cx.waker().clone());

            cortex_m::interrupt::free(|_| {
                // Enable the interrupt
                exti.imr.modify(|r, w| unsafe { w.bits(r.bits() | 1 << N) });
            });

            Poll::Pending
        }
    }
}
