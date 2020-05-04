use core::marker::PhantomData;
use core::ops::{Index, IndexMut};
use core::sync::atomic::{AtomicBool, Ordering};
use lc3_traits::peripherals::gpio::GpioState::Interrupt;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};
use embedded_hal::digital::v2::{InputPin, OutputPin};

//Generic imports
use crate::peripherals_generic::gpio as gpio_generic;
use crate::peripherals_generic::gpio::{IntoOutput, IntoInput, Interrupts};

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
pub struct physical_pins<'a, G0In, G1In, G2In, G3In, G4In, G5In, G6In, G7In> 
where
	G0In: InputPin + IntoOutput + Interrupts + IntoInput,
    G1In: InputPin + IntoOutput + Interrupts + IntoInput,
    G2In: InputPin + IntoOutput + Interrupts + IntoInput,
    G3In: InputPin + IntoOutput + Interrupts + IntoInput,
    G4In: InputPin + IntoOutput + Interrupts + IntoInput,
    G5In: InputPin + IntoOutput + Interrupts + IntoInput,
    G6In: InputPin + IntoOutput + Interrupts + IntoInput,
    G7In: InputPin + IntoOutput + Interrupts + IntoInput,
 {
    states: GpioPinArr<State>,
    flags: GpioPinArr<Option<&'a AtomicBool>>,
    //mapping: [physical_pin_mappings; 8],
    pin_block: Option<gpio_generic::GpioPinBlock::<G0In, G1In, G2In, G3In, G4In, G5In, G6In, G7In>>,
    //Peripheral_set: Option<&'a mut tm4c123x_hal::Peripherals>,
}

impl <'a, G0, G1, G2, G3, G4, G5, G6, G7> Index<GpioPin> for physical_pins<'_, G0, G1, G2, G3, G4, G5, G6, G7>
where
    G0: InputPin + IntoOutput + Interrupts + IntoInput,
    G1: InputPin + IntoOutput + Interrupts + IntoInput,
    G2: InputPin + IntoOutput + Interrupts + IntoInput,
    G3: InputPin + IntoOutput + Interrupts + IntoInput,
    G4: InputPin + IntoOutput + Interrupts + IntoInput,
    G5: InputPin + IntoOutput + Interrupts + IntoInput,
    G6: InputPin + IntoOutput + Interrupts + IntoInput,
    G7: InputPin + IntoOutput + Interrupts + IntoInput,

{
    type Output = State;

    fn index(&self, pin: GpioPin) -> &State {
        &self.states[pin]
    }
}

impl<'a, G0, G1, G2, G3, G4, G5, G6, G7>  IndexMut<GpioPin> for physical_pins<'_, G0, G1, G2, G3, G4, G5, G6, G7> 
where
    G0: InputPin + IntoOutput + Interrupts + IntoInput,
    G1: InputPin + IntoOutput + Interrupts + IntoInput,
    G2: InputPin + IntoOutput + Interrupts + IntoInput,
    G3: InputPin + IntoOutput + Interrupts + IntoInput,
    G4: InputPin + IntoOutput + Interrupts + IntoInput,
    G5: InputPin + IntoOutput + Interrupts + IntoInput,
    G6: InputPin + IntoOutput + Interrupts + IntoInput,
    G7: InputPin + IntoOutput + Interrupts + IntoInput,

{
    fn index_mut(&mut self, pin: GpioPin) -> &mut State {
        &mut self.states[pin]
    }
}



