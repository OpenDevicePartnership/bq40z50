use core::future::{Future, ready};
use core::hash::Hasher;

use device_driver::FieldsetMetadata;
#[cfg(feature = "embassy-timeout")]
use embassy_time::with_timeout;
use embedded_hal_async::delay::DelayNs as DelayTrait;
use embedded_hal_async::i2c::I2c as I2cTrait;

use crate::common::Config;
use crate::consts::{
    BQ_ADDR, DEFAULT_ERROR_BACKOFF_DELAY_MS, DF_FIRST_ADDRESS, DF_LAST_ADDRESS, LARGEST_BUF_SIZE_BYTES,
    LARGEST_CMD_SIZE_BYTES, LARGEST_DF_BLOCK_SIZE_BYTES, LARGEST_REG_SIZE_BYTES, MAC_CMD, MAC_CMD_ADDR_SIZE_BITS,
    MAC_CMD_ADDR_SIZE_BYTES,
};
use crate::error::BQ40Z50Error;

/// BQ40Z50 interface, common to all chip revisions, which takes an async I2C bus
pub struct DeviceInterface<I2C: I2cTrait, DELAY: DelayTrait> {
    /// embedded-hal-async compliant I2C bus
    pub i2c: I2C,
    pub delay: DELAY,
    pub config: Config,
}

impl<I2C: I2cTrait, DELAY: DelayTrait> DeviceInterface<I2C, DELAY> {
    #[must_use]
    pub const fn new(i2c: I2C, delay: DELAY) -> Self {
        DeviceInterface {
            i2c,
            delay,
            config: Config::new(),
        }
    }

    #[must_use]
    pub const fn new_with_config(i2c: I2C, delay: DELAY, config: Config) -> Self {
        DeviceInterface { i2c, delay, config }
    }
}

impl<I2C: I2cTrait, DELAY: DelayTrait> DeviceInterface<I2C, DELAY> {
    pub(crate) async fn mac_write_with_retries(&mut self, write: &[u8]) -> Result<(), BQ40Z50Error<I2C::Error>> {
        // Same functionality as regular SMBus writes, write buffer just needs to be properly formed.
        self.write_with_retries(write, self.config.pec_write).await
    }

