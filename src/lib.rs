//! This is a platform-agnostic Rust driver for the Texas Instruments BQ40Z50 Battery
//! fuel/gas gauge based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://docs.rs/embedded-hal
//!
//! For further details of the device architecture and operation, please refer
//! to the official [`Datasheet`].
//!
//! [`Datasheet R1`]: https://www.ti.com/lit/ug/sluua43a/sluua43a.pdf
//! [`Datasheet R3`]: https://www.ti.com/lit/ug/sluubu5a/sluubu5a.pdf
//! [`Datasheet R4`]: https://www.ti.com/lit/ug/sluuch2/sluuch2.pdf
//! [`Datasheet R5`]: https://www.ti.com/lit/ug/sluucn4b/sluucn4b.pdf

#![doc = include_str!("../README.md")]
#![cfg_attr(not(test), no_std)]
#![allow(missing_docs)]

use core::cell::Cell;

use embedded_batteries_async::smart_battery::{
    self, BatteryModeFields, BatteryStatusFields, CapacityModeSignedValue, CapacityModeValue, DeciKelvin, ErrorCode,
    SpecificationInfoFields,
};
use embedded_hal_async::i2c::I2c as I2cTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
/// BQ40Z50 Errors
pub enum BQ40Z50Error<I2cError> {
    I2c(I2cError),
    BatteryStatus(ErrorCode),
}

// Gated as future revisions of this chip may have larger register sizes.
#[cfg(any(feature = "r1", feature = "r3", feature = "r4", feature = "r5"))]
const LARGEST_REG_SIZE_BYTES: usize = 5;
#[cfg(any(feature = "r1", feature = "r3", feature = "r4", feature = "r5"))]
const LARGEST_CMD_SIZE_BYTES: usize = 32;
#[cfg(any(feature = "r1", feature = "r3", feature = "r4", feature = "r5"))]
const LARGEST_BUF_SIZE_BYTES: usize = 33;

const BQ_ADDR: u8 = 0x0Bu8;
const MAC_CMD_ADDR_SIZE_BYTES: u8 = 2;
const MAC_CMD_ADDR_SIZE_BITS: u8 = MAC_CMD_ADDR_SIZE_BYTES * 8;
const MAC_CMD: u8 = 0x44u8;

// Special case MAC commands
const SECURITY_KEYS_CMD: [u8; MAC_CMD_ADDR_SIZE_BYTES as usize] = 0x0035u16.to_le_bytes();
const SECURITY_KEYS_DATA_LEN_BYTES: u8 = 8;
const SECURITY_KEYS_LEN_BYTES: u8 = SECURITY_KEYS_DATA_LEN_BYTES + MAC_CMD_ADDR_SIZE_BYTES;

const AUTH_KEY_CMD: [u8; MAC_CMD_ADDR_SIZE_BYTES as usize] = 0x0037u16.to_le_bytes();
const AUTH_KEY_DATA_LEN_BYTES: u8 = 16;
const AUTH_KEY_LEN_BYTES: u8 = AUTH_KEY_DATA_LEN_BYTES + MAC_CMD_ADDR_SIZE_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum CapacityModeState {
    Milliamps = 0,
    Centiwatt = 1,
}

impl From<field_sets::BatteryStatus> for BatteryStatusFields {
    fn from(value: field_sets::BatteryStatus) -> Self {
        BatteryStatusFields::new()
            .with_error_code(value.ec())
            .with_fully_discharged(value.fd())
            .with_fully_charged(value.fc())
            .with_discharging(value.dsg())
            .with_initialized(value.init())
            .with_remaining_time_alarm(value.rta())
            .with_remaining_capacity_alarm(value.rca())
            .with_terminate_discharge_alarm(value.tda())
            .with_over_temp_alarm(value.ota())
            .with_terminate_charge_alarm(value.tca())
            .with_over_charged_alarm(value.oca())
    }
}

impl From<field_sets::BatteryMode> for BatteryModeFields {
    fn from(value: field_sets::BatteryMode) -> Self {
        let battery_mode_raw: [u8; 2] = value.into();
        u16::from_le_bytes(battery_mode_raw).into()
    }
}

