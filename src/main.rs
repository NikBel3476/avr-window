#![no_std]
#![no_main]

extern crate avr_std_stub;

use avrd::atmega128rfa1::{
	DDRE,
	PORTE,
	SREG,
	TCCR1A,
	TCCR1B,
	OCR1A,
	TCNT1
};
use core::ptr::{read_volatile, write_volatile};

#[no_mangle]
pub extern fn main() {
	unsafe {
		write_volatile(SREG, !(1 << 7));
		write_volatile(DDRE, read_volatile(DDRE) | 0b11111111);
		write_volatile(PORTE, 0);

		write_volatile(TCCR1A, 0);

		loop {
			write_volatile(PORTE, 0b00000001);
			write_volatile(PORTE, 0b00000000);
		}
	}
}