    #[allow(clippy::cast_possible_truncation)]
    pub(crate) async fn mac_write_to_df_with_retries(
        &mut self,
        starting_address: u16,
        write: &[u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        check_df_range(starting_address, write.len())?;

        let use_pec = self.config.pec_write;

        // Number of caller payload bytes the gauge has accepted so far. Data flash is committed
        // one chunk at a time and the driver cannot roll a committed chunk back, so if a later
        // chunk fails this count is handed to the caller via `PartialDataFlashWrite`.
        let mut committed = 0usize;

        let mut bytes_left_to_write = write.len();
        while bytes_left_to_write > 0 {
            // Largest single write block is 1 byte MAC command + 1 byte size + 2 bytes starting address + 32 bytes data + 1 PEC byte.
            let mut output_buf = [0u8; 4 + LARGEST_DF_BLOCK_SIZE_BYTES + 1];
            // Determine how many bytes to write to the bus for this chunk.
            let output_buf_end_idx = core::cmp::min(output_buf.len() - 1, bytes_left_to_write + 4);

            let start_idx = write.len() - bytes_left_to_write;
            let end_idx = start_idx + output_buf_end_idx - 4;
            // `check_df_range` has already proven the whole transfer fits inside 0x4000-0x5FFF,
            // so neither the conversion nor the add can fail here. They are still written as
            // checked operations so that a chunk address can never silently wrap onto a low data
            // flash address and clobber calibration data, safety thresholds or the security keys.
            let starting_address_chunk = u16::try_from(start_idx)
                .ok()
                .and_then(|offset| starting_address.checked_add(offset))
                .ok_or(BQ40Z50Error::DataFlashAddressOutOfRange)?
                .to_le_bytes();
            output_buf[0] = MAC_CMD;
            // Safe cast as output_buf_end_idx can only be as high as output_buf.len(), which is 36
            output_buf[1] = output_buf_end_idx as u8 - 2;
            output_buf[2] = starting_address_chunk[0];
            output_buf[3] = starting_address_chunk[1];
            output_buf[4..output_buf_end_idx].copy_from_slice(&write[start_idx..end_idx]);

            let res = if use_pec {
                // Add PEC at the end.
                let mut pec = smbus_pec::Pec::new();
                pec.write(&[BQ_ADDR << 1]);
                pec.write(&output_buf[..output_buf_end_idx]);
                // Safe cast as SMBUS PEC is a u8, returned value is u64 because of the Hasher trait.
                output_buf[output_buf_end_idx] = pec.finish() as u8;
                self.write_with_retries_internal(&output_buf[..=output_buf_end_idx])
                    .await
            } else {
                self.write_with_retries_internal(&output_buf[..output_buf_end_idx])
                    .await
            };

            if let Err(e) = res {
                // Nothing was committed, so there is no partial state to describe and the caller
                // is better served by the underlying cause. Only report a partial write once at
                // least one chunk has actually reached flash.
                return Err(if committed == 0 {
                    e
                } else {
                    BQ40Z50Error::PartialDataFlashWrite { committed }
                });
            }

            committed += end_idx - start_idx;
            bytes_left_to_write = bytes_left_to_write.saturating_sub(LARGEST_DF_BLOCK_SIZE_BYTES);
        }

        Ok(())
    }
}

/// Checks that a data flash transfer of `len` bytes starting at `starting_address` lies entirely
/// inside the documented data flash window.
///
/// All four TRMs title the data flash section with the window itself, for example SLUUCN4B
/// 16.1.101 `"ManufacturerAccess() 0x4000-0x5FFF DataFlashAccess"`.
///
/// The *end* of the transfer is validated, not just the start, so a multi-chunk transfer cannot
/// begin inside the window and run off the end of it. The end is computed with checked
/// arithmetic, so an address near the top of the `u16` space is rejected rather than wrapping.
///
/// An empty transfer touches no addresses and is accepted as a no-op.
fn check_df_range<E>(starting_address: u16, len: usize) -> Result<(), BQ40Z50Error<E>> {
    let Some(last_offset) = len.checked_sub(1) else {
        // Empty transfer: no addresses are touched.
        return Ok(());
    };

    if starting_address < DF_FIRST_ADDRESS {
        return Err(BQ40Z50Error::DataFlashAddressOutOfRange);
    }

    let last_address = u16::try_from(last_offset)
        .ok()
        .and_then(|offset| starting_address.checked_add(offset))
        .ok_or(BQ40Z50Error::DataFlashAddressOutOfRange)?;

    if last_address > DF_LAST_ADDRESS {
        return Err(BQ40Z50Error::DataFlashAddressOutOfRange);
    }

    Ok(())
}

/// Performs a single I2C bus operation, bounded by `config.timeout`.
///
/// Expands to a `Result<(), BQ40Z50Error<I2C::Error>>`: a bus error becomes
/// [`BQ40Z50Error::I2c`] and a timeout becomes [`BQ40Z50Error::Timeout`].
///
/// This is a macro rather than an `async fn` helper because the timeout lives in
/// `self.config` while the operation future mutably borrows `self.i2c`; a helper taking
/// `&mut self` plus a future cannot express that disjoint field borrow.
#[cfg(feature = "embassy-timeout")]
macro_rules! bus_op {
    ($self:expr, $op:expr) => {
        match with_timeout($self.config.timeout, $op).await {
            Err(_) => Err(BQ40Z50Error::Timeout),
            Ok(Err(bus_err)) => Err(BQ40Z50Error::I2c(bus_err)),
            Ok(Ok(())) => Ok(()),
        }
    };
}

/// Performs a single I2C bus operation.
///
/// Expands to a `Result<(), BQ40Z50Error<I2C::Error>>`: a bus error becomes
/// [`BQ40Z50Error::I2c`]. Without the `embassy-timeout` feature there is no timeout, so
/// this variant simply awaits the operation; call sites are identical either way.
#[cfg(not(feature = "embassy-timeout"))]
macro_rules! bus_op {
    ($self:expr, $op:expr) => {
        $op.await.map_err(BQ40Z50Error::I2c)
    };
}

impl<I2C: I2cTrait, DELAY: DelayTrait> DeviceInterface<I2C, DELAY> {
    async fn write_with_retries_internal(&mut self, write: &[u8]) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut retries = self.config.max_bus_retries;

