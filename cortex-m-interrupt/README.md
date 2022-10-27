# Function-like, trait-based interrupt handler registration.

This crate provides a method of delegating creation of occupations, and attempts to improve pains associated with creating registrations.

### Usage

As an end-user of this crate, you will want to use the [`take_nvic_interrupt`] and [`take_exception`] macros. These generate implementors of the [`NvicInterruptRegistration`] and [`ExceptionRegistration`] traits respectively, which can be passed to functions that make use of interrupts.

To see information on how you can use [`InterruptRegistration`], [`NvicInterruptRegistration`] and [`ExceptionRegistration`] refer to the docs on those traits.


## The problem that this crate tries to solve.

To demonstrate the problem that this crate tries to solve, we will use an example use case.

### Definitions

To help explain the use case of this crate, we use the following definitions:
1. a registration: the function pointer placed in an interrupt vector, the declaration of an interrupt handler.
2. an occupation: the code that should run when servicing an interrupt, the body of an interrupt handler.

### The usecase

Our goal with the use case is:
1. Configure the `SysTick` interrupt so that it triggers once every `1337` cycles.
2. Increment a counter every time the `SysTick` interrupt occurs.

When using the `cortex_m_rt` crate, we would do something like this:
```rust,ignore
# fn systick_reload(_: u32) {}
# fn setup_systick_exception(_: u32) {}
use cortex_m_rt::{exception, entry};

#[exception]
fn SysTick() { // This is the "registration" (can only
               // be provided once in entirety of the
               // program, incl. libraries)

    static mut COUNTER: u32 = 0; // This is part of the "occupation"
    *COUNTER += 1;               // This is the "occupation", the
    systick_reload(1337);        // actual code to be executed
                                 // when handling the SysTick
                                 // interrupt.

}

#[entry]
fn main() {
    setup_systick_exception(1337);
    loop {}
}
```

And within the crate providing `setup_systick_exception` and `systick_reload`:
```rust,no_run
pub fn setup_systick_exception(reload_value: u32) {
    /* Setup systick so that it triggers the SysTick interrupt
       after `reload_value` cycles
    */ 
}

pub fn systick_reload(reload_value: u32) {
    // Reload the systick value
}
```

In this example:
1. There is no semantic connection between `setup_systick_exception` and the registration/occupation besides naming and possibly documentation.
2. The responsibility of adding the correct occupation to the correct registration falls entirely on the person writing the program.
3. The maintainer of the crate that provides `setup_systick_exception` and `systick_reload` has no control over the occupation or registration.
4. There is no need for a trampoline to setup up the interrupt handler.

### A solution?

To represent registrations, the [`InterruptRegistration`], [`NvicInterruptRegistration`], and [`ExceptionRegistration`] traits are provided. They can be used to insert occupations in a more dynamic way.

These traits allow for the following:
1. Code that wishes to provide an occupation without directly creating a registration can be written by requiring that user code provides an registration.
2. Code that wishes to provide an occupation can verify that a provided registration is correct for the to-be-created occupation.

To alleviate difficulties with creating registrations, the [`take_nvic_interrupt`] and [`take_exception`] proc-macros are provided. They perform the less self explanatory parts of the setting up a registration, and provide an implementor of [`NvicInterruptRegistration`] and [`ExceptionRegistration`], respectively.

### A revised example

With these new tools, we can rewrite our code to look as follows:
```rust,ignore
# fn setup_systick_exception<T: cortex_m_interrupt::InterruptRegistration>(_: u32, _: T, _: fn()) {}
use cortex_m_rt::entry;
use cortex_m::peripheral::scb::Exception::SysTick;

static mut COUNTER: u32 = 0; 

fn increase_counter() {
    unsafe { COUNTER += 1 };
}

#[entry]
fn main() -> ! {
    // We create the registration
    let systick_registration = cortex_m_interrupt::take_exception!(SysTick);
    // And pass it to some function that will do some configuration and
    // provide a occupation for that registration. It also allows us to
    // inject our own expansion to the occupation.
    setup_systick_exception(1337, systick_registration, increase_counter);
    loop {}
}
```

In the crate providing `setup_systick_exception`:
```rust
# use cortex_m_interrupt::ExceptionRegistration;
pub fn setup_systick_exception<Registration: ExceptionRegistration>(
    reload_value: u32,
    registration: Registration,
    f: fn(),
) {
    // Assert that we've been given a registration of the correct
    // exception/interrupt.
    assert_eq!(
        cortex_m::peripheral::scb::Exception::SysTick,
        registration.exception()
    );

    /* Setup systick so that it triggers the SysTick interrupt
       after `reload_value` cycles
    */ 

    use core::mem::MaybeUninit;
    static mut USER_HANDLE: MaybeUninit<fn()> = MaybeUninit::uninit();
    static mut RELOAD_VALUE: MaybeUninit<u32> = MaybeUninit::uninit();

    unsafe { USER_HANDLE.write(f) };
    unsafe { RELOAD_VALUE.write(reload_value) };

    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::Release);

    registration.occupy(|| {
        systick_reload(unsafe { RELOAD_VALUE.assume_init() });

        // Call extra user code
        unsafe { (USER_HANDLE.assume_init())(); }
    });
}

fn systick_reload(reload_value: u32) {
    // Reload the systick value
}

```

In the revised example:
1. There is a more defined semantic connection between the registration and the occupation.
2. The implementor of `setup_systick_exception` has full control over the occupation, and can optionally allow user code to perform some extra actions.
3. The implementor of `setup_systick_exception` can verify that the correct registration is passed to it.
4. A trampoline is now required in the interrupt handler, adding ~5 cycles of extra processing when an interrupt occurs.

### Main differences
The main differences between the `cortex-m-rt` approach and what `cortex-m-interrupt` provides are the following:
1. `cortex-m-interrupt` offers a way of creating clearer separation of responsibilities when it comes to registering interrupt handlers.
2. The traits provided by `cortex-m-interrupt` support a tighter semantic connection between creating a registration and creating an occupation.
3. The method provided by `cortex-m-rt` has slightly less overhead as it does not require a trampoline. 