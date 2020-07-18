//! Example on spawning a software/hardware task.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;
use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal as _;

#[app(device = stm32l4xx_hal::stm32, peripherals = true)]
const APP: () = {
    struct Resources {
        // Resources go here!
    }

    #[init(spawn = [hello_world_task])]
    fn init(cx: init::Context) {
        // Enable logging
        rtt_init_print!();

        // Any spawn in init will run after init finishes.
        cx.spawn.hello_world_task().ok();

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
    fn hello_world_task(_cx: hello_world_task::Context) {
        rprintln!("Hello world from task!");
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn DFSDM1();
    }
};
