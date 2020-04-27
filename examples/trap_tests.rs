#![no_main]
#![no_std]

extern crate panic_halt;
extern crate tm4c123x_hal as hal;
use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
use hal::prelude::*;
use core::fmt::Write;

use lc3_tm4c::persistent_data_management::page::Paging;
use lc3_tm4c::paging_impl::tm4c_flash_paging_config::*;
use lc3_tm4c::paging_impl::tm4c_flash_paging_config;
use lc3_tm4c::peripherals_tm4c::flash;
use lc3_tm4c::peripherals_tm4c::flash::*;

use lc3_tm4c::memory_impl::tm4c_memory_impl::*;


use lc3_baseline_sim::interp::*;

use lc3_traits::memory;
use lc3_traits::memory::*;
use lc3_traits::peripherals::stubs;
use lc3_traits::peripherals::stubs::*;
use lc3_traits::peripherals::PeripheralSet;

use lc3_traits::peripherals::pwm::{
    Pwm, PwmPin, PwmPinArr,  PwmState,
};


use lc3_traits::peripherals::adc::{
    Adc, AdcMiscError, AdcPin as Pin, AdcPinArr as PinArr, AdcReadError as ReadError, AdcState,
    AdcStateMismatch as StateMismatch,
};

use hal::adc as ad ;
use hal::{gpio::*, gpio::gpioe::*};
use lc3_tm4c::peripherals_tm4c::adc::required_components as adc_req;
use lc3_tm4c::peripherals_tm4c::adc as adc;

use lc3_tm4c::peripherals_tm4c::gpio;
use lc3_tm4c::peripherals_tm4c::gpio::required_components as gpio_req;
use lc3_traits::peripherals::gpio::{
    Gpio, GpioMiscError, GpioPin, GpioPinArr, GpioReadError, GpioState, GpioWriteError,
};
use lc3_tm4c::peripherals_tm4c::pwm;
use lc3_tm4c::peripherals_tm4c::pwm::required_components as pwm_req;

use lc3_tm4c::peripherals_tm4c::Peripheralstm4c;


use lc3_tm4c::peripherals_tm4c::timers;
use lc3_tm4c::peripherals_tm4c::timers::required_components as timer_req;

use lc3_tm4c::peripherals_tm4c::clock;
use lc3_tm4c::peripherals_tm4c::clock::required_components as clock_req;

use lc3_isa::{
    Addr, Instruction,
    Reg::{self, *},
    Word, ACCESS_CONTROL_VIOLATION_EXCEPTION_VECTOR, ILLEGAL_OPCODE_EXCEPTION_VECTOR,
    INTERRUPT_VECTOR_TABLE_START_ADDR, MEM_MAPPED_START_ADDR,
    PRIVILEGE_MODE_VIOLATION_EXCEPTION_VECTOR, TRAP_VECTOR_TABLE_START_ADDR,
    USER_PROGRAM_START_ADDR,
};


#[entry]
fn main() -> ! {
    let p = hal::Peripherals::take().unwrap();
        let p_core = hal::CorePeripherals::take().unwrap();
        let nvic = p_core.NVIC;
    let mut sc = p.SYSCTL;
    let mut sys = sc.constrain();
     sys.clock_setup.oscillator = hal::sysctl::Oscillator::Main(
        hal::sysctl::CrystalFrequency::_16mhz,
        hal::sysctl::SystemClock::UsePll(hal::sysctl::PllOutputFrequency::_80_00mhz),
    );
    let clocks = sys.clock_setup.freeze();
   // let mut portf = p.GPIO_PORTF;
   // let mut porte = p.GPIO_PORTE;
    let mut adc0 = p.ADC0;
    let mut adc1= p.ADC1;
    let mut pwm0 = p.PWM0;
    let mut pwm1 = p.PWM1;
    let mut flash = p.FLASH_CTRL;

    let mut portb = p.GPIO_PORTB;
    let mut portd = p.GPIO_PORTD;
    let mut portf = p.GPIO_PORTF;
    let mut porte = p.GPIO_PORTE;
   // let mut porta = p.GPIO_PORTA.split(&sys.power_control);
    let mut t0 = p.TIMER0;
    let mut t1= p.TIMER1;
    let mut t2= p.TIMER2;
    
    let mut flash_unit = flash::tm4c_flash_unit{
        flash_ctrl: flash,
    };
    // let mut pins = gpio::physical_pins::new(
    //     &sys.power_control,
    //     gpio_req {
    //         portf: portf,
    //         portb: portb,
    //     },
    // );

    let mut adc_shim = adc::AdcShim::new(&sys.power_control, adc_req{adc0: adc0, adc1:adc1, porte: porte });
    let mut swap_obj = Tm4c_flash_page_unit_for_lc3::new(flash_unit);


swap_obj.write_primary(12288,0x5020);
swap_obj.write_primary(12289,0x2205);
swap_obj.write_primary(12290,0xe406);
swap_obj.write_primary(12291,0xf060);
swap_obj.write_primary(12292,0x2203);
swap_obj.write_primary(12293,0x05fe);
swap_obj.write_primary(12294,0xf025);
swap_obj.write_primary(12295,0x0bb8);
swap_obj.write_primary(12296,0x0000);
swap_obj.write_primary(12297,0x1dbf);
swap_obj.write_primary(12298,0x7180);
swap_obj.write_primary(12299,0x5020);
swap_obj.write_primary(12300,0x1021);
swap_obj.write_primary(12301,0x31fa);
swap_obj.write_primary(12302,0x6180);
swap_obj.write_primary(12303,0x1da1);
swap_obj.write_primary(12304,0x8000);

swap_obj.write_primary(1532,0x0000);
swap_obj.write_primary(1533,0x0000);
swap_obj.write_primary(1534,0x0000);
swap_obj.write_primary(1535,0x0000);
swap_obj.write_primary(1536,0x3000);
swap_obj.write_primary(1537,0x0001);
swap_obj.write_primary(1538,0x0700);
swap_obj.write_primary(1539,0x0000);
swap_obj.write_primary(1540,0x0000);
swap_obj.write_primary(1541,0x0000);


    let mut pwm_shim = pwm::PwmShim::new(pwm_req{
        //sysctl: sc,
        portb: portb,
        portd: portd,
        pwm0: pwm0,
        pwm1: pwm1,
    }, &sys.power_control);

    let mut timer_shim = timers::TimersShim::new(&sys.power_control, timer_req{timer0: t0, timer1: t1});

    let mut tm4c_mem = tm4c_lc3_memory{
        tm4c_mem_obj: swap_obj,
    };


    let mut clock_req = clock::Tm4cClock::new(clock_req{timer: t2}, &sys.power_control);
    let x: PeripheralsStub;
    let peripherals = PeripheralSet::new(
        GpioStub,
        adc_shim,
        pwm_shim,
        timer_shim,
        clock_req,
        InputStub,
        OutputStub,
    );




        let mut interp = Interpreter::<tm4c_lc3_memory, 
        Peripheralstm4c>::new(
        tm4c_mem,
        peripherals,
        OwnedOrRef::<PeripheralInterruptFlags>::default(),
        [0 as Word; Reg::NUM_REGS],
        0x3000,
        MachineState::Halted,


    );
        let mut pc = interp.get_pc();
        let mut out = interp.get_register(Reg::R0);
        while (interp.get_pc() != 12300){
          pc = interp.get_pc();
          interp.step();
          out = interp.get_register(Reg::R0);
          let x = 1;
          if(pc == 12294){
            let came_here = 1;
          }

        }
        loop {
        }
    }