impl From<BatteryModeFields> for field_sets::BatteryMode {
    fn from(value: BatteryModeFields) -> Self {
        u16::from(value).to_le_bytes().into()
    }
}

impl From<field_sets::SpecificationInfo> for SpecificationInfoFields {
    fn from(value: field_sets::SpecificationInfo) -> Self {
        let spec_info_raw: [u8; 2] = value.into();
        u16::from_le_bytes(spec_info_raw).into()
    }
}

/// BQ40Z50 interface, which takes an async I2C bus
pub struct DeviceInterface<I2C: I2cTrait> {
    /// embedded-hal-async compliant I2C bus
    pub i2c: I2C,
}

#[cfg(feature = "r1")]
device_driver::create_device!(
    device_name: Device,
    manifest: "device_R1.yaml"
);

#[cfg(feature = "r3")]
device_driver::create_device!(
    device_name: Device,
    manifest: "device_R3.yaml"
);

#[cfg(feature = "r4")]
device_driver::create_device!(
    device_name: Device,
    manifest: "device_R4.yaml"
);

#[cfg(feature = "r5")]
device_driver::create_device!(
    device_name: Device,
    manifest: "device_R5.yaml"
);

impl<I2C: I2cTrait> device_driver::AsyncRegisterInterface for DeviceInterface<I2C> {
    type Error = BQ40Z50Error<I2C::Error>;
    type AddressType = u8;

    async fn write_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &[u8],
    ) -> Result<(), Self::Error> {
        debug_assert!((data.len() <= LARGEST_REG_SIZE_BYTES), "Register size too big");

        // Add one byte for register address
        let mut buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES];
        buf[0] = address;
        buf[1..=data.len()].copy_from_slice(data);

        // Because the BQ40Z50's registers vary in size, we pass in a slice of
        // the appropriate size so we do not accidentally write to the register
        // at address + 1 when writing to a 1 byte register
        self.i2c
            .write(BQ_ADDR, &buf[..=data.len()])
            .await
            .map_err(BQ40Z50Error::I2c)
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        _size_bits: u32,
        data: &mut [u8],
    ) -> Result<(), Self::Error> {
        self.i2c
            .write_read(BQ_ADDR, &[address], data)
            .await
            .map_err(BQ40Z50Error::I2c)
    }
}

impl<I2C: I2cTrait> device_driver::AsyncCommandInterface for DeviceInterface<I2C> {
    type Error = BQ40Z50Error<I2C::Error>;
    type AddressType = u32;

    async fn dispatch_command(
        &mut self,
        address: Self::AddressType,
        size_bits_in: u32,
        _input: &[u8],
        size_bits_out: u32,
        output: &mut [u8],
    ) -> Result<(), Self::Error> {
        // For this driver, dispatch_command() is used for interfacing with MAC registers.
        // There are 3 possible scenarios, read only, write only, or read/write registers.
        // Read commands have an output size but no input size.
        // Write commands do not have an input size nor output size since they are pure commands.
        // Read/write commands, like Security Keys and Authentication Key are special cases
        // and are handled on a per-register basis not within this function.

        // Block write first to send a command (write only commands) or to read command data from the fuel gauge
        let mut buf = [0u8; 2 + MAC_CMD_ADDR_SIZE_BYTES as usize];
        buf[0] = ((address >> MAC_CMD_ADDR_SIZE_BITS) & 0xFF) as u8;
        buf[1] = MAC_CMD_ADDR_SIZE_BYTES;
        buf[2] = ((address >> 8) & 0xFF) as u8;
        buf[3] = (address & 0xFF) as u8;

        // Block write intended register.
        self.i2c.write(BQ_ADDR, &buf).await.map_err(BQ40Z50Error::I2c)?;

        if size_bits_in == 0 && size_bits_out > 0 {
            // For read only commands.
            // Block read using I2C write_read, sending 0x44 as the command.
            // Response looks like [ Length (1 byte) | Command (2 bytes) | Data (output.len() bytes)]
            let mut output_buf = [0u8; 1 + LARGEST_CMD_SIZE_BYTES + MAC_CMD_ADDR_SIZE_BYTES as usize];

            self.i2c
                .write_read(
                    BQ_ADDR,
                    &[buf[0]],
                    &mut output_buf[..1 + MAC_CMD_ADDR_SIZE_BYTES as usize + output.len()],
                )
                .await
                .map_err(BQ40Z50Error::I2c)?;

            output.copy_from_slice(
                &output_buf
                    [(1 + MAC_CMD_ADDR_SIZE_BYTES as usize)..(1 + MAC_CMD_ADDR_SIZE_BYTES as usize + output.len())],
            );

            Ok(())
        } else if size_bits_in == 0 && size_bits_out == 0 {
            // Write only, writes don't have an output size nor an input size because
            // writes only consist of the register/command address. So we are done.
            Ok(())
        } else {
            // Read/write, to be handled in other functions as special cases.
            unreachable!();
        }
    }
}

