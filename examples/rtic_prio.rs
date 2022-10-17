#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::IrqHandle;

use cortex_m::peripheral::scb::Exception::SysTick;
use stm32f1xx_hal::pac::interrupt::EXTI15_10;

fn _test() {
    let handle = cortex_m_interrupt::take!(EXTI15_10, 4);
    let handle = cortex_m_interrupt::take!(SysTick, 4);

    handle.register(|| panic!("Yhellow"));
}
