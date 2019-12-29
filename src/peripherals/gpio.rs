use core::ops::{Index, IndexMut};
use core::sync::atomic::{AtomicBool, Ordering};
use core::marker::PhantomData;
use lc3_traits::peripherals::gpio::GpioState::Interrupt;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};

use tm4c123x_hal::gpio::{gpioa::*, gpioe::*};
use tm4c123x_hal::gpio::*;
use tm4c123x_hal::{Peripherals, prelude::*};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Input(bool),
    Output(bool),
    Interrupt(bool),
    Disabled,
}

impl From<State> for GpioState {
    fn from(state: State) -> GpioState {
        use GpioState::*;

        match state {
            State::Input(_) => Input,
            State::Output(_) => Output,
            State::Interrupt(_) => Interrupt,
            State::Disabled => Disabled,
        }
    }
}

enum physical_pin_mappings {
    GPIO0(PA0<Output<PushPull>>),
    GPIO1(PA1<Output<PushPull>>),
    GPIO2(PA2<Output<PushPull>>),
    GPIO3(PA3<Output<PushPull>>),
    GPIO4(PE0<Input<PullUp>>),
    GPIO5(PE1<Input<PullUp>>),
    GPIO6(PE2<Input<PullUp>>),
    GPIO7(PE3<Input<PullUp>>),
}
pub struct mapping<T>(pub [T; GpioPin::NUM_PINS]);

pub struct physical_pins<'a>{
    states: GpioPinArr<State>,
    flags:  GpioPinArr<Option<&'a AtomicBool>>,
    mapping: Vec<physical_pin_mappings>,

}
impl Default for physical_pins<'_>{

    fn default()->Self{
     let mut states_init = [State::Output(false), State::Output(false),       State::Output(false), State::Output(false), State::Input(false), State::Input(false), State::Input(false), State::Input(false)];
     Self{
       states: GpioPinArr(states_init),
       flags: GpioPinArr([None; GpioPin::NUM_PINS]),
       mapping: vec![

       ],
    }


}
}


fn sys_init() -> tm4c123x_hal::sysctl::Sysctl{
    let p_st = Peripherals::take().unwrap();
    let mut sc = p_st.SYSCTL.constrain();
    sc
}

fn init_pins() -> physical_pins<'static>{
    let mut phys_default = physical_pins::default();
    //println!("?{}", phys_default);
    let p_st = Peripherals::take().unwrap();
    let mut sc = sys_init();
    let mut porta = p_st.GPIO_PORTA.split(&sc.power_control);
    let mut gpioa0 = porta.pa0.into_push_pull_output();
    gpioa0.set_low();
    let mut gpioa1 = porta.pa1.into_push_pull_output();
    gpioa1.set_low();
    let mut gpioa2 = porta.pa2.into_push_pull_output();
    gpioa2.set_low();
    let mut gpioa3 = porta.pa3.into_push_pull_output();
    gpioa3.set_low();
 
    let mut porte = p_st.GPIO_PORTE.split(&sc.power_control);
    let mut gpioe0 = porte.pe0.into_pull_up_input();
 //   gpioe0.set_low();            //input - no init state                        
    let mut gpioe1 = porte.pe1.into_pull_up_input();
  //  gpioe1.set_low();
    let mut gpioe2 = porte.pe2.into_pull_up_input();
  //  gpioe2.set_low();
    let mut gpioe3 = porte.pe3.into_pull_up_input();   
   // gpioe3.set_low();

    let pin_mapping = vec![

     physical_pin_mappings::GPIO0(gpioa0),
     physical_pin_mappings::GPIO1(gpioa1),
     physical_pin_mappings::GPIO2(gpioa2),
     physical_pin_mappings::GPIO3(gpioa3),
     physical_pin_mappings::GPIO4(gpioe0),
     physical_pin_mappings::GPIO5(gpioe1),
     physical_pin_mappings::GPIO6(gpioe2),
     physical_pin_mappings::GPIO7(gpioe3),     
    ];

       physical_pins{
       states: phys_default.states,
       flags: phys_default.flags,
       mapping: pin_mapping
       }

}


pub struct GpioShim<'a> {
    states: GpioPinArr<State>,
    flags: GpioPinArr<Option<&'a AtomicBool>>,
}

impl Index<GpioPin> for physical_pins<'_> {
    type Output = State;

    fn index(&self, pin: GpioPin) -> &State {
        &self.states[pin]
    }
}

impl IndexMut<GpioPin> for physical_pins<'_> {
    fn index_mut(&mut self, pin: GpioPin) -> &mut State {
        &mut self.states[pin]
    }
}


//SOME NOTES: This current implementation approach is very verbose due to a subtle ownership issue. TODO: Wrap into a macro, or take a different approach to the ownership.
// Also VERY IMPORTANT: Make the self[pin] update and the actual physical pin update atomic. To wrap in lock.

