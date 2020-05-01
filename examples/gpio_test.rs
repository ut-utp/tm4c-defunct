//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;
//use hal::{gpio, Peripherals};
use lc3_tm4c::peripherals_tm4c::gpio;
use lc3_tm4c::peripherals_tm4c::gpio::required_components;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();

    //   //let clocks = sc.clock_setup.freeze();

    let mut portf = p.GPIO_PORTF.split(&sc.power_control);
    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    let mut pins = gpio::physical_pins::new(
        &sc.power_control,
        required_components {
            pf1: portf.pf1.into_push_pull_output(),
            pf2: portf.pf2.into_push_pull_output(),
            pf4: portf.pf4.into_push_pull_output(),
            pb0: portb.pb0.into_push_pull_output(),
            pb1: portb.pb1.into_pull_up_input(),
            pb2: portb.pb2.into_pull_up_input(),
            pb3: portb.pb3.into_pull_up_input(),
            pb4: portb.pb4.into_pull_up_input(),
        },
    );
    // pins.set_pin(GpioPin::G4, true);
    //let mut pins = gpio::physical_pins::default();
    // pins.set_pin(GpioPin::G4, false);
    // pins.set_pin(GpioPin::G5, true);
   pins.set_state(GpioPin::G0, GpioState::Output);
   pins.set_state(GpioPin::G1, GpioState::Output);
   pins.set_state(GpioPin::G2, GpioState::Output);
   pins.set_state(GpioPin::G3, GpioState::Output);
    pins.set_pin(GpioPin::G0, false);
    //pins.set_pin(GpioPin::G3, true);
    pins.set_pin(GpioPin::G1, false);
    pins.set_pin(GpioPin::G2, true);
    pins.set_pin(GpioPin::G3, false);
    //pins.set_pin(GpioPin::G2, false);
   // pins.set_state(GpioPin::G0, GpioState::Input);
   // pins.set_state(GpioPin::G0, GpioState::Output);
   // pins.set_pin(GpioPin::G0, true);

    //let mut porta = p.GPIO_PORTA.split(&sc.power_control);

    //  Activate UART
    //  let mut uart = hal::serial::Serial::uart0(
    //      p.UART0,
    //      porta
    //          .pa1
    //          .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //      porta
    //          .pa0
    //          .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //      (),
    //      (),
    //      115200_u32.bps(),
    //      hal::serial::NewlineMode::SwapLFtoCRLF,
    //      &clocks,
    //      &sc.power_control,
    //  );
    // let p =  hal::Peripherals::take().unwrap();
    //  let mut sc = p.SYSCTL.constrain();
    //  let mut portb = p.GPIO_PORTF.split(&sc.power_control);
    //  //let timer_output_pin = portb.pb0.into_af_push_pull::<gpio::AF7>(&mut portb.control);
    // // let uart_tx_pin = portb.pb1.into_af_open_drain::<gpio::AF1, gpio::PullUp>(&mut portb.control);
    //  let mut blue_led = portb.pf2.into_push_pull_output();
    //  blue_led.set_high();
    //  //blue_led.set_low();
    //  let button = portb.pf3.into_pull_up_input();
    //  let mut counter = 0u32;
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    //debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
