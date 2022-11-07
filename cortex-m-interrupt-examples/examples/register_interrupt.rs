#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Connect to a peripheral with 1:1 IRQ and driver relationship
cortex_m_interrupt::register_interrupt!(SpiToken, hal::pac::Interrupt::Spi0 -> hal::Spi);

// Connect to a peripheral with 1:2 IRQ and driver relationship
cortex_m_interrupt::register_interrupt!(Uart01Token, hal::pac::Interrupt::Uart0_1 -> hal::Uart0, hal::Uart1);

#[cortex_m_rt::entry]
fn main() -> ! {
    loop {}
}

//
// HAL impl
//
// This takes an interrupt handle and checks that the correct
// handler was registered.
//

pub mod pac {
    pub struct SPI0;

    pub struct UART0;

    pub struct UART1;

    pub struct UART2;

    pub enum Interrupt {
        Int1,
        Int2,
        Int3,
        Spi0,
        Uart0_1,
        Uart2,
    }
}

pub mod hal {
    use cortex_m_interrupt::{InterruptRegistration, InterruptToken};

    pub use super::pac;

    pub struct Spi {
        // ...
    }

    impl Spi {
        pub fn new<Handle>(spi: pac::SPI0, interrupt_handle: Handle) -> Self
        where
            Handle: InterruptToken<Spi>,
        {
            Spi {}
        }
    }

    impl InterruptRegistration<pac::Interrupt> for Spi {
        const VECTOR: pac::Interrupt = pac::Interrupt::Spi0;

        // It might have a dependency that you can't call `handle.activate()`
        // until peripheral setup is complete.
        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    //
    // ---
    //

    pub struct Uart0 {}

    impl Uart0 {
        pub fn new<Handle>(uart: pac::UART0, interrupt_handle: Handle) -> Self
        where
            Handle: InterruptToken<Uart0>,
        {
            Uart0 {}
        }
    }

    impl InterruptRegistration<pac::Interrupt> for Uart0 {
        const VECTOR: pac::Interrupt = pac::Interrupt::Uart0_1;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    pub struct Uart1 {}

    impl Uart1 {
        pub fn new<Handle>(uart: pac::UART1, interrupt_handle: Handle) -> Self
        where
            Handle: InterruptToken<Uart1>,
        {
            Uart1 {}
        }
    }

    impl InterruptRegistration<pac::Interrupt> for Uart1 {
        const VECTOR: pac::Interrupt = pac::Interrupt::Uart0_1;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    pub struct Uart2 {}

    impl Uart2 {
        pub fn new<Handle>(uart: pac::UART2, interrupt_handle: Handle) -> Self
        where
            Handle: InterruptToken<Uart2>,
        {
            Uart2 {}
        }
    }

    impl InterruptRegistration<pac::Interrupt> for Uart2 {
        const VECTOR: pac::Interrupt = pac::Interrupt::Uart2;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }
}
