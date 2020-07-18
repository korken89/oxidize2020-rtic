//! Example on using resources and locks to claim resources.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::{app, cyccnt::U32Ext};
use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal as _;

#[app(device = stm32l4xx_hal::stm32, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        // Resources go here!
        value: u32,
    }

    #[init(schedule = [high_prio_task, low_prio_task])]
    fn init(cx: init::Context) -> init::LateResources {
        // When using schedule and a monotonic timer, remember to start the timer!

        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic timer (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Enable logging
        rtt_init_print!();

        // Schedule the tasks so that the high priority task will run inside the low priority task.
        cx.schedule
            .low_prio_task(cx.start + 1_000_000.cycles())
            .ok();
        cx.schedule
            .high_prio_task(cx.start + 2_000_000.cycles())
            .ok();

        rprintln!("Hello from init!");

        init::LateResources { value: 42 }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Hello from idle!");

        loop {
            continue;
        }
    }

    #[task(priority = 2, resources = [value])]
    fn high_prio_task(cx: high_prio_task::Context) {
        rprintln!("High prio begin");

        // Highest priority task accessing data, NO lock is needed.
        // This will most likely change to v0.6 of RTIC to be symmetric.
        *cx.resources.value += 1;

        rprintln!("High prio end");
    }

    #[task(resources = [value])]
    fn low_prio_task(mut cx: low_prio_task::Context) {
        // Lower priority task accessing data, lock is needed.
        let value = cx.resources.value.lock(|v| *v);

        rprintln!("Low prio begin, value = {}", value);

        // Poor man's delay. High prio will preempt somewhere in here.
        for _ in 0..1_000_000 {
            unsafe { core::ptr::read_volatile(&0) };
        }

        let value = cx.resources.value.lock(|v| *v);
        rprintln!("Low prio end, value = {}", value);
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn DFSDM1();
        fn DFSDM2();
    }
};
