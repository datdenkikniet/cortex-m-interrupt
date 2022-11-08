#![no_std]
#![no_main]

#[panic_handler]
fn panic_handler(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// Connect to a peripheral with 1:1 IRQ and driver relationship
cortex_m_interrupt::register_interrupt!(SpiToken, hal::pac::Interrupt::Spi0 -> hal::Spi);

// Connect to a peripheral with 1:2 IRQ and driver relationship
cortex_m_interrupt::register_interrupt!(Uart01Token,
    hal::pac::Interrupt::Uart0_1 -> hal::Uart0,
    hal::pac::Interrupt::Uart0_1 -> hal::Uart1,
);

// Mega peripheral
cortex_m_interrupt::register_interrupt!(MegaTimerToken,
    hal::pac::Interrupt::TIM1_BRK -> hal::Brk<pac::TIM1>,
    hal::pac::Interrupt::TIM1_CC -> hal::Cc<pac::TIM1>,
    hal::pac::Interrupt::TIM1_TRG_COM_TIM11 -> hal::Trg<pac::TIM1>,
    hal::pac::Interrupt::TIM1_TRG_COM_TIM11 -> hal::Com<pac::TIM1>,
    hal::pac::Interrupt::TIM1_UP -> hal::Up<pac::TIM1>,
);

// Connect with an exception
use cortex_m::peripheral::scb::Exception;
cortex_m_interrupt::register_interrupt!(SystickToken, Exception::SysTick -> hal::SysTickDelay);

#[cortex_m_rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take();
    let c = cortex_m::Peripherals::take().unwrap();

    // Works
    let spi1 = hal::Spi::new(p.spi0, SpiToken);

    let uart0 = hal::Uart0::new(p.uart0, Uart01Token);
    let uart1 = hal::Uart1::new(p.uart1, Uart01Token);

    let systick_delay = hal::SysTickDelay::new(c.SYST, SystickToken);

    // Fails
    let uart2 = hal::Uart2::new(p.uart2, Uart01Token);

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

    pub struct TIM1;

    pub struct Peripherals {
        pub spi0: SPI0,
        pub uart0: UART0,
        pub uart1: UART1,
        pub uart2: UART2,
        pub tim1: TIM1,
    }

    impl Peripherals {
        pub fn take() -> Self {
            Peripherals {
                spi0: SPI0,
                uart0: UART0,
                uart1: UART1,
                uart2: UART2,
                tim1: TIM1,
            }
        }
    }

    #[allow(non_camel_case_types)]
    pub enum Interrupt {
        Int1,
        Int2,
        Int3,
        Spi0,
        Uart0_1,
        Uart2,
        TIM1_BRK,
        TIM1_CC,
        TIM1_TRG_COM_TIM11,
        TIM1_UP,
    }
}

pub mod hal {
    use core::marker::PhantomData;

    use cortex_m::peripheral::scb::Exception;
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

    //
    // -- Exception driver (systick)
    //

    pub struct SysTickDelay {}

    impl SysTickDelay {
        pub fn new<Handle>(syst: cortex_m::peripheral::SYST, interrupt_handle: Handle) -> Self
        where
            Handle: InterruptToken<SysTickDelay>,
        {
            SysTickDelay {}
        }
    }

    impl InterruptRegistration<Exception> for SysTickDelay {
        const VECTOR: Exception = Exception::SysTick;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    //
    // -- Mega timer
    //

    pub struct Brk<TIMER> {
        _p: PhantomData<TIMER>,
    }

    impl InterruptRegistration<pac::Interrupt> for Brk<pac::TIM1> {
        const VECTOR: pac::Interrupt = pac::Interrupt::TIM1_BRK;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    pub struct Cc<TIMER> {
        _p: PhantomData<TIMER>,
    }

    impl InterruptRegistration<pac::Interrupt> for Cc<pac::TIM1> {
        const VECTOR: pac::Interrupt = pac::Interrupt::TIM1_CC;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    pub struct Trg<TIMER> {
        _p: PhantomData<TIMER>,
    }

    impl InterruptRegistration<pac::Interrupt> for Trg<pac::TIM1> {
        const VECTOR: pac::Interrupt = pac::Interrupt::TIM1_TRG_COM_TIM11;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    pub struct Com<TIMER> {
        _p: PhantomData<TIMER>,
    }

    impl InterruptRegistration<pac::Interrupt> for Com<pac::TIM1> {
        const VECTOR: pac::Interrupt = pac::Interrupt::TIM1_TRG_COM_TIM11;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }

    pub struct Up<TIMER> {
        _p: PhantomData<TIMER>,
    }

    impl InterruptRegistration<pac::Interrupt> for Up<pac::TIM1> {
        const VECTOR: pac::Interrupt = pac::Interrupt::TIM1_UP;

        fn on_interrupt() {
            // Doing stuff ...
        }
    }
}
