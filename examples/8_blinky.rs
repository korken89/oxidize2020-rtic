//! Example for the app macro.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::{app, cyccnt::U32Ext};
use rtt_target::{rprintln, rtt_init_print};
use stm32l4xx_hal::{
    gpio::{gpiob::PB13, Output, PushPull},
    prelude::*,
};

#[app(device = stm32l4xx_hal::stm32, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: PB13<Output<PushPull>>,
    }

    #[init(spawn = [blinky])]
    fn init(cx: init::Context) -> init::LateResources {
        // When using schedule and a monotonic timer, remember to start the timer!

        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic timer (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Device access (Peripheral Access Crate)
        let pac = cx.device;

        // Enable logging
        rtt_init_print!();
        rprintln!("Hello from init!");

        let mut rcc = pac.RCC.constrain();
        // let mut gpioa = pac.GPIOA.split(&mut rcc.ahb2);
        let mut gpiob = pac.GPIOB.split(&mut rcc.ahb2);

        let mut led = gpiob
            .pb13
            .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

        led.set_low().ok();

        // Start the blinky task!
        cx.spawn.blinky().ok();

        init::LateResources {
            // Move the LED to the resources.
            led,
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        rprintln!("Hello from idle!");

        loop {
            continue;
        }
    }

    #[task(schedule = [blinky], resources = [led])]
    fn blinky(cx: blinky::Context) {
        // RTIC's safe static muts!
        static mut FLAG: bool = false;

        // Extract the LED
        let led = cx.resources.led;

        if *FLAG == false {
            led.set_low().ok();
            rprintln!("LED Off");
        } else {
            led.set_high().ok();
            rprintln!("LED On");
        }

        cx.schedule.blinky(cx.scheduled + 2_000_000.cycles()).ok();

        *FLAG = !*FLAG;
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn DFSDM1();
        fn DFSDM2();
    }
};
