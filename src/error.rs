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
            Self::Timeout | Self::Pec | Self::DataTooLarge | Self::CapacityModeMismatch => {
                embedded_batteries_async::smart_battery::ErrorKind::Other
            }
        }
    }
}
