//! Example for the app macro.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;
use stm32l4xx_hal as _;

#[app(device = stm32l4xx_hal::stm32)]
const APP: () = {
    // RTIC app is written in here!
};
