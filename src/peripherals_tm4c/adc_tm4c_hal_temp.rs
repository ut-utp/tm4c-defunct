extern crate embedded_hal as hal;
use embedded_hal::adc;
extern crate tm4c123x;
use tm4c123x::{ADC0};
extern crate nb;
extern crate tm4c123x_hal;
use tm4c123x_hal::{sysctl, gpio::gpioe};
use tm4c123x_hal::{gpio::*, gpio::gpioe::*};
//TODO: Macro this
trait AdcPIn{}

//#[derive(Copy)]

pub struct components{
	pub adc1: Tm4cAdc<Channel_pe3>,
	pub channel1: Channel_pe3,
	pub adc2: Tm4cAdc<Channel_pe2>,
	pub channel2: Channel_pe2,
	pub adc3: Tm4cAdc<Channel_pe1>,
	pub channel3: Channel_pe1,
	pub adc4: Tm4cAdc<Channel_pe0>,
	pub channel4: Channel_pe0,		
}
pub struct Tm4cAdc<T>//<T: Sized>
{
	//adc: ADC0,
	//pins: (T, T, T, T, T, T),   // having some recursive trait bound issues.
								  // just hardcoding pins for now
								  // TODO: Find a way to use generics here
	phantom: 	Option<T>,
	//pins: (PE3<AnalogIn>, PE2<AnalogIn>, PE1<AnalogIn>, 
	//PE0<AnalogIn>, PE5<AnalogIn>, PE4<AnalogIn>),
	// adc1: Tm4cAdc,
	// channel1: Channel_pe3,
	// adc2: Tm4cAdc,
	// channel2: Channel_pe2,
	// adc3: Tm4cAdc,
	// channel3: Channel_pe1,
	// adc4: Tm4cAdc,
	// channel4: Channel_pe0,					  
}
//Macro this
impl adc::Channel<Tm4cAdc<PE3<AnalogIn>>> for PE3<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		0
	}
}
impl adc::Channel<Tm4cAdc<PE2<AnalogIn>>> for PE2<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		1
	}
}
impl adc::Channel<Tm4cAdc<PE1<AnalogIn>>> for PE1<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		2
	}
}
impl adc::Channel<Tm4cAdc<PE0<AnalogIn>>> for PE0<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		3
	}
}
impl adc::Channel<Tm4cAdc<PE5<AnalogIn>>> for PE5<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		8
	}
}
impl adc::Channel<Tm4cAdc<PE4<AnalogIn>>> for PE4<AnalogIn>{
	type ID = u8;
	fn channel() -> Self::ID{
		9
	}
}
//#[derive(Copy)]
pub struct Channel_pe0;
//#[derive(Copy)]
pub struct Channel_pe1;
//#[derive(Copy)]
pub struct Channel_pe2;
//#[derive(Copy)]
pub struct Channel_pe3;
//#[derive(Copy)]
pub struct Channel_pe5;
//#[derive(Copy)]
pub struct Channel_pe4;

impl adc::Channel<Tm4cAdc<Channel_pe3>> for Channel_pe3{
	type ID =u8;
	fn channel() -> Self::ID{
		0
	}
}

impl adc::Channel<Tm4cAdc<Channel_pe2>> for Channel_pe2{
	type ID =u8;
	fn channel() -> Self::ID{
		1
	}
}

impl adc::Channel<Tm4cAdc<Channel_pe1>> for Channel_pe1{
	type ID =u8;
	fn channel() -> Self::ID{
		2
	}
}

impl adc::Channel<Tm4cAdc<Channel_pe0>> for Channel_pe0{
	type ID =u8;
	fn channel() -> Self::ID{
		3
	}
}

impl adc::Channel<Tm4cAdc<Channel_pe5>> for Channel_pe5{
	type ID =u8;
	fn channel() -> Self::ID{
		8
	}
}

impl adc::Channel<Tm4cAdc<Channel_pe4>> for Channel_pe4{
	type ID =u8;
	fn channel() -> Self::ID{
		9
	}
}
 impl From<u32> for Channel_pe0{
   fn from(x: u32)->Self{
     Channel_pe0
   }
 }

  impl From<u32> for Channel_pe1{
   fn from(x: u32)->Self{
     Channel_pe1
   }
 }

  impl From<u32> for Channel_pe2{
   fn from(x: u32)->Self{
     Channel_pe2
   }
 }

  impl From<u32> for Channel_pe3{
   fn from(x: u32)->Self{
     Channel_pe3
   }
 }

  impl From<u32> for Channel_pe5{
   fn from(x: u32)->Self{
     Channel_pe5
   }
 }

  impl From<u32> for Channel_pe4{
   fn from(x: u32)->Self{
     Channel_pe4
   }
 }


 //  impl Into<Channel_pe0> for u32{
 //   fn into(self)->Channel_pe0{
 //   		let mut res = Channel_pe0(-1);
 //   		if(self==0){
 // 	  	res = Channel_pe0(0);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe1> for u32{
 //   fn into(self)->Channel_pe1{
 //   		let mut res = Channel_pe1(-1);
 //   		if(self==1){
 // 	  	res = Channel_pe1(1);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe2> for u32{
 //   fn into(self)->Channel_pe2{
 //   		let mut res = Channel_pe2(-1);
 //   		if(self==2){
 // 	  	res = Channel_pe2(2);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe3> for u32{
 //   fn into(self)->Channel_pe3{
 //   		let mut res = Channel_pe3(-1);
 //   		if(self==3){
 // 	  	res = Channel_pe3(3);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe5> for u32{
 //   fn into(self)->Channel_pe5{
 //   		let mut res = Channel_pe5(-1);
 //   		if(self==5){
 // 	  	res = Channel_pe5(5);
 //    	}
 //    	res
 //   }
 // }

 //  impl Into<Channel_pe4> for u32{
 //   fn into(self)->Channel_pe4{
 //   		let mut res = Channel_pe4(-1);
 //   		if(self==4){
 // 	  	res = Channel_pe4(4);
 //    	}
 //    	res
 //   }
 // }
