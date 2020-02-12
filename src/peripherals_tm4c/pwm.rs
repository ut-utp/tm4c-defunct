use core::num::NonZeroU8;
use lc3_traits::peripherals::pwm::{
    Pwm, PwmPin, PwmPinArr, PwmSetDutyError, PwmSetPeriodError, PwmState,
};

extern crate tm4c123x;
use core::sync::atomic::{AtomicBool, Ordering};
use tm4c123x_hal::gpio;
use tm4c123x_hal::gpio::*;
use tm4c123x_hal::gpio::{gpiob::*, gpioe::*};
use tm4c123x_hal::sysctl::Clocks;
use tm4c123x_hal::time::MegaHertz;
use tm4c123x_hal::{prelude::*, Peripherals};
//use core::ops::{Index, IndexMut};

static PWM_SHIM_PINS: PwmPinArr<AtomicBool> =
    PwmPinArr([AtomicBool::new(false), AtomicBool::new(false)]);

pub struct PwmShim {
    states: PwmPinArr<PwmState>,
    duty_cycle: PwmPinArr<u8>,
    //components: Option<required_components>,
    //guards: PwmPinArr<Option<timer::Guard>>,
}

pub struct required_components {
    pub sysctl: tm4c123x::SYSCTL,
    pub portb: tm4c123x::GPIO_PORTB,
    pub pwm0: tm4c123x::PWM0,
    pub pwm1: tm4c123x::PWM1,
}

impl Default for PwmShim {
    fn default() -> Self {
        let p = Peripherals::take().unwrap();
        let mut sc = p.SYSCTL.constrain();

        let pwm_sysctl = Peripherals::take()
            .unwrap()
            .SYSCTL
            .rcgcpwm
            .write(|w| unsafe { w.bits(1) }); //activate pwm0
        let portb_sysctl = Peripherals::take()
            .unwrap()
            .SYSCTL
            .rcgcgpio
            .write(|w| unsafe { w.bits(2) }); //activate port b
        let mut portb = p.GPIO_PORTB.split(&sc.power_control);
        let pwm_output_pin = portb.pb6.into_af_push_pull::<gpio::AF4>(&mut portb.control); //pwm0 pb6
        let p = Peripherals::take().unwrap().SYSCTL;
        let pwm_divider = p
            .rcc
            .write(|w| unsafe { w.bits((p.rcc.read().bits() & !0x000E0000) | (0x00100000)) });
        let p = Peripherals::take().unwrap().PWM0;
        p.ctl.write(|w| unsafe { w.bits(0) });
        p._0_gena.write(|w| unsafe { w.bits(0xC8) });

        Self {
            states: PwmPinArr([PwmState::Disabled; PwmPin::NUM_PINS]),
            duty_cycle: PwmPinArr([0; PwmPin::NUM_PINS]), // start with duty_cycle low
                                                          //components: None
                                                          // guards: PwmPinArr([None, None]),
        }
    }
}

