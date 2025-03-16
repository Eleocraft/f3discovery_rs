#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, Delay, DelayMs, LedArray, OutputSwitch};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, LedArray) = aux5::init();

    let period = 800_u16;
    let step = period / 16_u16;
    let mut state = 0;

    loop {
        for (i, led) in leds.iter_mut().enumerate() {
            if (state - (i as i16 * 2)).abs() <= 1 {
                led.on().ok();
            } else {
                led.off().ok();
            }
        }
        delay.delay_ms(step);

        state += 1;
        if state >= 16 {
            state = 0;
        }
    }
}
