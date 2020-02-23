extern crate embedded_hal;
extern crate tm4c123x_hal;


use embedded_hal::digital::v2::{InputPin, OutputPin};
use tm4c123x_hal::gpio::{gpioa::*, gpiob::*, gpioe::*, gpiof::*};
use tm4c123x_hal::gpio::*;
use tm4c123x_hal::{
    prelude::_embedded_hal_digital_InputPin, prelude::_embedded_hal_digital_OutputPin,
};


pub trait IntoInput: OutputPin + Sized{
    type Input: InputPin + IntoOutput<Output = Self>;

    fn into_input(self) -> Self::Input;
}

pub trait IntoOutput: InputPin + Sized{
    type Output: OutputPin + IntoInput<Input = Self>;
    
    fn into_output(self) -> Self::Output;
}

pub trait Interrupts: InputPin {
    fn enable_interrupts(&mut self);
    fn disable_interrupts(&mut self);
}

// let a = gpioe0.into_pull_up_input();

// a.into_output();

// PE0::into_output(a)

impl IntoInput for PF1<Output<PushPull>> {
    type Input = PF1<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PF1<Input<PullUp>> {
    type Output = PF1<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}

impl IntoInput for PF2<Output<PushPull>> {
    type Input = PF2<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PF2<Input<PullUp>> {
    type Output = PF2<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}

impl IntoInput for PF3<Output<PushPull>> {
    type Input = PF3<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PF3<Input<PullUp>> {
    type Output = PF3<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}

impl IntoInput for PF4<Output<PushPull>> {
    type Input = PF4<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PF4<Input<PullUp>> {
    type Output = PF4<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}




impl IntoInput for PE0<Output<PushPull>> {
    type Input = PE0<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PE0<Input<PullUp>> {
    type Output = PE0<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}

impl IntoInput for PE1<Output<PushPull>> {
    type Input = PE1<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PE1<Input<PullUp>> {
    type Output = PE1<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}

impl IntoInput for PE2<Output<PushPull>> {
    type Input = PE2<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PE2<Input<PullUp>> {
    type Output = PE2<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}

impl IntoInput for PE3<Output<PushPull>> {
    type Input = PE3<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

impl IntoOutput for PE3<Input<PullUp>> {
    type Output = PE3<Output<PushPull>>;
    
    fn into_output(self) -> Self::Output {
        self.into_push_pull_output()
    }
}



pub enum PhysGpioPin<I: InputPin, O: OutputPin> {
    Input(I),
    Output(O),
}

// One way:
/*
struct GpioPinBlock<G0Out, G0In, G1Out, G1In, ...>
where
    G0Out: OutputPin + IntoInput<Input = G0In>,
    G0In: InputPin + Interrupts + IntoOutput<Output = G0Out>,
{
    
}
*/

// Another way:
pub struct GpioPinBlock<G0In, G1In, G2In, G3In, G4In, G5In, G6In, G7In>
where
    G0In: InputPin + IntoOutput + IntoInput + Interrupts,
    G1In: InputPin + IntoOutput + IntoInput + Interrupts,
    G2In: InputPin + IntoOutput + IntoInput + Interrupts,
    G3In: InputPin + IntoOutput + IntoInput + Interrupts,
    G4In: InputPin + IntoOutput + IntoInput + Interrupts,
    G5In: InputPin + IntoOutput + IntoInput + Interrupts,
    G6In: InputPin + IntoOutput + IntoInput + Interrupts,
    G7In: InputPin + IntoOutput + IntoInput + Interrupts,
{
   pub g0: PhysGpioPin<G0In, <G0In as IntoOutput>::Output>,
   pub g1: PhysGpioPin<G1In, <G1In as IntoOutput>::Output>,
   pub g2: PhysGpioPin<G2In, <G2In as IntoOutput>::Output>,
   pub g3: PhysGpioPin<G3In, <G3In as IntoOutput>::Output>,
   pub g4: PhysGpioPin<G4In, <G4In as IntoOutput>::Output>,
   pub g5: PhysGpioPin<G5In, <G5In as IntoOutput>::Output>,
   pub g6: PhysGpioPin<G6In, <G6In as IntoOutput>::Output>,
   pub g7: PhysGpioPin<G7In, <G7In as IntoOutput>::Output>,
}

impl<G0, G1, G2, G3, G4, G5, G6, G7> GpioPinBlock<G0, G1, G2, G3, G4, G5, G6, G7>
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
    pub fn new(g0: G0, g1: G1, g2: G2, g3: G3, g4: G4, g5: G5, g6: G6, g7: G7) -> Self {
        Self {
            g0: PhysGpioPin::Input(g0),
            g1: PhysGpioPin::Input(g1),
            g2: PhysGpioPin::Input(g2),
            g3: PhysGpioPin::Input(g3),
            g4: PhysGpioPin::Input(g4),
            g5: PhysGpioPin::Input(g5),
            g6: PhysGpioPin::Input(g6),
            g7: PhysGpioPin::Input(g7),
        }
    }
}
