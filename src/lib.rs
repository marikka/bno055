extern crate byteorder;
extern crate embedded_hal as hal;
extern crate i2csensors;

use byteorder::{ByteOrder, LittleEndian};
use hal::blocking::i2c::{Write, Read, WriteRead};
use i2csensors::{Accelerometer, Gyroscope, Thermometer, Magnetometer, Vec3};

use std::thread;
use std::time::Duration;
use std::mem;

pub const BNO055_DEFAULT_ADDR: u8 = 0x28;
pub const BNO055_ALTERNATE_ADDR: u8 = 0x29;
pub const BNO055_ID: u8 = 0xA0;

pub const BNO055_PAGE_ID: u8 = 0x07;

pub const BNO055_CHIP_ID: u8 = 0x00;
pub const BNO055_ACC_ID: u8 = 0x01;
pub const BNO055_MAG_ID: u8 = 0x02;
pub const BNO055_GYR_ID: u8 = 0x03;
pub const BNO055_SW_REV_ID_LSB: u8 = 0x04;
pub const BNO055_SW_REV_ID_MSB: u8 = 0x05;
pub const BNO055_BL_REV_ID: u8 = 0x06;

pub const BNO055_ACC_DATA_X_LSB: u8 = 0x08;
pub const BNO055_ACC_DATA_X_MSB: u8 = 0x09;
pub const BNO055_ACC_DATA_Y_LSB: u8 = 0x0A;
pub const BNO055_ACC_DATA_Y_MSB: u8 = 0x0B;
pub const BNO055_ACC_DATA_Z_LSB: u8 = 0x0C;
pub const BNO055_ACC_DATA_Z_MSB: u8 = 0x0D;

pub const BNO055_MAG_DATA_X_LSB: u8 = 0x0E;
pub const BNO055_MAG_DATA_X_MSB: u8 = 0x0F;
pub const BNO055_MAG_DATA_Y_LSB: u8 = 0x10;
pub const BNO055_MAG_DATA_Y_MSB: u8 = 0x11;
pub const BNO055_MAG_DATA_Z_LSB: u8 = 0x12;
pub const BNO055_MAG_DATA_Z_MSB: u8 = 0x13;

pub const BNO055_GYR_DATA_X_LSB: u8 = 0x14;
pub const BNO055_GYR_DATA_X_MSB: u8 = 0x15;
pub const BNO055_GYR_DATA_Y_LSB: u8 = 0x16;
pub const BNO055_GYR_DATA_Y_MSB: u8 = 0x17;
pub const BNO055_GYR_DATA_Z_LSB: u8 = 0x18;
pub const BNO055_GYR_DATA_Z_MSB: u8 = 0x19;

pub const BNO055_EUL_HEADING_LSB: u8 = 0x1A;
pub const BNO055_EUL_HEADING_MSB: u8 = 0x1B;
pub const BNO055_EUL_ROLL_LSB: u8 = 0x1C;
pub const BNO055_EUL_ROLL_MSB: u8 = 0x1D;
pub const BNO055_EUL_PITCH_LSB: u8 = 0x1E;
pub const BNO055_EUL_PITCH_MSB: u8 = 0x1F;

/// Quaternion data
pub const BNO055_QUA_DATA_W_LSB: u8 = 0x20;
pub const BNO055_QUA_DATA_W_MSB: u8 = 0x21;
pub const BNO055_QUA_DATA_X_LSB: u8 = 0x22;
pub const BNO055_QUA_DATA_X_MSB: u8 = 0x23;
pub const BNO055_QUA_DATA_Y_LSB: u8 = 0x24;
pub const BNO055_QUA_DATA_Y_MSB: u8 = 0x25;
pub const BNO055_QUA_DATA_Z_LSB: u8 = 0x26;
pub const BNO055_QUA_DATA_Z_MSB: u8 = 0x27;

/// Linear acceleration data
pub const BNO055_LIA_DATA_X_LSB: u8 = 0x28;
pub const BNO055_LIA_DATA_X_MSB: u8 = 0x29;
pub const BNO055_LIA_DATA_Y_LSB: u8 = 0x2A;
pub const BNO055_LIA_DATA_Y_MSB: u8 = 0x2B;
pub const BNO055_LIA_DATA_Z_LSB: u8 = 0x2C;
pub const BNO055_LIA_DATA_Z_MSB: u8 = 0x2D;

