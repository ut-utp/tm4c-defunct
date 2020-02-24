#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;

use lc3_traits::peripherals::timers::{
    TimerArr, TimerId, TimerMiscError, TimerState, TimerStateMismatch, Timers,
};
use lc3_tm4c::peripherals_tm4c::timers;
use lc3_tm4c::peripherals_tm4c::timers::required_components;

#[entry]
fn main() -> ! {
 	    let p = hal::Peripherals::take().unwrap();
 	    let p_core = hal::CorePeripherals::take().unwrap();
 	    let nvic = p_core.NVIC;
 	    //let mut porte = p.GPIO_PORTE;
 	    let mut t0 = p.TIMER0;
 	    let mut t1= p.TIMER1;
 	    let mut sc = p.SYSCTL.constrain();
 	   // let mut pwm1 = p.PWM1;
 	    let mut timer_shim = timers::TimersShim::new(&sc.power_control, required_components{timer0: t0, timer1: t1}, nvic);
 	    //timer_shim.
 	    //adc_shim.set_state(Pin::A0, AdcState::Enabled);
 	    loop{
 	    //adc_shim.read(Pin::A0);
 	}

		
}