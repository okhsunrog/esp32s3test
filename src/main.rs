use anyhow::Result;
use esp_idf_svc::hal::{gpio::AnyIOPin, prelude::*, spi::*};
use smart_leds::SmartLedsWriteAsync;
use smart_leds::{brightness, RGB8};
use ws2812_async::{Grb, Ws2812};
use embedded_hal_async::spi::SpiBus;

const NUM_LEDS: usize = 20;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let p = Peripherals::take()?;
    let spi = p.spi2;
    let led_pin = p.pins.gpio10;

    // do I need to configure it in any special way for DMA to work?
    let mut driver: SpiDriver<'_> = SpiDriver::new_without_sclk::<SPI2>(
        spi,
        led_pin,
        AnyIOPin::none(),
        &SpiDriverConfig::new(),
    )?;
    let config = config::Config::new().baudrate(3200.kHz().into());
    let mut device = SpiBusDriver::new(&mut driver, &config)?;
    let mut ws: Ws2812<_, Grb, { 12 * NUM_LEDS }> = Ws2812::new(&mut device);

    log::info!("Hello, world!");
    Ok(())
}
