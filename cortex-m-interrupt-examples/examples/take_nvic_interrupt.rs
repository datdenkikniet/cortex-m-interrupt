#![no_std]
#![no_main]

use cortex_m_interrupt_examples as hal;

use hal::WakerQueue;

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

static mut WAKER_STORAGE: WakerQueue = WakerQueue::new();

#[cortex_m_rt::entry]
fn main() -> ! {
    let pin = cortex_m_interrupt_examples::hw_setup();

    // NOTE(unsafe): this is the only place in which we take a (mutable) reference
    // to WAKER_STORAGE
    let storage = unsafe { &mut WAKER_STORAGE };

    #[cfg(any(
        feature = "stm32f1xx-hal",
        feature = "stm32f4xx-hal",
        feature = "stm32f7xx-hal"
    ))]
    let irq = {
        let handle = cortex_m_interrupt::take_nvic_interrupt!(hal::Interrupt::EXTI1, 4);
        let async_irq = hal::AsyncExtiPin::new(storage, handle, pin);
        async_irq
    };

    #[cfg(feature = "nrf52832-hal")]
    let irq = {
        let gpiote_channel = hal::channel(&pin);

        let handle = cortex_m_interrupt::take_nvic_interrupt!(hal::Interrupt::GPIOTE, 4);
        let async_irq = hal::AsyncIrqPin::new(storage, handle, gpiote_channel, pin);
        async_irq
    };

    async { irq.await };

    loop {}
}
