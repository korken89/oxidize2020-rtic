//! Example for the app macro.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::{
    app,
    cyccnt::{Instant, U32Ext},
};
use stm32l4xx_hal as _;

#[app(device = stm32l4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        // Resources go here!
        nothing_for_now: (),
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // Core access (`cortex_m::Peripherals` without the SysTick)
        let core = cx.core;

        // Device access (Peripheral Access Crate)
        let pac = cx.device;

        init::LateResources {
            // Late initializations.
            nothing_for_now: (),
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        loop {
            // Do some work or WFI.
            continue;
        }
    }

    extern "C" {
        fn DFSDM1();
        fn DFSDM2();
    }
};
