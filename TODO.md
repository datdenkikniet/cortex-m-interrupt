* Figure out how to deal with:
   ```rust
   f() -> impl InterruptHandle { take_exception!(SysTick); }
   g() { let h1 = f(); let h2 = f(); panic!(); }
   ```
   Most likely:
   ```rust
   static registered: AtomicBool = AtomicBool::new(false); 
   // In `register` impl:
   if registered.swap(true, Acquire) { panic!(); }
   ```


   Done: used code as suggested by alexmoon

* Figure out if we can directly replace the handler instead of creating a trampoline function
   Seems doable:
   Use 
   ```rust
    union Vector {
        default: unsafe extern "C" fn(),
        handler_fn: fn(),
    }
    ```
    to set the correct default handler.
    Override value in vector table on `register`?    
    
* See if we want to support calling `register` more than once.
      How to deal with multiple peripherals that can cause the interrupt to fire, but having the occupation overwritten?
      registered status guarded by an `AtomicBool`
      Implemented