impl<I2C: I2cTrait> device_driver::BufferInterfaceError for DeviceInterface<I2C> {
    type Error = BQ40Z50Error<I2C::Error>;
}

impl<I2C: I2cTrait> device_driver::AsyncBufferInterface for DeviceInterface<I2C> {
    type AddressType = u8;

    async fn read(&mut self, address: Self::AddressType, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.i2c
            .write_read(BQ_ADDR, &[address], buf)
            .await
            .map_err(BQ40Z50Error::I2c)?;
        Ok(buf.len())
    }

    async fn write(&mut self, address: Self::AddressType, buf: &[u8]) -> Result<usize, Self::Error> {
        debug_assert!((buf.len() <= LARGEST_BUF_SIZE_BYTES), "Buffer size too big");

        // Add one byte for register address
        let mut data = [0u8; 1 + LARGEST_BUF_SIZE_BYTES];
        data[0] = address;
        data[1..=buf.len()].copy_from_slice(buf);
        self.i2c.write(BQ_ADDR, &data).await.map_err(BQ40Z50Error::I2c)?;
        Ok(buf.len())
    }

    async fn flush(&mut self, _address: Self::AddressType) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<E: embedded_hal_async::i2c::Error> smart_battery::Error for BQ40Z50Error<E> {
    fn kind(&self) -> smart_battery::ErrorKind {
        match self {
            Self::I2c(_) => smart_battery::ErrorKind::CommError,
            Self::BatteryStatus(e) => smart_battery::ErrorKind::BatteryStatus(*e),
        }
    }
}

pub struct Bq40z50<I2C: I2cTrait> {
    pub device: Device<DeviceInterface<I2C>>,
    capacity_mode_state: Cell<CapacityModeState>,
}

impl<I2C: I2cTrait> Bq40z50<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Bq40z50 {
            device: Device::new(DeviceInterface { i2c }),
            capacity_mode_state: Cell::new(CapacityModeState::Milliamps),
        }
    }

    fn set_capacity_mode_state(&self, battery_mode_fields: BatteryModeFields) {
        self.capacity_mode_state.set(if battery_mode_fields.capacity_mode() {
            CapacityModeState::Centiwatt
        } else {
            CapacityModeState::Milliamps
        });
    }

    /// Read MAC Register 0x0035 Security Keys.
    ///
    /// This function has special functionality compared to the rest of the MAC commands and so it is handled in its
    /// own function.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn read_security_keys(
        &mut self,
        output_buf: &mut [u8; SECURITY_KEYS_DATA_LEN_BYTES as usize],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut buf = [0u8; 2 + MAC_CMD_ADDR_SIZE_BYTES as usize];
        buf[0] = MAC_CMD;
        buf[1] = MAC_CMD_ADDR_SIZE_BYTES;
        buf[2] = SECURITY_KEYS_CMD[0];
        buf[3] = SECURITY_KEYS_CMD[1];

        // Block write intended register.
        self.device
            .interface
            .i2c
            .write(BQ_ADDR, &buf)
            .await
            .map_err(BQ40Z50Error::I2c)?;

        // [ Length (1 byte) | Command (2 bytes) | Security Keys (8 bytes)]
        let mut raw_output_buf = [0u8; 1 + SECURITY_KEYS_LEN_BYTES as usize];

        self.device
            .interface
            .i2c
            .write_read(BQ_ADDR, &[buf[0]], &mut raw_output_buf)
            .await
            .map_err(BQ40Z50Error::I2c)?;

        output_buf.copy_from_slice(
            &raw_output_buf[(1 + MAC_CMD_ADDR_SIZE_BYTES as usize)..=SECURITY_KEYS_LEN_BYTES as usize],
        );

        Ok(())
    }

    /// Write MAC Register 0x0035 Security Keys.
    ///
    /// This function has special functionality compared to the rest of the MAC commands and so it is handled in its
    /// own function.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn write_security_keys(
        &mut self,
        security_keys: &[u8; SECURITY_KEYS_DATA_LEN_BYTES as usize],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut buf = [0u8; 2 + MAC_CMD_ADDR_SIZE_BYTES as usize + SECURITY_KEYS_DATA_LEN_BYTES as usize];
        buf[0] = MAC_CMD;
        buf[1] = SECURITY_KEYS_LEN_BYTES;
        buf[2] = SECURITY_KEYS_CMD[0];
        buf[3] = SECURITY_KEYS_CMD[1];
        buf[4..].copy_from_slice(security_keys);

        self.device
            .interface
            .i2c
            .write(BQ_ADDR, &buf)
            .await
            .map_err(BQ40Z50Error::I2c)
    }

    /// Read MAC Register 0x0037 Authentication Key.
    ///
    /// This function has special functionality compared to the rest of the MAC commands and so it is handled in its
    /// own function.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn read_authentication_key(
        &mut self,
        output_buf: &mut [u8; AUTH_KEY_DATA_LEN_BYTES as usize],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut buf = [0u8; 2 + MAC_CMD_ADDR_SIZE_BYTES as usize];
        buf[0] = MAC_CMD;
        buf[1] = MAC_CMD_ADDR_SIZE_BYTES;
        buf[2] = AUTH_KEY_CMD[0];
        buf[3] = AUTH_KEY_CMD[1];

        // Block write intended register.
        self.device
            .interface
            .i2c
            .write(BQ_ADDR, &buf)
            .await
            .map_err(BQ40Z50Error::I2c)?;

        // [ Length (1 byte) | Command (2 bytes) | Auth Key (16 bytes)]
        let mut raw_output_buf = [0u8; 1 + AUTH_KEY_LEN_BYTES as usize];

        self.device
            .interface
            .i2c
            .write_read(BQ_ADDR, &[buf[0]], &mut raw_output_buf)
            .await
            .map_err(BQ40Z50Error::I2c)?;

        output_buf
            .copy_from_slice(&raw_output_buf[(1 + MAC_CMD_ADDR_SIZE_BYTES as usize)..=AUTH_KEY_LEN_BYTES as usize]);

        Ok(())
    }

    /// Write MAC Register 0x0037 Authentication Key.
    ///
    /// This function has special functionality compared to the rest of the MAC commands and so it is handled in its
    /// own function.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn write_authentication_key(
        &mut self,
        auth_key: &[u8; AUTH_KEY_LEN_BYTES as usize],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut buf = [0u8; 2 + MAC_CMD_ADDR_SIZE_BYTES as usize + AUTH_KEY_LEN_BYTES as usize];
        buf[0] = MAC_CMD;
        buf[1] = AUTH_KEY_LEN_BYTES;
        buf[2] = AUTH_KEY_CMD[0];
        buf[3] = AUTH_KEY_CMD[1];
        buf[4..].copy_from_slice(auth_key);

        self.device
            .interface
            .i2c
            .write(BQ_ADDR, &buf)
            .await
            .map_err(BQ40Z50Error::I2c)
    }

    /// Read data from an arbitrary register from the device.
    ///
    /// If the register you are trying to read exists in the manifest file, use that function instead.
    ///
    /// Warning: To avoid loss of data, make sure that the `data` slice passed in is large enough to hold all expected data.
    /// Otherwise an `Overrun` error will be returned.
    /// Ensure that the register address is valid in the datasheet. Also ensure that the register is able to be written to.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    ///
    /// # Safety
    ///
    /// This function should only be called with valid register addresses and data slice size.
    /// Ensure that the register is able to be written to.
    #[allow(unsafe_code)]
    pub async unsafe fn read_register_unchecked(
        &mut self,
        reg_address: u8,
        data: &mut [u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        self.device
            .interface
            .i2c
            .write_read(BQ_ADDR, &[reg_address], data)
            .await
            .map_err(BQ40Z50Error::I2c)
    }

    /// Write data to an arbitrary register on the device.
    ///
    /// If the register you are trying to read exists in the manifest file, use that function instead.
    ///
    /// Warning: To avoid loss of data, make sure that the `data` slice passed in is the right size.
    /// Ensure that the register address is valid in the datasheet. Also ensure that the register is able to be written to.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    ///
    /// # Safety
    ///
    /// This function should only be called with valid register addresses and data slice size.
    /// Ensure that the register is able to be written to.
    #[allow(unsafe_code)]
    pub async unsafe fn write_register_unchecked(
        &mut self,
        reg_address: u8,
        data: &[u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        debug_assert!((data.len() <= LARGEST_REG_SIZE_BYTES), "Register size too big");

        // Add one byte for register address
        let mut buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES];
        buf[0] = reg_address;
        buf[1..=data.len()].copy_from_slice(data);
        self.device
            .interface
            .i2c
            .write(BQ_ADDR, &buf[..=data.len()])
            .await
            .map_err(BQ40Z50Error::I2c)
    }
}

