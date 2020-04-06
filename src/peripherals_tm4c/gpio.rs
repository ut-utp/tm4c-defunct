use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::sync::atomic::{AtomicBool, Ordering};
use lc3_traits::peripherals::gpio::GpioState::Interrupt;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};
extern crate embedded_hal;
extern crate tm4c123x;
use tm4c123x_hal::gpio::*;
use tm4c123x_hal::gpio::{gpioa::*, gpiob::*, gpioe::*, gpiof::*};
use tm4c123x_hal::timer;
use tm4c123x_hal::{
    prelude::_embedded_hal_digital_InputPin, prelude::_embedded_hal_digital_OutputPin,
};
use tm4c123x_hal::{prelude::*, Peripherals};
extern crate cortex_m;
use cortex_m::interrupt as cortex_int;


static mut GPIO_INTERRPUT_B: i32 = 0;
static mut GPIO_INTERRPUT_F: i32 = 0;

#[derive(Copy, Clone, Debug, PartialEq)]

//static mut peripheral: tm4c123x_hal::Peripherals = Peripherals::take().unwrap();
pub enum State {
    Input(bool),
    Output(bool),
    Interrupt(bool),
    Disabled,
}

pub struct required_components {
    pub portf: tm4c123x::GPIO_PORTF,
    pub portb: tm4c123x::GPIO_PORTB,
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
    GPIO0(PF1<Output<PushPull>>),
    GPIO1(PF2<Output<PushPull>>),
    GPIO2(PF3<Output<PushPull>>),
    GPIO3(PB3<Output<PushPull>>),
    GPIO4(PB4<Input<PullUp>>),
    GPIO5(PB5<Input<PullUp>>),
    GPIO6(PB6<Input<PullUp>>),
    GPIO7(PB7<Input<PullUp>>),
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
    g0(State2<PF1<Input<PullUp>>, PF1<Output<PushPull>>>),
    g1(State2<PF2<Input<PullUp>>, PF2<Output<PushPull>>>),
    g2(State2<PF3<Input<PullUp>>, PF3<Output<PushPull>>>),
    g3(State2<PB3<Input<PullUp>>, PB3<Output<PushPull>>>),
    g4(State2<PB4<Input<PullUp>>, PB4<Output<PushPull>>>),
    g5(State2<PB5<Input<PullUp>>, PB5<Output<PushPull>>>),
    g6(State2<PB6<Input<PullUp>>, PB6<Output<PushPull>>>),
    g7(State2<PB7<Input<PullUp>>, PB7<Output<PushPull>>>),
}

