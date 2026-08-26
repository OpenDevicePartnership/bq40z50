use core::cell::Cell;

use device_driver::Block;
use embedded_hal_async::delay::DelayNs as DelayTrait;
use embedded_hal_async::i2c::I2c as I2cTrait;

use crate::common::{CapacityModeState, Config};
use crate::consts::{
    AUTH_KEY_CMD, AUTH_KEY_DATA_LEN_BYTES, AUTH_KEY_LEN_BYTES, LARGEST_REG_SIZE_BYTES, MAC_CMD,
    MAC_CMD_ADDR_SIZE_BYTES, MFG_INFO_CMD, SECURITY_KEYS_CMD, SECURITY_KEYS_DATA_LEN_BYTES, SECURITY_KEYS_LEN_BYTES,
};
use crate::error::BQ40Z50Error;
use crate::interface::DeviceInterface;
pub use crate::versions::gen_r1::*;

pub struct Bq40z50R1<I2C: I2cTrait, DELAY: DelayTrait> {
    pub device: Device<DeviceInterface<I2C, DELAY>>,
    capacity_mode_state: core::cell::Cell<CapacityModeState>,
}

impl<I2C: I2cTrait, DELAY: DelayTrait> Bq40z50R1<I2C, DELAY> {
    pub fn new(i2c: I2C, delay: DELAY) -> Self {
        Bq40z50R1 {
            device: Device::new(DeviceInterface::new(i2c, delay)),
            capacity_mode_state: core::cell::Cell::new(CapacityModeState::Milliamps),
        }
    }

    pub fn new_with_config(i2c: I2C, delay: DELAY, config: Config) -> Self {
        Bq40z50R1 {
            device: Device::new(DeviceInterface::new_with_config(i2c, delay, config)),
            capacity_mode_state: Cell::new(CapacityModeState::Milliamps),
        }
    }

    /// Change interface config.
    ///
    /// Concurrency is guaranteed by the mutable borrow, ensuring the config cannot change
    /// while a register transaction is in flight.
    pub fn update_config(&mut self, config: Config) {
        self.device.interface().config = config;
    }

    pub fn config(&mut self) -> Config {
        self.device.interface().config
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
        self.device.interface().mac_read_with_retries(&buf, output_buf).await
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

        self.device.interface().mac_write_with_retries(&buf).await
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

        self.device.interface().mac_read_with_retries(&buf, output_buf).await
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

        self.device.interface().mac_write_with_retries(&buf).await
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
            .interface()
            .read_with_retries(&[reg_address], data, false)
            .await
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
            .interface()
            .write_with_retries(&buf[..=data.len()], false)
            .await
    }

    /// Seal the fuel gauge.
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn seal_fg(&mut self) -> Result<(), BQ40Z50Error<I2C::Error>> {
        self.device.mac_seal().dispatch_async().await
    }

