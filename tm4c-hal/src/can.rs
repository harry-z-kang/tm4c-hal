//! Can code that is generic to both the TM4C123 and TM4C129, such as the pin traits.

/// The minimum CAN pre-divisor is 1.
pub const CAN_MIN_PRE_DIVISOR: u32 = 1;
/// The maximum CAN pre-divisor is 1024.
pub const CAN_MAX_PRE_DIVISOR: u32 = 1024;

/// The minimum CAN bit timing divisor is 4.
pub const CAN_MIN_BIT_DIVISOR: u32 = 4;
/// The maximum CAN bit timing divisor is 19.
pub const CAN_MAX_BIT_DIVISOR: u32 = 19;

/// Index for SEG1 bit in CAN_BIT_VALUES
pub const SEG1: usize = 0;
/// Index for SEG2 bit in CAN_BIT_VALUES
pub const SEG2: usize = 1;
/// Index for SJW bit in CAN_BIT_VALUES
pub const SJW: usize = 2;

/// This table is used by the set_bit_rate() API as the register defaults for
/// the bit timing values.
pub const CAN_BIT_VALUES: [[u8; 3]; 16] = [
    [2, 1, 1],  // 4 clocks/bit
    [3, 1, 1],  // 5 clocks/bit
    [3, 2, 2],  // 6 clocks/bit
    [4, 2, 2],  // 7 clocks/bit
    [4, 3, 3],  // 8 clocks/bit
    [5, 3, 3],  // 9 clocks/bit
    [5, 4, 4],  // 10 clocks/bit
    [6, 4, 4],  // 11 clocks/bit
    [6, 5, 4],  // 12 clocks/bit
    [7, 5, 4],  // 13 clocks/bit
    [7, 6, 4],  // 14 clocks/bit
    [8, 6, 4],  // 15 clocks/bit
    [8, 7, 4],  // 16 clocks/bit
    [9, 7, 4],  // 17 clocks/bit
    [9, 8, 4],  // 18 clocks/bit
    [10, 8, 4], // 19 clocks/bit
];

/// This structure is used for encapsulating the values associated with setting
/// up the bit timing for a CAN controller.  The structure is used when calling
/// the CANGetBitTiming and CANSetBitTiming functions.
pub struct CanBitBlkParms {
    /// This value holds the sum of the Synchronization, Propagation, and Phase
    /// Buffer 1 segments, measured in time quanta.  The valid values for this
    /// setting range from 2 to 16.
    pub sync_prop_phase1_seg: u32,
    /// This value holds the Phase Buffer 2 segment in time quanta.  The valid
    /// values for this setting range from 1 to 8.
    pub phase2_seg: u32,
    /// This value holds the Resynchronization Jump Width in time quanta.  The
    /// valid values for this setting range from 1 to 4.
    pub sjw: u32,
    /// This value holds the CAN_CLK divider used to determine time quanta.
    /// The valid values for this setting range from 1 to 1023.
    pub quantum_prescaler: u32,
}

/// CAN error
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// A Valid Bit Rate Combination Not Found
    BitRateCombinationNotFound,

    /// CAN Timeout
    Timeout,
}

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

        /// Initializes the CAN controller after reset.
        pub fn init(&mut self) {
          self.can.ctl.write(|w| unsafe { w.init().set_bit() });

          while self.can.if1crq.read().busy().bit_is_set() {}

          self.can.if1cmsk.write(|w| unsafe { w.wrnrd().set_bit().arb().set_bit().control().set_bit() });

          self.can.if1arb2.reset();

          self.can.if1mctl.reset();

          for msg in 1..33 {
            while self.can.if1crq.read().busy().bit_is_set() {}

            self.can.if1crq.write(|w| unsafe { w.bits(msg) });

          }

          self.can.if1cmsk.write(|w| unsafe { w.newdat().set_bit().clrintpnd().set_bit() });

          for msg in 1..33 {
            while self.can.if1crq.read().busy().bit_is_set() {}

            self.can.if1crq.write(|w| unsafe { w.bits(msg) });
          }
        }

        /// Enable the CAN Controller
        pub fn enable(&mut self) {
          self.can.ctl.modify(|_, w| w.init().clear_bit());
        }

        /// Disable the CAN Controller
        pub fn disable(&mut self) {
          self.can.ctl.modify(|_, w| w.init().set_bit());
        }

        /// Read the current settings for the CAN Controller bit timing
        pub fn get_bit_timing(self) -> CanBitBlkParms {
          let bit_timing = self.can.bit_.read();

          CanBitBlkParms {
            quantum_prescaler: ((bit_timing.brp().bits() as u32 | ((self.can.brpe.read().bits() as u32) << 6)) + 1) as u32,
            sjw: (bit_timing.sjw().bits() + 1) as u32,
            sync_prop_phase1_seg: (bit_timing.tseg1().bits() + 1) as u32,
            phase2_seg: (bit_timing.tseg2().bits() + 1) as u32,
          }
        }

        /// Sets the CAN bit timing values to a nominal setting based on a desired
        /// bit rate.
        pub fn set_bit_rate(&mut self, clocks: &crate::sysctl::Clocks, bit_rate: u32) -> nb::Result<u32, Error> {
          let mut desired_ratio: u32 = clocks.sysclk().0 / bit_rate;

          if clocks.sysclk().0 / desired_ratio > bit_rate {
            desired_ratio += 1;
          }

          while desired_ratio <= (CAN_MAX_PRE_DIVISOR * CAN_MAX_BIT_DIVISOR) {
            for can_bits in CAN_MAX_BIT_DIVISOR..CAN_MIN_BIT_DIVISOR {
              let can_pre_divisor = desired_ratio / can_bits;

              if can_pre_divisor * can_bits == desired_ratio {
                let reg_value = CAN_BIT_VALUES[can_bits as usize - CAN_MIN_BIT_DIVISOR as usize];

                let can_ctl = self.can.ctl.read().bits();

                self.can.ctl.modify(|_, w| unsafe { w.init().set_bit().cce().set_bit() });

                self.can.bit_.write(|w| unsafe { w.brp().bits((can_pre_divisor - 1) as u8).sjw().bits(reg_value[SJW]).tseg1().bits(reg_value[SEG1]).tseg2().bits(reg_value[SEG2]) });

                self.can.brpe.write(|w| unsafe { w.bits((can_pre_divisor - 1) >> 6) });

                self.can.ctl.write(|w| unsafe { w.bits(can_ctl) });

                return Ok(clocks.sysclk().0 / (can_pre_divisor * can_bits));
              }
            }

            desired_ratio += 1;
          }

          Err(nb::Error::Other(Error::BitRateCombinationNotFound))
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