/// Gravity vector data
pub const BNO055_GRV_DATA_X_LSB: u8 = 0x2E;
pub const BNO055_GRV_DATA_X_MSB: u8 = 0x2F;
pub const BNO055_GRV_DATA_Y_LSB: u8 = 0x30;
pub const BNO055_GRV_DATA_Y_MSB: u8 = 0x31;
pub const BNO055_GRV_DATA_Z_LSB: u8 = 0x32;
pub const BNO055_GRV_DATA_Z_MSB: u8 = 0x33;

/// Temperature data
pub const BNO055_TEMP: u8 = 0x34;

/// Calibration Status
/// TODO
pub const BNO055_CALIB_STAT: u8 = 0x35;

pub const BNO055_ST_RESULT: u8 = 0x36;
pub const BNO055_INT_STA: u8 = 0x37;
pub const BNO055_SYS_CLK_STATUS: u8 = 0x38;
pub const BNO055_SYS_STATUS: u8 = 0x39;
pub const BNO055_SYS_ERR: u8 = 0x3A;
pub const BNO055_UNIT_SEL: u8 = 0x3B;
pub const BNO055_OPR_MODE: u8 = 0x3D;
pub const BNO055_PWR_MODE: u8 = 0x3E;
pub const BNO055_SYS_TRIGGER: u8 = 0x3F;
pub const BNO055_TEMP_SOURCE: u8 = 0x40;
pub const BNO055_AXIS_MAP_CONFIG: u8 = 0x41;
pub const BNO055_AXIS_MAP_SIGN: u8 = 0x42;

pub const BNO055_ACC_OFFSET_X_LSB: u8 = 0x55;
pub const BNO055_ACC_OFFSET_X_MSB: u8 = 0x56;
pub const BNO055_ACC_OFFSET_Y_LSB: u8 = 0x57;
pub const BNO055_ACC_OFFSET_Y_MSB: u8 = 0x58;
pub const BNO055_ACC_OFFSET_Z_LSB: u8 = 0x59;
pub const BNO055_ACC_OFFSET_Z_MSB: u8 = 0x5A;

pub const BNO055_MAG_OFFSET_X_LSB: u8 = 0x5B;
pub const BNO055_MAG_OFFSET_X_MSB: u8 = 0x5C;
pub const BNO055_MAG_OFFSET_Y_LSB: u8 = 0x5D;
pub const BNO055_MAG_OFFSET_Y_MSB: u8 = 0x5E;
pub const BNO055_MAG_OFFSET_Z_LSB: u8 = 0x5F;
pub const BNO055_MAG_OFFSET_Z_MSB: u8 = 0x60;

pub const BNO055_GYR_OFFSET_X_LSB: u8 = 0x61;
pub const BNO055_GYR_OFFSET_X_MSB: u8 = 0x62;
pub const BNO055_GYR_OFFSET_Y_LSB: u8 = 0x63;
pub const BNO055_GYR_OFFSET_Y_MSB: u8 = 0x64;
pub const BNO055_GYR_OFFSET_Z_LSB: u8 = 0x65;
pub const BNO055_GYR_OFFSET_Z_MSB: u8 = 0x66;

pub const BNO055_ACC_RADIUS_LSB: u8 = 0x67;
pub const BNO055_ACC_RADIUS_MSB: u8 = 0x68;
pub const BNO055_MAG_RADIUS_LSB: u8 = 0x69;
pub const BNO055_MAG_RADIUS_MSB: u8 = 0x6A;

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BNO055SystemStatusCode {
    SystemIdle = 0,
    SystemError = 1,
    InitPeripherals = 2,
    SystemInit = 3,
    Executing = 4,
    Running = 5,
    RunningWithoutFusion = 6,
}

#[derive(Debug, Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BNO055SystemErrorCode {
    None = 0,
    PeripheralInit = 1,
    SystemInit = 2,
    SelfTest = 3,
    RegisterMapValue = 4,
    RegisterMapAddress = 5,
    RegisterMapWrite = 6,
    LowPowerModeNotAvail = 7,
    AccelPowerModeNotAvail = 8,
    FusionAlgoConfig = 9,
    SensorConfig = 10,
}