impl<I2C: I2cTrait> smart_battery::ErrorType for Bq40z50<I2C> {
    type Error = BQ40Z50Error<I2C::Error>;
}

impl<I2C: I2cTrait> smart_battery::SmartBattery for Bq40z50<I2C> {
    async fn remaining_capacity_alarm(&mut self) -> Result<smart_battery::CapacityModeValue, Self::Error> {
        Ok(match self.capacity_mode_state.get() {
            CapacityModeState::Milliamps => smart_battery::CapacityModeValue::MilliAmpUnsigned(
                self.device
                    .remaining_capacity_alarm()
                    .read_async()
                    .await?
                    .remaining_capacity_alarm(),
            ),
            CapacityModeState::Centiwatt => smart_battery::CapacityModeValue::CentiWattUnsigned(
                self.device
                    .remaining_capacity_alarm()
                    .read_async()
                    .await?
                    .remaining_capacity_alarm(),
            ),
        })
    }

    async fn set_remaining_capacity_alarm(
        &mut self,
        capacity: smart_battery::CapacityModeValue,
    ) -> Result<(), Self::Error> {
        self.device
            .remaining_capacity_alarm()
            .write_async(|d| {
                d.set_remaining_capacity_alarm(match capacity {
                    CapacityModeValue::MilliAmpUnsigned(value) | CapacityModeValue::CentiWattUnsigned(value) => value,
                });
            })
            .await
    }

