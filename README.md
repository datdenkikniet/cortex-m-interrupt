# cortex-m-interrupt: function-like, trait-based interrupt handler registration.

This crate provides a method of delegating creation of occupations, and attempts to improve pains associated with creating registrations.

### Definitions

To help explain the use case of this crate, we use the following definitions:
1. a registration: the function pointer placed in an interrupt vector, the declaration of an interrupt handler.
2. an occupation: the code that should run when servicing an interrupt, the body of an interrupt handler.


### An example
```rust,no_run
# fn systick_reload(_: u32) {}
# fn setup_systick_exception(_: u32) {}
static mut COUNTER: u32 = 0;  // This is part of the "occupation": a
                              // variable that is used within the
                              // interrupt handler

pub extern "C" fn SysTick() { // This is the "registration" (can only
                              // be provided once in entirety of the
                              // program, incl. libraries)

    unsafe { COUNTER += 1 };  // This is the "occupation", the
    systick_reload(1337);     // actual code to be executed
                              // when handling the SysTick
                              // interrupt.


}

fn main(){
    setup_systick_exception(1337);
    loop {}
}
```
Or, alternatively, using the `cortex_m_rt` crate:
```rust,no_run
# fn systick_reload(_: u32) {}
# fn setup_systick_exception(_: u32) {}
use cortex_m_rt::exception;
#[exception]
fn SysTick() { // This is the "registration" (can only
               // be provided once in entirety of the
               // program, incl. libraries)

    static mut COUNTER: u32 = 0; // This is part of the "occupation"
    *COUNTER += 1;                // This is the "occupation", the
    systick_reload(1337);        // actual code to be executed
                                 // when handling the SysTick
                                 // interrupt.

}

fn main(){
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

Difficulties with the example above include:
1. There is no semantic connection between `setup_systick_exception` and the registration/occupation, besides naming.
2. The responsibility of adding the correct occupation falls entirely on the person writing the program.
3. The maintainer of the crate that provides `setup_systick_exception` and `systick_reload` has no control over the occupation.

### A solution?

To represent registrations, the `InterruptHandle`, `NvicInterruptHandle`, and `ExceptionHandle` traits are provided. They then be used to insert occupations in a more dynamic way.

These traits allow for the following:
1. Code that wishes to provide an occupation without directly creating a registration can be written by requiring that user code provides an registration.
2. Code that wishes to provide an occupation can verify that a provided registration is correct for the to-be-created occupation.

To alleviate difficulties with creating registrations, the `take_nvic_interrupt` and `take_exception` proc-macros are provided. They perform the less self explanatory parts of the setting up a registration, and provide an implementor of `NvicInterruptHandle` and `ExceptionHandle`, respectively.

### A revised example
In the user crate:
```rust,no_run
# use cortex_m::peripheral::scb::Exception::SysTick;
# fn setup_systick_exception<T: cortex_m_interrupt::InterruptHandle>(_: u32, _: T, _: fn()) {}
static mut COUNTER: u32 = 0; 

fn increase_counter() {
    unsafe { COUNTER += 1 };
}

fn main() {
    // We create the registration
    let systick_handle = cortex_m_interrupt::take_exception!(SysTick);
    // And pass it to some function that will do some configuration and
    // provide a occupation for that registration. It also allows us to
    // inject our own expansion to the occupation.
    setup_systick_exception(1337, systick_handle, increase_counter);
    loop {}
}
```

In the crate providing `setup_systick_exception`:
```rust
# use cortex_m_interrupt::ExceptionHandle;
pub fn setup_systick_exception<Handle: ExceptionHandle>(
    reload_value: u32,
    handle: Handle,
    f: fn(),
) {
    // Assert that we've been given a handle to the correct
    // exception/interrupt.
    assert_eq!(
        cortex_m::peripheral::scb::Exception::SysTick,
        handle.exception()
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

    handle.register(|| {
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
2. The implementor of `setup_systick_exception` has full control over what is put into the occupation, and can optionally allow user code to perform some extra actions.