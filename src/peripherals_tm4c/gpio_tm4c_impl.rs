use tm4c123x_hal::gpio::*;
use tm4c123x_hal::gpio::{gpioa::*, gpiob::*, gpioe::*, gpiof::*};
use tm4c123x_hal::timer;

use tm4c123x_hal::{
    prelude::_embedded_hal_digital_InputPin, prelude::_embedded_hal_digital_OutputPin,
};

use crate::peripherals_generic::gpio as gpio_generic;
use crate::peripherals_generic::gpio::{IntoOutput, IntoInput, Interrupts};
impl IntoInput for PF1<Output<PushPull>> {
    type Input = PF1<Input<PullUp>>;
    
    fn into_input(self) -> Self::Input {
        self.into_pull_up_input()
    }
}

// impl set_bit for PF1<Output<PushPull>> {
//     //type Output = PF1<Ouptput<PullUp>>;
    
//     fn change_bit(&mut self, val: bool) {
//         if(val){
//         self.set_high();
//     }
//         else{
//             self.set_low();
//         }
//        // self
//     }
// }

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
