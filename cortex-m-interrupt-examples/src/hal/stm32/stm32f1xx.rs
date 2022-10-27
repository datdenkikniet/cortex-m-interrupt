use core::{
    future::Future,
    pin::Pin,
    task::{self, Poll},
};

use crate::hal::{WakerProducer, WakerQueue};

use crate::hal::WakerConsumer;

use cortex_m_interrupt::NvicInterruptRegistration;

use stm32f1xx_hal::{
    flash::FlashExt,
    gpio::{self, ExtiPin, Floating, GpioExt, Input, CRL},
    pac::EXTI,
    rcc::RccExt,
};

pub use stm32f1xx_hal::pac::Interrupt;

pub type InputPin<CR, const P: char, const N: u8> = gpio::Pin<Input<Floating>, CR, P, N>;

/// Sets up clocks and PC1 in Floating Input mode
pub fn hw_setup() -> InputPin<CRL, 'C', 1> {
    let p = stm32f1xx_hal::pac::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let _clocks = p.RCC.constrain().cfgr.freeze(&mut flash.acr);

    let mut gpioc = p.GPIOC.split();

    let pin = gpioc.pc1.into_floating_input(&mut gpioc.crl);

    pin
}

pub struct AsyncExtiPin<CR, const P: char, const N: u8> {
    send_waker: WakerProducer<'static>,
    irq: InputPin<CR, P, N>,
}

impl<CR, const P: char, const N: u8> AsyncExtiPin<CR, P, N> {
    pub fn new<T: NvicInterruptRegistration<stm32f1xx_hal::pac::Interrupt>>(
        waker_queue: &'static mut WakerQueue,
        registration: T,
        irq: InputPin<CR, P, N>,
    ) -> AsyncExtiPin<CR, P, N> {
        use core::mem::MaybeUninit;
        static mut WAKER: MaybeUninit<WakerConsumer<'static>> = MaybeUninit::uninit();

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

        Self { send_waker: w, irq }
    }
}

impl<CR, const P: char, const N: u8> Future for AsyncExtiPin<CR, P, N>
where
    CR: Unpin,
{
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
