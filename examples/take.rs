#![no_std]

#[cfg(target = "arm")]
#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

use cortex_m_interrupt::InterruptHandle;

fn _test() {
    let handle_raw = cortex_m_interrupt::take!(EXTI9_5);
    let handle_raw_exception = cortex_m_interrupt::take!(SysTick);

    unsafe {
        handle_raw.register(|| {
            panic!("In normal interrupt. (Won't ever trigger if NVIC is not configured)")
        });
        handle_raw_exception.register(|| panic!("In SysTick"));
    }
}

fn main() {}
