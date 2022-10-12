#![no_main]
#![no_std]

use cortex_m_interrupt::IrqHandle;

fn _test() {
    let handle = cortex_m_interrupt::take!(stm32f1xx_hal::pac, EXTI15_10, 4);

    handle.register(|| panic!("Yhellow"));
}
