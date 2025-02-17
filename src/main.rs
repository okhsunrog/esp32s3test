use anyhow::Result;
use edge_executor::LocalExecutor;
use embassy_time::{Duration, Timer};
use esp_idf_svc::hal::{gpio::AnyIOPin, prelude::*, spi::*, task::block_on};
// use smart_led_effects::{
//     strip::{self, EffectIterator},
//     Srgb,
// };
use smart_leds::SmartLedsWriteAsync;
use smart_leds::{brightness, RGB8};
use ws2812_async::{Grb, Ws2812};

const NUM_LEDS: usize = 20;

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();
    let local_ex: LocalExecutor = Default::default();
    let p = Peripherals::take()?;
    let spi = p.spi2;
    let led_pin = p.pins.gpio21;

    // do I need to configure it in any special way for DMA to work?
    let driver: SpiDriver<'_> = SpiDriver::new_without_sclk::<SPI2>(
        spi,
        led_pin,
        AnyIOPin::none(),
        &SpiDriverConfig::new(),
    )?;
    let config = config::Config::new().baudrate(3200.kHz().into());
    let device = SpiBusDriver::new(driver, &config)?;
    let mut ws: Ws2812<_, Grb, { 12 * NUM_LEDS }> = Ws2812::new(device);

    let task = local_ex.spawn(async move {
        let mut data = [RGB8::default(); NUM_LEDS];
        loop {
            for j in 0..(256 * 5) {
                for (i, pixel) in data.iter_mut().enumerate() {
                    *pixel = wheel((((i * 256) as u16 / NUM_LEDS as u16 + j as u16) & 255) as u8);
                }
                ws.write(brightness(data.iter().cloned(), 32)).await.ok();
                Timer::after(Duration::from_millis(10)).await;
            }
        }
    });

    log::info!("Hello, world!");
    block_on(local_ex.run(task));
    Ok(())
}

fn wheel(mut wheel_pos: u8) -> RGB8 {
    wheel_pos = 255 - wheel_pos;
    if wheel_pos < 85 {
        return (255 - wheel_pos * 3, 0, wheel_pos * 3).into();
    }
    if wheel_pos < 170 {
        wheel_pos -= 85;
        return (0, wheel_pos * 3, 255 - wheel_pos * 3).into();
    }
    wheel_pos -= 170;
    (wheel_pos * 3, 255 - wheel_pos * 3, 0).into()
}
