#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::EventHandle;

fn _test() {
    let handle_raw = cortex_m_interrupt::take!(stm32f1xx_hal::pac::interrupt::EXTI9_5);

    handle_raw.register(|| panic!("Oo, raw priorities!"));
}