#[derive(Debug)]
pub struct BNO055SystemStatus {
    status: BNO055SystemStatusCode,
    // TODO: bitflagify this
    selftest: Option<u8>,
    error: BNO055SystemErrorCode,
}

#[derive(Debug)]
pub struct BNO055Revision {
    pub software: u16,
    pub bootloader: u8,
    pub accelerometer: u8,
    pub magnetometer: u8,
    pub gyroscope: u8,
}

#[derive(Debug)]
pub struct BNO055CalibrationStatus {
    pub sys: bool,
    pub gyr: bool,
    pub acc: bool,
    pub mag: bool,
}

#[derive(Debug)]
pub struct BNO055QuaternionReading {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BNO055RegisterPage {
    Page0 = 0,
    Page1 = 1,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BNO055PowerMode {
    Normal = 0b00,
    LowPower = 0b01,
    Suspend = 0b10,
}

#[derive(Copy, Clone, PartialEq)]
#[repr(u8)]
pub enum BNO055OperationMode {
    ConfigMode = 0b0000,
    AccOnly = 0b0001,
    MagOnly = 0b0010,
    GyroOnly = 0b0011,
    AccMag = 0b0100,
    AccGyro = 0b0101,
    MagGyro = 0b0110,
    AMG = 0b0111,
    IMU = 0b1000,
    Compass = 0b1001,
    M4G = 0b1010,
    NdofFmcOff = 0b1011,
    Ndof = 0b1100,
}

#[derive(Copy, Clone)]
pub struct BNO055<I2C> {
    pub i2cdev: I2C,
    pub mode: BNO055OperationMode,
    device_address: u8
}

impl<I2C, E> BNO055<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
{
    pub fn new(i2cdev: I2C, device_address: u8) -> Result<Self, E> {
        let mut bno = BNO055 {
            i2cdev: i2cdev,
            device_address,
            mode: BNO055OperationMode::ConfigMode,
        };
        let chip_id = bno.read_byte_data(BNO055_CHIP_ID)?;

        if chip_id != BNO055_ID {
            // TODO: Do correct error handling
            panic!("BNO055_CHIP_ID was not valid!");
        }
        bno.set_mode(BNO055OperationMode::ConfigMode)?;
        bno.set_page(BNO055RegisterPage::Page0)?;
        bno.set_power_mode(BNO055PowerMode::Normal)?;
        bno.write_byte_data(BNO055_SYS_TRIGGER, 0x0)?;

        Ok(bno)
    }

    fn write_byte_data(&mut self, register: u8, value: u8) -> Result<(), E> {
        self.i2cdev.write(self.device_address, &[register, value])?;
        Ok(())
    }

    fn write_block_data(&mut self, register: u8, values: &[u8]) -> Result<(), E> {
        self.i2cdev.write(self.device_address, &[&[register], &values[..]].concat())?;
        Ok(())
    }

    fn read_byte_data(&mut self, register: u8) -> Result<u8, E> {
        let mut buf = [0;1];
        self.i2cdev.write_read(self.device_address, &[register], &mut buf)?;
        Ok(buf[0])
    }

    fn read_block_data(&mut self, register: u8, len: usize) -> Result<Vec<u8>, E> {
        let mut buf = vec![0; len];
        self.i2cdev.write_read(self.device_address, &[register], &mut buf)?;
        Ok(buf.to_vec())
    }

    /// Reset the BNO055, initializing the register map to default values
    /// More in section 3.2
    pub fn reset(&mut self) -> Result<(), E> {
        Ok(self.write_byte_data(BNO055_SYS_TRIGGER, 0x20)?)
    }

    /// Sets the operating mode, see [BNO055OperationMode](enum.BNO055OperationMode.html)
    /// More in section 3.3
    pub fn set_mode(&mut self, mode: BNO055OperationMode) -> Result<(), E> {
        if self.mode != mode {
            self.write_byte_data(
                BNO055_OPR_MODE,
                mode as u8,
            )?;

            // Table 3-6 says 19ms to switch to CONFIG_MODE
            thread::sleep(Duration::from_millis(19));
        }
        Ok(())
    }