    async fn remaining_time_alarm(&mut self) -> Result<smart_battery::Minutes, Self::Error> {
        Ok(self
            .device
            .remaining_time_alarm()
            .read_async()
            .await?
            .remaining_time_alarm())
    }

    async fn set_remaining_time_alarm(&mut self, time: smart_battery::Minutes) -> Result<(), Self::Error> {
        self.device
            .remaining_time_alarm()
            .write_async(|f| f.set_remaining_time_alarm(time))
            .await
    }

    async fn battery_mode(&mut self) -> Result<BatteryModeFields, Self::Error> {
        Ok(self.device.battery_mode().read_async().await?.into())
    }

    async fn set_battery_mode(&mut self, flags: BatteryModeFields) -> Result<(), Self::Error> {
        self.set_capacity_mode_state(flags);
        self.device.battery_mode().write_async(|f| *f = flags.into()).await
    }

    async fn at_rate(&mut self) -> Result<smart_battery::CapacityModeSignedValue, Self::Error> {
        Ok(match self.capacity_mode_state.get() {
            CapacityModeState::Milliamps => smart_battery::CapacityModeSignedValue::MilliAmpSigned(
                self.device.at_rate().read_async().await?.at_rate(),
            ),
            CapacityModeState::Centiwatt => smart_battery::CapacityModeSignedValue::CentiWattSigned(
                self.device.at_rate().read_async().await?.at_rate(),
            ),
        })
    }

