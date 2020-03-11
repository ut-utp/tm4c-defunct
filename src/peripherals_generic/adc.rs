extern crate embedded_hal;

use embedded_hal::adc::Channel;
extern crate tm4c123x;
extern crate tm4c123x_hal;
use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};

use embedded_hal::adc::{Channel, OneShot};

pub struct tm4c_impl{

}
pub struct AdcShim {
    states: PinArr<State>,
    //components: Option<required_components>,
}

pub struct adc_input;



struct MyAdc<ONESHOT>{
  one_shot: ONESHOT,

} // 10-bit ADC, with 5 channels

impl <T> From<Channel<T>> for MyAdc<ONESHOT>
where T: 
{


}


impl <T> Adc for MyAdc<ONESHOT> 

where ONESHOT: OneShot<Error=T>,
where T      : From<u8>,
{
    fn set_state(&mut self, pin: Pin, state: AdcState) -> Result<(), ()> {
      Ok(())

    }

    fn get_state(&self, pin: Pin) -> AdcState {
       // self.states[pin].into()
       AdcState::Enabled
    }

  // ADC0_PSSI_R = 0x0008;            // 1) initiate SS3
  // while((ADC0_RIS_R&0x08)==0){};   // 2) wait for conversion done
  //   // if you have an A0-A3 revision number, you need to add an 8 usec wait here
  // result = ADC0_SSFIFO3_R&0xFFF;   // 3) read result
  // ADC0_ISC_R = 0x0008;             // 4) acknowledge completion
  // return result;

    fn read(&self, pin: Pin) -> Result<u8, ReadError> {
      // match pin{
      //   PwmPin::P0 =>{

      //   },
      //   PwmPin::P1 =>{

      //   },
      //   PwmPin::P1 =>{

      //   },
      //   PwmPin::P2 =>{

      //   },
      //   PwmPin::P3 =>{

      //   },
      //    PwmPin::P0 =>{

      //   },      
      // }
     // self.one_shot.read()
     Ok((8 as u8))

    }
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
