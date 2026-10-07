pub(crate) const LARGEST_REG_SIZE_BYTES: usize = 32;
pub(crate) const LARGEST_CMD_SIZE_BYTES: usize = 32;
pub(crate) const LARGEST_BUF_SIZE_BYTES: usize = 33;
pub(crate) const LARGEST_DF_BLOCK_SIZE_BYTES: usize = 32;

pub(crate) const BQ_ADDR: u8 = 0x0B;
pub(crate) const MAC_CMD_ADDR_SIZE_BYTES: u8 = 2;
pub(crate) const MAC_CMD_ADDR_SIZE_BITS: u8 = MAC_CMD_ADDR_SIZE_BYTES * 8;
pub(crate) const MAC_CMD: u8 = 0x44;

/// First address of the data flash window.
///
/// All four TRMs title the data flash section with the window itself: SLUUA43A 12.1.60
/// `"0x4000-0x5FFF Data Flash Access()"`, SLUUBU5A 15.1.83, SLUUCH2 16.1.98 and SLUUCN4B
/// 16.1.101 `"ManufacturerAccess() 0x4000-0x5FFF DataFlashAccess"`.
pub(crate) const DF_FIRST_ADDRESS: u16 = 0x4000;
/// Last valid (inclusive) address of the data flash window.
pub(crate) const DF_LAST_ADDRESS: u16 = 0x5FFF;

// Special case MAC commands
pub(crate) const SECURITY_KEYS_CMD: [u8; MAC_CMD_ADDR_SIZE_BYTES as usize] = 0x0035u16.to_le_bytes();

// The Security Keys block gained a key pair at each silicon revision, so its length is
// revision specific. Each value below is the byte count of the worked example in that
// revision's TRM, counted after the two `35 00` command bytes.

/// R1: UNSEAL and FULL ACCESS keys.
///
/// SLUUA43A 12.1.33 `"= 35 00 34 12 78 56 FF FF FF FF"`.
#[cfg(feature = "r1")]
pub(crate) const SECURITY_KEYS_R1_DATA_LEN_BYTES: u8 = 8;

/// R3: adds the Manual PF and Lifetimes Reset keys.
///
/// SLUUBU5A 15.1.33 `"= 35 00 34 12 78 56 FF FF FF FF 57 28 98 2A 14 2B 8A 2C"`.
#[cfg(feature = "r3")]
pub(crate) const SECURITY_KEYS_R3_DATA_LEN_BYTES: u8 = 16;

/// R4: adds the DF Read Only and Override keys.
///
/// SLUUCH2 16.1.33
/// `"= 35 00 34 12 78 56 FF FF FF FF 32 76 12 17 57 28 98 2A 14 2B 8A 2C 18 2C 9B 2E"`.
#[cfg(feature = "r4")]
pub(crate) const SECURITY_KEYS_R4_DATA_LEN_BYTES: u8 = 24;

/// R5: adds the `MfgInfoC` Write key.
///
/// SLUUCN4B 16.1.34
/// `"= 35 00 34 12 78 56 FF FF FF FF 32 76 12 17 57 28 98 2A 14 2B 8A 2C 18 2D 9B 2E 45 3C 89 5D"`.
#[cfg(feature = "r5")]
pub(crate) const SECURITY_KEYS_R5_DATA_LEN_BYTES: u8 = 28;

pub(crate) const AUTH_KEY_CMD: [u8; MAC_CMD_ADDR_SIZE_BYTES as usize] = 0x0037u16.to_le_bytes();
/// The authentication key is 128 bits on every revision: SLUUA43A 12.1.34, SLUUBU5A 15.1.34,
/// SLUUCH2 16.1.34 and SLUUCN4B 16.1.35 all say `"Send the AuthenticationKey() + the new 128-bit
/// authentication key to ManufacturerBlockAccess()"`.
pub(crate) const AUTH_KEY_DATA_LEN_BYTES: u8 = 16;

pub(crate) const MFG_INFO_CMD: u8 = 0x70;

#[cfg(not(all(feature = "r1", not(any(feature = "r3", feature = "r4", feature = "r5")))))]
pub(crate) const CHRG_VOLTAGE_OVERRIDE_CMD: [u8; MAC_CMD_ADDR_SIZE_BYTES as usize] = 0x00B0u16.to_le_bytes();
#[cfg(not(all(feature = "r1", not(any(feature = "r3", feature = "r4", feature = "r5")))))]
pub(crate) const CHRG_VOLTAGE_OVERRIDE_SIZE_BYTES: u8 = 10;

pub(crate) const DEFAULT_BUS_RETRIES: usize = 3;
pub(crate) const DEFAULT_ERROR_BACKOFF_DELAY_MS: u32 = 10;
#[cfg(feature = "embassy-timeout")]
pub(crate) const DEFAULT_TIMEOUT: Duration = Duration::from_millis(100);

#[cfg(feature = "embassy-timeout")]
use embassy_time::Duration;