    pub fn set_external_crystal(&mut self, ext: bool) -> Result<(), E> {
        let prev = self.mode;
        self.set_mode(BNO055OperationMode::ConfigMode)?;
        self.write_byte_data(
            BNO055_SYS_TRIGGER,
            if ext { 0x80 } else { 0x00 },
        )?;
        self.set_mode(prev)?;
        Ok(())
    }

    /// Sets the power mode, see [BNO055PowerMode](enum.BNO055PowerMode.html)
    /// More in section 3.2
    pub fn set_power_mode(&mut self, mode: BNO055PowerMode) -> Result<(), E> {
        self.write_byte_data(
            BNO055_PWR_MODE,
            mode as u8,
        )?;
        Ok(())
    }

    /// Sets the register page
    /// More in section 4.2
    pub fn set_page(&mut self, page: BNO055RegisterPage) -> Result<(), E> {
        self.write_byte_data(
            BNO055_PAGE_ID,
            page as u8,
        )?;
        Ok(())
    }

    /// Gets a quaternion reading from the BNO055
    /// Must be in a valid operating mode
    pub fn get_quaternion(&mut self) -> Result<BNO055QuaternionReading, E> {
        let buf = self.read_block_data(
            BNO055_QUA_DATA_W_LSB,
            8,
        )?;
        let w = LittleEndian::read_i16(&buf[0..2]);
        let x = LittleEndian::read_i16(&buf[2..4]);
        let y = LittleEndian::read_i16(&buf[4..6]);
        let z = LittleEndian::read_i16(&buf[6..8]);

        let scale = 1.0 / ((1 << 14) as f32);
        Ok(BNO055QuaternionReading {
            w: w as f32 * scale,
            x: x as f32 * scale,
            y: y as f32 * scale,
            z: z as f32 * scale,
        })
    }

    /// Gets the revision of software, bootloader, accelerometer, magnetometer, and gyroscope of
    /// the BNO055
    pub fn get_revision(&mut self) -> Result<BNO055Revision, E> {
        // TODO: Check page
        let buf = self.read_block_data(BNO055_ACC_ID, 6)?;
        Ok(BNO055Revision {
            software: LittleEndian::read_u16(&buf[3..5]),
            bootloader: buf[5],
            accelerometer: buf[0],
            magnetometer: buf[1],
            gyroscope: buf[2],
        })
    }

    /// Get the system status
    pub fn get_system_status(&mut self, run: bool) -> Result<BNO055SystemStatus, E> {
        let selftest = if run {
            let prev = self.mode;
            self.set_mode(BNO055OperationMode::ConfigMode)?;

            let sys_trigger = self.read_byte_data(BNO055_SYS_TRIGGER)?;
            self.write_byte_data(
                BNO055_SYS_TRIGGER,
                sys_trigger | 0x1,
            )?;

            thread::sleep(Duration::from_secs(1));

            let result = self.read_byte_data(BNO055_ST_RESULT)?;
            self.set_mode(prev)?;
            Some(result)
        } else {
            None
        };

        Ok(BNO055SystemStatus {
            status: unsafe { mem::transmute(self.read_byte_data(BNO055_SYS_STATUS)?) },
            error: unsafe { mem::transmute(self.read_byte_data(BNO055_SYS_ERR)?) },
            selftest,
        })
    }

    /// Get the calibration status
    pub fn get_calibration_status(&mut self) -> Result<BNO055CalibrationStatus, E> {
        let status = self.read_byte_data(BNO055_CALIB_STAT)?;
        let sys = (status & 0b11000000) >> 6 == 0b11;
        let gyr = (status & 0b00110000) >> 6 == 0b11;
        let acc = (status & 0b00001100) >> 6 == 0b11;
        let mag = (status & 0b00000011) >> 6 == 0b11;

        Ok(BNO055CalibrationStatus { sys, gyr, acc, mag })
    }

    // TODO: Make this calibration a struct
    /// Get the calibration details. Can be used with [set_calibration](fn.set_calibration.html) to
    /// load previous configs.
    pub fn get_calibration(&mut self) -> Result<Vec<u8>, E> {
        let prev = self.mode;
        let buf = self.read_block_data(
            BNO055_ACC_OFFSET_X_LSB,
            22,
        );
        self.set_mode(prev)?;
        return buf;
    }