    /// Unseal the fuel gauge.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn unseal_fg(
        &mut self,
        unseal_key_lower: u16,
        unseal_key_upper: u16,
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        self.send_access_key(unseal_key_lower, unseal_key_upper).await
    }

    /// Send access keys.
    ///
    /// Various keys are defined, please check the datasheet of the revision you're working with for
    /// the types of keys and their function.
    ///
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn send_access_key(
        &mut self,
        access_key_lower: u16,
        access_key_upper: u16,
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut buf = [0u8; 4];

        // Write lower access key
        buf[0] = MAC_CMD;
        buf[1] = MAC_CMD_ADDR_SIZE_BYTES;
        buf[2..4].copy_from_slice(&access_key_lower.to_le_bytes());
        self.device.interface().mac_write_with_retries(&buf).await?;

        // Write upper access key
        buf[0] = MAC_CMD;
        buf[1] = MAC_CMD_ADDR_SIZE_BYTES;
        buf[2..4].copy_from_slice(&access_key_upper.to_le_bytes());
        self.device.interface().mac_write_with_retries(&buf).await
    }

    /// Write to the `MfgInfo` register. Despite it not being a MAC cmd, it uses the `SMBus` block command.
    ///
    /// Requires fuel gauge to be unsealed. Send `unseal_fg()` first, and then reseal with `seal_fg()` after this command.
    ///
    /// `data` can be at most 32 bytes large.
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    #[allow(clippy::cast_possible_truncation)]
    pub async fn write_mfg_info(&mut self, data: &[u8]) -> Result<(), BQ40Z50Error<I2C::Error>> {
        if data.len() > LARGEST_REG_SIZE_BYTES {
            return Err(BQ40Z50Error::DataTooLarge);
        }
        let mut buf = [0u8; 2 + LARGEST_REG_SIZE_BYTES];
        buf[0] = MFG_INFO_CMD;
        buf[1] = data.len() as u8;
        buf[2..data.len() + 2].copy_from_slice(data);

        self.device
            .interface()
            .mac_write_with_retries(&buf[..data.len() + 2])
            .await
    }

    /// Read from the `MfgInfo` register.
    ///
    /// NOTE: `mfg_info` is unique in that it has a leading size byte,
    /// meaning you need to read 33 bytes to read 32 bytes of data.
    /// This fn will return the data including the size byte as the zeroth byte.
    ///
    /// `data` can be at most 33 bytes large, which includes the size byte in the zeroth position.
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn read_mfg_info(&mut self, data: &mut [u8]) -> Result<(), BQ40Z50Error<I2C::Error>> {
        if data.len() > LARGEST_REG_SIZE_BYTES + 1 {
            return Err(BQ40Z50Error::DataTooLarge);
        }
        if self.device.interface().config.pec_read {
            // If reading with PEC, the entire payload needs to be read to verify the PEC byte
            // This reduces performance because without PEC, we could read parts of the register and NACK early if we
            // know the size of the data we want to read.
            // [ size | data | data | ... | data | PEC (optional) ]
            let mut read_buf = [0u8; LARGEST_REG_SIZE_BYTES + 1];
            self.device
                .interface()
                .read_with_retries(&[MFG_INFO_CMD], &mut read_buf, true)
                .await?;
            data.copy_from_slice(&read_buf[..data.len()]);
            Ok(())
        } else {
            self.device
                .interface()
                .read_with_retries(&[MFG_INFO_CMD], data, false)
                .await
        }
    }

    /// Read from the data flash (DF). Refer to the datasheet for the data flash table.
    ///
    /// Starting address should be between 0x4000 and 0x5FFF.
    /// The input argument `read` slice size should reflect the desired number of bytes to be read.
    ///
    /// # Note
    /// The data flash supports returning up to 32 bytes of data per read transaction, however this method can
    /// handle reads of larger than 32 bytes. On the physical bus, the reads will be chunked into 32 byte blocks.
    /// Thus, the input argument `read` slice length can be larger than 32 bytes.
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn read_dataflash(
        &mut self,
        starting_address: u16,
        read: &mut [u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        if self.device.interface().config.pec_read {
            self.device
                .interface()
                .mac_read_from_df_with_retries_pec(starting_address, read)
                .await
        } else {
            self.device
                .interface()
                .mac_read_from_df_with_retries(starting_address, read)
                .await
        }
    }

    /// Write to the data flash (DF). Refer to the datasheet for the data flash table.
    ///
    /// Starting address should be between 0x4000 and 0x5FFF.
    /// The input argument `write` slice size should reflect the desired number of bytes to be written.
    ///
    /// # Note
    /// The data flash supports writing up to 32 bytes of data per write transaction, however this method can
    /// handle writes of larger than 32 bytes. On the physical bus, the writes will be chunked into 32 byte blocks.
    /// Thus, the input argument `write` slice length can be larger than 32 bytes.
    /// # Errors
    ///
    /// Will return `Err` if an I2C bus error occurs.
    pub async fn write_dataflash(
        &mut self,
        starting_address: u16,
        write: &[u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        self.device
            .interface()
            .mac_write_to_df_with_retries(starting_address, write)
            .await
    }
}

crate::common::implement_embedded_batteries!(Bq40z50R1);

crate::tests::bq40z50_tests!(Bq40z50R1);
