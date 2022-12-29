#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use core::borrow::Borrow;
use core::ptr::write_volatile;

use arduino_hal;
use arduino_hal::prelude::*;
use atmega_hal;
use atmega_hal::usart::{BaudrateExt};
use ebyte_e32::{
	mode::Normal,
	parameters::{AirBaudRate, Persistence},
	Ebyte,
};
use ruduino::cores::current;
use ruduino::cores::current::{port, DDRF, PORTF};
use ruduino::interrupt::without_interrupts;
use ruduino::legacy::serial;
use ruduino::modules::{ClockSource16, Timer16, WaveformGenerationMode16};
use ruduino::{Pin, Register};

// uart
const BAUD: u32 = 9600;
const UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

// timer
const DESIRED_HZ_TIM1: f64 = 1.0;
const TIM1_PRESCALER: u64 = 1024;
const INTERRUPT_EVERY_1_HZ_1024_PRESCALER: u16 =
	((ruduino::config::CPU_FREQUENCY_HZ as f64 / (DESIRED_HZ_TIM1 * TIM1_PRESCALER as f64)) as u64
		- 1) as u16;

static mut IS_WINDOW_OPEN: bool = false;
static mut IS_WINDOW_CLOSE: bool = true;

#[arduino_hal::entry]
fn main() -> ! {
	// without_interrupts(|| {
	// 	current::Timer16::setup()
	// 		.waveform_generation_mode(WaveformGenerationMode16::ClearOnTimerMatchOutputCompare)
	// 		.clock_source(ClockSource16::Prescale1024)
	// 		.output_compare_1(Some(INTERRUPT_EVERY_1_HZ_1024_PRESCALER))
	// 		.configure();

	// 	serial::Serial::new(UBRR)
	// 		.character_size(serial::CharacterSize::EightBits)
	// 		.mode(serial::Mode::Asynchronous)
	// 		.parity(serial::Parity::Disabled)
	// 		.stop_bits(serial::StopBits::OneBit)
	// 		.configure();
	// });

	let dp = atmega_hal::Peripherals::take().unwrap();

	let pins = atmega_hal::pins!(dp);

	let mut serial1 = arduino_hal::Usart::new(
		dp.USART0,
		pins.pe0,
		pins.pe1.into_output(),
		BaudrateExt::into_baudrate(9600),
	);

	let mut serial2 = arduino_hal::Usart::new(
		dp.USART1,
		pins.pd2,
		pins.pd3.into_output(),
		BaudrateExt::into_baudrate(9600),
	);

	// let mut led = pins.pb0.into_output().downgrade();
	// led.toggle();

	// TODO: init a ebyte module
	// let mut ebyte = Ebyte::new(
	// 	serial,
	// );

	// port::B5::set_output();
	// port::B5::set_high();
	// port::E0::set_output();
	// port::E0::set_high();

	// pin for engine control(relay)
	// port::E2::set_output();
	// port::E2::set_low();
	let mut relay = pins.pe2.into_output();
	relay.set_low();

	//pins for buttons handling
	// port::E3::set_input(); // enable engine
	// port::E4::set_input(); // disable engine
	// port::E5::set_input(); // first reed switch
	// port::E6::set_input(); // second reed switch

	let enable_engine_button = pins.pe3.into_floating_input();
	let disable_engine_button = pins.pe4.into_floating_input();
	let first_reed_switch = pins.pe5.into_floating_input();
	let second_reed_switch = pins.pe6.into_floating_input();

	// init leds for lora
	unsafe {
		(*arduino_hal::pac::PORTF::PTR).ddrf.write(|w| w.bits(0b11111111));
		(*arduino_hal::pac::PORTF::PTR).portf.write(|w| w.bits(0));
	}

	// DDRF::set_mask_raw(0b11111111);
	// PORTF::set_mask_raw(0b0);

	// engine rotation direction
	// port::B6::set_output(); // left
	// port::B5::set_output(); // right
	// port::B6::set_low();
	// port::B5::set_low();

	let mut engine_right = pins.pb5.into_output();
	engine_right.set_low();
	let mut engine_left = pins.pb6.into_output();
	engine_left.set_low();

	loop {
		// serial::transmit(0b00001111);

		// transmitting data via uart
		// for &b in b"Hello, from Rust!\n" {
		// 	serial::transmit(b);
		// }

		// Read a byte from the serial connection
        if let Ok(b) = serial1.read() {
			unsafe {
				(*arduino_hal::pac::PORTF::PTR).portf.write(|w| w.bits(b));
			}
			// PORTF::write(b);

			match b.to_ascii_lowercase() as char {
				'h' => relay.set_low(),
				'f' => relay.set_high(),
				_ => {}
			}
		}

		if let Ok(bluetooth_byte) = serial2.read() {
			match bluetooth_byte.to_ascii_lowercase() as char {
				'e' => relay.set_high(),
				'd' => relay.set_low(),
				_ => {}
			}
		};

		if enable_engine_button.is_high() {
			relay.set_high();
		}

		if disable_engine_button.is_high() {
			relay.set_low();
		}

		unsafe {
			if IS_WINDOW_CLOSE && second_reed_switch.is_high() {
				IS_WINDOW_CLOSE = false;
				IS_WINDOW_OPEN = true;
				relay.set_low();
				engine_left.set_low();
				engine_right.set_high();
			}
			if IS_WINDOW_OPEN && first_reed_switch.is_high() {
				IS_WINDOW_OPEN = false;
				IS_WINDOW_CLOSE = true;
				relay.set_low();
				engine_left.set_high();
				engine_right.set_low();
			}
		}

		// read byte if there is something available
		// if let Some(b) = serial::try_receive() {
		// 	PORTF::write(0b0);
		// 	PORTF::set_mask_raw(b);
		// }
	}
}

// #[no_mangle]
// pub unsafe extern "avr-interrupt" fn __vector_17() {
// 	// port::B5::toggle();
// 	port::E0::toggle();
// }

// #[avr_device::interrupt(atmega328p)]
// fn TIMER1_COMPA() {
    
// }