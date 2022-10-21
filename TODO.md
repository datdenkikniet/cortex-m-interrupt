- [x] Figure out how to deal with:
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

- [ ] Figure out if we can directly replace the handler instead of creating a trampoline function
   Unlikely: requires `scb` hackery at runtime (load VTOR, calculate correct offset in VT for given interrupt/exception, update value)    
    
- [ ] See if we want to support calling `register` more than once.
     Use `REGISTERED` as an indicator for being occupied, as opposed to just a "can do again" flag
