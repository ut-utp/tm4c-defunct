extern crate embedded_hal;
use embedded_hal as hal;
use hal::Pwm as hal_pwm;
use hal::PwmPin as hal_pwm_pin;


use lc3_traits::peripherals::pwm::{
    Pwm, PwmPin, PwmPinArr, PwmState,
};

pub trait Channel{}

pub struct Lc3Pwm <PWM: hal_pwm, Pin: hal_pwm_pin<Duty=u32>+Sized> {
	pwm: PWM,
	pins: [Pin; 2],
	pin_states:  [PwmState; 2],// PwmPinArr<PwmState>.
	pwm_duties:  [u32; 2],
	
}


// impl <PWM, Pin> Lc3Pwm<PWM, Pin>
// where PWM: hal_pwm,
//       Pin: Channel//hal_pwm_pin<Duty=u32>+Sized

// {
// 	fn new(pins: (Pin, Pin), pwm: PWM) -> Self{
// 		let (pin1, pin2) = pins;
// 		Lc3Pwm{
// 			pwm: pwm,
// 			pins: [pin1, pin2],
// 			pin_states: [PwmState::Disabled; 2],
// 			pwm_duties: [200; 2],

// 		}

// 	}

// }
// impl From<u32> for Channel{

//     fn from(x: ) -> Self {
//         //CliError::IoError(error)
//     }

// }


impl <PWM, Pin, C, D, T> Pwm for Lc3Pwm<PWM, Pin> 
where PWM: hal_pwm<Channel=C, Duty=D, Time=T> + From<u8>,
      Pin: hal_pwm_pin<Duty=u32>+Sized,
      T  : From<u8>,
      C  : From<u8>,
      D  : From<u8>,
{
    fn set_state(&mut self, pin: PwmPin, state: PwmState){
       // use PwmState::*;
       match pin{
       	PwmPin::P0 =>{
        match state{
        	PwmState::Enabled(x) => {
        		//self.pwm.enable()
        		//self.pins[0].enable();
        		self.pin_states[0] = state;
        		self.pwm.enable(0.into());
        	},
        	PwmState::Disabled =>{
        		//self.pins[1].enable();
        		self.pin_states[0] = state;
        		self.pwm.disable(0.into());
        	},
        }
    },
       	PwmPin::P1 =>{
        match state{
        	PwmState::Enabled(x) => {
        		//self.pwm.enable()
        		//self.pins[1].disable();
        		self.pin_states[1] = state;
        		self.pwm.enable(1.into());
        	},
        	PwmState::Disabled =>{
        		//self.pins[1].disable();
        		self.pin_states[1] = state;
        		self.pwm.enable(1.into());

        	},
        }
    }    
    }
    	
       // Ok(())
    }

    fn get_state(&self, pin: PwmPin) -> PwmState {
        self.pin_states[usize::from(pin)]
       //unimplemented!()
    }

    //Questionable feature. Don't think it makes sense to get physical value of pin Embedded hsl doesn't have
    //this either
    // fn get_pin(&self, pin: PwmPin) -> bool {
    // 	unimplemented!()
    //    // true
    // }

    fn set_duty_cycle(&mut self, pin: PwmPin, duty: u8) {
        match pin{
       	PwmPin::P0 =>{
          self.pwm.set_duty(0.into(), duty.into());
          self.pwm_duties[0] = duty as u32;
        }
    //},
       	PwmPin::P1 =>{

          self.pwm.set_duty(1.into(), duty.into());
          self.pwm_duties[1] = duty as u32;
    }    
    }
      //  Ok(())
    }

    fn get_duty_cycle(&self, pin: PwmPin) -> u8 {
    	match pin{
    		PwmPin::P0 => {
    			self.pwm_duties[0] as u8

    		},
    		PwmPin::P1 => {
    			self.pwm_duties[1] as u8

    		},


    	}
        
    }
}



impl <PWM, Pin> Default for Lc3Pwm<PWM, Pin>
where PWM: hal_pwm,
      Pin: hal_pwm_pin<Duty=u32>+Sized,
 {
    fn default() -> Self {
       //  let p = Peripherals::take().unwrap();
       //  let mut sc = p.SYSCTL.constrain();

       //  let pwm_sysctl = Peripherals::take()
       //      .unwrap()
       //      .SYSCTL
       //      .rcgcpwm
       //      .write(|w| unsafe { w.bits(1) }); //activate pwm0
       //  let portb_sysctl = Peripherals::take()
       //      .unwrap()
       //      .SYSCTL
       //      .rcgcgpio
       //      .write(|w| unsafe { w.bits(2) }); //activate port b
       //  let mut portb = p.GPIO_PORTB.split(&sc.power_control);
       //  let pwm_output_pin = portb.pb6.into_af_push_pull::<gpio::AF4>(&mut portb.control); //pwm0 pb6
       // // let pb6 = peripheral_set.pb6.into_af_push_pull::<gpio::AF4>(power); //pwm0 pb6
       //  let pb7 = portb.pb7.into_af_push_pull::<gpio::AF4>(&mut portb.control); //pwm0 pb6
       //  let p = Peripherals::take().unwrap().SYSCTL;
       //  let pwm_divider = p
       //      .rcc
       //      .write(|w| unsafe { w.bits((p.rcc.read().bits() & !0x000E0000) | (0x00100000)) });
       //  let p = Peripherals::take().unwrap().PWM0;
       //  p.ctl.write(|w| unsafe { w.bits(0) });
       //  p._0_gena.write(|w| unsafe { w.bits(0xC8) });

       //  Self {
       //      states: PwmPinArr([PwmState::Disabled; PwmPin::NUM_PINS]),
       //      duty_cycle: PwmPinArr([0; PwmPin::NUM_PINS]), // start with duty_cycle low
       //      pwm_physical_pins: [PhysicalPins::p0(pwm_output_pin), PhysicalPins::p1(pb7)],
       //                                                    //components: None
       //                                                    // guards: PwmPinArr([None, None]),
       //  }
       unimplemented!()
    }
}

