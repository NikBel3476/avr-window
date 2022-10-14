#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use ruduino::{Pin};
use ruduino::cores::current;
use ruduino::cores::current::{port};
use ruduino::modules::{Timer16, WaveformGenerationMode16, ClockSource16};
use ruduino::interrupt::without_interrupts;

const DESIRED_HZ_TIM1: f64 = 1.0;
const TIM1_PRESCALER: u64 = 1024;
const INTERRUPT_EVERY_1_HZ_1024_PRESCALER: u16 =
    ((ruduino::config::CPU_FREQUENCY_HZ as f64 / (DESIRED_HZ_TIM1 * TIM1_PRESCALER as f64)) as u64 - 1) as u16;

#[no_mangle]
pub extern fn main() {
	without_interrupts(|| {
		current::Timer16::setup()
			.waveform_generation_mode(WaveformGenerationMode16::ClearOnTimerMatchOutputCompare)
			.clock_source(ClockSource16::Prescale1024)
			.output_compare_1(Some(INTERRUPT_EVERY_1_HZ_1024_PRESCALER))
			.configure();
	});

	port::B5::set_output();
	port::B5::set_high();
	port::E0::set_output();
	port::E0::set_high();

	loop {

	}
}

#[no_mangle]
pub unsafe extern "avr-interrupt" fn __vector_17() {
    port::B5::toggle();
		port::E0::toggle();
}