//! Impl of the UTP platform for the TI TM4C.
//!
//! TODO!

// TODO: forbid
#![warn(
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    legacy_directory_ownership,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    safe_extern_statics,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_lifetimes,
    unused_comparisons,
    unused_parens,
    while_true
)]
// TODO: deny
#![warn(
    missing_debug_implementations,
    intra_doc_link_resolution_failure,
    missing_docs,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    rust_2018_idioms
)]
#![doc(test(attr(deny(rust_2018_idioms, warnings))))]
#![doc(html_logo_url = "")] // TODO!

#![no_std]
#![no_main]

extern crate panic_halt;
// extern crate panic_semihosting;
extern crate tm4c123x_hal as hal;

use cortex_m_rt::entry;
use hal::prelude::*;

use lc3_traits::control::rpc::{
    SimpleEventFutureSharedState, Device, RequestMessage, ResponseMessage
};
use lc3_baseline_sim::interp::{
    Interpreter, InstructionInterpreter, InterpreterBuilder,
    PeripheralInterruptFlags, OwnedOrRef, MachineState,
};
use lc3_baseline_sim::sim::Simulator;
use lc3_traits::peripherals::{PeripheralSet, stubs::PeripheralsStub};
use lc3_traits::memory::MemoryStub;
use lc3_traits::control::Control;
use lc3_device_support::{
    memory::PartialMemory,
    rpc::{
        transport::uart_simple::UartTransport,
        encoding::{PostcardEncode, PostcardDecode, Cobs},
    },
    util::Fifo,
};

use core::fmt::Write;

static FLAGS: PeripheralInterruptFlags = PeripheralInterruptFlags::new();

// fn entry() -> ! { main() }

#[inline(never)]
fn foo(a: f32) -> f32 { (a * 2.0) % 203.43f32 }

#[inline(never)]
fn test() -> SimpleEventFutureSharedState {
    SimpleEventFutureSharedState::new()
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
    let mut u0 = p.UART0;

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

    let (mut tx, mut rx) = uart.split();

    // write!(tx, "zero");
    let state: SimpleEventFutureSharedState = SimpleEventFutureSharedState::new();

    let state = test();
    // write!(tx, "uno");

    // let mut interp: Interpreter::<'static, PartialMemory, PeripheralsStub> = InterpreterBuilder::new()
    //     // .with_interrupt_flags_by_ref(&FLAGS)
    //     .with_defaults()
    //     .build();

    let mut memory = PartialMemory::default();

    let mut interp: Interpreter<'static, _, PeripheralsStub> = Interpreter::new(
        &mut memory,
        PeripheralsStub::default(),
        OwnedOrRef::Ref(&FLAGS),
        [0; 8],
        0x200,
        MachineState::Running,

    );

    // write!(tx, "dos");

    let mut sim = Simulator::new_with_state(interp, &state);

    let func: &dyn Fn() -> Cobs<Fifo<u8>> = &|| Cobs::try_new(Fifo::new()).unwrap();

    let enc = PostcardEncode::<ResponseMessage, _, _>::new(func);
    let dec = PostcardDecode::<RequestMessage, Cobs<Fifo<u8>>>::new();
    // let transport = UartTransport::new(rx, tx);

    let mut device = Device::<UartTransport<_, _>, _, RequestMessage, ResponseMessage, _, _>::new(
        enc,
        dec,
        UartTransport::new(rx, tx),
    );

    loop { device.step(&mut sim); }

    // let mut a = 2.3f32;

    // loop { a = foo(a); write!(tx, "hey: {}\n", a); write!(tx, "Hello\n"); sim.step(); }
}
