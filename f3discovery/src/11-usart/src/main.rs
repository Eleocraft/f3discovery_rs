#![no_main]
#![no_std]

#[allow(unused_imports)]
use aux11::{entry, iprint, iprintln, usart1};
use core::fmt::{self, Write};
use core::str;
use heapless::Vec;

macro_rules! uprint {
    ($serial:expr, $($arg:tt)*) => {
        $serial.write_fmt(format_args!($($arg)*)).ok()
    };
}

macro_rules! uprintln {
    ($serial:expr, $fmt:expr) => {
        uprint!($serial, concat!($fmt, "\n"))
    };
    ($serial:expr, $fmt:expr, $($arg:tt)*) => {
        uprint!($serial, concat!($fmt, "\n"), $($arg)*)
    };
}


struct SerialPort {
    usart1: &'static mut usart1::RegisterBlock,
}

impl fmt::Write for SerialPort {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.chars() {
            // wait until it's safe to write to TDR
            while self.usart1.isr.read().txe().bit_is_clear() {} // <- NEW!
        
            self.usart1
                .tdr
                .write(|w| w.tdr().bits(byte as u16));
        }
        Ok(())
    }
}

// #[entry]
// fn main() -> ! {
//     let (usart1, mono_timer, mut itm) = aux11::init();
//
//     let msg = "The quick brown fox jumps over the lazy dog.\n";
//     let instant = mono_timer.now();
//
//
//     for byte in msg.chars() {
//         // wait until it's safe to write to TDR
//         while usart1.isr.read().txe().bit_is_clear() {}
//         
//         usart1
//             .tdr
//             .write(|w| w.tdr().bits(byte as u16));
//     }
//     let elapsed = instant.elapsed(); // in ticks
//
//     iprintln!(
//         &mut itm.stim[0],
//         "`for` loop took {} ticks ({} us)",
//         elapsed,
//         elapsed as f32 / mono_timer.frequency().0 as f32 * 1e6
//     );
//     let (usart1, _mono_timer, _itm) = aux11::init();
//
//     let mut serial = SerialPort { usart1 };
//
//     uprintln!(serial, "The answer is {}", 40 + 2);
//
//     loop {}
// }
// #[entry]
// fn main() -> ! {
//     let (usart1, _mono_timer, _itm) = aux11::init();
//
//     loop {
//         // Wait until there's data available
//         while usart1.isr.read().rxne().bit_is_clear() {}
//
//         // Retrieve the data
//         let byte = usart1.rdr.read().rdr().bits() as u8;
//         
//         // Wait until you can write data
//         while usart1.isr.read().txe().bit_is_clear() {}
//
//         // Write back
//         usart1
//             .tdr
//             .write(|w| w.tdr().bits(byte as u16));
//     }
// }
#[entry]
fn main() -> ! {
    let (usart1, _mono_timer, _itm) = aux11::init();
    
    let mut serial = SerialPort { usart1 };

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8, 32> = Vec::new();

    loop {
        buffer.clear();

        loop {
            while serial.usart1.isr.read().rxne().bit_is_clear() {}
            let byte = serial.usart1.rdr.read().rdr().bits() as u8;
            if byte == b'\r' {
                break;
            }
            if let Err(e) = buffer.push(byte) {
                uprintln!(serial, "Could not read byte: {}", e);
                break;
            }
        }

        buffer.reverse();
        uprintln!(serial, "your word is {}", str::from_utf8(&buffer).unwrap());
    }
}
