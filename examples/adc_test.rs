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

use hal::adc as ad ;
use hal::{gpio::*, gpio::gpioe::*};
use lc3_tm4c::peripherals_tm4c::adc::*;
use lc3_tm4c::peripherals_tm4c::adc as adc;

#[entry]
fn main() -> ! {
 	    let p = hal::Peripherals::take().unwrap();
 	     let mut porte = p.GPIO_PORTE;
 	     let mut adc0 = p.ADC0;
 	     let mut adc1= p.ADC1;
 	    let mut sc = p.SYSCTL.constrain();
 	//    // let mut pwm1 = p.PWM1;
 	    let mut adc_shim = adc::AdcShim::new(&sc.power_control, required_components{adc0: adc0, adc1:adc1, porte: porte });
 	    adc_shim.set_state(Pin::A0, AdcState::Enabled);
 	    loop{
 	    adc_shim.read(Pin::A0);
 	}
 //    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
 //    let mut porte = p.GPIO_PORTE.split(&sc.power_control);
 //    let pe3 = porte.pe3.into_analog_input();
 //    let pe2 = porte.pe2.into_analog_input();
 //    let pe1 = porte.pe1.into_analog_input();
 //    let pe0 = porte.pe0.into_analog_input();
 //    let pe5 = porte.pe5.into_analog_input();
 //    let pe4 = porte.pe4.into_analog_input();


 //    let adct = ad::components::adc0(p.ADC0, &sc.power_control, (pe3, pe2, pe1, pe0, pe5, pe4));
	// let mut out = gen_adc::MyAdc::<gen_adc::generic_adc_res,ad::Tm4cAdc<ad::Channel_pe3>, ad::Channel_pe3,
	// 							   ad::Tm4cAdc<ad::Channel_pe2>, ad::Channel_pe2,
	// 							   ad::Tm4cAdc<ad::Channel_pe1>, ad::Channel_pe1,
	// 							   ad::Tm4cAdc<ad::Channel_pe0>, ad::Channel_pe0>::new(
	// 							   adct.adc1, adct.adc2, adct.adc3, adct.adc4,
	// 							   adct.channel1, adct.channel2, adct.channel3, adct.channel4);
								   

	// let mut result = out.read(Pin::A0);
	// 	loop{
	// 		result = out.read(Pin::A0);;
	// 	}
}