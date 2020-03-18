extern crate embedded_hal;
use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};

use embedded_hal::adc::{Channel, OneShot};

pub struct generic_adc_res(u32);
// pub struct AdcShim {
//    // states: PinArr<State>,
//     //components: Option<required_components>,
// }


impl From<u32> for generic_adc_res{
  fn from(x: u32)->Self{
    generic_adc_res(x)
  }
}

impl Into<u32> for generic_adc_res{
  fn into(self)->u32{
    let generic_adc_res(res) = self;
    res
  }
}

pub struct adc_input;



pub struct MyAdc <ONESHOT, U, T: Channel<ONESHOT, ID=u8>>{//<ONESHOT: OneShot<ONESHOT, T, R: Channel<ONESHOT, ID=u8>, Error= T>>{
  one_shot: ONESHOT,
  _channel: Option<T>,  // These 2 are useless unused fields put just
  _ch:    Option<U>     // to satisfy unconstrained trait bounds

} 


impl <ONESHOT, U, T> MyAdc<ONESHOT, U, T> 
where ONESHOT: OneShot<ONESHOT, U, T>,  
      U      : Into<u32>+From<u32>,
      T      : Channel<ONESHOT, ID=u8> + From<u32>,


{
    fn set_state(&mut self, pin: Pin, state: AdcState) -> Result<(), ()> {
      Ok(())

    }

    fn get_state(&self, pin: Pin) -> AdcState {
       AdcState::Enabled
    }

    pub fn read(&mut self, pin: Pin) -> Result<u8, ReadError> {
      let res;
      let mut ret: u8 = 8;
      match pin{
        Pin::A0 =>{

          res = self.one_shot.read(&mut(0.into()));
          match res{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}

          }
          //res = u32::from(res.unwrap());
        },
        Pin::A1 =>{
          res = self.one_shot.read(&mut(1.into()));
          match res{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}

          }
        },  
        Pin::A2 =>{
          res = self.one_shot.read(&mut(2.into()));
          match res{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}

          }
        },  
        Pin::A3 =>{
          res = self.one_shot.read(&mut(3.into()));
          match res{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}

          }
        },  


      }

     Ok(ret)

    }
}

impl <ONESHOT, U, T> MyAdc<ONESHOT, U, T> 
where ONESHOT: OneShot<ONESHOT, U, T>,  
      T      : Channel<ONESHOT, ID=u8>,
{
    fn default() -> Self {
      unimplemented!()
    }
}


impl <ONESHOT, U, T> MyAdc<ONESHOT, U, T> 
where ONESHOT: OneShot<ONESHOT, U, T>,  
      T      : Channel<ONESHOT, ID=u8>,
{
    pub fn new(adc: ONESHOT) -> Self {
      MyAdc{
        one_shot: adc,
        _channel: None,
        _ch: None
      }
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
