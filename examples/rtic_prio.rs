#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::EventHandle;

use stm32f1xx_hal::pac::interrupt;

fn _test() {
    let handle = cortex_m_interrupt::take!(interrupt::EXTI15_10, 4);

    handle.register(|| panic!("Yhellow"));
}
