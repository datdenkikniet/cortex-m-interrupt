#![no_main]
#![no_std]

use cortex_m_interrupt::IrqHandle;

fn _test() {
    let handle_raw = cortex_m_interrupt::take_raw_prio!(stm32f1xx_hal::pac, EXTI9_5, 0b11110000);

    handle_raw.register(|| panic!("Oo, raw priorities!"));
}
