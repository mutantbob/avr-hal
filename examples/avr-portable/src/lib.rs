#![no_std]

//! If you want library routines to be portable between different AVR implementations,
//! it is best to use types from [avr_hal_generic] instead of [arduino_hal]

use avr_hal_generic::usart::{Usart, UsartOps};
// use avr_hal_generic::serial::Read;
use avr_hal_generic::adc::AdcChannel;
use embedded_hal::serial::Read;
pub use void::ResultVoidErrExt as _;
pub use void::ResultVoidExt as _;

pub fn report<H, USART: UsartOps<H, RX, TX>, RX, TX, CLOCK>(
    serial: &mut Usart<H, USART, RX, TX, CLOCK>,
) {
    // Read a byte from the serial connection
    let b = nb::block!(serial.read()).void_unwrap();

    // Answer
    ufmt::uwriteln!(serial, "Got {}!\r", b).void_unwrap();
}

pub fn report_adc_single<
    H,
    USART: UsartOps<H, RX, TX>,
    RX,
    TX,
    ADCOPS: avr_hal_generic::adc::AdcOps<H>,
    CLOCK: avr_hal_generic::clock::Clock,
    PIN: AdcChannel<H, ADCOPS>,
>(
    serial: &mut Usart<H, USART, RX, TX, CLOCK>,
    adc: &mut avr_hal_generic::adc::Adc<H, ADCOPS, CLOCK>,
    i: usize,
    analog_pin: &PIN,
) {
    let v = adc.read_blocking(analog_pin);
    ufmt::uwrite!(serial, "A{}: {} ", i, v).void_unwrap();
}

pub fn report_adc_multi<
    H,
    USART: UsartOps<H, RX, TX>,
    RX,
    TX,
    ADCOPS: avr_hal_generic::adc::AdcOps<H>,
    CLOCK: avr_hal_generic::clock::Clock,
>(
    serial: &mut Usart<H, USART, RX, TX, CLOCK>,
    adc: &mut avr_hal_generic::adc::Adc<H, ADCOPS, CLOCK>,
    channels: &[avr_hal_generic::adc::Channel<H, ADCOPS>],
) {
    for (i, ch) in channels.iter().enumerate() {
        let v = adc.read_blocking(ch);
        ufmt::uwrite!(serial, "A{}: {} ", i, v).void_unwrap();
    }

    ufmt::uwriteln!(serial, "").void_unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}