impl<'a, G0, G1, G2, G3, G4, G5, G6, G7> physical_pins<'_, G0, G1, G2, G3, G4, G5, G6, G7>
where
    G0: InputPin + IntoOutput + IntoInput + Interrupts,
    G1: InputPin + IntoOutput + IntoInput + Interrupts,
    G2: InputPin + IntoOutput + IntoInput + Interrupts,
    G3: InputPin + IntoOutput + IntoInput + Interrupts,
    G4: InputPin + IntoOutput + IntoInput + Interrupts,
    G5: InputPin + IntoOutput + IntoInput + Interrupts,
    G6: InputPin + IntoOutput + IntoInput + Interrupts,
    G7: InputPin + IntoOutput + IntoInput + Interrupts,
{
    fn new(g0: G0, g1: G1, g2: G2, g3: G3, g4: G4, g5: G5, g6: G6, g7: G7) -> Self {
        let mut states_init = [
            State::Input(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
            State::Input(false),
        ];
        Self {
            states: GpioPinArr(states_init),
 			flags:     GpioPinArr([None; GpioPin::NUM_PINS]),
 			pin_block: Some(gpio_generic::GpioPinBlock::new(g0, g1, g2, g3, g4, g5, g6, g7)),

        }
    }
}


impl<'a, G0, G1, G2, G3, G4, G5, G6, G7>   Gpio<'a> for physical_pins<'_, G0, G1, G2, G3, G4, G5, G6, G7>
where

    G0: InputPin + IntoOutput + IntoInput + Interrupts,
    G1: InputPin + IntoOutput + IntoInput + Interrupts,
    G2: InputPin + IntoOutput + IntoInput + Interrupts,
    G3: InputPin + IntoOutput + IntoInput + Interrupts,
    G4: InputPin + IntoOutput + IntoInput + Interrupts,
    G5: InputPin + IntoOutput + IntoInput + Interrupts,
    G6: InputPin + IntoOutput + IntoInput + Interrupts,
    G7: InputPin + IntoOutput + IntoInput + Interrupts,
 {   
    fn set_state(&mut self, pin: GpioPin, state: GpioState) -> Result<(), GpioMiscError>{
        
        use crate::peripherals_generic::gpio::PhysGpioPin;
        let x = usize::from(pin);

        macro_rules! set_state_input {
            ($resp:ident) => {
                        let opt_handle = unsafe{
                                core::mem::replace(
                                    &mut self.pin_block,
                                    core::mem::uninitialized(),
                                )
                            };
                        match opt_handle{
                        Some(mut handle) => {

                            let mut handle2 = unsafe {
                                core::mem::replace(
                                    &mut handle.$resp,
                                    core::mem::uninitialized(),
                                )
                            };


                        match handle2{
                            PhysGpioPin::Output(pin) =>{
                               let inp = pin.into_input();
                            core::mem::replace(
                            &mut handle.$resp,
                            PhysGpioPin::Input(inp),
                            );

                            },
                            _=>{},



                        };

                        core::mem::replace(
                            &mut self.pin_block,
                            Some(handle),
                        );
                        },
                        None =>{},


                    }

                    self[pin] = State::Input(false);

        }
    }
        macro_rules! set_state_output {
            ( $resp:ident) => {
                        let opt_handle = unsafe{
                                core::mem::replace(
                                    &mut self.pin_block,
                                    core::mem::uninitialized(),
                                )
                            };
                        match opt_handle{
                        Some(mut handle) => {

                            let mut handle2 = unsafe {
                                core::mem::replace(
                                    &mut handle.$resp,
                                    core::mem::uninitialized(),
                                )
                            };


                        match handle2{
                            PhysGpioPin::Input(pin) =>{
                               let out = pin.into_output();
                            core::mem::replace(
                            &mut handle.$resp,
                            PhysGpioPin::Output(out),
                            );

                            },
                            _=>{},



                        };

                       // handle2.g0 = pin_handle;


                        core::mem::replace(
                            &mut self.pin_block,
                            Some(handle),
                        );
                        },
                        None =>{},


                    }

                    self[pin] = State::Output(false);

        }
    }
    match state{
        Output=>{
        match x{
            0 => {
                set_state_output!(g0);
            },

            1=>{

                set_state_output!(g1);
            },

            2 =>{
                set_state_output!(g2);
            },
            3 => {
                set_state_output!(g3);
            },
            4 => {
                set_state_output!(g4);
            },
            5 => {
                set_state_output!(g5);
            },
            6 => {
                set_state_output!(g6);
            },
            7 => {
                set_state_output!(g7);
            },
            _=>{}
         }

        }

        Input=>{
        match x{
            0 => {
                set_state_input!(g0);
            },

            1=>{

                set_state_input!(g1);
            },

            2 =>{
                set_state_input!(g2);
            },
            3 => {
                set_state_input!(g3);
            },
            4 => {
                set_state_input!(g4);
            },
            5 => {
                set_state_input!(g5);
            },
            6 => {
                set_state_input!(g6);
            },
            7 => {
                set_state_input!(g7);
            },
            _=>{}
         }

        }
        _=>{}
    }
    	Ok(())

    }

    fn get_state(&self, pin: GpioPin) -> GpioState {
    	//GpioState::Input
        //self.get_pin_state(pin)
        self[pin].into()
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
        // use State::*;

        if let State::Output(_) = self[pin] {
            self[pin] = State::Output(bit);

       use crate::peripherals_generic::gpio::PhysGpioPin;
        let x = usize::from(pin);
       


        macro_rules! set_pin {
            ( $resp:ident) => {

                        let opt_handle = unsafe{
                                core::mem::replace(
                                    &mut self.pin_block,
                                    core::mem::uninitialized(),
                                )
                            };
                        match opt_handle{
                        Some(mut handle) => {

                            let mut handle2 = unsafe {
                                core::mem::replace(
                                    &mut handle.$resp,
                                    core::mem::uninitialized(),
                                )
                            };


                        match handle2{
                            PhysGpioPin::Output(mut pin) =>{
                            if(bit)
                            {
                                pin.set_high();
                            }
                            else {
                                pin.set_low();
                            }
                            core::mem::replace(
                            &mut handle.$resp,
                            PhysGpioPin::Output(pin),
                            );

                            },
                            _=>{},



                        };

                       // handle2.g0 = pin_handle;


                        core::mem::replace(
                            &mut self.pin_block,
                            Some(handle),
                        );
                        self[pin] = State::Output(bit);
                        },
                        None =>{},


                    }

                }
            }


        match x{
            0 => {  

                set_pin!(g0);

            },

            1=>{
                set_pin!(g1);
            },

            2 =>{
                set_pin!(g2);
            },
            3 => {
                set_pin!(g3);
            },
            4 => {
                set_pin!(g4);
            },
            5 => {
                set_pin!(g5);
            },
            6 => {
                set_pin!(g6);
            },
            7 => {
                set_pin!(g7);
            },
            _=>{

            },

        }






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
        // match self.flags[pin] {
        //     Some(flag) => {
        //         let occurred = flag.load(Ordering::SeqCst);
        //         self.interrupts_enabled(pin) && occurred
        //     }
        //     None => unreachable!(),
        // }
        true
    }

    // TODO: decide functionality when no previous flag registered
    fn reset_interrupt_flag(&mut self, pin: GpioPin) {
        // match self.flags[pin] {
        //     Some(flag) => flag.store(false, Ordering::SeqCst),
        //     None => unreachable!(),
        // }
    }

    // TODO: make this default implementation?
    fn interrupts_enabled(&self, pin: GpioPin) -> bool {
        // self.get_state(pin) == Interrupt
        true
    }
}

extern crate embedded_hal;
extern crate tm4c123x_hal;



use tm4c123x_hal::gpio::{gpioa::*, gpiob::*, gpioe::*, gpiof::*};
use tm4c123x_hal::gpio::*;
use tm4c123x_hal::{
    prelude::_embedded_hal_digital_InputPin, prelude::_embedded_hal_digital_OutputPin,
};

impl <'a, G0, G1, G2, G3, G4, G5, G6, G7>  Default for physical_pins<'_, G0, G1, G2, G3, G4, G5, G6, G7> 
where

    G0: InputPin + IntoOutput + IntoInput + Interrupts,
    G1: InputPin + IntoOutput + IntoInput + Interrupts,
    G2: InputPin + IntoOutput + IntoInput + Interrupts,
    G3: InputPin + IntoOutput + IntoInput + Interrupts,
    G4: InputPin + IntoOutput + IntoInput + Interrupts,
    G5: InputPin + IntoOutput + IntoInput + Interrupts,
    G6: InputPin + IntoOutput + IntoInput + Interrupts,
    G7: InputPin + IntoOutput + IntoInput + Interrupts,
{


	fn default() -> Self {
        // let p_st = tm4c123x_hal::Peripherals::take().unwrap();
        // let mut sc = p_st.SYSCTL.constrain();
        // let mut porta = p_st.GPIO_PORTF.split(&sc.power_control);
        // let mut gpioa0 = porta.pf1;
        // gpioa0.set_low();
        // let mut gpioa1 = porta.pf2;
        // gpioa1.set_high();
        // let mut gpioa2 = porta.pf3;
        // gpioa2.set_low();
        // let mut gpioa3 = porta.pf4;
        // gpioa3.set_low();

        // let mut porte = p_st.GPIO_PORTE.split(&sc.power_control);
        // let mut gpioe0 = porte.pe0;
        // //   gpioe0.set_low();            //input - no init state
        // let mut gpioe1 = porte.pe1;
        // //  gpioe1.set_low();
        // let mut gpioe2 = porte.pe2;
        // //  gpioe2.set_low();
        // let mut gpioe3 = porte.pe3;
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
			Self{
            states: GpioPinArr(states_init),
 			flags:     GpioPinArr([None; GpioPin::NUM_PINS]),
 			pin_block: None,
 		}
	}


}





