//! Can

#![allow(clippy::too_many_arguments)]

pub use tm4c123x::{CAN0, CAN1};
pub use tm4c_hal::{can::*, can_hal_macro, can_pin_macro};

#[rustfmt::skip]
use crate:: {
    gpio::{
        gpioa, gpiob,
        AlternateFunction, OutputMode, AF2,
    },
    sysctl,
};
use core::marker::PhantomData;

/// Can abstraction
pub struct Can<CAN, TX, RX> {
    can: CAN,
    tx_pin: TX,
    rx_pin: RX,
}

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

can_pin_macro!(CAN0, rx: [(gpiob::PB4, AF2)], tx: [(gpiob::PB5, AF2)],);

can_pin_macro!(CAN1, rx: [(gpioa::PA0, AF2)], tx: [(gpioa::PA1, AF2)],);

can_hal_macro! {
    CAN0: (Can0, can0),
    CAN1: (Can1, can1),
}