#![no_std]
#![no_main]

use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::prelude::*;
use esp_hal::gpio::{Output, Level, GpioPin};
use esp_hal::analog::adc::{Adc, AdcPin, AdcConfig, Attenuation};
use esp_println::println;
use {defmt_rtt as _, esp_backtrace as _};

extern crate alloc;

type AdcCal = esp_hal::analog::adc::AdcCalLine<esp_hal::peripherals::ADC1>;

#[embassy_executor::task]
async fn hall_sensor_task(
    mut adc1: Adc<'static, esp_hal::peripherals::ADC1>,
    mut adc1_pin: AdcPin<GpioPin<3>, esp_hal::peripherals::ADC1, AdcCal>,
) {
    loop {
        // Non-blocking read of ADC value
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
                Err(e) => {
                    // Handle other errors if necessary
                    println!("ADC read error: {:?}", e);
                    break;
                }
            }
        }

        if let Some(pin_mv) = pin_mv {
            // Print reading
            println!("Reading: {:?}", pin_mv);
        }

        // Wait for 1 second before the next reading
        Timer::after(Duration::from_secs(1)).await;
    }
}


#[main]
async fn main(spawner: Spawner) {
    let peripherals = esp_hal::init({
        let mut config = esp_hal::Config::default();
        config.cpu_clock = CpuClock::max();
        config
    });

    esp_alloc::heap_allocator!(72 * 1024);

    let timer0 = esp_hal::timer::systimer::SystemTimer::new(peripherals.SYSTIMER)
        .split::<esp_hal::timer::systimer::Target>();
    esp_hal_embassy::init(timer0.alarm0);

    info!("Embassy initialized!");

    let timer1 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let _init = esp_wifi::init(
        timer1.timer0,
        esp_hal::rng::Rng::new(peripherals.RNG),
        peripherals.RADIO_CLK,
    )
    .unwrap();



    let mut led_1 = Output::new(peripherals.GPIO23, Level::Low);
    let mut led_2 = Output::new(peripherals.GPIO22, Level::Low);
    let mut led_3 = Output::new(peripherals.GPIO21, Level::Low);
    let mut led_4 = Output::new(peripherals.GPIO20, Level::Low);
    let mut led_5 = Output::new(peripherals.GPIO19, Level::Low);
    let mut led_6 = Output::new(peripherals.GPIO18, Level::Low);
    let mut led_7 = Output::new(peripherals.GPIO15, Level::Low);
    let mut led_8 = Output::new(peripherals.GPIO14, Level::Low);
    let mut led_9 = Output::new(peripherals.GPIO1, Level::Low);
    let mut led_10 = Output::new(peripherals.GPIO0, Level::Low);

    // Set up ADC (using GPIO1 as example, modify as needed)
    let analog_pin = io.pins.gpio1;
    let mut adc1_config = AdcConfig::new();
    let adc1_pin = adc1_config.enable_pin_with_cal::<_, AdcCal>(analog_pin, Attenuation::Attenuation11dB);
    let adc1 = Adc::new(peripherals.ADC1, adc1_config);
    
    // TODO: Spawn some tasks
    let _ = spawner;

    loop {
        
        led_1.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_1.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_2.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_2.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_3.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_3.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_4.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_4.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_5.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_5.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_6.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_6.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_7.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_7.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_8.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_8.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_9.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_9.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms

        led_10.set_high(); // Turn the LED on
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms
        
        led_10.set_low(); // Turn the LED off
        Timer::after(Duration::from_millis(500)).await; // Wait for 500ms
    }

}

