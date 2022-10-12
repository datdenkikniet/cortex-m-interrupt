// Re-export this path
pub use cortex_m;

pub use cortex_m_interrupt_macro::take_raw_prio;

#[cfg(feature = "rtic-priority")]
pub use cortex_m_interrupt_macro::take;
#[cfg(feature = "rtic-priority")]
pub use rtic::export::logical2hw;

pub trait IrqHandle {
    fn register(self, f: fn());
}
