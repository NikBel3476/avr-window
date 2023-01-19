#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use arduino_hal::{
	delay_ms,
	hal::port,
	port::{mode::Output, Pin},
	prelude::*,
};
use atmega_hal::usart::BaudrateExt;
use core::{
	mem,
	sync::atomic::{AtomicBool, Ordering},
};
use ebyte_e32::{
	mode::{Mode, Normal, Program},
	parameters::{AirBaudRate, Persistence},
	Ebyte,
};
use embedded_hal::digital::v2::InputPin;
use nb::block;
use panic_halt as _;
// use ruduino::cores::current;
// use ruduino::cores::current::{port, DDRF, PORTF};
// use ruduino::interrupt::without_interrupts;
// use ruduino::legacy::serial;
// use ruduino::modules::{ClockSource16, Timer16, WaveformGenerationMode16};
// use ruduino::{Pin, Register};

// uart
// const BAUD: u32 = 9600;
// const UBRR: u16 = (ruduino::config::CPU_FREQUENCY_HZ / 16 / BAUD - 1) as u16;

// // timer
// const DESIRED_HZ_TIM1: f64 = 1.0;
// const TIM1_PRESCALER: u64 = 1024;
// const INTERRUPT_EVERY_1_HZ_1024_PRESCALER: u16 =
// 	((ruduino::config::CPU_FREQUENCY_HZ as f64 / (DESIRED_HZ_TIM1 * TIM1_PRESCALER as f64)) as u64
// 		- 1) as u16;

