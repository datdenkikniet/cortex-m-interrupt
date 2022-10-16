#![no_std]

#[cfg(target = "arm")]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::{take_nvic_interrupt, InterruptHandle};
use stm32f1xx_hal::device::Interrupt;

fn _test() {
    let handle = take_nvic_interrupt!(Interrupt::EXTI15_10, 5);

    unsafe {
        handle.register(|| panic!("In NVIC interrupt"));
    }
}

fn main() {}
