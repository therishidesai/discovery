#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

	let mut curr_light = 0_usize;
    loop {
        leds[curr_light].on().ok();
		leds[(curr_light+1) % 8].on().ok();
        delay.delay_ms(50_u16);

        leds[curr_light].off().ok();
        delay.delay_ms(50_u16);
		curr_light = (curr_light + 1) % 8;
    }
}