impl PwmShim {
    pub fn new(peripheral_set: required_components) -> Self {
        let sys = peripheral_set.sysctl.constrain();
        let mut portb = peripheral_set.portb.split(&sys.power_control);
        let p = unsafe { &*tm4c123x::SYSCTL::ptr() };
        p.rcgcpwm
            .write(|w| unsafe { w.bits(p.rcgcpwm.read().bits() | 3) }); //activate pwm0
        let pwm_output_pin = portb.pb6.into_af_push_pull::<gpio::AF4>(&mut portb.control); //pwm0 pb6
        let pwm_output_pin2 = portb.pb7.into_af_push_pull::<gpio::AF4>(&mut portb.control); //pwm0 pb6
        let pwm_divider = p
            .rcc
            .write(|w| unsafe { w.bits((p.rcc.read().bits() & !0x000E0000) | (0x00100000)) });
        //let portb_sysctl = peripheral_set.sysctl.rcgcgpio.write(|w| unsafe{w.bits(2)});
        peripheral_set.pwm0.ctl.write(|w| unsafe { w.bits(0) });
        peripheral_set.pwm1.ctl.write(|w| unsafe { w.bits(0) });
        peripheral_set
            .pwm0
            ._0_gena
            .write(|w| unsafe { w.bits(0xC8) });

        peripheral_set
            .pwm1
            ._1_gena
            .write(|w| unsafe { w.bits(0xC8) });

        Self {
            states: PwmPinArr([PwmState::Disabled; PwmPin::NUM_PINS]),
            duty_cycle: PwmPinArr([0; PwmPin::NUM_PINS]), // start with duty_cycle low
                                                          //components: Some(peripheral_set),
                                                          // guards: PwmPinArr([None, None]),
        }
    }
    pub fn get_pin_state(&self, pin: PwmPin) -> PwmState {
        self.states[pin].into()
    }
}
impl Pwm for PwmShim {
    fn set_state(&mut self, pin: PwmPin, state: PwmState) -> Result<(), PwmSetPeriodError> {
        use PwmState::*;
        let x = usize::from(pin);
        match x {
            0 => {
                match state {
                    Enabled(_) => {
                        let p = unsafe { &*tm4c123x::PWM0::ptr() };
                        //let new_duty = ((duty as u32)*period/256);
                        p._0_load.write(|w| unsafe { w.bits(40000) });
                        p._0_cmpa.write(|w| unsafe { w.bits(4000) });
                        p._0_ctl
                            .write(|w| unsafe { w.bits(p._0_ctl.read().bits() | 1) });
                        p.enable
                            .write(|w| unsafe { w.bits(p.enable.read().bits() | 1) });
                    }
                    Disabled => {
                        let p = unsafe { &*tm4c123x::PWM0::ptr() };
                        p.enable
                            .write(|w| unsafe { w.bits(p.enable.read().bits() & !1) });
                    }
                }
            }

            1 => {
                match state {
                    Enabled(_) => {
                        // let p = Peripherals::take().unwrap().PWM1;
                        let p = unsafe { &*tm4c123x::PWM1::ptr() };
                        p._1_load.write(|w| unsafe{w.bits(40000)});
                        p._1_cmpa.write(|w| unsafe{w.bits(4000)});
                        p._1_ctl.write(|w| unsafe{w.bits(p._1_ctl.read().bits() | 1)});
                        p.enable
                            .write(|w| unsafe { w.bits(p.enable.read().bits() | 1) });
                   


                    }
                    Disabled => {
                        let p = unsafe { &*tm4c123x::PWM1::ptr() };
                        //let p = Peripherals::take().unwrap().PWM1;
                        p.enable
                            .write(|w| unsafe { w.bits(p.enable.read().bits() & !1) });
                    }
                }
            }

            _ => {}
        }
        self.states[pin] = state;

        Ok(())
    }

    fn get_state(&self, pin: PwmPin) -> PwmState {
        self.states[pin]
    }

    fn get_pin(&self, pin: PwmPin) -> bool {
        return PWM_SHIM_PINS[pin].load(Ordering::SeqCst);
    }

    fn set_duty_cycle(&mut self, pin: PwmPin, duty: u8) -> Result<(), PwmSetDutyError> {
        match pin {
            P0 => {
                let p = unsafe { &*tm4c123x::PWM0::ptr() };
                p.enable
                    .write(|w| unsafe { w.bits(p.enable.read().bits() & !1) });

                let period = p._0_load.read().bits();

                let new_duty = ((duty as u32) * period / 256);
                p._0_cmpa.write(|w| unsafe { w.bits(new_duty) });
                p.enable
                    .write(|w| unsafe { w.bits(p.enable.read().bits() | 1) });
            }

            P1 => {
                let p = unsafe { &*tm4c123x::PWM1::ptr() };
                p.enable
                    .write(|w| unsafe { w.bits(p.enable.read().bits() & !1) });

                let period = p._1_load.read().bits();

                let new_duty = ((duty as u32) * period / 256);
                p._1_cmpa.write(|w| unsafe { w.bits(new_duty) });
                p.enable
                    .write(|w| unsafe { w.bits(p.enable.read().bits() | 1) });
            }
        }
        self.duty_cycle[pin] = duty;
        Ok(())
    }

    fn get_duty_cycle(&self, pin: PwmPin) -> u8 {
        self.duty_cycle[pin]
    }
}
