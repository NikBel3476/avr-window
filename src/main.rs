#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use ruduino::{Pin, Register};
use ruduino::cores::current;
use ruduino::cores::current::{port, DDRF, PORTF};
use ruduino::legacy::serial;
use ruduino::modules::{Timer16, WaveformGenerationMode16, ClockSource16};
use ruduino::interrupt::without_interrupts;

// uart
const BAUD: u32 = 9600;
const UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

// timer
const DESIRED_HZ_TIM1: f64 = 1.0;
const TIM1_PRESCALER: u64 = 1024;
const INTERRUPT_EVERY_1_HZ_1024_PRESCALER: u16 =
    ((ruduino::config::CPU_FREQUENCY_HZ as f64 / (DESIRED_HZ_TIM1 * TIM1_PRESCALER as f64)) as u64 - 1) as u16;

static mut IS_WINDOW_OPEN: bool = false;
static mut IS_WINDOW_CLOSE: bool = true;

#[no_mangle]
pub extern fn main() {
	without_interrupts(|| {
		current::Timer16::setup()
			.waveform_generation_mode(WaveformGenerationMode16::ClearOnTimerMatchOutputCompare)
			.clock_source(ClockSource16::Prescale1024)
			.output_compare_1(Some(INTERRUPT_EVERY_1_HZ_1024_PRESCALER))
			.configure();

		serial::Serial::new(UBRR)
			.character_size(serial::CharacterSize::EightBits)
			.mode(serial::Mode::Asynchronous)
			.parity(serial::Parity::Disabled)
			.stop_bits(serial::StopBits::OneBit)
			.configure();
	});

	port::B5::set_output();
	port::B5::set_high();
	port::E0::set_output();
	port::E0::set_high();

	// pin for engine control
	port::E2::set_output();
	port::E2::set_low();

	//pins for buttons handling
	port::E3::set_input();
	port::E4::set_input();
	port::E5::set_input();
	port::E6::set_input();

	DDRF::set_mask_raw(0b11111111);
	PORTF::set_mask_raw(0b0);

	loop {
		// serial::transmit(0b00001111);

		// transmitting data via uart
		// for &b in b"Hello, from Rust!\n" {
		// 	serial::transmit(b);
		// }

		if port::E3::is_high() {
			port::E2::set_high();
		}

		if port::E4::is_high() {
			port::E2::set_low()
		}

		unsafe {
			if IS_WINDOW_CLOSE && port::E6::is_high() {
				IS_WINDOW_CLOSE = false;
				IS_WINDOW_OPEN = true;
				port::E2::set_low();
			}
			if IS_WINDOW_OPEN && port::E5::is_high() {
				IS_WINDOW_OPEN = false;
				IS_WINDOW_CLOSE = true;
				port::E2::set_low();
			}
		}

		// read byte if there is something available
		if let Some(b) = serial::try_receive() {
			PORTF::write(0b0);
			PORTF::set_mask_raw(b);
		}
	}
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_17() {
    port::B5::toggle();
	port::E0::toggle();
}