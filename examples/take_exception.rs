#![no_std]

#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::{take_exception, InterruptHandle};

fn _test() {
    let handle = take_exception!(SysTick);

    handle.register(|| panic!("In SysTick interrupt"));
}

fn main() {}