    // TODO: Use a calibration struct, check for buf length
    /// Set the calibration details. Can be used with [get_calibration](fn.get_calibration.html) to
    /// load previous configs.
    pub fn set_calibration(&mut self, buf: Vec<u8>) -> Result<(), E> {
        let prev = self.mode;
        self.write_block_data(
            BNO055_ACC_OFFSET_X_LSB,
            &buf,
        )?;
        self.set_mode(prev)?;
        Ok(())
    }

    // TODO: Axis remap

    /// Get euler angle representation of orientation.
    /// The `x` component is the heading, `y` is the roll, `z` is pitch, all in radians
    pub fn get_euler(&mut self) -> Result<Vec3, E> {
        let buf = self.read_block_data(
            BNO055_EUL_HEADING_LSB,
            6,
        )?;
        let x = LittleEndian::read_i16(&buf[0..2]) as f32;
        let y = LittleEndian::read_i16(&buf[2..4]) as f32;
        let z = LittleEndian::read_i16(&buf[4..6]) as f32;

        let scale = 1.0 / 900.0;
        Ok(Vec3 {
            x: x * scale,
            y: y * scale,
            z: z * scale,
        })
    }

    pub fn get_linear_acceleration(&mut self) -> Result<Vec3, E> {
        let buf = self.read_block_data(
            BNO055_LIA_DATA_X_LSB,
            6,
        )?;
        let x = LittleEndian::read_i16(&buf[0..2]) as f32;
        let y = LittleEndian::read_i16(&buf[2..4]) as f32;
        let z = LittleEndian::read_i16(&buf[4..6]) as f32;

        let scale = 1.0 / 100.0;
        Ok(Vec3 {
            x: x * scale,
            y: y * scale,
            z: z * scale,
        })
    }

    // TODO: linear acceleration, gravity
}

impl<I2C, E> Magnetometer for BNO055<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    E: std::error::Error
{
    type Error = E;

    fn magnetic_reading(&mut self) -> Result<Vec3, Self::Error> {
        let buf = self.read_block_data(
            BNO055_MAG_DATA_X_LSB,
            6,
        )?;
        let x = LittleEndian::read_i16(&buf[0..2]) as f32;
        let y = LittleEndian::read_i16(&buf[2..4]) as f32;
        let z = LittleEndian::read_i16(&buf[4..6]) as f32;

        let scale = 1.0 / 16.0;
        Ok(Vec3 {
            x: x * scale,
            y: y * scale,
            z: z * scale,
        })
    }
}

impl<I2C, E> Gyroscope for BNO055<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    E: std::error::Error
{
    type Error = E;

    fn angular_rate_reading(&mut self) -> Result<Vec3, Self::Error> {
        let buf = self.read_block_data(
            BNO055_GYR_DATA_X_LSB,
            6,
        )?;
        let x = LittleEndian::read_i16(&buf[0..2]) as f32;
        let y = LittleEndian::read_i16(&buf[2..4]) as f32;
        let z = LittleEndian::read_i16(&buf[4..6]) as f32;

        let scale = 1.0 / 900.0;
        Ok(Vec3 {
            x: x * scale,
            y: y * scale,
            z: z * scale,
        })
    }
}

impl<I2C, E> Accelerometer for BNO055<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    E: std::error::Error
{
    type Error = E;

    fn acceleration_reading(&mut self) -> Result<Vec3, Self::Error> {
        let buf = self.read_block_data(
            BNO055_ACC_DATA_X_LSB,
            6,
        )?;
        let x = LittleEndian::read_i16(&buf[0..2]) as f32;
        let y = LittleEndian::read_i16(&buf[2..4]) as f32;
        let z = LittleEndian::read_i16(&buf[4..6]) as f32;

        let scale = 1.0 / 100.0;
        Ok(Vec3 {
            x: x * scale,
            y: y * scale,
            z: z * scale,
        })
    }
}

impl<I2C, E> Thermometer for BNO055<I2C>
where
    I2C: Read<Error = E> + Write<Error = E> + WriteRead<Error = E>,
    E: std::error::Error
{
    type Error = E;

    fn temperature_celsius(&mut self) -> Result<f32, Self::Error> {
        Ok(self.read_byte_data(BNO055_TEMP)? as u8 as f32)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
