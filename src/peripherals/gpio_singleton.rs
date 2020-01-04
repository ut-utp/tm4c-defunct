use core::ops::{Index, IndexMut};
use core::sync::atomic::{AtomicBool, Ordering};
use core::marker::PhantomData;
use lc3_traits::peripherals::gpio::GpioState::Interrupt;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};
extern crate embedded_hal;
use tm4c123x_hal::gpio::{gpioa::*, gpioe::*};
use tm4c123x_hal::gpio::*;
use tm4c123x_hal::gpio::{};
use tm4c123x_hal::{Peripherals, prelude::*};
use tm4c123x_hal::{prelude::_embedded_hal_digital_InputPin, prelude::_embedded_hal_digital_OutputPin};
use tm4c123x_hal::timer;
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

pub enum State2<In, Out>
where
    In: embedded_hal::digital::v2::InputPin,
    Out: embedded_hal::digital::v2::OutputPin,
{
    Input(In),
    Output(Out),
    Interrupt(In),
    Disabled,
}

pub enum PhysicalPins {
    g0  (State2<PA0<Input<PullUp>>, PA0<Output<PushPull>>>),
    g1  (State2<PA1<Input<PullUp>>, PA1<Output<PushPull>>>),
    g2  (State2<PA2<Input<PullUp>>, PA2<Output<PushPull>>>),
    g3  (State2<PA3<Input<PullUp>>, PA3<Output<PushPull>>>),
    g4  (State2<PE0<Input<PullUp>>, PE0<Output<PushPull>>>),
    g5  (State2<PE1<Input<PullUp>>, PE1<Output<PushPull>>>),
    g6  (State2<PE2<Input<PullUp>>, PE2<Output<PushPull>>>),
    g7  (State2<PE3<Input<PullUp>>, PE3<Output<PushPull>>>),
}

