#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(not(feature = "std"), no_main)]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![feature(return_position_impl_trait_in_trait)]

use embassy_executor::Spawner;
use embassy_time;

#[path = "../hal/mod.rs"]
mod hal;
#[path = "../libs/mod.rs"]
mod libs;
use hal::*;
use libs::*;

#[cfg(feature = "defmt")]
use {defmt_rtt as _, panic_probe as _};

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    Board::init();
    let led = Board::led1();
    let mut led = match led {
        Some(l) => l,
        None => {
            fmt::error!("board doesn't have led1 defined");
            return;
        }
    };
    loop {
        let _ = led.set_high();
        embassy_time::Timer::after(embassy_time::Duration::from_millis(300)).await;
        let _ = led.set_low();
        embassy_time::Timer::after(embassy_time::Duration::from_millis(300)).await;
    }
}