    async fn set_at_rate(&mut self, rate: smart_battery::CapacityModeSignedValue) -> Result<(), Self::Error> {
        self.device
            .at_rate()
            .write_async(|f| {
                f.set_at_rate(match rate {
                    CapacityModeSignedValue::MilliAmpSigned(value)
                    | CapacityModeSignedValue::CentiWattSigned(value) => value,
                });
            })
            .await
    }

    async fn at_rate_time_to_full(&mut self) -> Result<smart_battery::Minutes, Self::Error> {
        Ok(self
            .device
            .at_rate_time_to_full()
            .read_async()
            .await?
            .at_rate_time_to_full())
    }

    async fn at_rate_time_to_empty(&mut self) -> Result<smart_battery::Minutes, Self::Error> {
        Ok(self
            .device
            .at_rate_time_to_empty()
            .read_async()
            .await?
            .at_rate_time_to_empty())
    }

    async fn at_rate_ok(&mut self) -> Result<bool, Self::Error> {
        // 0 = false, anything else is true
        Ok(!matches!(self.device.at_rate_ok().read_async().await?.at_rate_ok(), 0))
    }

    async fn temperature(&mut self) -> Result<DeciKelvin, Self::Error> {
        Ok(self.device.temperature().read_async().await?.temperature())
    }

    async fn voltage(&mut self) -> Result<smart_battery::MilliVolts, Self::Error> {
        Ok(self.device.voltage().read_async().await?.voltage())
    }

    async fn current(&mut self) -> Result<smart_battery::MilliAmpsSigned, Self::Error> {
        Ok(self.device.current().read_async().await?.current())
    }

    async fn average_current(&mut self) -> Result<smart_battery::MilliAmpsSigned, Self::Error> {
        Ok(self.device.avg_current().read_async().await?.avg_current())
    }

    async fn max_error(&mut self) -> Result<smart_battery::Percent, Self::Error> {
        Ok(self.device.max_error().read_async().await?.max_error())
    }

    async fn relative_state_of_charge(&mut self) -> Result<smart_battery::Percent, Self::Error> {
        Ok(self
            .device
            .relative_state_of_charge()
            .read_async()
            .await?
            .relative_state_of_charge())
    }

    async fn absolute_state_of_charge(&mut self) -> Result<smart_battery::Percent, Self::Error> {
        Ok(self
            .device
            .absolute_state_of_charge()
            .read_async()
            .await?
            .absolute_state_of_charge())
    }

    async fn remaining_capacity(&mut self) -> Result<smart_battery::CapacityModeValue, Self::Error> {
        Ok(match self.capacity_mode_state.get() {
            CapacityModeState::Milliamps => smart_battery::CapacityModeValue::MilliAmpUnsigned(
                self.device
                    .remaining_capacity()
                    .read_async()
                    .await?
                    .remaining_capacity(),
            ),
            CapacityModeState::Centiwatt => smart_battery::CapacityModeValue::CentiWattUnsigned(
                self.device
                    .remaining_capacity()
                    .read_async()
                    .await?
                    .remaining_capacity(),
            ),
        })
    }

    async fn full_charge_capacity(&mut self) -> Result<smart_battery::CapacityModeValue, Self::Error> {
        Ok(match self.capacity_mode_state.get() {
            CapacityModeState::Milliamps => smart_battery::CapacityModeValue::MilliAmpUnsigned(
                self.device
                    .full_charge_capacity()
                    .read_async()
                    .await?
                    .full_charge_capacity(),
            ),
            CapacityModeState::Centiwatt => smart_battery::CapacityModeValue::CentiWattUnsigned(
                self.device
                    .full_charge_capacity()
                    .read_async()
                    .await?
                    .full_charge_capacity(),
            ),
        })
    }

    async fn run_time_to_empty(&mut self) -> Result<smart_battery::Minutes, Self::Error> {
        Ok(self.device.run_time_to_empty().read_async().await?.run_time_to_empty())
    }

