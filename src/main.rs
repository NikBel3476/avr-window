#![no_std]
#![no_main]

use ruduino::{Pin, Register};
use ruduino::cores::current::{port, DDRE};

#[no_mangle]
pub extern fn main() {
	loop {
		DDRE::set_mask_raw(port::E0::MASK);

		port::E0::set_high();
		ruduino::delay::delay_ms(1000);
		port::E0::set_low();
		ruduino::delay::delay_ms(1000);
	}
}