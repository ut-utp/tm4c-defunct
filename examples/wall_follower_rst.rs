#![no_main]
#![no_std]
//arm-none-eabi-objcopy -O binary <inp> out.axf
extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;

use lc3_traits::peripherals::pwm::{
    Pwm, PwmPin, PwmPinArr,  PwmState,
};

use lc3_tm4c::peripherals_tm4c::pwm;
use lc3_tm4c::peripherals_tm4c::pwm::required_components;
use tm4c123x_hal::sysctl;
use lc3_tm4c::peripherals_tm4c::gpio;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};
use lc3_tm4c::peripherals_tm4c::adc::required_components as adc_comps;
use lc3_tm4c::peripherals_tm4c::adc as adc;
use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};


// Front Sensor  Right Sensor  Situation Action
// Off Off Robot is driving away from wall.  Come back to wall, turn right.
// On  Off Robot is away from wall but headed towards a wall or obstacle.  Turn hard left to get back parallel with the wall.
// Off On  Robot is following the wall.  Drive forward.
// On  On  The robot is at a corner. Turn hard left.

use lc3_baseline_sim::interp;
#[entry]
fn main() -> ! {
    //let builder = interp::InterpreterBuilder::<'_,_, _>::new();
    let p = hal::Peripherals::take().unwrap();
    let mut sc = p.SYSCTL;
    let mut portb = p.GPIO_PORTB;
    let mut portd = p.GPIO_PORTD;
    let mut porta = p.GPIO_PORTF;
    let mut porte = p.GPIO_PORTE;
    let mut adc0 = p.ADC0;
    let mut adc1= p.ADC1;
    let mut pwm0 = p.PWM0;
    let mut pwm1 = p.PWM1;
    let sys = sc.constrain();
    let mut pwm_shim = pwm::PwmShim::new(required_components {
        //sysctl: sc,
        portb: portb,
        portd: portd,
        pwm0: pwm0,
        pwm1: pwm1,
    }, &sys.power_control);

    let mut adc_shim = adc::AdcShim::new(&sys.power_control, adc_comps{adc0: adc0, adc1:adc1, porte: porte });
   // adc_shim.set_state(Pin::A0, AdcState::Enabled);

    // let mut pins = gpio::physical_pins::new(
    //     &sys.power_control,
    //     gpio::required_components {
    //         porta: porta,
    //         porte: porte,
    //     },
    // );
   // pins.set_state(GpioPin::G0, GpioState::Output);
   // pins.set_state(GpioPin::G1, GpioState::Output);
   // pins.set_state(GpioPin::G2, GpioState::Output);
   // pins.set_state(GpioPin::G3, GpioState::Output);

  //pins.set_pin(GpioPin::G0, true);
  // pins.set_pin(GpioPin::G1, true);
  // pins.set_pin(GpioPin::G2, true);
  // pins.set_pin(GpioPin::G3, true);
       //pwm_shim.set_state(
        //PwmPin::P1,
      //  PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()),
    //);

    loop {
   // pins.set_pin(GpioPin::G0, true);
   // pins.set_pin(GpioPin::G1, true);
   // pins.set_pin(GpioPin::G2, true);
   // pins.set_pin(GpioPin::G3, true);
    adc_shim.set_state(Pin::A0, AdcState::Enabled);
    let out1 = adc_shim.read(Pin::A0).unwrap();
    adc_shim.set_state(Pin::A1, AdcState::Enabled);
    let out2 = adc_shim.read(Pin::A1).unwrap();

    if(out1>130){
        pwm_shim.set_state(
        PwmPin::P0,
        PwmState::Enabled(core::num::NonZeroU8::new(125).unwrap()));
        
        pwm_shim.set_state(
        PwmPin::P1,
        PwmState::Enabled(core::num::NonZeroU8::new(125).unwrap()));      
    }

    if(out1<130){
        pwm_shim.set_state(
        PwmPin::P1,
        PwmState::Enabled(core::num::NonZeroU8::new(125).unwrap()));
        
        pwm_shim.set_state(
        PwmPin::P0,
        PwmState::Disabled);      
    }

    // if(out1<130 && out2<130){
    //     pwm_shim.set_state(
    //     PwmPin::P1,
    //     PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()));
        
    //     pwm_shim.set_state(
    //     PwmPin::P0,
    //     PwmState::Disabled);      
    // }

    // if(out1>130 && out2<130){
    //     pwm_shim.set_state(
    //     PwmPin::P0,
    //     PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()));
        
    //     pwm_shim.set_state(
    //     PwmPin::P1,
    //     PwmState::Disabled);      
   // }
  //  let 
    // match out1{
    //   Ok(out) =>{
    //     if (out>110){
    //           pwm_shim.set_state(
    //     PwmPin::P0,
    //     PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()));
    //           pwm_shim.set_state(
    //     PwmPin::P1,
    //     PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()));
    //     }
    //    else{
    //           pwm_shim.set_state(
    //     PwmPin::P0,
    //     PwmState::Disabled,
    // );   
    //   }
    //   },
    //   _=>{},
    // }



    // match out2{
    //   Ok(out) =>{
    //     if (out<110){
    //           pwm_shim.set_state(
    //     PwmPin::P0,
    //     PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()));
    //           pwm_shim.set_state(
    //     PwmPin::P1,
    //     PwmState::Enabled(core::num::NonZeroU8::new(150).unwrap()));
    //     }
    //    else{
    //           pwm_shim.set_state(
    //     PwmPin::P1,
    //     PwmState::Disabled,
    // );
    // // pwm_shim.set_state(
    // //     PwmPin::P1,
    // //     PwmState::Disabled,
    // // );        
    //   }
    //   },
    //   _=>{},
    // }
    // pins.set_pin(GpioPin::G0, false);
    // pins.set_pin(GpioPin::G1, false);
    // pins.set_pin(GpioPin::G2, false);
    // pins.set_pin(GpioPin::G3, false);


    }
}