pub struct physical_pins<'a>{
    states: GpioPinArr<State>,
    flags:  GpioPinArr<Option<&'a AtomicBool>>,
  //  mapping: Vec<physical_pin_mappings>,
  //  mapping2: Vec<PhysicalPins>,

}
impl Default for physical_pins<'_>{

    fn default()->Self{
     let mut states_init = [State::Output(false), State::Output(false),       State::Output(false), State::Output(false), State::Input(false), State::Input(false), State::Input(false), State::Input(false)];
     Self{
       states: GpioPinArr(states_init),
       flags: GpioPinArr([None; GpioPin::NUM_PINS]),
       // mapping: vec![

       // ],
       //        mapping2: vec![

       // ],
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

    // let pin_mapping = vec![

    //  // physical_pin_mappings::GPIO0(gpioa0),
    //  // physical_pin_mappings::GPIO1(gpioa1),
    //  // physical_pin_mappings::GPIO2(gpioa2),
    //  // physical_pin_mappings::GPIO3(gpioa3),
    //  // physical_pin_mappings::GPIO4(gpioe0),
    //  // physical_pin_mappings::GPIO5(gpioe1),
    //  // physical_pin_mappings::GPIO6(gpioe2),
    //  // physical_pin_mappings::GPIO7(gpioe3),     
    // ];
    // let state_mapping = vec![

    // // State2::Output(gpioa0),
    //     PhysicalPins::g0(State2::<PA0<Input<PullUp>>, PA0<Output<PushPull>>>::Output(gpioa0)),
    //    PhysicalPins::g1(State2::<PA1<Input<PullUp>>, PA1<Output<PushPull>>>::Output(gpioa1)),
    //    PhysicalPins::g2(State2::<PA2<Input<PullUp>>, PA2<Output<PushPull>>>::Output(gpioa2)),
    //    PhysicalPins::g3(State2::<PA3<Input<PullUp>>, PA3<Output<PushPull>>>::Output(gpioa3)),
    //    PhysicalPins::g4(State2::<PE0<Input<PullUp>>, PE0<Output<PushPull>>>::Input(gpioe0)),
    //    PhysicalPins::g5(State2::<PE1<Input<PullUp>>, PE1<Output<PushPull>>>::Input(gpioe1)),
    //    PhysicalPins::g6(State2::<PE2<Input<PullUp>>, PE2<Output<PushPull>>>::Input(gpioe2)),
    //    PhysicalPins::g7(State2::<PE3<Input<PullUp>>, PE3<Output<PushPull>>>::Input(gpioe3)),
    //  // State2::Output(gpioa2),
    //  // State2::Output(gpioa3),
    //  // State2::Output(gpioe0),
    //  // State2::Output(gpioe1),
    //  // State2::Output(gpioe2),
    //  // State2::Output(gpioe3),     
    // ];
       physical_pins{
       states: phys_default.states,
       flags: phys_default.flags,
       //mapping: pin_mapping,
       //mapping2: state_mapping,
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
        use PhysicalPins::*;

        match self[pin] {
            Output(_) => {
                self[pin]=Output(bit);
                let mut x = usize::from(pin);
                match x{
                    0 => {//let y = self.mapping2.remove(0);
                          //let xy;
                            // match y{
                            //      g0(mut vb) => {
                            //         match vb{
                            //             State2::Input(mut ins) => {},
                            //             State2::Output(mut out) => {
                            //                 {
                            //                  if bit { out.set_high(); } else { out.set_low(); };
                            //             };
                            //             self.mapping2.insert(0, PhysicalPins::g0(State2::Output(out)));

                            //            },
                            //             _ => {},

                            //         }
                                    
                            //      }
                            //     _ => {},
                            // }
                            let p = Peripherals::take().unwrap().GPIO_PORTA;

                            if(p.dir.read().bits() & 0x1 == 1){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x1) }
                                else{
                                    w.bits(p.data.read().bits() & !0x1) 
                                }
                            });
                        }

                    },
                     1 => {//let y = self.mapping2.remove(1);
                    //       //let xy;
                    //         match y{
                    //              g1(mut vb) => {
                    //                 match vb{
                    //                     State2::Input(mut ins) => {},
                    //                     State2::Output(mut out) => {
                    //                         {
                    //                          if bit { out.set_high(); } else { out.set_low(); };
                    //                     };
                    //                     self.mapping2.insert(1, PhysicalPins::g1(State2::Output(out)));

                    //                    },
                    //                     _ => {},

                    //                 }
                                    
                    //              }
                    //             _ => {},
                    //         }
                            let p = Peripherals::take().unwrap().GPIO_PORTA;

                            if(p.dir.read().bits() & 0x2 == 2){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x2) }
                                else{
                                    w.bits(p.data.read().bits() & !0x2) 
                                }
                            });
                        }
                        //     if(p.dir.read().bits() & 0x1 == 1){

                        //     p.data.write(|w| unsafe { w.bits(p.data.read().bits() | 0x1) });
                        // }

                    },


                     2 => {//let y = self.mapping2.remove(2);
                    //       //let xy;
                    //         match y{
                    //              g2(mut vb) => {
                    //                 match vb{
                    //                     State2::Input(mut ins) => {},
                    //                     State2::Output(mut out) => {
                    //                         {
                    //                          if bit { out.set_high(); } else { out.set_low(); };
                    //                     };
                    //                     self.mapping2.insert(2, PhysicalPins::g2(State2::Output(out)));

                    //                    },
                    //                     _ => {},

                    //                 }
                                    
                    //              }
                    //             _ => {},
                    //         }

                            let p = Peripherals::take().unwrap().GPIO_PORTA;

                            if(p.dir.read().bits() & 0x4 == 4){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x4) }
                                else{
                                    w.bits(p.data.read().bits() & !0x4) 
                                }
                            });
                        }
                    },
                     3 => {//let y = self.mapping2.remove(3);
                    //       //let xy;
                    //         match y{
                    //              g3(mut vb) => {
                    //                 match vb{
                    //                     State2::Input(mut ins) => {},
                    //                     State2::Output(mut out) => {
                    //                         {
                    //                          if bit { out.set_high(); } else { out.set_low(); };
                    //                     };
                    //                     self.mapping2.insert(3, PhysicalPins::g3(State2::Output(out)));

                    //                    },
                    //                     _ => {},

                    //                 }
                                    
                    //              }
                    //             _ => {},
                    //         }
                            let p = Peripherals::take().unwrap().GPIO_PORTA;

                            if(p.dir.read().bits() & 0x8 == 8){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x8) }
                                else{
                                    w.bits(p.data.read().bits() & !0x8) 
                                }
                            });
                        }

                    },
                     4 => {//let y = self.mapping2.remove(4);
                    //       //let xy;
                    //         match y{
                    //              g4(mut vb) => {
                    //                 match vb{
                    //                     State2::Input(mut ins) => {},
                    //                     State2::Output(mut out) => {
                    //                         {
                    //                          if bit { out.set_high(); } else { out.set_low(); };
                    //                     };
                    //                     self.mapping2.insert(4, PhysicalPins::g4(State2::Output(out)));

                    //                    },
                    //                     _ => {},

                    //                 }
                                    
                    //              }
                    //             _ => {},
                    //         }
                            let p = Peripherals::take().unwrap().GPIO_PORTE;

                            if(p.dir.read().bits() & 0x1 == 1){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x1) }
                                else{
                                    w.bits(p.data.read().bits() & !0x1) 
                                }
                            });
                        }

                    },
                     5 => {//let y = self.mapping2.remove(5);
                    //       //let xy;
                    //         match y{
                    //              g5(mut vb) => {
                    //                 match vb{
                    //                     State2::Input(mut ins) => {},
                    //                     State2::Output(mut out) => {
                    //                         {
                    //                          if bit { out.set_high(); } else { out.set_low(); };
                    //                     };
                    //                     self.mapping2.insert(5, PhysicalPins::g5(State2::Output(out)));

                    //                    },
                    //                     _ => {},

                    //                 }
                                    
                    //              }
                    //             _ => {},
                    //         }
                            let p = Peripherals::take().unwrap().GPIO_PORTE;

                            if(p.dir.read().bits() & 0x2 == 2){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x2) }
                                else{
                                    w.bits(p.data.read().bits() & !0x2) 
                                }
                            });
                        }

                    },
                     6 => {//let y = self.mapping2.remove(6);
                    //       //let xy;
                    //         match y{
                    //              g6(mut vb) => {
                    //                 match vb{
                    //                     State2::Input(mut ins) => {},
                    //                     State2::Output(mut out) => {
                    //                         {
                    //                          if bit { out.set_high(); } else { out.set_low(); };
                    //                     };
                    //                     self.mapping2.insert(6, PhysicalPins::g6(State2::Output(out)));

                    //                    },
                    //                     _ => {},

                    //                 }
                                    
                    //              }
                    //             _ => {},
                    //         }
                            let p = Peripherals::take().unwrap().GPIO_PORTE;

                            if(p.dir.read().bits() & 0x4 == 4){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x4) }
                                else{
                                    w.bits(p.data.read().bits() & !0x4) 
                                }
                            });
                        }

                    },
                     7 => {//let y = self.mapping2.remove(7);
                    //       //let xy;
                    //         match y{
                    //              g7(mut vb) => {
                    //                 match vb{
                    //                     State2::Input(mut ins) => {},
                    //                     State2::Output(mut out) => {
                    //                         {
                    //                          if bit { out.set_high(); } else { out.set_low(); };
                    //                     };
                    //                     self.mapping2.insert(7, PhysicalPins::g7(State2::Output(out)));

                    //                    },
                    //                     _ => {},

                    //                 }
                                    
                    //              }
                    //             _ => {},
                    //         }
                            let p = Peripherals::take().unwrap().GPIO_PORTE;

                            if(p.dir.read().bits() & 0x8 == 8){

                            p.data.write(|w| unsafe { 

                                if(bit)

                                {w.bits(p.data.read().bits() | 0x8) }
                                else{
                                    w.bits(p.data.read().bits() & !0x8) 
                                }
                            });
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
            Input(_) | Disabled => return None,
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
                   // Peripherals::take().unwrap().GPIO_PORTE.dir.write(|w| unsafe);
                 let handle = Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa0.into_pull_up_input();
                // self.mapping2.remove(0);
                // self.mapping2.insert(0, PhysicalPins::g0(State2::Input(handle)));
                },
                G1 => {
                 let handle = Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa1.into_pull_up_input();
                // self.mapping2.remove(1);
                // self.mapping2.insert(1, PhysicalPins::g1(State2::Input(handle)));
               },
                G2 =>{
                let handle = Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa2.into_pull_up_input();
                // self.mapping2.remove(2);
                // self.mapping2.insert(2, PhysicalPins::g2(State2::Input(handle)));
               },
                G3 =>{
                 let handle = Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa3.into_pull_up_input();
                // self.mapping2.remove(3);
                // self.mapping2.insert(3, PhysicalPins::g3(State2::Input(handle)));
            },
                G4 =>{
                 let handle = Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe0.into_pull_up_input();
                // self.mapping2.remove(4);
                // self.mapping2.insert(4, PhysicalPins::g4(State2::Input(handle)));
            },
                G5 =>{
                 let handle=Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe1.into_pull_up_input();
                // self.mapping2.remove(5);
                // self.mapping2.insert(5, PhysicalPins::g5(State2::Input(handle)));

            },
                G6 =>{
                 let handle=Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe2.into_pull_up_input();
                // self.mapping2.remove(6);
                // self.mapping2.insert(6, PhysicalPins::g6(State2::Input(handle)));

            },
                G7 =>{
                 let handle=Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe3.into_pull_up_input();
                // self.mapping2.remove(7);
                // self.mapping2.insert(7, PhysicalPins::g7(State2::Input(handle)));
            },
                }

             },
            Output => {
                self[pin]=State::Output(false);
                match pin{
                G0 => {
                 let handle=Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa0.into_push_pull_output();
                // self.mapping2.remove(0);
                // self.mapping2.insert(0, PhysicalPins::g0(State2::Output(handle)));
            },
                G1 => {
                 let handle=Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa1.into_push_pull_output();
                // self.mapping2.remove(1);
                // self.mapping2.insert(1, PhysicalPins::g1(State2::Output(handle)));
            },
                G2 =>{
                let handle=Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa2.into_push_pull_output();
                // self.mapping2.remove(2);
                // self.mapping2.insert(2, PhysicalPins::g2(State2::Output(handle)));
            },
                G3 =>{
                // let handle=Peripherals::take().unwrap().GPIO_PORTA.split(&sys_init().power_control).pa3.into_push_pull_output();
                // self.mapping2.remove(3);
                // self.mapping2.insert(3, PhysicalPins::g3(State2::Output(handle)));
            },
                G4 =>{
                 let handle=Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe0.into_push_pull_output();
                // self.mapping2.remove(4);
                // self.mapping2.insert(4, PhysicalPins::g4(State2::Output(handle)));
            },
                G5 =>{
                 let handle=Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe1.into_push_pull_output();
                // self.mapping2.remove(5);
                // self.mapping2.insert(5, PhysicalPins::g5(State2::Output(handle)));
            },
                G6 =>{
                 let handle=Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe2.into_push_pull_output();
                // self.mapping2.remove(6);
                // self.mapping2.insert(6, PhysicalPins::g6(State2::Output(handle)));
            },
                G7 =>{
                 let handle=Peripherals::take().unwrap().GPIO_PORTE.split(&sys_init().power_control).pe3.into_push_pull_output();
                // self.mapping2.remove(7);
                // self.mapping2.insert(7, PhysicalPins::g7(State2::Output(handle)));
            },
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
