#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;

use lc3_traits::peripherals::timers::{
    TimerArr, TimerId, TimerState, Timers, TimerMode
};
use lc3_tm4c::peripherals_tm4c::timers;
use lc3_tm4c::peripherals_tm4c::timers::required_components;
use lc3_tm4c::peripherals_tm4c::gpio;
use lc3_tm4c::peripherals_tm4c::gpio::required_components as gpio_req;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};

use lc3_traits::peripherals::clock::Clock;

use lc3_tm4c::peripherals_tm4c::clock;
use lc3_tm4c::peripherals_tm4c::clock::required_components as clock_req;
#[entry]
fn main() -> ! {

 	    let p = hal::Peripherals::take().unwrap();
 	    let p_core = hal::CorePeripherals::take().unwrap();
 	    let nvic = p_core.NVIC;
	   let mut portf = p.GPIO_PORTF;
	    let mut portb = p.GPIO_PORTB;
	    let mut sc = p.SYSCTL.constrain();
	    let mut pins = gpio::physical_pins::new(
	        &sc.power_control,
	        gpio_req {
	            portf: portf,
	            portb: portb,
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
 	    //let mut porte = p.GPIO_PORTE;
 	    let mut t0 = p.TIMER0;
 	    let mut t1= p.TIMER1;
 	    
 	   // let mut pwm1 = p.PWM1;
    let mut t2= p.TIMER2;
    let mut clock = clock::Tm4cClock::new(clock_req{timer: t2}, &sc.power_control);
 	    let mut timer_shim = timers::TimersShim::new(&sc.power_control, required_components{timer0: t0, timer1: t1});
 	    timer_shim.set_state(TimerId::T0, TimerState::WithPeriod(core::num::NonZeroU16::new(600).unwrap()));
 	    //timer_shim.
 	    //adc_shim.set_state(Pin::A0, AdcState::Enabled);
 	    loop{
 	    //adc_shim.read(Pin::A0);
 	    let ts = clock.get_milliseconds();
 	    let x = ts;
 	}

		
}