static mut WINDOW_IS_OPEN: AtomicBool = AtomicBool::new(false);
static mut WINDOW_IS_CLOSE: AtomicBool = AtomicBool::new(true);
static mut LED: mem::MaybeUninit<Pin<Output, port::PB5>> = mem::MaybeUninit::uninit();

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

	// let mut serial2 = arduino_hal::Usart::new(
	// 	dp.USART1,
	// 	pins.pd2,
	// 	pins.pd3.into_output(),
	// 	BaudrateExt::into_baudrate(9600),
	// );

	// let m0 = pins.pe2.into_output();
	// let m1 = pins.pe3.into_output();
	// let aux = pins.pe4.into_floating_input();
	// let delay_lora = arduino_hal::Delay::new();

	// let mut ebyte = Ebyte::new(serial1, aux, m0, m1, delay_lora).unwrap_or_else(|_| loop {
	// 	serial2.write_char('e');
	// });

	// let mut led = pins.pb5.into_output();
	// led.set_low();
	// unsafe {
	// 	LED = mem::MaybeUninit::new(led);
	// 	core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
	// }

	// let mut led = pins.pb0.into_output().downgrade();
	// led.toggle();

	// port::B5::set_output();
	// port::B5::set_high();
	// port::E0::set_output();
	// port::E0::set_high();

	// pin for engine control(relay)
	// port::E2::set_output();
	// port::E2::set_low();
	// TODO: change pin
	// let mut relay = pins.pe2.into_output();
	// relay.set_low();

	// TODO: change pins
	// let enable_engine_button = pins.pe3.into_floating_input();
	// let disable_engine_button = pins.pe4.into_floating_input();

	// init leds for lora
	// unsafe {
	// 	(*arduino_hal::pac::PORTF::PTR)
	// 		.ddrf
	// 		.write(|w| w.bits(0b11111111));
	// 	(*arduino_hal::pac::PORTF::PTR).portf.write(|w| w.bits(0));
	// }

	/// engine rotation direction
	/// low -> close; high -> open
	let mut engine_direction = pins.pe2.into_output();
	engine_direction.set_high();

	let mut engine_disable = pins.pe3.into_output();
	engine_disable.set_high();

	// reed switches
	let close_reed_switch = pins.pe5.into_floating_input();
	let open_reed_switch = pins.pe6.into_floating_input();

	// add uart tx interrupt
	// serial.listen(atmega_hal::usart::Event::TxComplete);
	// unsafe {
	// 	avr_device::interrupt::enable();
	// }

	loop {
		// serial::transmit(0b00001111);

		// transmitting data via uart
		// for &b in b"Hello, from Rust!\n" {
		// 	serial::transmit(b);
		// }

		// Read a byte from the serial connection
		// if let Ok(b) = serial1.read() {
		// 	unsafe {
		// 		(*arduino_hal::pac::PORTF::PTR).portf.write(|w| w.bits(b));
		// 	}
		// 	// PORTF::write(b);

		if let Ok(byte) = serial1.read() {
			serial1.write_byte(byte);
			match byte.to_ascii_lowercase() as char {
				'o' => unsafe {
					if WINDOW_IS_CLOSE.load(Ordering::SeqCst) {
						engine_direction.set_high();
						engine_disable.set_low();
					}
				}
				'c' => unsafe {
					if WINDOW_IS_OPEN.load(Ordering::SeqCst) {
						engine_direction.set_low();
						engine_disable.set_low();
					}
				}
				_ => {}
			}
		}

		// delay_ms(1000);
		// match ebyte.model_data() {
		// 	Ok(model_data) => {
		// 		ufmt::uwriteln!(
		// 			&mut serial2,
		// 			"{},{},{}\r",
		// 			model_data.model,
		// 			model_data.version,
		// 			model_data.features
		// 		)
		// 		.void_unwrap();
		// 	}
		// 	Err(_) => {
		// 		serial2.write_char('e');
		// 	}
		// };

		// delay_ms(1000);
		// match ebyte.parameters() {
		// 	Ok(params) => {
		// 		let bytes = params.to_bytes();
		// 		ufmt::uwriteln!(
		// 			&mut serial2,
		// 			"{},{},{},{},{}\r",
		// 			bytes[0],
		// 			bytes[1],
		// 			bytes[2],
		// 			bytes[3],
		// 			bytes[4],
		// 		)
		// 		.void_unwrap();
		// 	}
		// 	Err(_) => {
		// 		serial2.write_char('e');
		// 	}
		// }

		// if let Ok(byte) = serial2.read() {
		// 	match byte.to_ascii_lowercase() as char {
		// 		'm' => {
		// 			let model_data = ebyte.model_data().unwrap_or_else(|e| {
		// 			match e {
		// 				ebyte_e32::Error::SerialWrite => {
		// 					block!(serial2.write('w' as u8)).unwrap();
		// 				}
		// 				ebyte_e32::Error::SerialRead => {
		// 					block!(serial2.write('r' as u8)).unwrap();
		// 				}
		// 				ebyte_e32::Error::ReadModelData => {
		// 					block!(serial2.write('n' as u8)).unwrap();
		// 				}
		// 				_ => {
		// 					block!(serial2.write('e' as u8)).unwrap();
		// 				}
		// 			}
		// 			loop {
		// 				block!(serial2.write('e' as u8)).unwrap();
		// 			}
		// 		});

		// 		block!(serial2.write(model_data.model)).unwrap();
		// 		block!(serial2.write(model_data.version)).unwrap();
		// 		block!(serial2.write(model_data.features)).unwrap();
		// 		},
		// 		'p' => {},
		// 		_ => {}
		// 	}
		// };

		// if enable_engine_button.is_high() {
		// 	relay.set_high();
		// }

		// if disable_engine_button.is_high() {
		// 	relay.set_low();
		// }

		unsafe {
			if WINDOW_IS_CLOSE.load(Ordering::SeqCst) && open_reed_switch.is_high() {
				WINDOW_IS_CLOSE.store(false, Ordering::SeqCst);
				WINDOW_IS_OPEN.store(true, Ordering::SeqCst);
				engine_disable.set_high();
				engine_direction.set_high();
			}
			if WINDOW_IS_OPEN.load(Ordering::SeqCst) && close_reed_switch.is_high() {
				WINDOW_IS_OPEN.store(false, Ordering::SeqCst);
				WINDOW_IS_CLOSE.store(true, Ordering::SeqCst);
				engine_disable.set_high();
				engine_direction.set_high();
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

// #[avr_device::interrupt(atmega128rfa1)]
// fn USART1_TX() {
// 	let led = unsafe { &mut *LED.as_mut_ptr() };

// 	led.toggle();
// }
