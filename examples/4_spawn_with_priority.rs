//! Example for the app macro.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal as _;

#[app(device = stm32l4xx_hal::stm32)]
const APP: () = {
    struct Resources {
        // Resources go here!
    }

    #[init(spawn = [low_prio_task, high_prio_task])]
    fn init(cx: init::Context) {
        // Enable logging
        rtt_init_print!();

        // Spawn the low priority task first and the the high priority task.
        cx.spawn.low_prio_task().ok();
        cx.spawn.high_prio_task().ok(); // Even though it is spawned later it will run first!

        rprintln!("Hello from init!");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Hello from idle!");

        loop {
            continue;
        }
    }

    #[task]
    fn low_prio_task(_cx: low_prio_task::Context) {
        rprintln!("Low prio task!");
    }

    #[task(priority = 2)]
    fn high_prio_task(_cx: high_prio_task::Context) {
        rprintln!("High prio task!");
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn DFSDM1();
        fn DFSDM2();
    }
};
