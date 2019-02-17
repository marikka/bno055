
extern crate i2cdev_bno055;
extern crate i2csensors;
extern crate linux_embedded_hal as hal;

use i2cdev_bno055::*;

#[cfg(any(target_os = "linux"))]
use hal::{I2cdev};

#[cfg(not(any(target_os = "linux")))]
fn main() {}

#[cfg(any(target_os = "linux"))]
fn main() {
    match I2cdev::new("/dev/i2c-1") {
        Ok(device) => {
            let mut bno = BNO055::new(device, BNO055_DEFAULT_ADDR).unwrap();
            println!("{:?}", bno.get_revision().unwrap());
            bno.set_mode(BNO055OperationMode::Ndof).unwrap();
            loop {
                let accel = bno.get_linear_acceleration().unwrap();
                println!("{:+2.2}\t{:+2.2}\t{:+2.2}", accel.x, accel.y, accel.z);
            }
        }
        Err(e) => {}
    }
}
