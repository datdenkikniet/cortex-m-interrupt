#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::EventHandle;

fn _test() {
    let handle_raw = cortex_m_interrupt::take!(EXTI9_5);
    let handle_raw_exception = cortex_m_interrupt::take!(SysTick);

    handle_raw.register(|| panic!("Oo, raw priorities!"));
    handle_raw_exception.register(|| panic!("In SysTick"));
}
