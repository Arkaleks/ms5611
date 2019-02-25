//! Raspberry Pi demo

extern crate linux_embedded_hal as hal;
extern crate ms5611;

use hal::spidev::{self, SpidevOptions};
use hal::sysfs_gpio::Direction;
use hal::{Delay, Pin, Spidev};
use ms5611::{Ms5611, Oversampling};

fn main() {
    let mut spi = Spidev::open("/dev/spidev0.0").unwrap();
    let options = SpidevOptions::new()
        .max_speed_hz(1_000_000)
        .mode(spidev::SPI_MODE_3)
        .build();
    spi.configure(&options).unwrap();

    let ncs = Pin::new(25);
    ncs.export().unwrap();
    while !ncs.is_exported() {}
    ncs.set_direction(Direction::Out).unwrap();
    ncs.set_value(1).unwrap();

    let mut ms5611 = Ms5611::new(spi, ncs, Delay).unwrap();
    let sample = ms5611
        .get_second_order_sample(Oversampling::OS_2048)
        .unwrap();
    println!("{:?}", sample);
}
