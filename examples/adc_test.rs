#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;

use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};

use lc3_tm4c::peripherals_tm4c::adc;
use lc3_tm4c::peripherals_tm4c::adc::required_components;
#[entry]
fn main() -> ! {
 	    let p = hal::Peripherals::take().unwrap();
 	    let mut porte = p.GPIO_PORTE;
 	    let mut adc0 = p.ADC0;
 	    let mut adc1= p.ADC1;
 	    let mut sc = p.SYSCTL.constrain();
 	   // let mut pwm1 = p.PWM1;
 	    let mut adc_shim = adc::AdcShim::new(&sc.power_control, required_components{adc0: adc0, adc1:adc1, porte: porte });
 	    adc_shim.set_state(Pin::A0, AdcState::Enabled);
 	    loop{
 	    adc_shim.read(Pin::A0);
 	}

		
}