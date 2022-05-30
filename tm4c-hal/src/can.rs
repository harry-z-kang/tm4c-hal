//! Can code that is generic to both the TM4C123 and TM4C129, such as the pin traits.

/// TX pin - DO NOR IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<UART> {}

/// RX pin - DO NOR IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<UART> {}

unsafe impl<U> TxPin<U> for () {}

unsafe impl<U> RxPin<U> for () {}
