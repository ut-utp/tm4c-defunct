#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;
use core::fmt::Write;

use lc3_tm4c::peripherals_tm4c::clock;
use lc3_tm4c::peripherals_tm4c::clock::required_components as clock_req;

#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
 	let p_core = hal::CorePeripherals::take().unwrap();
 	let mut sys = p.SYSCTL.constrain();
    let mut t2= p.TIMER2;
    let mut clock_req = clock::Tm4cClock::new(clock_req{timer: t2}, &sys.power_control);
    loop{}
}