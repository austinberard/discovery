#![deny(unsafe_code)]
#![no_main]
#![no_std]

use volatile::Volatile;
use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};


#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();
    let period = 50_u16;
    loop {
        for led in leds.iter_mut() {
            led.on().ok();
            delay.delay_ms(period);
            led.off().ok();
        }
    }
}
