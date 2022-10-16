#![no_main]
#![no_std]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::IrqHandle;

fn _test() {
    cortex_m::peripheral::scb::Exception::SysTick;
    let handle_raw =
        cortex_m_interrupt::take_exception!(cortex_m::peripheral::scb::Exception::SysTick);

    handle_raw.register(|| panic!("Oo, raw priorities!"));
}
