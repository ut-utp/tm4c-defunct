//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;
use hal::{gpio, Peripherals};

#[entry]
fn main() -> ! {
	    let p = hal::Peripherals::take().unwrap();

let mut sc = p.SYSCTL.constrain();


  hprintln!("Hellkmknjo, worldjhjh!").unwrap();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_20mhz),
);

  let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);

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
    let p = Peripherals::take().unwrap();
    let mut sc = p.SYSCTL.constrain();
    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    let timer_output_pin = portb.pb0.into_af_push_pull::<gpio::AF7>(&mut portb.control);
    let uart_tx_pin = portb.pb1.into_af_open_drain::<gpio::AF1, gpio::PullUp>(&mut portb.control);
    let blue_led = portb.pb2.into_push_pull_output();
    let button = portb.pb3.into_pull_up_input();
    let mut counter = 0u32;
    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    debug::exit(debug::EXIT_SUCCESS);
    
    loop {}
}