    async fn average_time_to_empty(&mut self) -> Result<smart_battery::Minutes, Self::Error> {
        Ok(self
            .device
            .average_time_to_empty()
            .read_async()
            .await?
            .average_time_to_empty())
    }

    async fn average_time_to_full(&mut self) -> Result<smart_battery::Minutes, Self::Error> {
        Ok(self
            .device
            .average_time_to_full()
            .read_async()
            .await?
            .average_time_to_full())
    }

    async fn battery_status(&mut self) -> Result<smart_battery::BatteryStatusFields, Self::Error> {
        let status: smart_battery::BatteryStatusFields = self.device.battery_status().read_async().await?.into();

        match status.error_code() {
            ErrorCode::Ok => Ok(status),
            _ => Err(BQ40Z50Error::BatteryStatus(status.error_code())),
        }
    }

    async fn cycle_count(&mut self) -> Result<smart_battery::Cycles, Self::Error> {
        Ok(self.device.cycle_count().read_async().await?.cycle_count())
    }

    async fn design_capacity(&mut self) -> Result<smart_battery::CapacityModeValue, Self::Error> {
        Ok(match self.capacity_mode_state.get() {
            CapacityModeState::Milliamps => smart_battery::CapacityModeValue::MilliAmpUnsigned(
                self.device.design_capacity().read_async().await?.design_capacity(),
            ),
            CapacityModeState::Centiwatt => smart_battery::CapacityModeValue::CentiWattUnsigned(
                self.device.design_capacity().read_async().await?.design_capacity(),
            ),
        })
    }

    async fn design_voltage(&mut self) -> Result<smart_battery::MilliVolts, Self::Error> {
        Ok(self.device.design_voltage().read_async().await?.design_voltage())
    }

    async fn specification_info(&mut self) -> Result<SpecificationInfoFields, Self::Error> {
        Ok(self.device.specification_info().read_async().await?.into())
    }

    async fn manufacture_date(&mut self) -> Result<smart_battery::ManufactureDate, Self::Error> {
        Ok(self
            .device
            .manufacture_date()
            .read_async()
            .await?
            .manufacture_date()
            .into())
    }

    async fn serial_number(&mut self) -> Result<u16, Self::Error> {
        Ok(self.device.serial_number().read_async().await?.serial_number())
    }

    async fn manufacturer_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        self.device.manufacture_name().read_async(name).await.map(|_f| ())
    }

    async fn device_name(&mut self, name: &mut [u8]) -> Result<(), Self::Error> {
        self.device.device_name().read_async(name).await.map(|_f| ())
    }

    async fn device_chemistry(&mut self, chemistry: &mut [u8]) -> Result<(), Self::Error> {
        self.device.device_chemistry().read_async(chemistry).await.map(|_f| ())
    }

    async fn charging_current(&mut self) -> Result<embedded_batteries_async::charger::MilliAmps, Self::Error> {
        Ok(self.device.charging_current().read_async().await?.charging_current())
    }

    async fn charging_voltage(&mut self) -> Result<smart_battery::MilliVolts, Self::Error> {
        Ok(self.device.charging_voltage().read_async().await?.charging_voltage())
    }
}

#[cfg(test)]
mod tests {
    use embedded_batteries_async::smart_battery::SmartBattery;
    use embedded_hal_mock::eh1::i2c::{Mock, Transaction};

    use super::*;

