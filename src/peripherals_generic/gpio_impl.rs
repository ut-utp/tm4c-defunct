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

pub enum State {
    Input(bool),
    Output(bool),
    Interrupt(bool),
    Disabled,
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
   // states: GpioPinArr<State>,
    flags: GpioPinArr<Option<&'a AtomicBool>>,
    //mapping: [physical_pin_mappings; 8],
    pin_block: Option<gpio_generic::GpioPinBlock::<G0In, G1In, G2In, G3In, G4In, G5In, G6In, G7In>>,
    //Peripheral_set: Option<&'a mut tm4c123x_hal::Peripherals>,
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
        Self {
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
    	Ok(())

    }

    fn get_state(&self, pin: GpioPin) -> GpioState {
    	GpioState::Input
       // self.get_pin_state(pin)
    }

    fn read(&self, pin: GpioPin) -> Result<bool, GpioReadError> {
        //use State::*;

        // if let Input(b) | Interrupt(b) = self[pin] {
        //     Ok(b)
        // } else {
        //     Err(GpioReadError((pin, self[pin].into())))
        // }
        Ok((true))
    }

    fn write(&mut self, pin: GpioPin, bit: bool) -> Result<(), GpioWriteError> {
        // use State::*;

        // if let Output(_) = self[pin] {
        //     self[pin] = Output(bit);
        //     Ok(())
        // } else {
        //     Err(GpioWriteError((pin, self[pin].into())))
        // }

        Ok(())
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
			Self{
 			flags:     GpioPinArr([None; GpioPin::NUM_PINS]),
 			pin_block: None,
 		}
	}


}


