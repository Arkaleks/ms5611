# `MS5611`

> no_std driver for the MS5611 (barometric pressure sensor)

## The Device
The TE Connectivity MS5611 is a high resolution barometric pressure sensor. The device supports both SPI and I2C bus interfaces (this driver only supports SPI though).

More information and datasheet can be found at https://www.te.com/usa-en/product-CAT-BLPS0036.html#mdp-tabs-content

## Status
- [x] SPI access
- [x] Measure Pressure
- [x] Measure Temperature
- [x] Get factory coefficients
- [x] Check Coefficients CRC
- [x] Basic tests (CRC, basic converstion)
- [ ] I2C access
- [ ] CI
- [ ] Documentation

## License
Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.