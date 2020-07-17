//! This is not the file you are looking for!
//!
//! Go to the `examples` folder for all the code. :)

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;
use stm32l4xx_hal as _;

#[app(device = stm32l4xx_hal::stm32)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {}
};
