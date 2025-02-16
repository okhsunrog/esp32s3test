use anyhow::Result;
use esp_idf_svc::hal::{prelude::*, spi};

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let p = Peripherals::take()?;
    let spi = p.spi2;
    let led_pin = p.pins.gpio10;

    log::info!("Hello, world!");
    Ok(())
}
