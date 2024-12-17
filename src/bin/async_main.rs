#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_println::println;
use {defmt_rtt as _, esp_backtrace as _};
use esp_hal::{
    analog::adc::{Adc, AdcPin, AdcConfig, Attenuation},
    clock::ClockControl,
    gpio::{GpioPin, Io },
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    timer::timg::TimerGroup,
};

extern crate alloc;

type AdcCal = esp_hal::analog::adc::AdcCalLine<esp_hal::peripherals::ADC1>;

#[embassy_executor::task]
async fn hall_sensor_task(
    mut adc1: Adc<'static, esp_hal::peripherals::ADC1>,
    mut adc1_pin: AdcPin<GpioPin<3>, esp_hal::peripherals::ADC1, AdcCal>,
) {
    info!("Hall sensor task started!");

    loop {
        info!("Reading ADC value...");

        let mut pin_mv = None;
        loop {
            match adc1.read_oneshot(&mut adc1_pin) {
                Ok(value) => {
                    pin_mv = Some(value);
                    break;
                }
                Err(nb::Error::WouldBlock) => {
                    // ADC is not ready, wait for a short duration to avoid busy-waiting
                    Timer::after(Duration::from_millis(10)).await;
                }
                Err(nb::Error::Other(_)) => {
                    // Handle other potential error cases
                    info!("Other error reading ADC");
                    break;
                }
            }
        }

        if let Some(pin_mv) = pin_mv {
            info!("ADC Reading: {:?}", pin_mv);
        }

        // Wait for 1 second before the next reading
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[main]
async fn main(spawner: Spawner) {
    info!("Starting program!...");

    esp_alloc::heap_allocator!(72 * 1024);

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timg0 = TimerGroup::new_async(peripherals.TIMG0, &clocks);
    esp_hal_embassy::init(&clocks, timg0);

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let hall_sensor_pin = io.pins.gpio3;

    let mut adc_config = AdcConfig::new();
    let adc1_pin = adc_config.enable_pin_with_cal::<_, AdcCal>(hall_sensor_pin, Attenuation::Attenuation11dB);
    let adc1 = Adc::new(peripherals.ADC1, adc_config);

    // Spawn the task to handle ADC readings from the Hall sensor
    spawner.spawn(hall_sensor_task(adc1, adc1_pin)).unwrap();

    loop {
        // Empty loop
    }
}