impl physical_pins<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    // pub fn new_shared() -> Arc<RwLock<Self>> {
    //     Arc::<RwLock<Self>>::default()
    // }

    /// Sets a pin if it's in input or interrupt mode.
    ///
    /// Returns `Some(())` on success and `None` on failure.
    pub fn set_pin(&mut self, pin: GpioPin, bit: bool) -> Option<()> {
        use State::*;
        use physical_pin_mappings::*;

        match self[pin] {
            Input(_) => {
                self[pin]=Input(bit);
                let mut x = usize::from(pin);
                match x{
                    0 => {let y = self.mapping.remove(0);
                            match y{
                                GPIO0(mut vb) => {if bit { vb.set_high(); } else { vb.set_low(); };},
                                _ => {},
                            }

                    },
                    1 => {let y = self.mapping.remove(1);
                            match y{
                                GPIO1(mut vb) => {
                                
                                if bit { vb.set_high(); } else { vb.set_low(); };
                                self.mapping.insert(1, GPIO1(vb));
                                },
                           
                                _ => {},
                            }


                    },
                    2 => {let y = self.mapping.remove(2);
                            match y{
                                GPIO2(mut vb) => {if bit { vb.set_high(); } else { vb.set_low(); };
                                self.mapping.insert(2, GPIO2(vb));
                               },
                                _ => {},
                            }

                    },
                    3 => {let y = self.mapping.remove(3);
                            match y{
                                GPIO3(mut vb) => {if bit { vb.set_high(); } else { vb.set_low(); };
                                self.mapping.insert(3, GPIO3(vb));
                               },
                                _ => {},
                            }

                    },
                    4 => {let y = self.mapping.remove(4);
                            match y{
                                GPIO4(mut vb) => {},
                                _ => {},
                            }

                    },
                    5 => {let y = self.mapping.remove(5);
                            match y{
                                GPIO5(mut vb) => {},
                                _ => {},
                            }

                    },
                    6 => {let y = self.mapping.remove(6);
                            match y{
                                GPIO6(mut vb) => {},
                                _ => {},
                            }

                    },
                    7 => {let y = self.mapping.remove(7);
                            match y{
                                GPIO7(mut vb) => {},
                                _ => {},
                            }

                    },
                    _ => {},
                    // physical_pin_mappings::GPIO0(y) => {let mut res  = PA0<Output<>>{_mode: PhantomData};},

                };


            },
            Interrupt(prev) => {
                // Rising edge!
                if bit && !prev {
                    self.raise_interrupt(pin)
                }

               // Interrupt(bit)
            }
            Output(_) | Disabled => return None,
         };

        Some(())
    }

    fn raise_interrupt(&self, pin: GpioPin) {
        match self.flags[pin] {
            Some(flag) => flag.store(true, Ordering::SeqCst),
            None => unreachable!(),
        }
    }

    /// Gets the value of a pin.
    ///
    /// Returns `None` when the pin is disabled.
    pub fn get_pin(&self, pin: GpioPin) -> Option<bool> {
        use State::*;

        match self[pin] {
            Input(b) | Output(b) | Interrupt(b) => Some(b),
            Disabled => None,
        }
    }

    /// Gets the state of a pin. Infallible.
    pub fn get_pin_state(&self, pin: GpioPin) -> GpioState {
        self[pin].into()
    }
}

impl<'a> Gpio<'a> for physical_pins<'a> {
    fn set_state(&mut self, pin: GpioPin, state: GpioState) -> Result<(), GpioMiscError> {
        use GpioState::*;
            match state {
            Input => {
            self[pin]=State::Input(false);
                //self[pin]=State::Output(false);
                match pin{
                G0 => {
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa0.into_pull_up_input();},
                G1 => {
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa1.into_pull_up_input();},
                G2 =>{
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa2.into_pull_up_input();},
                G3 =>{
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa3.into_pull_up_input();},
                G4 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe0.into_pull_up_input();},
                G5 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe1.into_pull_up_input();},
                G6 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe2.into_pull_up_input();},
                G7 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe3.into_push_pull_output();},
                }

             },
            Output => {
                self[pin]=State::Output(false);
                match pin{
                G0 => {
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa0.into_push_pull_output();},
                G1 => {
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa1.into_push_pull_output();},
                G2 =>{
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa2.into_push_pull_output();},
                G3 =>{
                Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa3.into_push_pull_output();},
                G4 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe0.into_push_pull_output();},
                G5 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe1.into_push_pull_output();},
                G6 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe2.into_push_pull_output();},
                G7 =>{
                Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe3.into_push_pull_output();},
                }
            },
            Interrupt => {
                self[pin]=State::Interrupt(false)
            },
            Disabled => {
                self[pin]=State::Disabled
            },
        };

        Ok(())
    }

    fn get_state(&self, pin: GpioPin) -> GpioState {
        self.get_pin_state(pin)
    }

    fn read(&self, pin: GpioPin) -> Result<bool, GpioReadError> {
        use State::*;

        if let Input(b) | Interrupt(b) = self[pin] {
            Ok(b)
        } else {
            Err(GpioReadError((pin, self[pin].into())))
        }
    }

    fn write(&mut self, pin: GpioPin, bit: bool) -> Result<(), GpioWriteError> {
        use State::*;

        if let Output(_) = self[pin] {
            self[pin] = Output(bit);
            Ok(())
        } else {
            Err(GpioWriteError((pin, self[pin].into())))
        }
    }

    // TODO: decide functionality when no previous flag registered
    fn register_interrupt_flag(&mut self, pin: GpioPin, flag: &'a AtomicBool) {
        self.flags[pin] = match self.flags[pin] {
            None => Some(flag),
            Some(_) => unreachable!(),
        }
    }

    fn interrupt_occurred(&self, pin: GpioPin) -> bool {
        match self.flags[pin] {
            Some(flag) => {
                let occurred = flag.load(Ordering::SeqCst);
                self.interrupts_enabled(pin) && occurred
            }
            None => unreachable!(),
        }
    }

    // TODO: decide functionality when no previous flag registered
    fn reset_interrupt_flag(&mut self, pin: GpioPin) {
        match self.flags[pin] {
            Some(flag) => flag.store(false, Ordering::SeqCst),
            None => unreachable!(),
        }
    }

    // TODO: make this default implementation?
    fn interrupts_enabled(&self, pin: GpioPin) -> bool {
        self.get_state(pin) == Interrupt
    }
}
