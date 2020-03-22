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

impl Into<u8> for generic_adc_res{
  fn into(self)->u8{
    let generic_adc_res(res) = self;
    res as u8
  }
}

// impl Into<u8> for u32{
//   fn into(self)->u8{
//     self as u8
//   }
// }

pub struct adc_input;



pub struct MyAdc < U,
                  ADC1      : OneShot<ADC1, U, C1>,
                  C1        : Channel<ADC1, ID=u8>,
                  ADC2      : OneShot<ADC2, U, C2>,
                  C2        : Channel<ADC2, ID=u8>,
                  ADC3      : OneShot<ADC3, U, C3>,
                  C3        : Channel<ADC3, ID=u8>,
                  ADC4      : OneShot<ADC4, U, C4>,
                  C4        : Channel<ADC4, ID=u8>,
                   >{
  //one_shot: ONESHOT,
  channel1: C1,  
  channel2: C2,
  channel3: C3,
  channel4: C4,
  adc1    : ADC1,
  adc2    : ADC2,
  adc3    : ADC3,
  adc4    : ADC4,
  _ch:    Option<U>     // to satisfy unconstrained trait bounds

} 


impl <U, ADC1, ADC2, ADC3, ADC4, C1, C2, C3, C4> MyAdc< U, ADC1, C1, ADC2, C2, ADC3, C3, ADC4, C4> 
where //ONESHOT: OneShot<ONESHOT, U, ADC1>,  
        ADC1      : OneShot<ADC1, U, C1>,
        C1        : Channel<ADC1, ID=u8>,
        ADC2      : OneShot<ADC2, U, C2>,
        C2        : Channel<ADC2, ID=u8>,
        ADC3      : OneShot<ADC3, U, C3>,
        C3        : Channel<ADC3, ID=u8>,
        ADC4      : OneShot<ADC4, U, C4>,
        C4        : Channel<ADC4, ID=u8>,
        U         : Into<u8>

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
          res = self.adc1.read(&mut(self.channel1));
          match res{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}
          }
          //res = u32::from(res.unwrap());
        },
        Pin::A1 =>{

          let res2;
           res2=self.adc2.read(&mut(self.channel2));
          match res2{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}

          }
        },  
        Pin::A2 =>{
          unsafe{
          let res3;
          res3 = self.adc3.read(&mut(self.channel3));
          match res3{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}

          }
        }
        },  
        Pin::A3 =>{
          unsafe{
           let res4;
           res4 = self.adc4.read(&mut(self.channel4));
          match res4{
            Ok(out) =>{
              ret = (out.into() as u8);
            },
            _=>{}

          }
        };
         }, 

         _=>{},
       


      }

     Ok(ret)

    }
}

impl <U, ADC1, ADC2, ADC3, ADC4, C1, C2, C3, C4> Default for MyAdc< U, ADC1, C1, ADC2, C2, ADC3, C3, ADC4, C4> 
where //ONESHOT: OneShot<ONESHOT, U, ADC1>,  
        ADC1      : OneShot<ADC1, U, C1>,
        C1        : Channel<ADC1, ID=u8>,
        ADC2      : OneShot<ADC2, U, C2>,
        C2        : Channel<ADC2, ID=u8>,
        ADC3      : OneShot<ADC3, U, C3>,
        C3        : Channel<ADC3, ID=u8>,
        ADC4      : OneShot<ADC4, U, C4>,
        C4        : Channel<ADC4, ID=u8>,
        U         : Into<u8>
{
    fn default() -> Self {
      unimplemented!()
    }
}


impl <U, ADC1, ADC2, ADC3, ADC4, C1, C2, C3, C4> MyAdc< U, ADC1, C1, ADC2, C2, ADC3, C3, ADC4, C4> 
where //ONESHOT: OneShot<ONESHOT, U, ADC1>,  
        ADC1      : OneShot<ADC1, U, C1>,
        C1        : Channel<ADC1, ID=u8>,
        ADC2      : OneShot<ADC2, U, C2>,
        C2        : Channel<ADC2, ID=u8>,
        ADC3      : OneShot<ADC3, U, C3>,
        C3        : Channel<ADC3, ID=u8>,
        ADC4      : OneShot<ADC4, U, C4>,
        C4        : Channel<ADC4, ID=u8>,
        U         : Into<u8>
     // ADC2      : Channel<ONESHOT, ID=u8>,
{
    pub fn new( adc1: ADC1, adc2: ADC2, adc3:ADC3, adc4: ADC4, c1: C1, c2: C2, c3: C3, c4:C4) -> Self {
      MyAdc{
        //one_shot: adc,
        channel1: c1,
        channel2: c2,
        channel3: c3,
        channel4: c4,
        adc1    : adc1,
        adc2    : adc2,
        adc3    : adc3,
        adc4    : adc4,

        _ch: None
      }
    }
}

// impl Channel<tm4ADC123x::ADC0> for tm4ADC123x_hal::gpio::gpioe::PE0<tm4ADC123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		3_u8
// 	}
// }

// impl Channel<tm4ADC123x::ADC0> for tm4ADC123x_hal::gpio::gpioe::PE1<tm4ADC123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		4_u8
// 	}
// }

// impl Channel<tm4ADC123x::ADC0> for tm4ADC123x_hal::gpio::gpioe::PE2<tm4ADC123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		5_u8
// 	}
// }

// impl Channel<tm4ADC123x::ADC0> for tm4ADC123x_hal::gpio::gpioe::PE3<tm4ADC123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		0_u8
// 	}
// }

// impl Channel<tm4ADC123x::ADC0> for tm4ADC123x_hal::gpio::gpioe::PE4<tm4ADC123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		1_u8
// 	}
// }

// impl Channel<tm4ADC123x::ADC0> for tm4ADC123x_hal::gpio::gpioe::PE5<tm4ADC123x_hal::gpio::Input<adc_input>>{

// 	type ID = u8;

// 	fn channel() -> Self::ID{
// 		2_u8
// 	}
// }
