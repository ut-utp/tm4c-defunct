use core::num::NonZeroU8;
use lc3_traits::peripherals::pwm::{
    Pwm, PwmPin, PwmPinArr, PwmSetDutyError, PwmSetPeriodError, PwmState,
};


use tm4c123x_hal::{Peripherals, prelude::*};
use tm4c123x_hal::time::MegaHertz;
use tm4c123x_hal::sysctl::Clocks;
use tm4c123x_hal::gpio::{gpiob::*, gpioe::*};
use tm4c123x_hal::gpio::*;
use tm4c123x_hal::gpio;
use core::sync::atomic::{AtomicBool, Ordering};
//use core::ops::{Index, IndexMut};

static PWM_SHIM_PINS: PwmPinArr<AtomicBool> =
    PwmPinArr([AtomicBool::new(false), AtomicBool::new(false)]);

pub struct PwmShim {
    states: PwmPinArr<PwmState>,
    duty_cycle: PwmPinArr<u8>,
    //guards: PwmPinArr<Option<timer::Guard>>,
}

impl Default for PwmShim {
    fn default() -> Self {

    let p = Peripherals::take().unwrap();
    let mut sc = p.SYSCTL.constrain();

    let pwm_sysctl = Peripherals::take().unwrap().SYSCTL.rcgcpwm.write(|w| unsafe{w.bits(1)});  //activate pwm0
    let portb_sysctl = Peripherals::take().unwrap().SYSCTL.rcgcgpio.write(|w| unsafe{w.bits(2)}); //activate port b
    let mut portb = p.GPIO_PORTB.split(&sc.power_control);
    let pwm_output_pin = portb.pb6.into_af_push_pull::<gpio::AF4>(&mut portb.control); //pwm0 pb6
    let p = Peripherals::take().unwrap().SYSCTL;
    let pwm_divider = p.rcc.write(|w| unsafe{w.bits((p.rcc.read().bits() & !0x000E0000) | (0x00100000 ))});
    let p = Peripherals::take().unwrap().PWM0; 
    p.ctl.write(|w| unsafe{w.bits(0)});
    p._0_gena.write(|w| unsafe{w.bits(0xC8)});


        Self {
            states: PwmPinArr([PwmState::Disabled; PwmPin::NUM_PINS]),
            duty_cycle: PwmPinArr([0; PwmPin::NUM_PINS]), // start with duty_cycle low
           // guards: PwmPinArr([None, None]),
        }
    }
}

impl PwmShim {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn get_pin_state(&self, pin: PwmPin) -> PwmState {
        self.states[pin].into()
    }
}
impl Pwm for PwmShim {
    fn set_state(&mut self, pin: PwmPin, state: PwmState) -> Result<(), PwmSetPeriodError> {
        use PwmState::*;
        match pin{
        P0 =>{
        match state{
            Enabled(_) => {
                let p = Peripherals::take().unwrap().PWM0; 
                p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() | 1)});

            }
            Disabled =>{
                 let p = Peripherals::take().unwrap().PWM0; 
                p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() & !1)});               
            }
        }
    }

    P1=> {
        match state{
            Enabled(_) => {
                let p = Peripherals::take().unwrap().PWM1; 
                p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() | 1)});

            }
            Disabled =>{
                 let p = Peripherals::take().unwrap().PWM1; 
                p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() & !1)});               
            }
        }

    }
    }
        self.states[pin]=state;

        Ok(())
    }

    fn get_state(&self, pin: PwmPin) -> PwmState {
        self.states[pin]
    }

    fn get_pin(&self, pin: PwmPin) -> bool {
        return PWM_SHIM_PINS[pin].load(Ordering::SeqCst);
    }

    fn set_duty_cycle(&mut self, pin: PwmPin, duty: u8) -> Result<(), PwmSetDutyError> {
        match pin{

        P0 =>{
        let p = Peripherals::take().unwrap().PWM0; 
        p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() & !1)});
        
        let period = p._0_load.read().bits();
        
        let new_duty = ((duty as u32)*period/256);
        p._0_cmpa.write(|w| unsafe{w.bits(new_duty)});
        p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() | 1)});
    }

    P1=>{
        let p = Peripherals::take().unwrap().PWM1; 
        p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() & !1)});
        
        let period = p._1_load.read().bits();
        
        let new_duty = ((duty as u32)*period/256);
        p._1_cmpa.write(|w| unsafe{w.bits(new_duty)});
        p.enable.write(|w| unsafe{w.bits(p.enable.read().bits() | 1)});

    }
    }
        self.duty_cycle[pin] = duty;
        Ok(())
    }

    fn get_duty_cycle(&self, pin: PwmPin) -> u8 {
        self.duty_cycle[pin]
    }
}


