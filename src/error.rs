#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
/// BQ40Z50 Errors
pub enum BQ40Z50Error<I2cError> {
    I2c(I2cError),
    BatteryStatus(embedded_batteries_async::smart_battery::ErrorCode),
    Timeout,
    Pec,
    DataTooLarge,
    /// The unit tag of the supplied value disagrees with the capacity mode currently
    /// latched in `BatteryMode()[CAPACITY_MODE]`.
    CapacityModeMismatch,
    /// The requested data flash transfer does not lie entirely inside the documented
    /// `0x4000`-`0x5FFF` data flash window.
    ///
    /// Returned when the starting address is outside the window, or when the starting
    /// address is inside it but the transfer would run past the end. No bus traffic is
    /// generated for a rejected transfer.
    DataFlashAddressOutOfRange,
}

#[cfg(feature = "embassy-timeout")]
impl<I2cError> From<embassy_time::TimeoutError> for BQ40Z50Error<I2cError> {
    fn from(_value: embassy_time::TimeoutError) -> Self {
        BQ40Z50Error::Timeout
    }
}

impl<E: embedded_hal_async::i2c::Error> embedded_batteries_async::smart_battery::Error for BQ40Z50Error<E> {
    fn kind(&self) -> embedded_batteries_async::smart_battery::ErrorKind {
        match self {
            Self::I2c(_) => embedded_batteries_async::smart_battery::ErrorKind::CommError,
            Self::BatteryStatus(e) => embedded_batteries_async::smart_battery::ErrorKind::BatteryStatus(*e),
            Self::Timeout
            | Self::Pec
            | Self::DataTooLarge
            | Self::CapacityModeMismatch
            | Self::DataFlashAddressOutOfRange => embedded_batteries_async::smart_battery::ErrorKind::Other,
        }
    }
}
