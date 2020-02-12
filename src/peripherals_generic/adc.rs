extern crate embedded_hal;

use embedded_hal::adc::Channel;
extern crate tm4c123x;
extern crate tm4c123x_hal;


pub struct tm4c_impl{

}

pub struct adc_input;

impl tm4c123x_hal::gpio::InputMode for adc_input{

}


// impl Channel<tm4c123x::ADC0> for tm4c123x_hal::gpio::gpioe::PE0<tm4c123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		3_u8
// 	}
// }

// impl Channel<tm4c123x::ADC0> for tm4c123x_hal::gpio::gpioe::PE1<tm4c123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		4_u8
// 	}
// }

// impl Channel<tm4c123x::ADC0> for tm4c123x_hal::gpio::gpioe::PE2<tm4c123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		5_u8
// 	}
// }

// impl Channel<tm4c123x::ADC0> for tm4c123x_hal::gpio::gpioe::PE3<tm4c123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		0_u8
// 	}
// }

// impl Channel<tm4c123x::ADC0> for tm4c123x_hal::gpio::gpioe::PE4<tm4c123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		1_u8
// 	}
// }

// impl Channel<tm4c123x::ADC0> for tm4c123x_hal::gpio::gpioe::PE5<tm4c123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		2_u8
// 	}
// }
