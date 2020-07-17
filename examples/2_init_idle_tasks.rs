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

    #[init]
    fn init(_cx: init::Context) {
        // Initialization code goes here
        //
        // ...

        // Enable logging
        rtt_init_print!();

        rprintln!("Hello from init!");
    }

    // Optional idle task, if left out idle will be a WFI.
    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Hello from idle!");

        loop {
            // Do some work or WFI.
            continue;
        }
    }

    #[task]
    fn my_first_software_task(_cx: my_first_software_task::Context) {
        // A software task, i.e. it is NOT bound to any specific interrupt vector.
    }

    #[task(binds = USART1)]
    fn my_first_hardware_task(_cx: my_first_hardware_task::Context) {
        // A hardware task, i.e. it IS bound to a specific interrupt vector.
        // In this case it is bound to the interrupt of USART1.
        //
        // Note that this does NOT set USART1 up, this needs to be done in init.
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn DFSDM1();
    }
};
