// #![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux8::entry;
use core::ptr;

#[entry]
fn main() -> ! {
    let (gpioe, rcc) = aux8::init();

    // initialize GPIOE
    // 1. directly
    // // a) enable gpioe pins
    // const RCC_REG: u32 = 0x4002_1000;
    // const AHBENR_OFF: u32 = 0x14;
    // let ahbenr_adr = RCC_REG + AHBENR_OFF;
    //
    // unsafe {
    //     ptr::write_volatile(ahbenr_adr as *mut u32, 1 << 21);
    // }
    // // b) set gpioe pins to output
    // const GPIOE_REG: u32 = 0x4800_1000;
    // const MODER_OFF: u32 = 0x00;
    // let moder_adr = GPIOE_REG + MODER_OFF;
    //
    // unsafe {
    //     ptr::write_volatile(moder_adr as *mut u32, 0b01010101_01010101_00000000_00000000);
    // }

    // 2. via register abstraction
    // a) enable gpioe pins
    rcc.ahbenr.write(|w| w.iopeen().set_bit());
    // b) set gpioe pins to output
    gpioe.moder.write(|w| {
        w.moder8().output();
        w.moder9().output();
        w.moder10().output();
        w.moder11().output();
        w.moder12().output();
        w.moder13().output();
        w.moder14().output();
        w.moder15().output();
        w
    });

    // Turn on all the LEDs in the compass
    gpioe.odr.write(|w| {
        w.odr8().set_bit();
        w.odr9().set_bit();
        w.odr10().set_bit();
        w.odr11().set_bit();
        w.odr12().set_bit();
        w.odr13().set_bit();
        w.odr14().set_bit();
        w.odr15().set_bit();
        w
    });

    aux8::bkpt();

    loop {}
}
