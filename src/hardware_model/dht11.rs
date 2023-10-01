// https://www.mouser.com/datasheet/2/758/DHT11-Technical-Data-Sheet-Translated-Version-1143054.pdf

use crate::{
    config::hardware::temperature::TemperatureConfig,
    error::PeripheralInitError,
    traits::Tick,
};
use rppal::{
    gpio::{ IoPin, Mode, Gpio, Level },
    hal::Delay,
};
use embedded_hal::blocking::delay::{ DelayMs, DelayUs };

pub struct Dht11 {
    data_pin: IoPin,
    sample_interval: u8,
    delay: Delay,
    last_measured_temp: (u8, u8),
    last_measured_rh: (u8, u8),
    last_read_valid: bool,
    successive_failures: u8,
}

#[derive(Debug)]
enum Error {
    Handshake,
    DataTransmission,
    Parity,
}

impl Dht11 {
    pub fn new(config: &TemperatureConfig) -> Result<Self, PeripheralInitError> {
        Ok(Self {
            data_pin: Gpio::new().map_err(|e| {
                PeripheralInitError{
                    message: format!(
                        "Failed to init Gpio for pin {}: {}",
                        config.gpio.data_pin,
                        e.to_string()
                    )
                }
            })?.get(config.gpio.data_pin).map_err(|e| {
                PeripheralInitError {
                    message: format!(
                        "Failed to get gpio pin {}: {}",
                        config.gpio.data_pin,
                        e.to_string()
                    )
                }
            })?.into_io(Mode::Output),

            sample_interval: config.sample_interval,
            delay: Delay::new(),

            last_measured_temp: (0, 0),
            last_measured_rh: (0, 0),
            last_read_valid: false,
            successive_failures: 0,
        })
    }

    fn read_sensor(&mut self) -> Result<(), Error> {
        self.send_start_signal()?;
        self.read_data_transmission()?;

        Ok(())
    }

    /*
     * To signal the sensor to transmit data, a start signal
     * must be sent. The start signal is as follows:
     * * High to low, hold for at least 18us.
     * * Low to high, wait for 20 to 40us
     * * Handshake: The sensor will send a low signal for
     *     40us, followed by a high signal for 40us to 
     *     acknowledge the start signal.
     */
    fn send_start_signal(&mut self) -> Result<(), Error> {
        self.data_pin.set_mode(Mode::Output);

        self.data_pin.set_high();
        self.delay.delay_ms(1u8);
        self.data_pin.set_low();
        self.delay.delay_ms(18u8);
        self.data_pin.set_high();
        self.delay.delay_us(40u8);

        self.data_pin.set_mode(Mode::Input);
        self.get_pulse_as_bit()
            .map(|_| ())
            .map_err(|_| Error::Handshake)
    }

    /*
     * The data transmission consists of 40 bits. The first two
     * bytes are humidity, the next two bytes are for temperature,
     * and the last byte is a checksum.
     */
    fn read_data_transmission(&mut self) -> Result<(), Error> {
        let mut buffer = [0u8; 5];

        for i in 0..40 {
            let idx = i/8;
            buffer[idx] <<= 1;
            if self.get_pulse_as_bit()
                .map_err(|_| Error::DataTransmission)? {

                buffer[idx] |= 1;
            }
        }

        if buffer[0] + buffer[1] + buffer[2] + buffer[3]
            != buffer[4] {
    
            return Err(Error::Parity);
        }

        self.last_measured_temp = (buffer[0], buffer[1]);
        self.last_measured_rh = (buffer[2], buffer[3]);

        Ok(())
    }

    /*
     * All low signals during data transmission last 50us. High signals
     * last 70us for 1, and ~30us for 0. If the high signal is longer than
     * the low signal, this is a 1. Otherwise it's a 0.
     */
    fn get_pulse_as_bit(&mut self) -> Result<bool, ()> {
        let low_duration = self.await_level_transition(Level::Low, 128)?; 
        let high_duration = self.await_level_transition(Level::High, 128)?; 
        
        Ok(high_duration > low_duration)
    }

    fn await_level_transition(
        &mut self,
        start_level: Level,
        timeout: u32
    ) -> Result<u32, ()> {
        let mut us_count = 0;

        while self.data_pin.read() == start_level {
            us_count += 1;

            if us_count > timeout {
                return Err(());
            }

            self.delay.delay_us(1u8);
        }

        Ok(us_count)
    }
}

impl Tick for Dht11 {
    fn tick(&mut self, tick_count: u128) {
        // TODO: use ms to ticks fn once implemented (see issue #12)
        if tick_count % (self.sample_interval as u128 * 10) == 0 {
            match self.read_sensor() {
                Ok(_) => {
                    self.last_read_valid = true;
                    self.successive_failures = 0;
                },
                Err(e) => {
                    eprintln!("DHT11 Failure: {:#?}", e);
                    self.last_read_valid = false;
                    self.successive_failures += 1;
                },
            }
        }
    }
}