pub struct physical_pins<'a> {
    states: GpioPinArr<State>,
    flags: GpioPinArr<Option<&'a AtomicBool>>,
    //mapping: [physical_pin_mappings; 8],
    mapping2: [PhysicalPins; 8],
    Peripheral_set: Option<&'a mut tm4c123x_hal::Peripherals>,
}
impl Default for physical_pins<'_> {
    fn default() -> Self {
        let mut states_init = [
            State::Output(false),
            State::Output(false),
            State::Output(false),
            State::Output(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
        ];
        //   let p =  hal::Peripherals::take().unwrap();
        //   let mut sc = p.SYSCTL.constrain();
        //   let mut portb = p.GPIO_PORTF.split(&sc.power_control);
        // //  //let timer_output_pin = portb.pb0.into_af_push_pull::<gpio::AF7>(&mut portb.control);
        // // // let uart_tx_pin = portb.pb1.into_af_open_drain::<gpio::AF1, gpio::PullUp>(&mut portb.control);
        //   let mut blue_led = portb.pf2.into_push_pull_output();
        //   blue_led.set_high();

        let p_st = Peripherals::take().unwrap();
        let mut sc = p_st.SYSCTL.constrain();
        let mut portf = p_st.GPIO_PORTF.split(&sc.power_control);
        let mut gpiof1 = portf.pf1.into_push_pull_output();
        gpiof1.set_low();
        let mut gpiof2 = portf.pf2.into_push_pull_output();
        gpiof2.set_high();
        let mut gpiof3 = portf.pf3.into_push_pull_output();
        gpiof3.set_low();
        // let mut gpioa3 = porta.pf4.into_push_pull_output();
        // gpioa3.set_low();

        let mut portb = p_st.GPIO_PORTB.split(&sc.power_control);
        let mut gpiob3 = portb.pb3.into_push_pull_output();
        //   gpioe0.set_low();            //input - no init state
        let mut gpiob4 = portb.pb4.into_pull_up_input();
        //  gpioe1.set_low();
        let mut gpiob5 = portb.pb5.into_pull_up_input();
        //  gpioe2.set_low();
        let mut gpiob6 = portb.pb6.into_pull_up_input();

        let mut gpiob7 = portb.pb7.into_pull_up_input();

        Self {
            states: GpioPinArr(states_init),
            flags: GpioPinArr([None; GpioPin::NUM_PINS]),
            //mapping: [],
            mapping2: ([
                PhysicalPins::g0(State2::<PF1<Input<PullUp>>, PF1<Output<PushPull>>>::Output(
                    gpiof1,
                )),
                PhysicalPins::g1(State2::<PF2<Input<PullUp>>, PF2<Output<PushPull>>>::Output(
                    gpiof2,
                )),
                PhysicalPins::g2(State2::<PF3<Input<PullUp>>, PF3<Output<PushPull>>>::Output(
                    gpiof3,
                )),
                PhysicalPins::g3(State2::<PB3<Input<PullUp>>, PB3<Output<PushPull>>>::Output(
                    gpiob3,
                )),
                PhysicalPins::g4(State2::<PB4<Input<PullUp>>, PB4<Output<PushPull>>>::Input(
                    gpiob4,
                )),
                PhysicalPins::g5(State2::<PB5<Input<PullUp>>, PB5<Output<PushPull>>>::Input(
                    gpiob5,
                )),
                PhysicalPins::g6(State2::<PB6<Input<PullUp>>, PB6<Output<PushPull>>>::Input(
                    gpiob6,
                )),
                PhysicalPins::g7(State2::<PB7<Input<PullUp>>, PB7<Output<PushPull>>>::Input(
                    gpiob7,
                )),
            ]),
            Peripheral_set: None,
        }
    }
}

impl physical_pins<'_> {
    pub fn new<'a>(
        power: &tm4c123x_hal::sysctl::PowerControl,
        peripheral_set: required_components,
    ) -> Self {
        let mut states_init = [
            State::Output(false),
            State::Output(false),
            State::Output(false),
            State::Output(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
        ];
        let p_st = peripheral_set;


        //let mut sc = sys_init();
        // let x = p_st.GPIO_PORTA;
        let mut portf = p_st.portf.split(power);
        let mut gpiof1 = portf.pf1.into_push_pull_output();
        gpiof1.set_low();
        let mut gpiof2 = portf.pf2.into_push_pull_output();
        gpiof2.set_high();
        let mut gpiof3 = portf.pf3.into_push_pull_output();
        gpiof3.set_low();
        // let mut gpioa3 = porta.pf4.into_push_pull_output();
        // gpioa3.set_low();

        let mut portb = p_st.portb.split(power);
        let mut gpiob3 = portb.pb3.into_push_pull_output();
        //   gpioe0.set_low();            //input - no init state
        let mut gpiob4 = portb.pb4.into_pull_up_input();
        //  gpioe1.set_low();
        let mut gpiob5 = portb.pb5.into_pull_up_input();
        //  gpioe2.set_low();
        let mut gpiob6 = portb.pb6.into_pull_up_input();

        let mut gpiob7 = portb.pb7.into_pull_up_input();
        //let mut gpioe4 = porte.pe4;
        //let r1 = gpioe4.into_pull_up_input();
        //let r2 = r1.into_push_pull_output();
        Self {
            states: GpioPinArr(states_init),
            flags: GpioPinArr([None; GpioPin::NUM_PINS]),
            //mapping: [],
            mapping2: ([
                PhysicalPins::g0(State2::<PF1<Input<PullUp>>, PF1<Output<PushPull>>>::Output(
                    gpiof1,
                )),
                PhysicalPins::g1(State2::<PF2<Input<PullUp>>, PF2<Output<PushPull>>>::Output(
                    gpiof2,
                )),
                PhysicalPins::g2(State2::<PF3<Input<PullUp>>, PF3<Output<PushPull>>>::Output(
                    gpiof3,
                )),
                PhysicalPins::g3(State2::<PB3<Input<PullUp>>, PB3<Output<PushPull>>>::Output(
                    gpiob3,
                )),
                PhysicalPins::g4(State2::<PB4<Input<PullUp>>, PB4<Output<PushPull>>>::Input(
                    gpiob4,
                )),
                PhysicalPins::g5(State2::<PB5<Input<PullUp>>, PB5<Output<PushPull>>>::Input(
                    gpiob5,
                )),
                PhysicalPins::g6(State2::<PB6<Input<PullUp>>, PB6<Output<PushPull>>>::Input(
                    gpiob6,
                )),
                PhysicalPins::g7(State2::<PB7<Input<PullUp>>, PB7<Output<PushPull>>>::Input(
                    gpiob7,
                )),
            ]),
            Peripheral_set: None,
        }
    }
}