// impl Into<u32> for PE0<AnalogIn>{
//   fn into(self)->u32{
//     3
//   }
// }

// impl Into<u32> for PE1<AnalogIn>{
//   fn into(self)->u32{
//     2
//   }
// }




impl <U8, Pin> adc::OneShot<Tm4cAdc<Pin>, U8, Pin> for Tm4cAdc<Pin>
where
   Pin: hal::adc::Channel<Tm4cAdc<Pin>, ID=u8>,
   U8:   From<u32>,
{
	type Error = u8;

	fn read(&mut self, _pin: &mut Pin) -> Result<U8, nb::Error<u8>>{
		//let p = *(_pin);
		let adc = unsafe { &*tm4c123x::ADC0::ptr() };
		let channel = Pin::channel();
		match channel{
                0 => {
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() & !0x000F )});
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() + 0 )});
                }
                1 => {
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() & !0x000F )});
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() + 1 )});
                }
                2 => {
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() & !0x000F )});
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() + 2 )});

                }
                3 => {
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() & !0x000F )});
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() + 3 )});
                }
                8 => {
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() & !0x000F )});
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() + 8 )});
                }
                9 => {
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() & !0x000F )});
                    adc.ssmux3.write(|w| unsafe{w.bits(adc.ssmux3.read().bits() + 9 )});
                }


                 _=> {

                }			
		}
        //let p = unsafe { &*tm4c123x::ADC0::ptr() };
        adc.actss.write(|w| unsafe{w.bits((adc.actss.read().bits() | 0x0008 ))});
        adc.pssi.write(|w| unsafe{w.bits(0x0008)});
        while adc.ris.read().bits()&0x08==0 {};
        let out = adc.ssfifo3.read().bits()& 0x0FFF;
        adc.isc.write(|w| unsafe{w.bits(0x00008)});
        adc.actss.write(|w| unsafe{w.bits((adc.actss.read().bits() | !0x0008 ))});
		Ok(out.into())
		// Ok(u8::from(4))
	}

}

impl components

{
	pub fn adc0(adcin: tm4c123x::ADC0, power: &sysctl::PowerControl, 
				pins: (PE3<AnalogIn>, PE2<AnalogIn>, PE1<AnalogIn>, 
				PE0<AnalogIn>, PE5<AnalogIn>, PE4<AnalogIn>)) -> Self{
		//let curradc = adcin;
		//adcin.pc.write(|w| unsafe{w.bits(adcin.pc.read().bits() & !0x0F) });
        //adcin.pc.write(|w| unsafe{w.bits(adcin.pc.read().bits() | 0x01 )});
        sysctl::control_power(
            power, sysctl::Domain::Adc0,
            sysctl::RunMode::Run, sysctl::PowerState::On);
        sysctl::reset(power, sysctl::Domain::Adc0);
        adcin.sspri.write(|w| unsafe{w.bits(0x0123)});
        adcin.actss.write(|w| unsafe{w.bits(adcin.actss.read().bits() & !0x0008) });
        adcin.emux.write(|w| unsafe{w.bits(adcin.emux.read().bits() & !0xF000) });
        adcin.ssmux3.write(|w| unsafe{w.bits(adcin.ssmux3.read().bits() & !0x000F )});
        adcin.ssmux3.write(|w| unsafe{w.bits(adcin.ssmux3.read().bits() + 9 )});
        adcin.ssctl3.write(|w| unsafe{w.bits(0x06)});
        adcin.im.write(|w| unsafe{w.bits(adcin.im.read().bits() & !0x0008 )});
        // let pins = [Pin{channel}]{

        // }
		components{
			// adc: adcin,
			// pins: pins,
			 adc1: Tm4cAdc::<Channel_pe3>{phantom: None},
			 adc2: Tm4cAdc::<Channel_pe2>{phantom: None},
			 adc3: Tm4cAdc::<Channel_pe1>{phantom: None},
			 adc4: Tm4cAdc::<Channel_pe0>{phantom: None},
			 channel1: Channel_pe3,
			 channel2: Channel_pe2,
			 channel3: Channel_pe1,
			 channel4: Channel_pe0,

			//Pins: []

		}
		//adc
	}


}