        // Because the BQ40Z50's registers vary in size, we pass in a slice of
        // the appropriate size so we do not accidentally write to the register
        // at address + 1 when writing to a 1 byte register
        loop {
            let res = bus_op!(self, self.i2c.write(BQ_ADDR, write));

            if res.is_ok() {
                return res;
            }

            if retries == 0 {
                // Return error
                return res;
            }
            retries -= 1;
            // Delay 10ms since the fuel gauge might be "thinking" from a previous command
            self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
        }
    }

    pub(crate) async fn write_with_retries(
        &mut self,
        write: &[u8],
        use_pec: bool,
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut write_buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES + 6];

        let write_buf_ref: &[u8] = if use_pec {
            let mut pec = smbus_pec::Pec::default();
            // Device Addr + Write Bit (0)
            pec.write_u8(BQ_ADDR << 1);
            pec.write(write);

            // Write one more byte (PEC)
            write_buf[..write.len()].copy_from_slice(write);
            write_buf[write.len()] = pec.finish().try_into().unwrap();

            // Include everything we want to write plus the PEC byte
            &write_buf[..=write.len()]
        } else {
            write
        };
        self.write_with_retries_internal(write_buf_ref).await
    }

    pub(crate) async fn read_with_retries(
        &mut self,
        write: &[u8],
        mut read: &mut [u8],
        use_pec: bool,
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let mut retries = self.config.max_bus_retries;
        // Read buffer with one extra space at the end, in case we use PEC, and one extra space in the front for `mfg_info`
        let mut read_buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES + 1];

        let read_len = read.len();

        let read_buf_ref = if use_pec {
            // Read one more byte (PEC)
            &mut read_buf[..=read_len]
        } else {
            &mut read
        };

        loop {
            let res = bus_op!(self, self.i2c.write_read(BQ_ADDR, write, read_buf_ref));

            if let Err(e) = res {
                if retries == 0 {
                    return Err(e);
                }
                retries -= 1;
                // Delay 10ms since the fuel gauge might be "thinking" from a previous command
                self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                continue;
            }

            if use_pec {
                let mut pec = smbus_pec::Pec::default();
                // Device Addr + Write Bit (0)
                pec.write_u8(BQ_ADDR << 1);
                pec.write(write);
                // Device Addr + Read Bit (1)
                pec.write_u8(BQ_ADDR << 1 | 0x01);

                let recvd_pec = read_buf_ref[read_len];
                pec.write(&read_buf_ref[..read_len]);

                // Check PEC
                if recvd_pec != pec.finish().try_into().unwrap() {
                    if retries == 0 {
                        return Err(BQ40Z50Error::Pec);
                    }
                    retries -= 1;
                    // Delay 10ms since the fuel gauge might be "thinking" from a previous command
                    self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                    continue;
                }
                // If all is good, copy bytes we read into read.
                read.copy_from_slice(&read_buf[..read_len]);
            }

            return Ok(());
        }
    }

    #[allow(clippy::range_plus_one)]
    pub(crate) async fn mac_read_with_retries(
        &mut self,
        write: &[u8],
        read: &mut [u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        let use_pec = self.config.pec_read;
        let mut retries = self.config.max_bus_retries;
        // Read buffer with one extra space at the end, in case we use PEC
        // Response looks like [ Length (1 byte) | Command (2 bytes) | Data (output.len() bytes)]
        let mut read_buf = [0u8; 1 + MAC_CMD_ADDR_SIZE_BYTES as usize + LARGEST_CMD_SIZE_BYTES + 1];
        // Write buffer with one extra space at the end, in case we use PEC
        // [ MAC_CMD (0x44) | CMD_SIZE | CMD_LSB | CMD_MSB | PEC ]
        let mut write_buf = [0u8; 1 + 2 + MAC_CMD_ADDR_SIZE_BYTES as usize];

        let write_buf_ref: &[u8];
        let read_buf_ref: &mut [u8];

        if use_pec {
            let mut pec = smbus_pec::Pec::default();
            pec.write_u8(BQ_ADDR << 1);
            pec.write(write);

            // Compute PEC for the Write Block
            write_buf[..write.len()].copy_from_slice(write);
            // Infalliable because the underlying crate is guaranteed to return a u8
            write_buf[write.len()] = pec.finish().try_into().unwrap();
            // Include everything we want to write plus the PEC byte
            write_buf_ref = &write_buf[..=write.len()];
            read_buf_ref = &mut read_buf[..1 + MAC_CMD_ADDR_SIZE_BYTES as usize + read.len() + 1];
        } else {
            write_buf_ref = write;
            read_buf_ref = &mut read_buf[..1 + MAC_CMD_ADDR_SIZE_BYTES as usize + read.len()];
        }

        // Loop until no bus errors or max bus retries are hit.
        loop {
            // Block write intended register.
            let res = bus_op!(self, self.i2c.write(BQ_ADDR, write_buf_ref));

            if res.is_err() {
                if retries == 0 {
                    return res;
                }
                self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                retries -= 1;
                continue;
            }

            // For read only commands.
            // Block read using I2C write_read, sending 0x44 as the command.
            let res = bus_op!(self, self.i2c.write_read(BQ_ADDR, &[write[0]], read_buf_ref));

            if res.is_err() {
                if retries == 0 {
                    return res;
                }
                self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                retries -= 1;
                continue;
            }

            if use_pec {
                let mut pec = smbus_pec::Pec::default();
                pec.write_u8(BQ_ADDR << 1);
                pec.write_u8(MAC_CMD);
                pec.write_u8(BQ_ADDR << 1 | 0x01);

                let recvd_pec = read_buf_ref[1 + MAC_CMD_ADDR_SIZE_BYTES as usize + read.len()];
                pec.write(&read_buf_ref[..1 + MAC_CMD_ADDR_SIZE_BYTES as usize + read.len()]);

                // Check PEC
                if recvd_pec != pec.finish().try_into().unwrap() {
                    if retries == 0 {
                        return Err(BQ40Z50Error::Pec);
                    }
                    retries -= 1;
                    // Delay 10ms since the fuel gauge might be "thinking" from a previous command
                    self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                    continue;
                }
            }

            read.copy_from_slice(
                &read_buf_ref
                    [(1 + MAC_CMD_ADDR_SIZE_BYTES as usize)..(1 + MAC_CMD_ADDR_SIZE_BYTES as usize + read.len())],
            );

            return Ok(());
        }
    }

    pub(crate) async fn mac_read_from_df_with_retries(
        &mut self,
        starting_address: u16,
        read: &mut [u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        check_df_range(starting_address, read.len())?;

        let mut retries = self.config.max_bus_retries;

        // Read in 32 byte chunks. The FG supports an auto-increment on the address during a DF read.
        // If an SMBus read block is sent, the gauge will return 32 bytes of DF data,
        // and if a subsequent SMBus read block is sent with command 0x44,
        // the gauge returns another 32 bytes of DF data starting at the starting address + 32.
        //
        // That auto-increment means the starting address only needs to be sent once for a clean
        // run of chunks. As soon as a chunk fails, however, the position of the gauge's read
        // pointer is no longer known, so the failing chunk's own starting address is re-sent
        // before retrying. Retrying without re-addressing would return the *next* block and the
        // driver would hand that to the caller as the block that was asked for.
        let mut bytes_read = 0;
        let mut send_address = true;

        while bytes_read < read.len() {
            if send_address {
                // Infallible: `check_df_range` proved the whole transfer fits in the DF window.
                let chunk_address = u16::try_from(bytes_read)
                    .ok()
                    .and_then(|offset| starting_address.checked_add(offset))
                    .ok_or(BQ40Z50Error::DataFlashAddressOutOfRange)?
                    .to_le_bytes();

                // Block write intended register.
                let res = bus_op!(
                    self,
                    self.i2c.write(
                        BQ_ADDR,
                        &[MAC_CMD, MAC_CMD_ADDR_SIZE_BYTES, chunk_address[0], chunk_address[1],],
                    )
                );

                if res.is_err() {
                    if retries == 0 {
                        return res;
                    }
                    self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                    retries -= 1;
                    continue;
                }

                send_address = false;
            }

            // Largest single read block is 1 byte size + 2 bytes starting address + 32 bytes data.
            let mut output_buf = [0u8; 1 + LARGEST_DF_BLOCK_SIZE_BYTES + MAC_CMD_ADDR_SIZE_BYTES as usize];
            // Determine how many bytes to read from the bus, ideally we want to minimize time reading from DF
            // so if we can read less than 32 bytes of DF data, do it.
            let output_buf_end_idx = core::cmp::min(
                output_buf.len(),
                (read.len() - bytes_read) + MAC_CMD_ADDR_SIZE_BYTES as usize + 1,
            );

            let res = bus_op!(
                self,
                self.i2c
                    .write_read(BQ_ADDR, &[MAC_CMD], &mut output_buf[..output_buf_end_idx])
            );

            if res.is_err() {
                if retries == 0 {
                    return res;
                }
                self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                retries -= 1;
                // The read pointer may or may not have advanced, so re-address before retrying.
                send_address = true;
                continue;
            }

            let data_start = MAC_CMD_ADDR_SIZE_BYTES as usize + 1;
            let chunk_len = output_buf_end_idx - data_start;
            read[bytes_read..bytes_read + chunk_len].copy_from_slice(&output_buf[data_start..output_buf_end_idx]);
            bytes_read += chunk_len;
        }

        Ok(())
    }

    pub(crate) async fn mac_read_from_df_with_retries_pec(
        &mut self,
        starting_address: u16,
        read: &mut [u8],
    ) -> Result<(), BQ40Z50Error<I2C::Error>> {
        check_df_range(starting_address, read.len())?;

        let mut retries = self.config.max_bus_retries;

        // See `mac_read_from_df_with_retries` for why the address is re-sent after a failed chunk
        // rather than relying on the gauge's read pointer auto-increment. A failed PEC check is
        // exactly such a failure: continuing without re-addressing would turn a detected CRC
        // error into silently returning the following block.
        let mut bytes_read = 0;
        let mut send_address = true;

        while bytes_read < read.len() {
            if send_address {
                // Infallible: `check_df_range` proved the whole transfer fits in the DF window.
                let chunk_address = u16::try_from(bytes_read)
                    .ok()
                    .and_then(|offset| starting_address.checked_add(offset))
                    .ok_or(BQ40Z50Error::DataFlashAddressOutOfRange)?
                    .to_le_bytes();

                let pec = smbus_pec::pec(&[
                    BQ_ADDR << 1,
                    MAC_CMD,
                    MAC_CMD_ADDR_SIZE_BYTES,
                    chunk_address[0],
                    chunk_address[1],
                ]);

                // Block write intended register.
                let res = bus_op!(
                    self,
                    self.i2c.write(
                        BQ_ADDR,
                        &[
                            MAC_CMD,
                            MAC_CMD_ADDR_SIZE_BYTES,
                            chunk_address[0],
                            chunk_address[1],
                            pec,
                        ],
                    )
                );

                if res.is_err() {
                    if retries == 0 {
                        return res;
                    }
                    self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                    retries -= 1;
                    continue;
                }

                send_address = false;
            }

            // Largest single read block is 1 byte size + 2 bytes starting address + 32 bytes data + 1 PEC byte.
            let mut output_buf = [0u8; 1 + LARGEST_DF_BLOCK_SIZE_BYTES + MAC_CMD_ADDR_SIZE_BYTES as usize + 1];

            // For PEC, we need to read in 32 byte chunks, even if we have <32 bytes left to read.
            let output_buf_end_idx = output_buf.len();

            let res = bus_op!(
                self,
                self.i2c
                    .write_read(BQ_ADDR, &[MAC_CMD], &mut output_buf[..output_buf_end_idx])
            );

            if res.is_err() {
                if retries == 0 {
                    return res;
                }
                self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                retries -= 1;
                // The read pointer may or may not have advanced, so re-address before retrying.
                send_address = true;
                continue;
            }

            let recvd_pec = output_buf[output_buf_end_idx - 1];
            let mut pec = smbus_pec::Pec::new();
            pec.write(&[BQ_ADDR << 1, MAC_CMD, BQ_ADDR << 1 | 0x01]);
            // Omit PEC
            pec.write(&output_buf[..output_buf_end_idx - 1]);
            let pec = pec.finish();

            if u64::from(recvd_pec) != pec {
                if retries == 0 {
                    return Err(BQ40Z50Error::Pec);
                }
                self.delay.delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS).await;
                retries -= 1;
                // The gauge has already advanced its read pointer past this block, so the retry
                // must re-send this chunk's starting address to read the same block again.
                send_address = true;
                continue;
            }

            let data_start = MAC_CMD_ADDR_SIZE_BYTES as usize + 1;
            let chunk_len = core::cmp::min(read.len() - bytes_read, LARGEST_DF_BLOCK_SIZE_BYTES);
            read[bytes_read..bytes_read + chunk_len].copy_from_slice(&output_buf[data_start..data_start + chunk_len]);
            bytes_read += chunk_len;
        }

        Ok(())
    }
}
impl<I2C: I2cTrait, DELAY: DelayTrait> device_driver::RegisterInterfaceBase for DeviceInterface<I2C, DELAY> {
    type Error = BQ40Z50Error<I2C::Error>;
    type AddressType = u8;
}

