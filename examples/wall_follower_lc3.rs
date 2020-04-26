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

    // let mut pwm_shim = pwm::PwmShim::new(pwm_req {
    //     //sysctl: sc,
    //     portb: portb,
    //     portd: portd,
    //     pwm0: pwm0,
    //     pwm1: pwm1,
    // }, &sys.power_control);





// swap_obj.write_primary(237,0x0261);
// swap_obj.write_primary(238,0x0261);
// swap_obj.write_primary(239,0x0261);
// swap_obj.write_primary(240,0x0261);
// swap_obj.write_primary(241,0x0261);
// swap_obj.write_primary(242,0x0261);
// swap_obj.write_primary(243,0x0261);
// swap_obj.write_primary(244,0x0261);
// swap_obj.write_primary(245,0x0261);
// swap_obj.write_primary(246,0x0261);
// swap_obj.write_primary(247,0x0261);
// swap_obj.write_primary(248,0x0261);
// swap_obj.write_primary(249,0x0261);
// swap_obj.write_primary(250,0x0261);
// swap_obj.write_primary(251,0x0261);
// swap_obj.write_primary(252,0x0261);
// swap_obj.write_primary(253,0x0261);
// swap_obj.write_primary(254,0x0261);
// swap_obj.write_primary(255,0x0261);
// swap_obj.write_primary(256,0x048d);
// swap_obj.write_primary(257,0x0490);
// swap_obj.write_primary(258,0x0493);
// swap_obj.write_primary(259,0x02bb);
// swap_obj.write_primary(260,0x02bb);
// swap_obj.write_primary(261,0x02bb);
// swap_obj.write_primary(262,0x02bb);
// swap_obj.write_primary(263,0x02bb);
// swap_obj.write_primary(264,0x02bb);
// swap_obj.write_primary(265,0x02bb);
// swap_obj.write_primary(266,0x02bb);
// swap_obj.write_primary(267,0x02bb);
// swap_obj.write_primary(268,0x02bb);
// swap_obj.write_primary(269,0x02bb);
// swap_obj.write_primary(270,0x02bb);
// swap_obj.write_primary(271,0x02bb);
// swap_obj.write_primary(272,0x02bb);
// swap_obj.write_primary(273,0x02bb);
// swap_obj.write_primary(274,0x02bb);
// swap_obj.write_primary(275,0x02bb);
// swap_obj.write_primary(276,0x02bb);
// swap_obj.write_primary(277,0x02bb);
// swap_obj.write_primary(278,0x02bb);
// swap_obj.write_primary(279,0x02bb);
// swap_obj.write_primary(280,0x02bb);
// swap_obj.write_primary(281,0x02bb);
// swap_obj.write_primary(282,0x02bb);
// swap_obj.write_primary(283,0x02bb);
// swap_obj.write_primary(284,0x02bb);
// swap_obj.write_primary(285,0x02bb);
// swap_obj.write_primary(286,0x02bb);
// swap_obj.write_primary(287,0x02bb);
// swap_obj.write_primary(288,0x02bb);
// swap_obj.write_primary(289,0x02bb);
// swap_obj.write_primary(290,0x02bb);
// swap_obj.write_primary(291,0x02bb);
// swap_obj.write_primary(292,0x02bb);
// swap_obj.write_primary(293,0x02bb);
// swap_obj.write_primary(294,0x02bb);
// swap_obj.write_primary(295,0x02bb);
// swap_obj.write_primary(296,0x02bb);
// swap_obj.write_primary(297,0x02bb);
// swap_obj.write_primary(298,0x02bb);
// swap_obj.write_primary(299,0x02bb);
// swap_obj.write_primary(300,0x02bb);
// swap_obj.write_primary(301,0x02bb);
// swap_obj.write_primary(302,0x02bb);
// swap_obj.write_primary(303,0x02bb);
// swap_obj.write_primary(304,0x02bb);
// swap_obj.write_primary(305,0x02bb);
// swap_obj.write_primary(306,0x02bb);
// swap_obj.write_primary(307,0x02bb);
// swap_obj.write_primary(308,0x02bb);
// swap_obj.write_primary(309,0x02bb);
// swap_obj.write_primary(310,0x02bb);
// swap_obj.write_primary(311,0x02bb);
// swap_obj.write_primary(312,0x02bb);
// swap_obj.write_primary(313,0x02bb);
// swap_obj.write_primary(314,0x02bb);
// swap_obj.write_primary(315,0x02bb);
// swap_obj.write_primary(316,0x02bb);
// swap_obj.write_primary(317,0x02bb);
// swap_obj.write_primary(318,0x02bb);
// swap_obj.write_primary(319,0x02bb);
// swap_obj.write_primary(320,0x02bb);
// swap_obj.write_primary(321,0x02bb);
// swap_obj.write_primary(322,0x02bb);
// swap_obj.write_primary(323,0x02bb);
// swap_obj.write_primary(324,0x02bb);
// swap_obj.write_primary(325,0x02bb);
// swap_obj.write_primary(326,0x02bb);
// swap_obj.write_primary(327,0x02bb);
// swap_obj.write_primary(328,0x02bb);
// swap_obj.write_primary(329,0x02bb);
// swap_obj.write_primary(330,0x02bb);
// swap_obj.write_primary(331,0x02bb);
// swap_obj.write_primary(332,0x02bb);
// swap_obj.write_primary(333,0x02bb);
// swap_obj.write_primary(334,0x02bb);
// swap_obj.write_primary(335,0x02bb);
// swap_obj.write_primary(336,0x02bb);
// swap_obj.write_primary(337,0x02bb);
// swap_obj.write_primary(338,0x02bb);
// swap_obj.write_primary(339,0x02bb);
// swap_obj.write_primary(340,0x02bb);
// swap_obj.write_primary(341,0x02bb);
// swap_obj.write_primary(342,0x02bb);
// swap_obj.write_primary(343,0x02bb);
// swap_obj.write_primary(344,0x02bb);
// swap_obj.write_primary(345,0x02bb);
// swap_obj.write_primary(346,0x02bb);
// swap_obj.write_primary(347,0x02bb);
// swap_obj.write_primary(348,0x02bb);
// swap_obj.write_primary(349,0x02bb);
// swap_obj.write_primary(350,0x02bb);
// swap_obj.write_primary(351,0x02bb);
// swap_obj.write_primary(352,0x02bb);
// swap_obj.write_primary(353,0x02bb);
// swap_obj.write_primary(354,0x02bb);
// swap_obj.write_primary(355,0x02bb);
// swap_obj.write_primary(356,0x02bb);
// swap_obj.write_primary(357,0x02bb);
// swap_obj.write_primary(358,0x02bb);
// swap_obj.write_primary(359,0x02bb);
// swap_obj.write_primary(360,0x02bb);
// swap_obj.write_primary(361,0x02bb);
// swap_obj.write_primary(362,0x02bb);
// swap_obj.write_primary(363,0x02bb);
// swap_obj.write_primary(364,0x02bb);
// swap_obj.write_primary(365,0x02bb);
// swap_obj.write_primary(366,0x02bb);
// swap_obj.write_primary(367,0x02bb);
// swap_obj.write_primary(368,0x02bb);
// swap_obj.write_primary(369,0x02bb);
// swap_obj.write_primary(370,0x02bb);
// swap_obj.write_primary(371,0x02bb);
// swap_obj.write_primary(372,0x02bb);
// swap_obj.write_primary(373,0x02bb);
// swap_obj.write_primary(374,0x02bb);
// swap_obj.write_primary(375,0x02bb);
// swap_obj.write_primary(376,0x02bb);
// swap_obj.write_primary(377,0x02bb);
// swap_obj.write_primary(378,0x02bb);
// swap_obj.write_primary(379,0x02bb);
// swap_obj.write_primary(380,0x02bb);
// swap_obj.write_primary(381,0x02bb);
// swap_obj.write_primary(382,0x02bb);
// swap_obj.write_primary(383,0x02bb);
// swap_obj.write_primary(384,0x02be);
// swap_obj.write_primary(385,0x02be);
// swap_obj.write_primary(386,0x02be);
// swap_obj.write_primary(387,0x02be);
// swap_obj.write_primary(388,0x02be);
// swap_obj.write_primary(389,0x02be);
// swap_obj.write_primary(390,0x02be);
// swap_obj.write_primary(391,0x02be);
// swap_obj.write_primary(392,0x02be);
// swap_obj.write_primary(393,0x02be);
// swap_obj.write_primary(394,0x02be);
// swap_obj.write_primary(395,0x02be);
// swap_obj.write_primary(396,0x02be);
// swap_obj.write_primary(397,0x02be);
// swap_obj.write_primary(398,0x02be);
// swap_obj.write_primary(399,0x02be);
// swap_obj.write_primary(400,0x02be);
// swap_obj.write_primary(401,0x02be);
// swap_obj.write_primary(402,0x02be);
// swap_obj.write_primary(403,0x02be);
// swap_obj.write_primary(404,0x02be);
// swap_obj.write_primary(405,0x02be);
// swap_obj.write_primary(406,0x02be);
// swap_obj.write_primary(407,0x02be);
// swap_obj.write_primary(408,0x02be);
// swap_obj.write_primary(409,0x02be);
// swap_obj.write_primary(410,0x02be);
// swap_obj.write_primary(411,0x02be);
// swap_obj.write_primary(412,0x02be);
// swap_obj.write_primary(413,0x02be);
// swap_obj.write_primary(414,0x02be);
// swap_obj.write_primary(415,0x02be);
// swap_obj.write_primary(416,0x02be);
// swap_obj.write_primary(417,0x02be);
// swap_obj.write_primary(418,0x02be);
// swap_obj.write_primary(419,0x02be);
// swap_obj.write_primary(420,0x02be);
// swap_obj.write_primary(421,0x02be);
// swap_obj.write_primary(422,0x02be);
// swap_obj.write_primary(423,0x02be);
// swap_obj.write_primary(424,0x02be);
// swap_obj.write_primary(425,0x02be);
// swap_obj.write_primary(426,0x02be);
// swap_obj.write_primary(427,0x02be);
// swap_obj.write_primary(428,0x02be);
// swap_obj.write_primary(429,0x02be);
// swap_obj.write_primary(430,0x02be);
// swap_obj.write_primary(431,0x02be);
// swap_obj.write_primary(432,0x02be);
// swap_obj.write_primary(433,0x02be);
// swap_obj.write_primary(434,0x02be);
// swap_obj.write_primary(435,0x02be);
// swap_obj.write_primary(436,0x02be);
// swap_obj.write_primary(437,0x02be);
// swap_obj.write_primary(438,0x02be);
// swap_obj.write_primary(439,0x02be);
// swap_obj.write_primary(440,0x02be);
// swap_obj.write_primary(441,0x02be);
// swap_obj.write_primary(442,0x02be);
// swap_obj.write_primary(443,0x02be);
// swap_obj.write_primary(444,0x02be);
// swap_obj.write_primary(445,0x02be);
// swap_obj.write_primary(446,0x02be);
// swap_obj.write_primary(447,0x02be);
// swap_obj.write_primary(448,0x02be);
// swap_obj.write_primary(449,0x02be);
// swap_obj.write_primary(450,0x02be);
// swap_obj.write_primary(451,0x02be);
// swap_obj.write_primary(452,0x02be);
// swap_obj.write_primary(453,0x02be);
// swap_obj.write_primary(454,0x02be);
// swap_obj.write_primary(455,0x02be);
// swap_obj.write_primary(456,0x02be);
// swap_obj.write_primary(457,0x02be);
// swap_obj.write_primary(458,0x02be);
// swap_obj.write_primary(459,0x02be);
// swap_obj.write_primary(460,0x02be);
// swap_obj.write_primary(461,0x02be);
// swap_obj.write_primary(462,0x02be);
// swap_obj.write_primary(463,0x02be);
// swap_obj.write_primary(464,0x02be);
// swap_obj.write_primary(465,0x02be);
// swap_obj.write_primary(466,0x02be);
// swap_obj.write_primary(467,0x02be);
// swap_obj.write_primary(468,0x02be);
// swap_obj.write_primary(469,0x02be);
// swap_obj.write_primary(470,0x02be);
// swap_obj.write_primary(471,0x02be);
// swap_obj.write_primary(472,0x02be);
// swap_obj.write_primary(473,0x02be);
// swap_obj.write_primary(474,0x02be);
// swap_obj.write_primary(475,0x02be);
// swap_obj.write_primary(476,0x02be);
// swap_obj.write_primary(477,0x02be);
// swap_obj.write_primary(478,0x02be);
// swap_obj.write_primary(479,0x02be);
// swap_obj.write_primary(480,0x02be);
// swap_obj.write_primary(481,0x02be);
// swap_obj.write_primary(482,0x02be);
// swap_obj.write_primary(483,0x02be);
// swap_obj.write_primary(484,0x02be);
// swap_obj.write_primary(485,0x02be);
// swap_obj.write_primary(486,0x02be);
// swap_obj.write_primary(487,0x02be);
// swap_obj.write_primary(488,0x02be);
// swap_obj.write_primary(489,0x02be);
// swap_obj.write_primary(490,0x02be);
// swap_obj.write_primary(491,0x02be);
// swap_obj.write_primary(492,0x02be);
// swap_obj.write_primary(493,0x02be);
// swap_obj.write_primary(494,0x02be);
// swap_obj.write_primary(495,0x02be);
// swap_obj.write_primary(496,0x02be);
// swap_obj.write_primary(497,0x02be);
// swap_obj.write_primary(498,0x02be);
// swap_obj.write_primary(499,0x02be);
// swap_obj.write_primary(500,0x02be);
// swap_obj.write_primary(501,0x02be);
// swap_obj.write_primary(502,0x02be);
// swap_obj.write_primary(503,0x02be);
// swap_obj.write_primary(504,0x02be);
// swap_obj.write_primary(505,0x02be);
// swap_obj.write_primary(506,0x02be);
// swap_obj.write_primary(507,0x02be);
// swap_obj.write_primary(508,0x02be);
// swap_obj.write_primary(509,0x02be);
// swap_obj.write_primary(510,0x02be);
// swap_obj.write_primary(511,0x02be);
// swap_obj.write_primary(512,0xac0b);
// swap_obj.write_primary(513,0xe008);
// swap_obj.write_primary(514,0xf022);
// swap_obj.write_primary(515,0x200e);
// swap_obj.write_primary(516,0x1dbf);
// swap_obj.write_primary(517,0x7180);
// swap_obj.write_primary(518,0xa004);
// swap_obj.write_primary(519,0x1dbf);
// swap_obj.write_primary(520,0x7180);
// swap_obj.write_primary(521,0x8000);
// swap_obj.write_primary(522,0x0000);
// swap_obj.write_primary(523,0x0600);
// swap_obj.write_primary(524,0x0602);
// swap_obj.write_primary(525,0xfe00);
// swap_obj.write_primary(526,0xfe02);
// swap_obj.write_primary(527,0xfe04);
// swap_obj.write_primary(528,0xfe06);
// swap_obj.write_primary(529,0xfffe);
// swap_obj.write_primary(530,0x8302);
// swap_obj.write_primary(531,0x7fff);
// swap_obj.write_primary(532,0x00ff);
// swap_obj.write_primary(533,0xa1f7);
// swap_obj.write_primary(534,0x07fe);
// swap_obj.write_primary(535,0xa1f6);
// swap_obj.write_primary(536,0x8000);
// swap_obj.write_primary(537,0x1dbf);
// swap_obj.write_primary(538,0x7380);
// swap_obj.write_primary(539,0xa3f3);
// swap_obj.write_primary(540,0x07fe);
// swap_obj.write_primary(541,0xb1f2);
// swap_obj.write_primary(542,0x6380);
// swap_obj.write_primary(543,0x1da1);
// swap_obj.write_primary(544,0x8000);
// swap_obj.write_primary(545,0x1dbe);
// swap_obj.write_primary(546,0x7181);
// swap_obj.write_primary(547,0x7380);
// swap_obj.write_primary(548,0x1220);
// swap_obj.write_primary(549,0x6040);
// swap_obj.write_primary(550,0x0403);
// swap_obj.write_primary(551,0xf021);
// swap_obj.write_primary(552,0x1261);
// swap_obj.write_primary(553,0x0ffb);
// swap_obj.write_primary(554,0x6380);
// swap_obj.write_primary(555,0x6181);
// swap_obj.write_primary(556,0x1da2);
// swap_obj.write_primary(557,0x8000);
// swap_obj.write_primary(558,0xe035);
// swap_obj.write_primary(559,0xf022);
// swap_obj.write_primary(560,0xf020);
// swap_obj.write_primary(561,0xf021);
// swap_obj.write_primary(562,0x1dbf);
// swap_obj.write_primary(563,0x7180);
// swap_obj.write_primary(564,0x5020);
// swap_obj.write_primary(565,0x102a);
// swap_obj.write_primary(566,0xf021);
// swap_obj.write_primary(567,0x6180);
// swap_obj.write_primary(568,0x1da1);
// swap_obj.write_primary(569,0x8000);
// swap_obj.write_primary(570,0x1dbc);
// swap_obj.write_primary(571,0x7183);
// swap_obj.write_primary(572,0x7382);
// swap_obj.write_primary(573,0x7581);
// swap_obj.write_primary(574,0x7780);
// swap_obj.write_primary(575,0x1220);
// swap_obj.write_primary(576,0x6440);
// swap_obj.write_primary(577,0x21d2);
// swap_obj.write_primary(578,0x5002);
// swap_obj.write_primary(579,0x0410);
// swap_obj.write_primary(580,0xf021);
// swap_obj.write_primary(581,0x5020);
// swap_obj.write_primary(582,0x1628);
// swap_obj.write_primary(583,0x14a0);
// swap_obj.write_primary(584,0x0601);
// swap_obj.write_primary(585,0x1021);
// swap_obj.write_primary(586,0x1000);
// swap_obj.write_primary(587,0x16ff);
// swap_obj.write_primary(588,0x0402);
// swap_obj.write_primary(589,0x1482);
// swap_obj.write_primary(590,0x0ff9);
// swap_obj.write_primary(591,0x1020);
// swap_obj.write_primary(592,0x0403);
// swap_obj.write_primary(593,0xf021);
// swap_obj.write_primary(594,0x1261);
// swap_obj.write_primary(595,0x0fec);
// swap_obj.write_primary(596,0x6780);
// swap_obj.write_primary(597,0x6581);
// swap_obj.write_primary(598,0x6382);
// swap_obj.write_primary(599,0x6183);
// swap_obj.write_primary(600,0x1da4);
// swap_obj.write_primary(601,0x8000);
// swap_obj.write_primary(602,0xe01e);
// swap_obj.write_primary(603,0xf022);
// swap_obj.write_primary(604,0xa1b4);
// swap_obj.write_primary(605,0x23b5);
// swap_obj.write_primary(606,0x5001);
// swap_obj.write_primary(607,0xb1b1);
// swap_obj.write_primary(608,0x0ff9);
// swap_obj.write_primary(609,0xe034);
// swap_obj.write_primary(610,0xf022);
// swap_obj.write_primary(611,0xf025);
// swap_obj.write_primary(612,0x000a);
// swap_obj.write_primary(613,0x0049);
// swap_obj.write_primary(614,0x006e);
// swap_obj.write_primary(615,0x0070);
// swap_obj.write_primary(616,0x0075);
// swap_obj.write_primary(617,0x0074);
// swap_obj.write_primary(618,0x0020);
// swap_obj.write_primary(619,0x0061);
// swap_obj.write_primary(620,0x0020);
// swap_obj.write_primary(621,0x0063);
// swap_obj.write_primary(622,0x0068);
// swap_obj.write_primary(623,0x0061);
// swap_obj.write_primary(624,0x0072);
// swap_obj.write_primary(625,0x0061);
// swap_obj.write_primary(626,0x0063);
// swap_obj.write_primary(627,0x0074);
// swap_obj.write_primary(628,0x0065);
// swap_obj.write_primary(629,0x0072);
// swap_obj.write_primary(630,0x003e);
// swap_obj.write_primary(631,0x0020);
// swap_obj.write_primary(632,0x0000);
// swap_obj.write_primary(633,0x000a);
// swap_obj.write_primary(634,0x000a);
// swap_obj.write_primary(635,0x002d);
// swap_obj.write_primary(636,0x002d);
// swap_obj.write_primary(637,0x002d);
// swap_obj.write_primary(638,0x0020);
// swap_obj.write_primary(639,0x0048);
// swap_obj.write_primary(640,0x0061);
// swap_obj.write_primary(641,0x006c);
// swap_obj.write_primary(642,0x0074);
// swap_obj.write_primary(643,0x0069);
// swap_obj.write_primary(644,0x006e);
// swap_obj.write_primary(645,0x0067);
// swap_obj.write_primary(646,0x0020);
// swap_obj.write_primary(647,0x0074);
// swap_obj.write_primary(648,0x0068);
// swap_obj.write_primary(649,0x0065);
// swap_obj.write_primary(650,0x0020);
// swap_obj.write_primary(651,0x004c);
// swap_obj.write_primary(652,0x0043);
// swap_obj.write_primary(653,0x002d);
// swap_obj.write_primary(654,0x0033);
// swap_obj.write_primary(655,0x0020);
// swap_obj.write_primary(656,0x002d);
// swap_obj.write_primary(657,0x002d);
// swap_obj.write_primary(658,0x002d);
// swap_obj.write_primary(659,0x000a);
// swap_obj.write_primary(660,0x000a);
// swap_obj.write_primary(661,0x0000);
// swap_obj.write_primary(662,0x000a);
// swap_obj.write_primary(663,0x000a);
// swap_obj.write_primary(664,0x002d);
// swap_obj.write_primary(665,0x002d);
// swap_obj.write_primary(666,0x002d);
// swap_obj.write_primary(667,0x0020);
// swap_obj.write_primary(668,0x0055);
// swap_obj.write_primary(669,0x006e);
// swap_obj.write_primary(670,0x0064);
// swap_obj.write_primary(671,0x0065);
// swap_obj.write_primary(672,0x0066);
// swap_obj.write_primary(673,0x0069);
// swap_obj.write_primary(674,0x006e);
// swap_obj.write_primary(675,0x0065);
// swap_obj.write_primary(676,0x0064);
// swap_obj.write_primary(677,0x0020);
// swap_obj.write_primary(678,0x0054);
// swap_obj.write_primary(679,0x0052);
// swap_obj.write_primary(680,0x0041);
// swap_obj.write_primary(681,0x0050);
// swap_obj.write_primary(682,0x0020);
// swap_obj.write_primary(683,0x0065);
// swap_obj.write_primary(684,0x0078);
// swap_obj.write_primary(685,0x0065);
// swap_obj.write_primary(686,0x0063);
// swap_obj.write_primary(687,0x0075);
// swap_obj.write_primary(688,0x0074);
// swap_obj.write_primary(689,0x0065);
// swap_obj.write_primary(690,0x0064);
// swap_obj.write_primary(691,0x0021);
// swap_obj.write_primary(692,0x0020);
// swap_obj.write_primary(693,0x002d);
// swap_obj.write_primary(694,0x002d);
// swap_obj.write_primary(695,0x002d);
// swap_obj.write_primary(696,0x000a);
// swap_obj.write_primary(697,0x000a);
// swap_obj.write_primary(698,0x0000);
// swap_obj.write_primary(699,0x2005);
// swap_obj.write_primary(700,0xf022);
// swap_obj.write_primary(701,0xf025);
// swap_obj.write_primary(702,0x203a);
// swap_obj.write_primary(703,0xf022);
// swap_obj.write_primary(704,0xf025);
// swap_obj.write_primary(705,0x000a);
// swap_obj.write_primary(706,0x000a);
// swap_obj.write_primary(707,0x002d);
// swap_obj.write_primary(708,0x002d);
// swap_obj.write_primary(709,0x002d);
// swap_obj.write_primary(710,0x0020);
// swap_obj.write_primary(711,0x0045);
// swap_obj.write_primary(712,0x006e);
// swap_obj.write_primary(713,0x0063);
// swap_obj.write_primary(714,0x006f);
// swap_obj.write_primary(715,0x0075);
// swap_obj.write_primary(716,0x006e);
// swap_obj.write_primary(717,0x0074);
// swap_obj.write_primary(718,0x0065);
// swap_obj.write_primary(719,0x0072);
// swap_obj.write_primary(720,0x0065);
// swap_obj.write_primary(721,0x0064);
// swap_obj.write_primary(722,0x0020);
// swap_obj.write_primary(723,0x0061);
// swap_obj.write_primary(724,0x006e);
// swap_obj.write_primary(725,0x0020);
// swap_obj.write_primary(726,0x0065);
// swap_obj.write_primary(727,0x0078);
// swap_obj.write_primary(728,0x0063);
// swap_obj.write_primary(729,0x0065);
// swap_obj.write_primary(730,0x0070);
// swap_obj.write_primary(731,0x0074);
// swap_obj.write_primary(732,0x0069);
// swap_obj.write_primary(733,0x006f);
// swap_obj.write_primary(734,0x006e);
// swap_obj.write_primary(735,0x0020);
// swap_obj.write_primary(736,0x0077);
// swap_obj.write_primary(737,0x0069);
// swap_obj.write_primary(738,0x0074);
// swap_obj.write_primary(739,0x0068);
// swap_obj.write_primary(740,0x006f);
// swap_obj.write_primary(741,0x0075);
// swap_obj.write_primary(742,0x0074);
// swap_obj.write_primary(743,0x0020);
// swap_obj.write_primary(744,0x0061);
// swap_obj.write_primary(745,0x0020);
// swap_obj.write_primary(746,0x0068);
// swap_obj.write_primary(747,0x0061);
// swap_obj.write_primary(748,0x006e);
// swap_obj.write_primary(749,0x0064);
// swap_obj.write_primary(750,0x006c);
// swap_obj.write_primary(751,0x0065);
// swap_obj.write_primary(752,0x0072);
// swap_obj.write_primary(753,0x0021);
// swap_obj.write_primary(754,0x0020);
// swap_obj.write_primary(755,0x002d);
// swap_obj.write_primary(756,0x002d);
// swap_obj.write_primary(757,0x002d);
// swap_obj.write_primary(758,0x000a);
// swap_obj.write_primary(759,0x000a);
// swap_obj.write_primary(760,0x0000);
// swap_obj.write_primary(761,0x000a);
// swap_obj.write_primary(762,0x000a);
// swap_obj.write_primary(763,0x002d);
// swap_obj.write_primary(764,0x002d);
// swap_obj.write_primary(765,0x002d);
// swap_obj.write_primary(766,0x0020);
// swap_obj.write_primary(767,0x0055);
// swap_obj.write_primary(768,0x006e);
// swap_obj.write_primary(769,0x0068);
// swap_obj.write_primary(770,0x0061);
// swap_obj.write_primary(771,0x006e);
// swap_obj.write_primary(772,0x0064);
// swap_obj.write_primary(773,0x006c);
// swap_obj.write_primary(774,0x0065);
// swap_obj.write_primary(775,0x0064);
// swap_obj.write_primary(776,0x0020);
// swap_obj.write_primary(777,0x0069);
// swap_obj.write_primary(778,0x006e);
// swap_obj.write_primary(779,0x0074);
// swap_obj.write_primary(780,0x0065);
// swap_obj.write_primary(781,0x0072);
// swap_obj.write_primary(782,0x0072);
// swap_obj.write_primary(783,0x0075);
// swap_obj.write_primary(784,0x0070);
// swap_obj.write_primary(785,0x0074);
// swap_obj.write_primary(786,0x0021);
// swap_obj.write_primary(787,0x0020);
// swap_obj.write_primary(788,0x002d);
// swap_obj.write_primary(789,0x002d);
// swap_obj.write_primary(790,0x002d);
// swap_obj.write_primary(791,0x000a);
// swap_obj.write_primary(792,0x000a);
// swap_obj.write_primary(793,0x0000);
// swap_obj.write_primary(794,0x1020);
// swap_obj.write_primary(795,0x0807);
// swap_obj.write_primary(796,0x993f);
// swap_obj.write_primary(797,0x1921);
// swap_obj.write_primary(798,0x1804);
// swap_obj.write_primary(799,0x0202);
// swap_obj.write_primary(800,0x1020);
// swap_obj.write_primary(801,0x0e01);
// swap_obj.write_primary(802,0x983f);
// swap_obj.write_primary(803,0xc1c0);
// swap_obj.write_primary(804,0x1dbe);
// swap_obj.write_primary(805,0x7981);
// swap_obj.write_primary(806,0x7f80);
// swap_obj.write_primary(807,0x5920);
// swap_obj.write_primary(808,0x1928);
// swap_obj.write_primary(809,0x4ff0);
// swap_obj.write_primary(810,0x0804);
// swap_obj.write_primary(811,0x28ac);
// swap_obj.write_primary(812,0x1900);
// swap_obj.write_primary(813,0x1900);
// swap_obj.write_primary(814,0x7300);
// swap_obj.write_primary(815,0x6f80);
// swap_obj.write_primary(816,0x6981);
// swap_obj.write_primary(817,0x1da2);
// swap_obj.write_primary(818,0xc1c0);
// swap_obj.write_primary(819,0x1dbe);
// swap_obj.write_primary(820,0x7381);
// swap_obj.write_primary(821,0x7f80);
// swap_obj.write_primary(822,0x5260);
// swap_obj.write_primary(823,0x1262);
// swap_obj.write_primary(824,0x4feb);
// swap_obj.write_primary(825,0x6f80);
// swap_obj.write_primary(826,0x6381);
// swap_obj.write_primary(827,0x1da2);
// swap_obj.write_primary(828,0x8000);
// swap_obj.write_primary(829,0x1dbe);
// swap_obj.write_primary(830,0x7381);
// swap_obj.write_primary(831,0x7f80);
// swap_obj.write_primary(832,0x5260);
// swap_obj.write_primary(833,0x1261);
// swap_obj.write_primary(834,0x4fe1);
// swap_obj.write_primary(835,0x6f80);
// swap_obj.write_primary(836,0x6381);
// swap_obj.write_primary(837,0x1da2);
// swap_obj.write_primary(838,0x8000);
// swap_obj.write_primary(839,0x1dbd);
// swap_obj.write_primary(840,0x7382);
// swap_obj.write_primary(841,0x7981);
// swap_obj.write_primary(842,0x7f80);
// swap_obj.write_primary(843,0x5920);
// swap_obj.write_primary(844,0x1928);
// swap_obj.write_primary(845,0x4fcc);
// swap_obj.write_primary(846,0x0809);
// swap_obj.write_primary(847,0x288d);
// swap_obj.write_primary(848,0x1900);
// swap_obj.write_primary(849,0x7300);
// swap_obj.write_primary(850,0x2885);
// swap_obj.write_primary(851,0x1900);
// swap_obj.write_primary(852,0x1900);
// swap_obj.write_primary(853,0x5260);
// swap_obj.write_primary(854,0x1263);
// swap_obj.write_primary(855,0x7300);
// swap_obj.write_primary(856,0x6f80);
// swap_obj.write_primary(857,0x6981);
// swap_obj.write_primary(858,0x6382);
// swap_obj.write_primary(859,0x1da3);
// swap_obj.write_primary(860,0x8000);
// swap_obj.write_primary(861,0x1dbe);
// swap_obj.write_primary(862,0x7381);
// swap_obj.write_primary(863,0x7f80);
// swap_obj.write_primary(864,0x5260);
// swap_obj.write_primary(865,0x4fc2);
// swap_obj.write_primary(866,0x6f80);
// swap_obj.write_primary(867,0x6381);
// swap_obj.write_primary(868,0x1da2);
// swap_obj.write_primary(869,0x8000);
// swap_obj.write_primary(870,0x1dbe);
// swap_obj.write_primary(871,0x7981);
// swap_obj.write_primary(872,0x7f80);
// swap_obj.write_primary(873,0x5920);
// swap_obj.write_primary(874,0x1928);
// swap_obj.write_primary(875,0x4fae);
// swap_obj.write_primary(876,0x0804);
// swap_obj.write_primary(877,0x286a);
// swap_obj.write_primary(878,0x1900);
// swap_obj.write_primary(879,0x1900);
// swap_obj.write_primary(880,0x6100);
// swap_obj.write_primary(881,0x6f80);
// swap_obj.write_primary(882,0x6981);
// swap_obj.write_primary(883,0x1da2);
// swap_obj.write_primary(884,0x8000);
// swap_obj.write_primary(885,0x1dbe);
// swap_obj.write_primary(886,0x7981);
// swap_obj.write_primary(887,0x7f80);
// swap_obj.write_primary(888,0x5920);
// swap_obj.write_primary(889,0x1928);
// swap_obj.write_primary(890,0x4f9f);
// swap_obj.write_primary(891,0x0805);
// swap_obj.write_primary(892,0x285b);
// swap_obj.write_primary(893,0x1900);
// swap_obj.write_primary(894,0x1900);
// swap_obj.write_primary(895,0x1921);
// swap_obj.write_primary(896,0x7300);
// swap_obj.write_primary(897,0x6f80);
// swap_obj.write_primary(898,0x6981);
// swap_obj.write_primary(899,0x1da2);
// swap_obj.write_primary(900,0x8000);
// swap_obj.write_primary(901,0x1dbe);
// swap_obj.write_primary(902,0x7981);
// swap_obj.write_primary(903,0x7f80);
// swap_obj.write_primary(904,0x5920);
// swap_obj.write_primary(905,0x1928);
// swap_obj.write_primary(906,0x4f8f);
// swap_obj.write_primary(907,0x0805);
// swap_obj.write_primary(908,0x284b);
// swap_obj.write_primary(909,0x1900);
// swap_obj.write_primary(910,0x1900);
// swap_obj.write_primary(911,0x1921);
// swap_obj.write_primary(912,0x6100);
// swap_obj.write_primary(913,0x6f80);
// swap_obj.write_primary(914,0x6981);
// swap_obj.write_primary(915,0x1da2);
// swap_obj.write_primary(916,0x8000);
// swap_obj.write_primary(917,0x1dbd);
// swap_obj.write_primary(918,0x7382);
// swap_obj.write_primary(919,0x7981);
// swap_obj.write_primary(920,0x7f80);
// swap_obj.write_primary(921,0x5920);
// swap_obj.write_primary(922,0x1926);
// swap_obj.write_primary(923,0x4f7e);
// swap_obj.write_primary(924,0x0804);
// swap_obj.write_primary(925,0x283b);
// swap_obj.write_primary(926,0x1900);
// swap_obj.write_primary(927,0x1900);
// swap_obj.write_primary(928,0x7300);
// swap_obj.write_primary(929,0x6f80);
// swap_obj.write_primary(930,0x6981);
// swap_obj.write_primary(931,0x6382);
// swap_obj.write_primary(932,0x1da3);
// swap_obj.write_primary(933,0xc1c0);
// swap_obj.write_primary(934,0x1dbe);
// swap_obj.write_primary(935,0x7381);
// swap_obj.write_primary(936,0x7f80);
// swap_obj.write_primary(937,0x5260);
// swap_obj.write_primary(938,0x1261);
// swap_obj.write_primary(939,0x4fe9);
// swap_obj.write_primary(940,0x6f80);
// swap_obj.write_primary(941,0x6381);
// swap_obj.write_primary(942,0x1da2);
// swap_obj.write_primary(943,0x8000);
// swap_obj.write_primary(944,0x1dbe);
// swap_obj.write_primary(945,0x7381);
// swap_obj.write_primary(946,0x7f80);
// swap_obj.write_primary(947,0x5260);
// swap_obj.write_primary(948,0x4fe0);
// swap_obj.write_primary(949,0x6f80);
// swap_obj.write_primary(950,0x6381);
// swap_obj.write_primary(951,0x1da2);
// swap_obj.write_primary(952,0x8000);
// swap_obj.write_primary(953,0x1dbe);
// swap_obj.write_primary(954,0x7981);
// swap_obj.write_primary(955,0x7f80);
// swap_obj.write_primary(956,0x5920);
// swap_obj.write_primary(957,0x1926);
// swap_obj.write_primary(958,0x4f5b);
// swap_obj.write_primary(959,0x0804);
// swap_obj.write_primary(960,0x2818);
// swap_obj.write_primary(961,0x1900);
// swap_obj.write_primary(962,0x1900);
// swap_obj.write_primary(963,0x6100);
// swap_obj.write_primary(964,0x6f80);
// swap_obj.write_primary(965,0x6981);
// swap_obj.write_primary(966,0x1da2);
// swap_obj.write_primary(967,0x8000);
// swap_obj.write_primary(968,0x1dbe);
// swap_obj.write_primary(969,0x7981);
// swap_obj.write_primary(970,0x7f80);
// swap_obj.write_primary(971,0x5920);
// swap_obj.write_primary(972,0x1926);
// swap_obj.write_primary(973,0x4f4c);
// swap_obj.write_primary(974,0x0805);
// swap_obj.write_primary(975,0x2809);
// swap_obj.write_primary(976,0x1900);
// swap_obj.write_primary(977,0x1900);
// swap_obj.write_primary(978,0x1921);
// swap_obj.write_primary(979,0x6100);
// swap_obj.write_primary(980,0x6f80);
// swap_obj.write_primary(981,0x6981);
// swap_obj.write_primary(982,0x1da2);
// swap_obj.write_primary(983,0x8000);
// swap_obj.write_primary(984,0xfe30);
// swap_obj.write_primary(985,0xfe40);
// swap_obj.write_primary(986,0xfe70);
// swap_obj.write_primary(987,0xfe60);
// swap_obj.write_primary(988,0xfe50);
// swap_obj.write_primary(989,0x01b0);
// swap_obj.write_primary(990,0x01e0);
// swap_obj.write_primary(991,0x1dbe);
// swap_obj.write_primary(992,0x7981);
// swap_obj.write_primary(993,0x7f80);
// swap_obj.write_primary(994,0x5920);
// swap_obj.write_primary(995,0x1922);
// swap_obj.write_primary(996,0x4f35);
// swap_obj.write_primary(997,0x0806);
// swap_obj.write_primary(998,0x29f5);
// swap_obj.write_primary(999,0x1900);
// swap_obj.write_primary(1000,0x1900);
// swap_obj.write_primary(1001,0x7300);
// swap_obj.write_primary(1002,0x1921);
// swap_obj.write_primary(1003,0x7500);
// swap_obj.write_primary(1004,0x6f80);
// swap_obj.write_primary(1005,0x6981);
// swap_obj.write_primary(1006,0x1da2);
// swap_obj.write_primary(1007,0x8000);
// swap_obj.write_primary(1008,0x1dbe);
// swap_obj.write_primary(1009,0x7981);
// swap_obj.write_primary(1010,0x7f80);
// swap_obj.write_primary(1011,0x5920);
// swap_obj.write_primary(1012,0x1922);
// swap_obj.write_primary(1013,0x4f24);
// swap_obj.write_primary(1014,0x0805);
// swap_obj.write_primary(1015,0x29e4);
// swap_obj.write_primary(1016,0x1900);
// swap_obj.write_primary(1017,0x1900);
// swap_obj.write_primary(1018,0x5fe0);
// swap_obj.write_primary(1019,0x7f00);
// swap_obj.write_primary(1020,0x6f80);
// swap_obj.write_primary(1021,0x6981);
// swap_obj.write_primary(1022,0x1da2);
// swap_obj.write_primary(1023,0x8000);
// swap_obj.write_primary(1024,0x1dbe);
// swap_obj.write_primary(1025,0x7981);
// swap_obj.write_primary(1026,0x7f80);
// swap_obj.write_primary(1027,0x5920);
// swap_obj.write_primary(1028,0x1922);
// swap_obj.write_primary(1029,0x4f14);
// swap_obj.write_primary(1030,0x0804);
// swap_obj.write_primary(1031,0x29d4);
// swap_obj.write_primary(1032,0x1900);
// swap_obj.write_primary(1033,0x1900);
// swap_obj.write_primary(1034,0x6100);
// swap_obj.write_primary(1035,0x6f80);
// swap_obj.write_primary(1036,0x6981);
// swap_obj.write_primary(1037,0x1da2);
// swap_obj.write_primary(1038,0x8000);
// swap_obj.write_primary(1039,0x1dbe);
// swap_obj.write_primary(1040,0x7981);
// swap_obj.write_primary(1041,0x7f80);
// swap_obj.write_primary(1042,0x5920);
// swap_obj.write_primary(1043,0x1922);
// swap_obj.write_primary(1044,0x4f05);
// swap_obj.write_primary(1045,0x0805);
// swap_obj.write_primary(1046,0x29c5);
// swap_obj.write_primary(1047,0x1900);
// swap_obj.write_primary(1048,0x1900);
// swap_obj.write_primary(1049,0x1921);
// swap_obj.write_primary(1050,0x6100);
// swap_obj.write_primary(1051,0x6f80);
// swap_obj.write_primary(1052,0x6981);
// swap_obj.write_primary(1053,0x1da2);
// swap_obj.write_primary(1054,0x8000);
// swap_obj.write_primary(1055,0x1dbe);
// swap_obj.write_primary(1056,0x7981);
// swap_obj.write_primary(1057,0x7f80);
// swap_obj.write_primary(1058,0x5920);
// swap_obj.write_primary(1059,0x1922);
// swap_obj.write_primary(1060,0x4ef5);
// swap_obj.write_primary(1061,0x0804);
// swap_obj.write_primary(1062,0x29b4);
// swap_obj.write_primary(1063,0x1900);
// swap_obj.write_primary(1064,0x1900);
// swap_obj.write_primary(1065,0x7300);
// swap_obj.write_primary(1066,0x6f80);
// swap_obj.write_primary(1067,0x6981);
// swap_obj.write_primary(1068,0x1da2);
// swap_obj.write_primary(1069,0xc1c0);
// swap_obj.write_primary(1070,0x1dbe);
// swap_obj.write_primary(1071,0x7981);
// swap_obj.write_primary(1072,0x7f80);
// swap_obj.write_primary(1073,0x5920);
// swap_obj.write_primary(1074,0x1922);
// swap_obj.write_primary(1075,0x4ee6);
// swap_obj.write_primary(1076,0x0805);
// swap_obj.write_primary(1077,0x29a5);
// swap_obj.write_primary(1078,0x1900);
// swap_obj.write_primary(1079,0x1900);
// swap_obj.write_primary(1080,0x1921);
// swap_obj.write_primary(1081,0x7300);
// swap_obj.write_primary(1082,0x6f80);
// swap_obj.write_primary(1083,0x6981);
// swap_obj.write_primary(1084,0x1da2);
// swap_obj.write_primary(1085,0xc1c0);
// swap_obj.write_primary(1086,0x1dbe);
// swap_obj.write_primary(1087,0x7f81);
// swap_obj.write_primary(1088,0x7380);
// swap_obj.write_primary(1089,0x5260);
// swap_obj.write_primary(1090,0x4fdc);
// swap_obj.write_primary(1091,0x6380);
// swap_obj.write_primary(1092,0x1da1);
// swap_obj.write_primary(1093,0x4fe8);
// swap_obj.write_primary(1094,0x2397);
// swap_obj.write_primary(1095,0x1240);
// swap_obj.write_primary(1096,0x7440);
// swap_obj.write_primary(1097,0x6f80);
// swap_obj.write_primary(1098,0x1da1);
// swap_obj.write_primary(1099,0x8000);
// swap_obj.write_primary(1100,0x1dbe);
// swap_obj.write_primary(1101,0x7f81);
// swap_obj.write_primary(1102,0x7380);
// swap_obj.write_primary(1103,0x5260);
// swap_obj.write_primary(1104,0x1261);
// swap_obj.write_primary(1105,0x4fcd);
// swap_obj.write_primary(1106,0x6380);
// swap_obj.write_primary(1107,0x1da1);
// swap_obj.write_primary(1108,0x4fd9);
// swap_obj.write_primary(1109,0x2388);
// swap_obj.write_primary(1110,0x1240);
// swap_obj.write_primary(1111,0x7440);
// swap_obj.write_primary(1112,0x6f80);
// swap_obj.write_primary(1113,0x1da1);
// swap_obj.write_primary(1114,0x8000);
// swap_obj.write_primary(1115,0x1dbe);
// swap_obj.write_primary(1116,0x7381);
// swap_obj.write_primary(1117,0x7f80);
// swap_obj.write_primary(1118,0x5260);
// swap_obj.write_primary(1119,0x4fce);
// swap_obj.write_primary(1120,0x6f80);
// swap_obj.write_primary(1121,0x6381);
// swap_obj.write_primary(1122,0x1da2);
// swap_obj.write_primary(1123,0x8000);
// swap_obj.write_primary(1124,0x1dbe);
// swap_obj.write_primary(1125,0x7981);
// swap_obj.write_primary(1126,0x7f80);
// swap_obj.write_primary(1127,0x5920);
// swap_obj.write_primary(1128,0x1922);
// swap_obj.write_primary(1129,0x4eb0);
// swap_obj.write_primary(1130,0x0804);
// swap_obj.write_primary(1131,0x296f);
// swap_obj.write_primary(1132,0x1900);
// swap_obj.write_primary(1133,0x1900);
// swap_obj.write_primary(1134,0x6100);
// swap_obj.write_primary(1135,0x6f80);
// swap_obj.write_primary(1136,0x6981);
// swap_obj.write_primary(1137,0x1da2);
// swap_obj.write_primary(1138,0x8000);
// swap_obj.write_primary(1139,0x1dbe);
// swap_obj.write_primary(1140,0x7981);
// swap_obj.write_primary(1141,0x7f80);
// swap_obj.write_primary(1142,0x5920);
// swap_obj.write_primary(1143,0x1922);
// swap_obj.write_primary(1144,0x4ea1);
// swap_obj.write_primary(1145,0x0805);
// swap_obj.write_primary(1146,0x2960);
// swap_obj.write_primary(1147,0x1900);
// swap_obj.write_primary(1148,0x1900);
// swap_obj.write_primary(1149,0x1921);
// swap_obj.write_primary(1150,0x6100);
// swap_obj.write_primary(1151,0x6f80);
// swap_obj.write_primary(1152,0x6981);
// swap_obj.write_primary(1153,0x1da2);
// swap_obj.write_primary(1154,0x8000);
// swap_obj.write_primary(1155,0x1dbf);
// swap_obj.write_primary(1156,0x7380);
// swap_obj.write_primary(1157,0x2354);
// swap_obj.write_primary(1158,0x7040);
// swap_obj.write_primary(1159,0x6380);
// swap_obj.write_primary(1160,0x1da1);
// swap_obj.write_primary(1161,0x8000);
// swap_obj.write_primary(1162,0x214f);
// swap_obj.write_primary(1163,0x6000);
// swap_obj.write_primary(1164,0x8000);
// swap_obj.write_primary(1165,0xe010);
// swap_obj.write_primary(1166,0xf022);
// swap_obj.write_primary(1167,0xf025);
// swap_obj.write_primary(1168,0xe046);
// swap_obj.write_primary(1169,0xf022);
// swap_obj.write_primary(1170,0xf025);
// swap_obj.write_primary(1171,0x1dbf);
// swap_obj.write_primary(1172,0x7180);
// swap_obj.write_primary(1173,0xe066);
// swap_obj.write_primary(1174,0xf022);
// swap_obj.write_primary(1175,0xa005);
// swap_obj.write_primary(1176,0x0401);
// swap_obj.write_primary(1177,0xf025);
// swap_obj.write_primary(1178,0x6180);
// swap_obj.write_primary(1179,0x1da1);
// swap_obj.write_primary(1180,0x8000);
// swap_obj.write_primary(1181,0x0601);
// swap_obj.write_primary(1182,0x000a);
// swap_obj.write_primary(1183,0x000a);
// swap_obj.write_primary(1184,0x002d);
// swap_obj.write_primary(1185,0x002d);
// swap_obj.write_primary(1186,0x002d);
// swap_obj.write_primary(1187,0x0020);
// swap_obj.write_primary(1188,0x0050);
// swap_obj.write_primary(1189,0x0072);
// swap_obj.write_primary(1190,0x0069);
// swap_obj.write_primary(1191,0x0076);
// swap_obj.write_primary(1192,0x0069);
// swap_obj.write_primary(1193,0x006c);
// swap_obj.write_primary(1194,0x0065);
// swap_obj.write_primary(1195,0x0067);
// swap_obj.write_primary(1196,0x0065);
// swap_obj.write_primary(1197,0x0020);
// swap_obj.write_primary(1198,0x006d);
// swap_obj.write_primary(1199,0x006f);
// swap_obj.write_primary(1200,0x0064);
// swap_obj.write_primary(1201,0x0065);
// swap_obj.write_primary(1202,0x0020);
// swap_obj.write_primary(1203,0x0076);
// swap_obj.write_primary(1204,0x0069);
// swap_obj.write_primary(1205,0x006f);
// swap_obj.write_primary(1206,0x006c);
// swap_obj.write_primary(1207,0x0061);
// swap_obj.write_primary(1208,0x0074);
// swap_obj.write_primary(1209,0x0069);
// swap_obj.write_primary(1210,0x006f);
// swap_obj.write_primary(1211,0x006e);
// swap_obj.write_primary(1212,0x0020);
// swap_obj.write_primary(1213,0x0028);
// swap_obj.write_primary(1214,0x0052);
// swap_obj.write_primary(1215,0x0054);
// swap_obj.write_primary(1216,0x0049);
// swap_obj.write_primary(1217,0x0020);
// swap_obj.write_primary(1218,0x0069);
// swap_obj.write_primary(1219,0x006e);
// swap_obj.write_primary(1220,0x0020);
// swap_obj.write_primary(1221,0x0075);
// swap_obj.write_primary(1222,0x0073);
// swap_obj.write_primary(1223,0x0065);
// swap_obj.write_primary(1224,0x0072);
// swap_obj.write_primary(1225,0x0020);
// swap_obj.write_primary(1226,0x006d);
// swap_obj.write_primary(1227,0x006f);
// swap_obj.write_primary(1228,0x0064);
// swap_obj.write_primary(1229,0x0065);
// swap_obj.write_primary(1230,0x0029);
// swap_obj.write_primary(1231,0x0021);
// swap_obj.write_primary(1232,0x0020);
// swap_obj.write_primary(1233,0x002d);
// swap_obj.write_primary(1234,0x002d);
// swap_obj.write_primary(1235,0x002d);
// swap_obj.write_primary(1236,0x000a);
// swap_obj.write_primary(1237,0x000a);
// swap_obj.write_primary(1238,0x0000);
// swap_obj.write_primary(1239,0x000a);
// swap_obj.write_primary(1240,0x000a);
// swap_obj.write_primary(1241,0x002d);
// swap_obj.write_primary(1242,0x002d);
// swap_obj.write_primary(1243,0x002d);
// swap_obj.write_primary(1244,0x0020);
// swap_obj.write_primary(1245,0x0049);
// swap_obj.write_primary(1246,0x006c);
// swap_obj.write_primary(1247,0x006c);
// swap_obj.write_primary(1248,0x0065);
// swap_obj.write_primary(1249,0x0067);
// swap_obj.write_primary(1250,0x0061);
// swap_obj.write_primary(1251,0x006c);
// swap_obj.write_primary(1252,0x0020);
// swap_obj.write_primary(1253,0x006f);
// swap_obj.write_primary(1254,0x0070);
// swap_obj.write_primary(1255,0x0063);
// swap_obj.write_primary(1256,0x006f);
// swap_obj.write_primary(1257,0x0064);
// swap_obj.write_primary(1258,0x0065);
// swap_obj.write_primary(1259,0x0020);
// swap_obj.write_primary(1260,0x0028);
// swap_obj.write_primary(1261,0x0030);
// swap_obj.write_primary(1262,0x0062);
// swap_obj.write_primary(1263,0x0031);
// swap_obj.write_primary(1264,0x0031);
// swap_obj.write_primary(1265,0x0030);
// swap_obj.write_primary(1266,0x0031);
// swap_obj.write_primary(1267,0x0029);
// swap_obj.write_primary(1268,0x0021);
// swap_obj.write_primary(1269,0x0020);
// swap_obj.write_primary(1270,0x002d);
// swap_obj.write_primary(1271,0x002d);
// swap_obj.write_primary(1272,0x002d);
// swap_obj.write_primary(1273,0x000a);
// swap_obj.write_primary(1274,0x000a);
// swap_obj.write_primary(1275,0x0000);
// swap_obj.write_primary(1276,0x000a);
// swap_obj.write_primary(1277,0x000a);
// swap_obj.write_primary(1278,0x002d);
// swap_obj.write_primary(1279,0x002d);
// swap_obj.write_primary(1280,0x002d);
// swap_obj.write_primary(1281,0x0020);
// swap_obj.write_primary(1282,0x0041);
// swap_obj.write_primary(1283,0x0063);
// swap_obj.write_primary(1284,0x0063);
// swap_obj.write_primary(1285,0x0065);
// swap_obj.write_primary(1286,0x0073);
// swap_obj.write_primary(1287,0x0073);
// swap_obj.write_primary(1288,0x0020);
// swap_obj.write_primary(1289,0x0063);
// swap_obj.write_primary(1290,0x006f);
// swap_obj.write_primary(1291,0x006e);
// swap_obj.write_primary(1292,0x0074);
// swap_obj.write_primary(1293,0x0072);
// swap_obj.write_primary(1294,0x006f);
// swap_obj.write_primary(1295,0x006c);
// swap_obj.write_primary(1296,0x0020);
// swap_obj.write_primary(1297,0x0076);
// swap_obj.write_primary(1298,0x0069);
// swap_obj.write_primary(1299,0x006f);
// swap_obj.write_primary(1300,0x006c);
// swap_obj.write_primary(1301,0x0061);
// swap_obj.write_primary(1302,0x0074);
// swap_obj.write_primary(1303,0x0069);
// swap_obj.write_primary(1304,0x006f);
// swap_obj.write_primary(1305,0x006e);
// swap_obj.write_primary(1306,0x0021);
// swap_obj.write_primary(1307,0x0020);
// swap_obj.write_primary(1308,0x002d);
// swap_obj.write_primary(1309,0x002d);
// swap_obj.write_primary(1310,0x002d);
// swap_obj.write_primary(1311,0x000a);
// swap_obj.write_primary(1312,0x000a);
// swap_obj.write_primary(1313,0x0000);
// swap_obj.write_primary(1314,0x0000);
// swap_obj.write_primary(1315,0x0000);
// swap_obj.write_primary(1316,0x0000);
// swap_obj.write_primary(1317,0x0000);
// swap_obj.write_primary(1318,0x0000);


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





   // let mut timer_shim = timers::TimersShim::new(&sys.power_control, timer_req{timer0: t0, timer1: t1}, nvic);
    // let wr_res = swap_obj.write_primary(650,0x82ac);
    // match wr_res {
    // 	Ok(()) => {
    // 		let ok = 0;
    // 	},
    //     _ => {
    //     	loop{}
    //     },
    // }

    let sev = swap_obj.read_primary(12288);
        match sev{
       	Ok(out)=>{
       		let x = out;

       	}
       	_=>{
       		loop{}
       	}
       }
    let mut tm4c_mem = tm4c_lc3_memory{
    	tm4c_mem_obj: swap_obj,
    };

   // let sys = sc.constrain();
    let mut pwm_shim = pwm::PwmShim::new(pwm_req{
        //sysctl: sc,
        portb: portb,
        portd: portd,
        pwm0: pwm0,
        pwm1: pwm1,
    }, &sys.power_control);

    let mut timer_shim = timers::TimersShim::new(&sys.power_control, timer_req{timer0: t0, timer1: t1});

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

 //     interp.set_word(0x3000, 0x1021);
 //     interp.set_word(0x3001, 0x1021);
 //     interp.set_word(0x3002, 0x1021);
 //     interp.set_word(0x3003, 0x1021);
 //     interp.set_word(0x3004, 0x1021);
 //   //  interp.set_word(0x3005, 0x1021);
 //     interp.set_word(0x3005, 0x5020);
 //     interp.set_word(0x3006, 0x1020);
      let mut out = interp.get_register(Reg::R0);
 //      //let mut out5 = interp.get_register(Reg::R0);
 //     interp.set_word(0x3007, 0xF031);
 //      interp.set_word(0x3008, 0x5020);
 //     interp.set_word(0x3009, 0x1020);
 //     interp.set_word(0x300A, 0x5260);
 //      interp.set_word(0x300B, 0x1261);
 //      out = interp.get_register(Reg::R1);
 //     // let mut ou = interp.get_register(Reg::R0);
 //      interp.set_word(0x300C, 0xF035);
 // //    interp.set_pc(0x3000);
  	  let mut pc = interp.get_pc();
      // interp.step();
      // pc = interp.get_pc();
      // interp.step();
      // interp.step();
      // interp.step();
      // interp.step();
      // interp.step();
      // interp.step();
      // interp.step();
      // interp.step();
      // interp.step();
      //  pc = interp.get_pc();
      //  let word = interp.get_word(pc);
      // interp.step();
      // pc = interp.get_pc();
      // interp.step();
      // interp.step();
      // pc = interp.get_pc();
      // while (interp.get_pc() != 0x3000){
      //  pc = interp.get_pc();
      //  let word = interp.get_word(pc).unwrap();
      // 	interp.step();

      // }
     // loop{}





    // let mut uart = hal::serial::Serial::uart0(
    //     p.UART0,
    //     porta
    //         .pa1
    //         .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //     porta
    //         .pa0
    //         .into_af_push_pull::<hal::gpio::AF1>(&mut porta.control),
    //     (),
    //     (),
    //     115200_u32.bps(),
    //     hal::serial::NewlineMode::SwapLFtoCRLF,
    //     &clocks,
    //     &sys.power_control,
    // );

       while (interp.get_pc() != 12288){
       pc = interp.get_pc();
       let word = interp.get_word(pc);
       match word {
           Ok(res) =>{
            let x = res;
           },
           _=> {
            let failed = 1;
           },
       }
        
        interp.step();


      }

      let mut a0=0;
      let mut a1=0;
      let mut a2=0;
     loop{
        interp.set_pc(12288);
         interp.set_register(Reg::R0, 0);
        while (interp.get_pc() != 12289){
          interp.step();
          pc = interp.get_pc();
        }       
        interp.set_pc(12289);
        interp.set_register(Reg::R0, 0);
        while (interp.get_pc() != 12290){
          interp.step();
          pc = interp.get_pc();
        }
        a0 = interp.get_register(Reg::R0);

        if (a0>105){
          interp.set_pc(12290);
          interp.set_register(Reg::R0, 0);
          interp.set_register(Reg::R1, 255);
          interp.set_register(Reg::R2, 125);
          while (interp.get_pc() != 12291){
          interp.step();
         }
          interp.set_pc(12290);
          interp.set_register(Reg::R0, 1);
          interp.set_register(Reg::R1, 255);
          interp.set_register(Reg::R2, 125);
          while (interp.get_pc() != 12291){
          interp.step();
         }      
        }

        else{
          interp.set_pc(12290);
          interp.set_register(Reg::R0, 1);
          interp.set_register(Reg::R1, 255);
          interp.set_register(Reg::R2, 125);
          while (interp.get_pc() != 12291){
          interp.step();
         }   
          interp.set_pc(12290);
          interp.set_register(Reg::R0, 0);
          interp.set_register(Reg::R1, 255);
          interp.set_register(Reg::R2, 1);
          while (interp.get_pc() != 12291){
          interp.step();
         }  
        }


     }

     
    loop{
 

       let came_here = 1;

    	
    }
}

