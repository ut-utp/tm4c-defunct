use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};

pub struct AdcShim {
    states: PinArr<State>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    Enabled(u8),
    Disabled,
}

pub struct required_Components{
    
}

impl From<State> for AdcState {
    fn from(state: State) -> AdcState {
        use AdcState::*;
        match state {
            State::Enabled(_) => Enabled,
            State::Disabled => Disabled,
        }
    }
}

const INIT_VALUE: u8 = 0;

impl Default for AdcShim {
    fn default() -> Self {
        Self {
            states: PinArr([State::Disabled; Pin::NUM_PINS]),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SetError(StateMismatch);

impl AdcShim {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_value(&mut self, pin: Pin, value: u8) -> Result<(), SetError> {
        use State::*;
        self.states[pin] = match self.states[pin] {
            Enabled(_) => Enabled(value),
            Disabled => return Err(SetError((pin, self.get_state(pin)))),
        };
        Ok(())
    }
}

impl Adc for AdcShim {
    fn set_state(&mut self, pin: Pin, state: AdcState) -> Result<(), ()> {
        use AdcState::*;
        self.states[pin] = match state {
            Enabled => State::Enabled(INIT_VALUE),
            Disabled => State::Disabled,
        };
        Ok(())
    }

    fn get_state(&self, pin: Pin) -> AdcState {
        self.states[pin].into()
    }

    fn read(&self, pin: Pin) -> Result<u8, ReadError> {
        use State::*;
        match self.states[pin] {
            Enabled(value) => Ok(value),
            valueless => Err(ReadError((pin, valueless.into()))),
        }
    }
}