impl<I2C: I2cTrait, DELAY: DelayTrait> device_driver::AsyncRegisterInterface for DeviceInterface<I2C, DELAY> {
    async fn write_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _meta_data: &FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        if data.len() > LARGEST_REG_SIZE_BYTES {
            return Err(BQ40Z50Error::DataTooLarge);
        }

        // Add one byte for register address
        let mut buf = [0u8; 1 + LARGEST_REG_SIZE_BYTES];
        buf[0] = address;
        buf[1..=data.len()].copy_from_slice(data);

        // Because the BQ40Z50's registers vary in size, we pass in a slice of
        // the appropriate size so we do not accidentally write to the register
        // at address + 1 when writing to a 1 byte register
        self.write_with_retries(&buf[..=data.len()], self.config.pec_write)
            .await
    }

    async fn read_register(
        &mut self,
        address: Self::AddressType,
        data: &mut [u8],
        _meta_data: &FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        if data.len() > LARGEST_REG_SIZE_BYTES {
            return Err(BQ40Z50Error::DataTooLarge);
        }
        self.read_with_retries(&[address], data, self.config.pec_read).await
    }
}

impl<I2C: I2cTrait, DELAY: DelayTrait> device_driver::CommandInterfaceBase for DeviceInterface<I2C, DELAY> {
    type Error = BQ40Z50Error<I2C::Error>;
    type AddressType = u32;
}

