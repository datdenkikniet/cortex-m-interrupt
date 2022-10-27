#[cfg(feature = "stm32f1xx-hal")]
mod stm32f1xx;
#[cfg(feature = "stm32f1xx-hal")]
pub use stm32f1xx::*;

#[cfg(any(feature = "stm32f4xx-hal", feature = "stm32f7xx-hal"))]
mod stm32f4xx_7xx;
#[cfg(any(feature = "stm32f4xx-hal", feature = "stm32f7xx-hal"))]
pub use stm32f4xx_7xx::*;
