#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::{take_exception, InterruptHandle};

fn _test() {
    let mut handle = take_exception!(SysTick);

    handle.register(|| panic!("In SysTick interrupt"));
}

#[cortex_m_rt::entry]
fn main() -> ! {
    _test();
    loop {}
}