impl<I2C: I2cTrait, DELAY: DelayTrait> device_driver::AsyncCommandInterface for DeviceInterface<I2C, DELAY> {
    async fn dispatch_command(
        &mut self,
        address: Self::AddressType,
        input: &mut [u8],
        _meta_data_in: &FieldsetMetadata,
        output: &mut [u8],
        _meta_data_out: &FieldsetMetadata,
    ) -> Result<(), Self::Error> {
        if input.len() > LARGEST_CMD_SIZE_BYTES || output.len() > LARGEST_CMD_SIZE_BYTES {
            return Err(BQ40Z50Error::DataTooLarge);
        }

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

        if input.is_empty() && output.is_empty() {
            // Write only, writes don't have an output size nor an input size because
            // writes only consist of the register/command address.
            self.mac_write_with_retries(&buf).await?;
        } else if input.is_empty() && !output.is_empty() {
            // For read only commands.
            self.mac_read_with_retries(&buf, output).await?;
        } else {
            // Read/write, to be handled in other functions as special cases.
            unreachable!();
        }
        Ok(())
    }
}

impl<I2C: I2cTrait, DELAY: DelayTrait> device_driver::BufferInterfaceBase for DeviceInterface<I2C, DELAY> {
    type Error = BQ40Z50Error<I2C::Error>;
    type AddressType = u8;
}

impl<I2C: I2cTrait, DELAY: DelayTrait> device_driver::AsyncBufferInterface for DeviceInterface<I2C, DELAY> {
    async fn read(&mut self, address: Self::AddressType, buf: &mut [u8]) -> Result<usize, Self::Error> {
        // Don't use PEC for these types of registers, because we don't know the size of the data.
        self.read_with_retries(&[address], buf, false).await.map(|()| buf.len())
    }

    async fn write(&mut self, address: Self::AddressType, buf: &[u8]) -> Result<usize, Self::Error> {
        if buf.len() > LARGEST_BUF_SIZE_BYTES {
            return Err(BQ40Z50Error::DataTooLarge);
        }

        // Add one byte for register address
        let mut data = [0u8; 1 + LARGEST_BUF_SIZE_BYTES];
        data[0] = address;
        data[1..=buf.len()].copy_from_slice(buf);

        self.write_with_retries(&data[..=buf.len()], self.config.pec_write)
            .await
            .map(|()| buf.len())
    }

    fn flush(&mut self, _address: Self::AddressType) -> impl Future<Output = Result<(), Self::Error>> {
        ready(Ok(()))
    }
}
