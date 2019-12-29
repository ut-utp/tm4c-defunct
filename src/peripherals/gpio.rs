use core::ops::{Index, IndexMut};
use core::sync::atomic::{AtomicBool, Ordering};
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

impl Index<GpioPin> for GpioShim<'_> {
    type Output = State;

    fn index(&self, pin: GpioPin) -> &State {
        &self.states[pin]
    }
}

impl IndexMut<GpioPin> for GpioShim<'_> {
    fn index_mut(&mut self, pin: GpioPin) -> &mut State {
        &mut self.states[pin]
    }
}

impl Default for GpioShim<'_> {
    fn default() -> Self {
        Self {
            states: GpioPinArr([State::Disabled; GpioPin::NUM_PINS]),
            flags: GpioPinArr([None; GpioPin::NUM_PINS]),
        }
    }
}
