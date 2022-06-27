// #![feature(generic_associated_types)]

//! Impl of the UTP platform for the TI TM4C.
//!
//! TODO!

// TODO: forbid
// #![warn(
//     bad_style,
//     const_err,
//     dead_code,
//     improper_ctypes,
//     non_shorthand_field_patterns,
//     no_mangle_generic_items,
//     overflowing_literals,
//     path_statements,
//     patterns_in_fns_without_body,
//     private_in_public,
//     unconditional_recursion,
//     unused,
//     unused_allocation,
//     unused_lifetimes,
//     unused_comparisons,
//     unused_parens,
//     while_true
// )]
// TODO: deny
// #![warn(
//     missing_debug_implementations,
//     // intra_doc_link_resolution_failure,
//     missing_docs,
//     unsafe_code,
//     trivial_casts,
//     trivial_numeric_casts,
//     unused_extern_crates,
//     unused_import_braces,
//     unused_qualifications,
//     unused_results,
//     rust_2018_idioms
// )]
#![doc(test(attr(deny(rust_2018_idioms, warnings))))]
#![doc(html_logo_url = "")] // TODO!

#![no_std]
#![no_main]

extern crate panic_halt as _;
extern crate tm4c123x_hal as hal;

mod generic_gpio;

use core::convert::Infallible;

use cortex_m_rt::entry;
use hal::prelude::*;

use lc3_traits::control::rpc::{
    SimpleEventFutureSharedState, Device, RequestMessage, ResponseMessage
};
use lc3_baseline_sim::interp::{
    Interpreter,
    PeripheralInterruptFlags, OwnedOrRef, MachineState,
};
use lc3_baseline_sim::sim::Simulator;
// use lc3_traits::peripherals::gpio::{GpioPinArr, GpioMiscError};
use lc3_traits::peripherals::stubs::{PwmStub, ClockStub, GpioStub, TimersStub, AdcStub};
use lc3_traits::peripherals::{
    PeripheralSet,
    stubs::{
        /*PeripheralsStub,*/ InputStub, OutputStub
    },
};
use lc3_device_support::{
    memory::PartialMemory,
    rpc::{
        transport::uart_simple::UartTransport,
        encoding::{PostcardEncode, PostcardDecode, Cobs},
    },
    util::Fifo,
};

// // use hal::{gpio::*, gpio::gpioe::*};
// use lc3_tm4c::peripherals_tm4c::{
//     // gpio::{
//     //     required_components as GpioComponents,
//     //     physical_pins as Tm4cGpio,
//     //     // GpioShim exists but it's not used for anything and doesn't impl Gpio?
//     // },
//     adc::{
//         required_components as AdcComponents,
//         AdcShim as Tm4cAdc,
//     },
//     pwm::{
//         required_components as PwmComponents,
//         PwmShim as Tm4cPwm,
//     },
//     timers::{
//         required_components as TimerComponents,
//         TimersShim as Tm4cTimers,
//     },
//     clock::{
//         required_components as ClockComponents,
//         Tm4cClock,
//     },
// };

// Unforuntately, this type alias is incomplete.
// use lc3_tm4c::peripherals_tm4c::Peripheralstm4c;

static FLAGS: PeripheralInterruptFlags = PeripheralInterruptFlags::new();

// type Tm4cPeripheralSet<'int> = PeripheralSet<
//     'int,
//     Tm4cGpio<'int>,
//     Tm4cAdc,
//     Tm4cPwm,
//     Tm4cTimers<'int>,
//     Tm4cClock,
//     InputStub,
//     OutputStub,
// >;

use tm4c123x_hal::gpio::{
    self as gp,
    PushPull,
    PullDown,
    gpiof::{self, PF1, PF2, PF3},
    gpiob::{self, PB3, PB4, PB5, PB6, PB7},
};

generic_gpio::io_pins_with_typestate! {
    #![allow(clippy::unit_arg)]
    //! TODO: module doc comment!

    for pins {
        /// ... (red)
        PF1 as G0,
        /// ... (blue)
        PF2 as G1,
        /// ... (green)
        PF3 as G2,
        /// ...
        PB3 as G3,
        /// ...
        PB4 as G4,
        /// ...
        PB5 as G5,
        /// ...
        PB6 as G6,
        /// ...
        PB7 as G7,
    } as Tm4cGpio;

    type Ctx = ();
    type Error = Infallible;

    type Disabled = gp::Tristate;
    type Input = gp::Input<PullDown>;
    type Output = gp::Output<PushPull>;

    => disabled = |x, ()| Ok(x.into_tri_state())
    => input    = |x, ()| Ok(x.into_pull_down_input())
    => output   = |x, ()| Ok(x.into_push_pull_output())

    => interrupts = |inp, ()| Ok(inp.set_interrupt_mode(gp::InterruptMode::EdgeRising))
    => interrupts = |inp, ()| Ok(inp.set_interrupt_mode(gp::InterruptMode::Disabled))
    => interrupts = |inp, ()| Ok(inp.get_interrupt_status())
    => interrupts = |inp, ()| Ok(inp.clear_interrupt())
}