    #[tokio::test]
    async fn read_chip_id() {
        let expectations = vec![Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x21, 0x00])];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        bq.mac_gauging().dispatch_async().await.unwrap();

        bq.interface.i2c.done();
    }

    #[tokio::test]
    async fn read_chip_id_2() {
        let expectations = vec![
            Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x01, 0x00]),
            Transaction::write_read(BQ_ADDR, vec![0x44], vec![0x04, 0x01, 0x00, 0x00, 0x00]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        bq.mac_device_type().dispatch_async().await.unwrap();
        bq.interface.i2c.done();
    }

    #[tokio::test]
    async fn read_firmware_version() {
        let expectations = vec![
            Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x02, 0x00]),
            Transaction::write_read(
                BQ_ADDR,
                vec![0x44],
                vec![
                    0x0A, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                ],
            ),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        bq.mac_firmware_version().dispatch_async().await.unwrap();
        bq.interface.i2c.done();
    }

    #[tokio::test]
    async fn read_too_large_manufacture_name() {
        let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x20], vec![0x00])];
        let i2c = Mock::new(&expectations);
        let mut bq = Device::new(DeviceInterface { i2c });

        let mut manufacture_name = [0u8; 1];

        bq.manufacture_name().read_async(&mut manufacture_name).await.unwrap();
        bq.interface.i2c.done();
    }

    #[tokio::test]
    async fn write_unseal_keys() {
        let expectations = vec![
            Transaction::write(
                BQ_ADDR,
                vec![0x44, 0x0A, 0x35, 0x00, 0x30, 0x30, 0x60, 0x60, 0x01, 0x01, 0x10, 0x10],
            ),
            Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x35, 0x00]),
            Transaction::write_read(
                BQ_ADDR,
                vec![0x44],
                vec![0x0A, 0x35, 0x00, 0x30, 0x30, 0x60, 0x60, 0x01, 0x01, 0x10, 0x10],
            ),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq40z50::new(i2c);

        let security_keys = [0x30u8, 0x30u8, 0x60u8, 0x60u8, 0x01u8, 0x01u8, 0x10u8, 0x10u8];

        bq.write_security_keys(&security_keys).await.unwrap();

        let mut result = [0u8; 8];
        bq.read_security_keys(&mut result).await.unwrap();

        assert_eq!(u16::from_le_bytes([result[0], result[1]]), 0x3030);
        assert_eq!(u16::from_le_bytes([result[2], result[3]]), 0x6060);
        assert_eq!(u16::from_le_bytes([result[4], result[5]]), 0x0101);
        assert_eq!(u16::from_le_bytes([result[6], result[7]]), 0x1010);
        bq.device.interface.i2c.done();
    }

    #[tokio::test]
    async fn test_battery_status() {
        let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x16], vec![0x30, 0x30])];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq40z50::new(i2c);

        let status = match bq.battery_status().await {
            Ok(status) => status,
            Err(e) => match e {
                _ => unreachable!(),
            },
        };

        assert_eq!(status.error_code(), ErrorCode::Ok);

        bq.device.interface.i2c.done();
    }

    #[tokio::test]
    #[allow(unsafe_code)]
    async fn test_read_write_unchecked() {
        let expectations = vec![
            Transaction::write_read(BQ_ADDR, vec![0x16], vec![0x30, 0x30]),
            Transaction::write(BQ_ADDR, vec![0x16, 0x2F, 0x30]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq40z50::new(i2c);

        let mut data = [0u8; 2];

        unsafe {
            bq.read_register_unchecked(0x16, &mut data).await.unwrap();
        }
        data[0] -= 1;

        unsafe {
            bq.write_register_unchecked(0x16, &data).await.unwrap();
        }

        bq.device.interface.i2c.done();
    }

    #[tokio::test]
    async fn test_capacity_mode() {
        let expectations = vec![
            Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x80]),
            Transaction::write_read(BQ_ADDR, vec![0x0F], vec![100, 0x00]),
            Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x00]),
            Transaction::write_read(BQ_ADDR, vec![0x0F], vec![80, 0x00]),
        ];
        let i2c = Mock::new(&expectations);
        let mut bq = Bq40z50::new(i2c);

        assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Milliamps);

        let mode = BatteryModeFields::new().with_capacity_mode(true);
        bq.set_battery_mode(mode).await.unwrap();
        assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Centiwatt);

        let mode = BatteryModeFields::new().with_capacity_mode(false);
        let rem_cap = bq.remaining_capacity().await.unwrap();
        assert!(matches!(rem_cap, CapacityModeValue::CentiWattUnsigned(100)));

        let info = bq.set_battery_mode(mode).await;
        assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Milliamps);

        let rem_cap = bq.remaining_capacity().await.unwrap();
        assert!(matches!(rem_cap, CapacityModeValue::MilliAmpUnsigned(80)));

        bq.device.interface.i2c.done();
    }
}
