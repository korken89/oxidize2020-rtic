//! Example for the app macro.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal as _;

#[app(device = stm32l4xx_hal::stm32)]
const APP: () = {
    #[init(spawn = [printer1, printer2])]
    fn init(cx: init::Context) {
        // Enable logging
        rtt_init_print!();

        // Print the value via message passing!
        cx.spawn.printer1(42).ok();

        // This will fail!
        if let Err(_) = cx.spawn.printer1(43) {
            rprintln!("Second spawn failed!");
        }

        // Print to the printer that can take multiple!
        cx.spawn.printer2(1).ok();
        cx.spawn.printer2(2).ok();
        cx.spawn.printer2(3).ok();
        cx.spawn.printer2(4).ok();

        rprintln!("Hello from init!");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Hello from idle!");

        loop {
            continue;
        }
    }

    // By adding an input parameter to the task we enable message passing!
    #[task]
    fn printer1(_cx: printer1::Context, val: u32) {
        rprintln!("Printer 1 says: {}", val);
    }

    // With capacity we can take multiple messages!
    #[task(capacity = 4)]
    fn printer2(_cx: printer2::Context, val: u32) {
        rprintln!("Printer 2 says: {}", val);
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn DFSDM1();
        fn DFSDM2();
    }
};
