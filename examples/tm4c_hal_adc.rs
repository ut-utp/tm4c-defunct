#![no_std]
#![no_main]

extern crate panic_halt; // you can put a breakpoint on `rust_begin_unwind` to catch panics
extern crate tm4c123x_hal as hal;
use core::fmt::Write;
use cortex_m_rt::entry;
use hal::prelude::*;
use hal::adc as ad;
use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};
use lc3_tm4c::peripherals_generic::adc::*;
use lc3_tm4c::peripherals_generic::adc as gen_adc;

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    let mut porte = p.GPIO_PORTE.split(&sc.power_control);
    let pe3 = porte.pe3.into_analog_input();
    let pe2 = porte.pe2.into_analog_input();
    let pe1 = porte.pe1.into_analog_input();
    let pe0 = porte.pe0.into_analog_input();
    let pe5 = porte.pe5.into_analog_input();
    let pe4 = porte.pe4.into_analog_input();
    let adct = ad::components::adc0(p. ADC0, &sc.power_control, (pe3, pe2, pe1, pe0, pe5, pe4));

    let mut out = gen_adc::MyAdc::<gen_adc::generic_adc_res,ad::Tm4cAdc<ad::Channel_pe3>, ad::Channel_pe3,
                                   ad::Tm4cAdc<ad::Channel_pe2>, ad::Channel_pe2,
                                   ad::Tm4cAdc<ad::Channel_pe1>, ad::Channel_pe1,
                                   ad::Tm4cAdc<ad::Channel_pe0>, ad::Channel_pe0>::new(
                                   adct.adc1, adct.adc2, adct.adc3, adct.adc4,
                                   adct.channel1, adct.channel2, adct.channel3, adct.channel4);
                                   

    let mut result = out.read(Pin::A0);
        loop{
            result = out.read(Pin::A0);;
        }

    // Activate UART
    // let mut uart = hal::serial::Serial::uart0(
    //     p.UART0,
    //     porta
    //         .pa1
    //         .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //     porta
    //         .pa0
    //         .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //     (),
    //     (),
    //     115200_u32.bps(),
    //     hal::serial::NewlineMode::SwapLFtoCRLF,
    //     &clocks,
    //     &sc.power_control,
    // );

    // let mut counter = 0u32;
    // loop {
    //     writeln!(uart, "Hello, world! counter={}", counter).unwrap();
    //     counter = counter.wrapping_add(1);
    // }
}
