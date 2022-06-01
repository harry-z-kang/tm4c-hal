//! Connected Area Network (CAN) peripheral

use crate::{
    gpio::{gpioa, gpiob, gpioe, gpiof, AlternateFunction, OutputMode, AF1, AF2},
    sysctl,
};
use core::marker::PhantomData;

pub use tm4c123x::{CAN0, CAN1, CAN2, CAN3, CAN4, CAN5, CAN6, CAN7, CAN8, CAN9};
pub use tm4c_hal::{can::*, can_hal_macro, can_pin_macro};

/// Can abstraction
pub struct Can<CAN, TX, RX> {
    can: CAN,
    tx_pin: TX,
    rx_pin: RX,
}

/// TX pin - DO NOR IMPLEMENT THIS TRAIT
pub unsafe trait TxPin<CAN> {}

/// RX pin - DO NOR IMPLEMENT THIS TRAIT
pub unsafe trait RxPin<CAN> {}

/// Can receiver
pub struct Rx<CAN, RX> {
    _can: PhantomData<CAN>,
    pin: RX,
}

/// CAN transmitter
pub struct Tx<CAN, TX> {
    can: CAN,
    pin: TX,
}

// CAN0-8 are aliases for module CAN0 but with different Tx/Rx Pin combination
can_pin_macro!(CAN0, rx: [(gpiob::PB4, AF2)], tx: [(gpiob::PB5, AF2)],);
can_pin_macro!(CAN1, rx: [(gpiob::PB4, AF2)], tx: [(gpiof::PF3, AF1)],);
can_pin_macro!(CAN2, rx: [(gpiob::PB4, AF2)], tx: [(gpioe::PE5, AF2)],);

can_pin_macro!(CAN3, rx: [(gpiof::PF0, AF2)], tx: [(gpiob::PB5, AF2)],);
can_pin_macro!(CAN4, rx: [(gpiof::PF0, AF2)], tx: [(gpiof::PF3, AF1)],);
can_pin_macro!(CAN5, rx: [(gpiof::PF0, AF2)], tx: [(gpioe::PE5, AF2)],);

can_pin_macro!(CAN6, rx: [(gpioe::PE4, AF2)], tx: [(gpiob::PB5, AF2)],);
can_pin_macro!(CAN7, rx: [(gpioe::PE4, AF2)], tx: [(gpiof::PF3, AF1)],);
can_pin_macro!(CAN8, rx: [(gpioe::PE4, AF2)], tx: [(gpioe::PE5, AF2)],);

// CAN9 corresponds to CAN1 module on the tm4c123x
can_pin_macro!(CAN9, rx: [(gpioa::PA0, AF2)], tx: [(gpioa::PA1, AF2)],);

can_hal_macro! {
    CAN0: (Can0, can0),
    CAN1: (Can1, can1),
    CAN2: (Can2, can2),
    CAN3: (Can3, can3),
    CAN4: (Can4, can4),
    CAN5: (Can5, can5),
    CAN6: (Can6, can6),
    CAN7: (Can7, can7),
    CAN8: (Can8, can8),
    CAN9: (Can9, can9),
}