#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();

    let mut sc = p.SYSCTL.constrain();
    sc.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );

    let clocks = sc.clock_setup.freeze();

    let mut porta = p.GPIO_PORTA.split(&sc.power_control);
    let u0 = p.UART0;
    // Peripheral Init:
    let peripheral_set = {
        // let portf = p.GPIO_PORTF;
        // let portb = p.GPIO_PORTB;

        // let gpio = Tm4cGpio::new(
        //     &sc.power_control,
        //     GpioComponents {
        //         portf,
        //         portb,
        //     },
        // );
        // let gpio = GpioStub;

        let portf = p.GPIO_PORTF;
        let portb = p.GPIO_PORTB;
        let gpiof::Parts { pf1: g0, pf2: g1, pf3: g2, .. } = portf.split(&sc.power_control);
        let gpiob::Parts { pb3: g3, pb4: g4, pb5: g5, pb6: g6, pb7: g7, .. } = portb.split(&sc.power_control);
        let gpio = Tm4cGpio::new(g0, g1, g2, g3, g4, g5, g6, g7, &FLAGS.gpio);

        // let adc0 = p.ADC0;
        // let adc1 = p.ADC1;
        // let porte = p.GPIO_PORTE;
        // let adc = Tm4cAdc::new(
        //     &sc.power_control,
        //     AdcComponents {
        //         adc0,
        //         adc1,
        //         porte,
        //     }
        // );
        let adc = AdcStub;

        // let portb = unsafe { hal::Peripherals::steal() }.GPIO_PORTB;
        // let portd = p.GPIO_PORTD;
        // let pwm0 = p.PWM0;
        // let pwm1 = p.PWM1;
        // Note: This will spin forever if you make the mistake of using an `lm4f` (which
        // does not have PWM...).
        // Perhaps we should have a `feature` for this at the very least?
        // let pwm = Tm4cPwm::new(
        //     PwmComponents {
        //         portb,
        //         portd,
        //         pwm0,
        //         pwm1,
        //     },
        //     &sc.power_control,
        // );
        let pwm = PwmStub;

        // let timer0 = p.TIMER0;
        // let timer1 = p.TIMER1;
        // let timers = Tm4cTimers::new(
        //     &sc.power_control,
        //     TimerComponents {
        //         timer0,
        //         timer1,
        //     }
        // );
        let timers = TimersStub;

        // let timer = p.TIMER2;
        // let clock = Tm4cClock::new(
        //     ClockComponents {
        //         timer,
        //     },
        //     &sc.power_control,
        // );
        let clock = ClockStub;

        PeripheralSet::new(
            gpio,
            adc,
            pwm,
            timers,
            clock,
            InputStub,
            OutputStub,
        )
    };

    // Activate UART
    let uart = hal::serial::Serial::uart0(
        u0,
        porta
            .pa1
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        porta
            .pa0
            .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
        (),
        (),
        1_500_000_u32.bps(),
        // hal::serial::NewlineMode::SwapLFtoCRLF,
        hal::serial::NewlineMode::Binary,
        &clocks,
        &sc.power_control,
    );

    let state: SimpleEventFutureSharedState = SimpleEventFutureSharedState::new();

    let mut memory = PartialMemory::default();

    let mut interp: Interpreter<'static, _, _> = Interpreter::new(
        &mut memory,
        peripheral_set,
        OwnedOrRef::Ref(&FLAGS),
        [0; 8],
        0x200,
        MachineState::Running,

    );

    let mut sim = Simulator::new_with_state(interp, &state);

    let func: &dyn Fn() -> Cobs<Fifo<u8>> = &|| Cobs::try_new(Fifo::new()).unwrap();
    let enc = PostcardEncode::<ResponseMessage, _, _>::new(func);
    let dec = PostcardDecode::<RequestMessage, Cobs<Fifo<u8>>>::new();

    let (tx, rx) = uart.split();

    let mut device = Device::<UartTransport<_, _>, _, RequestMessage, ResponseMessage, _, _>::new(
        enc,
        dec,
        UartTransport::new(rx, tx),
    );

    loop { let _ = device.step(&mut sim); }
}
