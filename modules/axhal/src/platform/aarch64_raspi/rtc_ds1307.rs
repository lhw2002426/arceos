extern crate ds1307;

use ds1307::{DateTimeAccess, Ds1307, NaiveDate};
use embedded_hal::blocking::i2c::{Write, WriteRead,SevenBitAddress};
//use linux_embedded_hal::I2cdev;
pub struct MyI2C;

impl Write for MyI2C {
    type Error = ();

    fn write(&mut self, address: SevenBitAddress, bytes: &[u8]) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl WriteRead for MyI2C {
    type Error = ();

    fn write_read(
        &mut self,
        address: SevenBitAddress,
        bytes: &[u8],
        buffer: &mut [u8],
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
pub fn test_ds1307() {
    //let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let dev = MyI2C;
    let mut rtc = Ds1307::new(dev);
    let datetime = NaiveDate::from_ymd_opt(2022, 1, 2)
        .unwrap()
        .and_hms_opt(19, 59, 58)
        .unwrap();
    rtc.set_datetime(&datetime).unwrap();
    // ...
    let datetime = rtc.datetime().unwrap();
    //println!("{datetime}");
    // This will print something like: 2022-01-02 19:59:58
}