fn sys_init() -> tm4c123x_hal::sysctl::Sysctl {
    let p_st = Peripherals::take().unwrap();
    let mut sc = p_st.SYSCTL.constrain();
    sc
}

pub trait IntoInput {}

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
    // pub fn new() -> Self {
    //     Self::default()
    // }

    // pub fn new_shared() -> Arc<RwLock<Self>> {
    //     Arc::<RwLock<Self>>::default()
    // }

    /// Sets a pin if it's in input or interrupt mode.
    ///
    /// Returns `Some(())` on success and `None` on failure.
    pub fn set_pin(&mut self, pin: GpioPin, bit: bool) -> Option<()> {
        use physical_pin_mappings::*;
        use PhysicalPins::*;
        use State::*;
         macro_rules! set_pin {
            ($ pin : expr, $resp:path) => {
                        let mut handle = {
                            unsafe {
                                core::mem::replace(
                                    &mut self.mapping2[pin as usize],
                                    core::mem::uninitialized(),
                                )
                            }
                        };
                        match handle {
                            $resp(mut vb) => match vb {
                                State2::Input(mut ins) => {
                                    core::mem::replace(
                                        &mut self.mapping2[pin as usize],
                                        $resp(State2::Input(ins)),
                                    );                                  
                                }
                                State2::Output(mut out) => {
                                    {
                                        if bit {
                                            out.set_high();
                                        } else {
                                            out.set_low();
                                        };
                                    };
                                    core::mem::replace(
                                        &mut self.mapping2[pin as usize],
                                        $resp(State2::Output(out)),
                                    );
                                }
                                _ => {}
                            },
                            _ => {}
                        }

            };


        }       
        match self[pin] {
            _ => {
                self[pin] = Output(bit);
                let mut x = usize::from(pin);
                match x {

                    0 => {
                        set_pin!(0, g0);
                    }
                    1 => {
                        set_pin!(1, g1);
                    }
                    2 => {
                        set_pin!(2, g2);
                    }
                    3 => {
                        set_pin!(3, g3);
                    }
                    4 => {
                        set_pin!(4, g4);
                    }
                    5 => {
                        set_pin!(5, g5);
                    }
                    6 => {
                        set_pin!(6, g6);
                    }
                    7 => {
                        set_pin!(7, g7);
                    }
                    _ => {}
                    //     // physical_pin_mappings::GPIO0(y) => {let mut res  = PF1<Output<>>{_mode: PhantomData};},
                };
            }
            // Interrupt(prev) => {
            //     // Rising edge!
            //     if bit && !prev {
            //         self.raise_interrupt(pin)
            //     }

            //    // Interrupt(bit)
            // }
            // Input(_) | Disabled => return None,
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
        macro_rules! set_state_input {
            ($ pin : expr, $resp:path) => {
                        let mut handle = {
                            unsafe {
                                core::mem::replace(
                                    &mut self.mapping2[pin as usize],
                                    core::mem::uninitialized(),
                                )
                            }
                        };

                          match handle {
                             $resp(x) => {
                                match x{
                                State2::Input(mut ins) => {
                                    let new_out = ins.into_pull_up_input();
                                    core::mem::replace(
                                        &mut self.mapping2[pin as usize],
                                        $resp(State2::Input(new_out)),
                                    );
                                }
                                State2::Output(mut out) => {
                                    let new_out = out.into_pull_up_input();
                                    core::mem::replace(
                                        &mut self.mapping2[pin as usize],
                                        $resp(State2::Input(new_out)),
                                    );
                                }
                                    _=> {}
                                }
                        //        // match $ret{
                        //         // State2::Input(mut ins) => {
                        //         //     let new_out = ins.into_pull_up_input();
                        //         //     core::mem::replace(
                        //         //         &mut self.mapping2[1],
                        //         //         //PhysicalPins::state(State2::Input(new_out)),
                        //         //     );
                        //         // }
                        //         // State2::Output(mut out) => {
                        //         //     let new_out = out.into_pull_up_input();
                        //         //     core::mem::replace(
                        //         //         &mut self.mapping2[1],
                        //         //        // PhysicalPins::state(State2::Input(new_out)),
                        //         //     );
                        //         // }
                        //         // _ => {}
                          },
                             _ => {}
                        // }
                         } 

            };


        }
        match state {
            Input => {
                self[pin] = State::Input(false);
                //self[pin]=State::Output(false);
                let mut x = usize::from(pin);
                match x {
                    0 => {
                        set_state_input!(0, PhysicalPins::g0);
                    }
                    1 => {
                        set_state_input!(1, PhysicalPins::g1);
                    }
                    2 => {
                        set_state_input!(2, PhysicalPins::g2);
                    }
                    3 => {
                        set_state_input!(3, PhysicalPins::g3);
                    }
                    4 => {
                        set_state_input!(4, PhysicalPins::g4);
                    }
                    5 => {
                        set_state_input!(5, PhysicalPins::g5);
                    }
                    6 => {
                        set_state_input!(6, PhysicalPins::g6);
                    }
                    7 => {
                        set_state_input!(7, PhysicalPins::g7);
                    }
                    _ => {}
                }
            }
            Output => {

        macro_rules! set_state_output {
            ($ pin : expr, $resp:path) => {
                        let mut handle = {
                            unsafe {
                                core::mem::replace(
                                    &mut self.mapping2[pin as usize],
                                    core::mem::uninitialized(),
                                )
                            }
                        };

                          match handle {
                             $resp(x) => {
                                match x{
                                State2::Input(mut ins) => {
                                    let new_out = ins.into_push_pull_output();
                                    core::mem::replace(
                                        &mut self.mapping2[pin as usize],
                                        $resp(State2::Output(new_out)),
                                    );
                                }
                                State2::Output(mut out) => {
                                    let new_out = out.into_push_pull_output();
                                    core::mem::replace(
                                        &mut self.mapping2[pin as usize],
                                        $resp(State2::Output(new_out)),
                                    );
                                }
                                    _=> {}
                                }
                          },
                             _ => {}
                        // }
                         } 

            };


        }

                self[pin] = State::Output(false);
                let mut x = usize::from(pin);
                match x {
                    0 => {
                        set_state_output!(0, PhysicalPins::g0);
                    }
                    1 => {
                        set_state_output!(1, PhysicalPins::g1); 
                    }
                    2 => {
                         set_state_output!(2, PhysicalPins::g2);
                    }
                    3 => {
                         set_state_output!(3, PhysicalPins::g3);
                    }
                    4 => {
                        set_state_output!(4, PhysicalPins::g4);
                    }
                    5 => {
                         set_state_output!(5, PhysicalPins::g5);
                    }
                    6 => {
                         set_state_output!(6, PhysicalPins::g6);
                    }
                    7 => {
                         set_state_output!(7, PhysicalPins::g7);
                    }
                    _ => {}
                }
            }
            Interrupt => self[pin] = State::Interrupt(false),
            Disabled => self[pin] = State::Disabled,
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
    fn register_interrupt_flags(&mut self, flags: &'a GpioPinArr<AtomicBool>) {
        // self.flags[pin] = match self.flags[pin] {
        //     None => Some(flag),
        //     Some(_) => unreachable!(),
        // }
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

        unsafe{GPIO_INTERRPUT_B = 0};
    }

    // TODO: make this default implementation?
    fn interrupts_enabled(&self, pin: GpioPin) -> bool {
        self.get_state(pin) == Interrupt
    }
}

use cortex_m_rt_macros::interrupt;
use tm4c123x::Interrupt as interrupt;

#[interrupt]
fn GPIOF(){
    unsafe{GPIO_INTERRPUT_F = 1};

}

#[interrupt]
fn GPIOB(){
    unsafe{GPIO_INTERRPUT_B = 1};
}
// fn SysTick() {
// }
