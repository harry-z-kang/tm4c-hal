//! Can code that is generic to both the TM4C123 and TM4C129, such as the pin traits.

/// An internal macro to help define all the different pin typestates
#[macro_export]
macro_rules! can_pin_macro {
    ($CANn:ident,
      rx: [$(($($rxgpio: ident)::*, $rxaf: ident)),*],
      tx: [$(($($txgpio: ident)::*, $txaf: ident)),*],
    ) => {
        $(
            unsafe impl<T> RxPin<$CANn> for $($rxgpio)::*<AlternateFunction<$rxaf, T>>
            where
                T: OutputMode,
            {}
        )*

        $(
            unsafe impl<T> TxPin<$CANn> for $($txgpio)::*<AlternateFunction<$txaf, T>>
            where
                T: OutputMode
            {}
        )*
    }
}

/// An internal macro to help define all the different pin typestates
#[macro_export]
macro_rules! can_hal_macro {
  ($(
      $CANX:ident: ($powerDomain:ident, $canX:ident),
  )+) => {
    $(
      impl<TX, RX> Can<$CANX, TX, RX> {
        /// Configures a CAN peripheral to provide can communication
        pub fn $canX(
          mut can: $CANX,
          tx_pin: TX,
          rx_pin: RX,
          baud_rate: crate::time::Bps,
          clocks: &crate::sysctl::Clocks,
          pc: &sysctl::PowerControl
        ) -> Self
        where
            TX: TxPin<$CANX>,
            RX: RxPin<$CANX>
        {
          sysctl::control_power(
            pc, sysctl::Domain::$powerDomain,
            sysctl::RunMode::Run, sysctl::PowerState::On);
          sysctl::reset(pc, sysctl::Domain::$powerDomain);

          can.ctl.reset();

          can.ctl.write(|w| unsafe { w.init().set_bit().cce().set_bit() });

          can.bit_.write(|w| unsafe { w.brp().bits(0b111111).sjw().bits(0b11).tseg1().bits(0b1111).tseg2().bits(0b1111) });

          can.ctl.write(|w| unsafe { w.init().clear_bit() });

          Can { can, tx_pin, rx_pin }
        }

        /// Change the current baud rate for the CAN. We need the
        /// `clocks` object in order to calculate the magic baud rate
        /// register values.
        pub fn change_baud_rate(&mut self, baud_rate: crate::time::Bps, clocks: &crate::sysctl::Clocks) {
          // Stop CAN Interrupt
          self.can.ctl.modify(|_, w| w.ie().bit(false));

          // Calculate baud rate dividers
          let baud_int: u32 = (((clocks.sysclk.0 * 8) / baud_rate.0) + 1) / 2;

          // Set baud rate
          self.can.brpe.write(|w|
              unsafe { w.bits((baud_int / 64) as u32) });

          // Start CAN Interrupt again
          self.can.ctl.modify(|_, w| w.ie().bit(true));
        }

        /// Splits the `Can` abstraction into a transmitter and a
        /// receiver half. If you do this you can transmit and receive
        /// in different threads.
        pub fn split(self) -> (Tx<$CANX, TX>, Rx<$CANX, RX>) {
          (
              Tx {
                  can: self.can,
                  pin: self.tx_pin,
              },
              Rx {
                  _can: PhantomData,
                  pin: self.rx_pin,
              },
          )
        }

        /// Re-combine a split UART
        pub fn combine(tx: Tx<$CANX, TX>, rx: Rx<$CANX, RX>) -> Can<$CANX, TX, RX> {
          Can {
              can: tx.can,
              rx_pin: rx.pin,
              tx_pin: tx.pin,
          }
        }
      }
    )+
  }
}
