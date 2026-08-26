#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
/// Configurable settings
pub struct Config {
    /// Max number of bus retries if an error occurs.
    pub max_bus_retries: usize,
    /// Verify data using PEC byte when reading.
    pub pec_read: bool,
    /// Append PEC byte when writing.
    pub pec_write: bool,
    #[cfg(feature = "embassy-timeout")]
    /// Timeout time
    pub timeout: embassy_time::Duration,
}

impl Config {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            max_bus_retries: crate::consts::DEFAULT_BUS_RETRIES,
            pec_read: false,
            pec_write: false,
            #[cfg(feature = "embassy-timeout")]
            timeout: crate::consts::DEFAULT_TIMEOUT,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum CapacityModeState {
    Milliamps = 0,
    Centiwatt = 1,
}

#[allow(clippy::struct_field_names)]
#[cfg(any(feature = "r3", feature = "r4", feature = "r5"))]
/// Charging Voltage Override config struct used in MAC command 0x00B0, not used in R1
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ChargingVoltageOverride {
    pub low_temp_chrg_mv: u16,
    pub std_low_temp_chrg_mv: u16,
    pub std_hi_temp_chrg_mv: u16,
    pub hi_temp_chrg_mv: u16,
    pub recommended_temp_chrg_mv: u16,
}

macro_rules! implement_embedded_batteries {
    ($impl_type:ident) => {
        use embedded_batteries_async::smart_battery::{
            self, BatteryModeFields, BatteryStatusFields, CapacityModeSignedValue, CapacityModeValue, DeciKelvin,
            ErrorCode, SpecificationInfoFields,
        };

        impl From<BatteryStatus> for BatteryStatusFields {
            fn from(value: BatteryStatus) -> Self {
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

        impl From<BatteryMode> for BatteryModeFields {
            fn from(value: BatteryMode) -> Self {
                let battery_mode_raw: [u8; 2] = value.into();
                u16::from_le_bytes(battery_mode_raw).into()
            }
        }

        impl From<BatteryModeFields> for BatteryMode {
            fn from(value: BatteryModeFields) -> Self {
                u16::from(value).to_le_bytes().into()
            }
        }

        impl From<SpecificationInfo> for SpecificationInfoFields {
            fn from(value: SpecificationInfo) -> Self {
                let spec_info_raw: [u8; 2] = value.into();
                u16::from_le_bytes(spec_info_raw).into()
            }
        }

        impl<I2C: I2cTrait, DELAY: DelayTrait> smart_battery::ErrorType for $impl_type<I2C, DELAY> {
            type Error = BQ40Z50Error<I2C::Error>;
        }

        impl<I2C: I2cTrait, DELAY: DelayTrait> smart_battery::SmartBattery for $impl_type<I2C, DELAY> {
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
                            CapacityModeValue::MilliAmpUnsigned(value)
                            | CapacityModeValue::CentiWattUnsigned(value) => value,
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
                self.device
                    .battery_mode()
                    .write_async(|f| *f = flags.into())
                    .await
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
                Ok(!matches!(
                    self.device.at_rate_ok().read_async().await?.at_rate_ok(),
                    0
                ))
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

            #[allow(clippy::cast_possible_truncation)]
            async fn max_error(&mut self) -> Result<smart_battery::Percent, Self::Error> {
                // Even though the datasheet of the fuel gauge says the data is 1 byte, through PEC testing the fuel gauge
                // actually returns 2 bytes. The rage of the data is 0 - 100 so it wll never exceed 1 byte, but in order to
                // receive the PEC byte we need to read 2 bytes of data.
                Ok(self.device.max_error().read_async().await?.max_error() as u8)
            }

            #[allow(clippy::cast_possible_truncation)]
            async fn relative_state_of_charge(&mut self) -> Result<smart_battery::Percent, Self::Error> {
                // Even though the datasheet of the fuel gauge says the data is 1 byte, through PEC testing the fuel gauge
                // actually returns 2 bytes. The rage of the data is 0 - 100 so it wll never exceed 1 byte, but in order to
                // receive the PEC byte we need to read 2 bytes of data.
                Ok(self
                    .device
                    .relative_state_of_charge()
                    .read_async()
                    .await?
                    .relative_state_of_charge() as u8)
            }

            #[allow(clippy::cast_possible_truncation)]
            async fn absolute_state_of_charge(&mut self) -> Result<smart_battery::Percent, Self::Error> {
                // Even though the datasheet of the fuel gauge says the data is 1 byte, through PEC testing the fuel gauge
                // actually returns 2 bytes. The rage of the data is 0 - 100 so it wll never exceed 1 byte, but in order to
                // receive the PEC byte we need to read 2 bytes of data.
                Ok(self
                    .device
                    .absolute_state_of_charge()
                    .read_async()
                    .await?
                    .absolute_state_of_charge() as u8)
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
                Ok(self
                    .device
                    .run_time_to_empty()
                    .read_async()
                    .await?
                    .run_time_to_empty())
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
                let status: smart_battery::BatteryStatusFields =
                    self.device.battery_status().read_async().await?.into();

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
                        self.device
                            .design_capacity()
                            .read_async()
                            .await?
                            .design_capacity(),
                    ),
                    CapacityModeState::Centiwatt => smart_battery::CapacityModeValue::CentiWattUnsigned(
                        self.device
                            .design_capacity()
                            .read_async()
                            .await?
                            .design_capacity(),
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
                self.device
                    .device_chemistry()
                    .read_async(chemistry)
                    .await
                    .map(|_f| ())
            }

            async fn charging_current(&mut self) -> Result<embedded_batteries_async::charger::MilliAmps, Self::Error> {
                Ok(self
                    .device
                    .charging_current()
                    .read_async()
                    .await?
                    .charging_current())
            }

            async fn charging_voltage(&mut self) -> Result<smart_battery::MilliVolts, Self::Error> {
                Ok(self
                    .device
                    .charging_voltage()
                    .read_async()
                    .await?
                    .charging_voltage())
            }
        }
    };
}

pub(crate) use implement_embedded_batteries;
