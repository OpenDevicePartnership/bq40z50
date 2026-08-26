macro_rules! bq40z50_tests {
    ($revision:ident) => {
        #[cfg(test)]
        mod tests {
            use embedded_batteries_async::smart_battery::SmartBattery;
            use embedded_hal_mock::eh1::delay::{CheckedDelay, NoopDelay, Transaction as DelayTransaction};
            use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
            use $revision as Bq40z50;

            use super::*;
            use crate::common::{CapacityModeState, Config};
            use crate::consts::BQ_ADDR;

            // Needed to compile in the pender symbol for embassy-time.
            // This is only enabled during tests and when actually using the driver,
            // the user must provide embassy-time symbols.
            #[cfg(feature = "embassy-timeout")]
            #[allow(dead_code)]
            fn setup() -> &'static mut embassy_executor::Executor {
                static EXECUTOR: static_cell::StaticCell<embassy_executor::Executor> = static_cell::StaticCell::new();
                EXECUTOR.init(embassy_executor::Executor::new())
            }

            #[tokio::test]
            async fn update_config() {
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x02, 0x00, 0x46]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x0A, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0,
                        ],
                    ),
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
                let mut bq = Bq40z50::new_with_config(
                    i2c,
                    NoopDelay::new(),
                    Config {
                        pec_read: true,
                        ..Default::default()
                    },
                );

                bq.device
                    .mac_firmware_version()
                    .dispatch_out_async()
                    .await
                    .unwrap();

                // Change the device config to not use PEC.
                let mut config = bq.config();
                config.pec_read = false;
                bq.update_config(config);
                bq.device
                    .mac_firmware_version()
                    .dispatch_out_async()
                    .await
                    .unwrap();

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn read_chip_id() {
                let expectations = vec![Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x21, 0x00])];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                bq.mac_gauging().dispatch_async().await.unwrap();

                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn read_chip_id_pec() {
                let expectations = vec![Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x21, 0x00, 0xD7])];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new_with_config(
                    i2c,
                    NoopDelay::new(),
                    Config {
                        pec_write: true,
                        ..Default::default()
                    },
                ));

                bq.mac_gauging().dispatch_async().await.unwrap();

                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn read_chip_id_2() {
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x01, 0x00]),
                    Transaction::write_read(BQ_ADDR, vec![0x44], vec![0x04, 0x01, 0x00, 0x00, 0x00]),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                bq.mac_device_type().dispatch_out_async().await.unwrap();
                bq.interface().i2c.done();
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
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                bq.mac_firmware_version().dispatch_out_async().await.unwrap();
                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn read_firmware_version_pec() {
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x02, 0x00, 0x46]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x0A, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF0,
                        ],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new_with_config(
                    i2c,
                    NoopDelay::new(),
                    Config {
                        pec_read: true,
                        ..Default::default()
                    },
                ));

                bq.mac_firmware_version().dispatch_out_async().await.unwrap();
                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn read_too_large_manufacture_name() {
                let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x20], vec![0x00])];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                let mut manufacture_name = [0u8; 1];

                bq.manufacture_name()
                    .read_async(&mut manufacture_name)
                    .await
                    .unwrap();
                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn write_unseal_keys() {
                let expectations = vec![
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 0x0A, 0x35, 0x00, 0x30, 0x30, 0x60, 0x60, 0x01, 0x01, 0x10, 0x10,
                        ],
                    ),
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x35, 0x00]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![0x0A, 0x35, 0x00, 0x30, 0x30, 0x60, 0x60, 0x01, 0x01, 0x10, 0x10],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                let security_keys = [0x30u8, 0x30u8, 0x60u8, 0x60u8, 0x01u8, 0x01u8, 0x10u8, 0x10u8];

                bq.write_security_keys(&security_keys).await.unwrap();

                let mut result = [0u8; 8];
                bq.read_security_keys(&mut result).await.unwrap();

                assert_eq!(u16::from_le_bytes([result[0], result[1]]), 0x3030);
                assert_eq!(u16::from_le_bytes([result[2], result[3]]), 0x6060);
                assert_eq!(u16::from_le_bytes([result[4], result[5]]), 0x0101);
                assert_eq!(u16::from_le_bytes([result[6], result[7]]), 0x1010);
                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_battery_status() {
                let expectations = vec![Transaction::write_read(BQ_ADDR, vec![0x16], vec![0x30, 0x30])];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                let status = match bq.battery_status().await {
                    Ok(status) => status,
                    Err(e) => match e {
                        _ => unreachable!(),
                    },
                };

                assert_eq!(status.error_code(), ErrorCode::Ok);

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_battery_status_pec() {
                // Full bus transaction looks like [ 0x0B << 1 || 0x16 || 0x0B << 1 | 1 || 0x30 || 0x30 || 0xB7 (PEC)]
                let expectations = vec![Transaction::write_read(
                    BQ_ADDR,
                    vec![0x16],
                    vec![0x30, 0x30, 0xB7],
                )];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new_with_config(
                    i2c,
                    NoopDelay::new(),
                    Config {
                        pec_read: true,
                        ..Default::default()
                    },
                );

                let status = match bq.battery_status().await {
                    Ok(status) => status,
                    Err(e) => match e {
                        _ => unreachable!(),
                    },
                };

                assert_eq!(status.error_code(), ErrorCode::Ok);

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            #[allow(unsafe_code)]
            async fn test_read_write_unchecked() {
                let expectations = vec![
                    Transaction::write_read(BQ_ADDR, vec![0x16], vec![0x30, 0x30]),
                    Transaction::write(BQ_ADDR, vec![0x16, 0x2F, 0x30]),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                let mut data = [0u8; 2];

                unsafe {
                    bq.read_register_unchecked(0x16, &mut data).await.unwrap();
                }
                data[0] -= 1;

                unsafe {
                    bq.write_register_unchecked(0x16, &data).await.unwrap();
                }

                bq.device.interface().i2c.done();
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
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Milliamps);

                let mode = BatteryModeFields::new().with_capacity_mode(true);
                bq.set_battery_mode(mode).await.unwrap();
                assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Centiwatt);

                let mode = BatteryModeFields::new().with_capacity_mode(false);
                let rem_cap = bq.remaining_capacity().await.unwrap();
                assert!(matches!(rem_cap, CapacityModeValue::CentiWattUnsigned(100)));

                let _info = bq.set_battery_mode(mode).await;
                assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Milliamps);

                let rem_cap = bq.remaining_capacity().await.unwrap();
                assert!(matches!(rem_cap, CapacityModeValue::MilliAmpUnsigned(80)));

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_reg_retries() {
                // Should have 3 retries
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x17, 100, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x17, 100, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x17, 100, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x17, 100, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let delay_expectations = vec![
                    DelayTransaction::delay_ms(10),
                    DelayTransaction::delay_ms(10),
                    DelayTransaction::delay_ms(10),
                ];
                let mut bq = Bq40z50::new(i2c, CheckedDelay::new(&delay_expectations));

                let res = bq
                    .device
                    .cycle_count()
                    .write_async(|f| f.set_cycle_count(100))
                    .await;

                assert_eq!(
                    res,
                    Err(BQ40Z50Error::I2c(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                        embedded_hal::i2c::NoAcknowledgeSource::Address
                    )))
                );

                bq.device.interface().i2c.done();
                bq.device.interface().delay.done();
            }

            #[tokio::test]
            async fn test_cmd_retries() {
                // Should have 3 retries
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 2, 0x53, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x44, 2, 0x53, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x44, 2, 0x53, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x44, 2, 0x53, 0]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let delay_expectations = vec![
                    DelayTransaction::delay_ms(10),
                    DelayTransaction::delay_ms(10),
                    DelayTransaction::delay_ms(10),
                ];
                let mut bq = Bq40z50::new(i2c, CheckedDelay::new(&delay_expectations));

                let res = bq.device.mac_pf_status().dispatch_out_async().await;

                assert_eq!(
                    res,
                    Err(BQ40Z50Error::I2c(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                        embedded_hal::i2c::NoAcknowledgeSource::Address
                    )))
                );

                bq.device.interface().i2c.done();
                bq.device.interface().delay.done();
            }

            #[cfg(not(feature = "r1"))]
            #[tokio::test]
            async fn test_charging_override_voltage() {
                let expectations = vec![
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 0x0C, 0xB0, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E,
                        ],
                    ),
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0xB0, 0x00]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x0C, 0xB0, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E,
                        ],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                bq.write_charging_voltage_override(&ChargingVoltageOverride {
                    low_temp_chrg_mv: 11800,
                    std_low_temp_chrg_mv: 12000,
                    std_hi_temp_chrg_mv: 12600,
                    hi_temp_chrg_mv: 12000,
                    recommended_temp_chrg_mv: 11800,
                })
                .await
                .unwrap();

                let override_struct = bq.read_charging_voltage_override().await.unwrap();

                assert_eq!(override_struct.low_temp_chrg_mv, 11800);
                assert_eq!(override_struct.std_low_temp_chrg_mv, 12000);
                assert_eq!(override_struct.std_hi_temp_chrg_mv, 12600);
                assert_eq!(override_struct.hi_temp_chrg_mv, 12000);
                assert_eq!(override_struct.recommended_temp_chrg_mv, 11800);
                bq.device.interface().i2c.done();
            }

            #[cfg(not(any(feature = "r1", feature = "r3")))]
            #[tokio::test]
            async fn test_read_mfg_info_c() {
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x7B, 0x00]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x7B, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E,
                            0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                            0x2E, 0x18, 0x2E, 0x44, 0x32,
                        ],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new_with_config(i2c, NoopDelay::new(), Config { ..Default::default() });

                let mut buf = [0u8; 32];
                bq.read_mfg_info_c(&mut buf).await.unwrap();

                bq.device.interface().i2c.done();
            }

            #[cfg(not(any(feature = "r1", feature = "r3")))]
            #[tokio::test]
            async fn test_read_mfg_info_c_pec() {
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x7B, 0x00, 89]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x7B, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E,
                            0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                            0x2E, 0x18, 0x2E, 0x44, 0x32, 0xD4,
                        ],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new_with_config(
                    i2c,
                    NoopDelay::new(),
                    Config {
                        pec_read: true,
                        pec_write: true,
                        ..Default::default()
                    },
                );

                let mut buf = [0u8; 32];
                bq.read_mfg_info_c(&mut buf).await.unwrap();

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_df_transactions() {
                let expectations = vec![
                    // Write 1, 4 bytes (1 block write)
                    Transaction::write(BQ_ADDR, vec![0x44, 0x06, 0x00, 0x40, 0xFE, 0xCA, 0xFE, 0xC0]),
                    // Write 2, 48 bytes (2 block writes)
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x00, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 18, 0x20, 0x40, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE,
                            0xC0, 0xFE, 0xCA, 0xFE, 0xC0,
                        ],
                    ),
                    // Write 3, 128 bytes (4 block writes)
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x00, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x20, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x40, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x60, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                        ],
                    ),
                    // Read 1, 4 bytes (1 block read)
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![0x22, 0x00, 0x40, 0x01, 0x02, 0x03, 0x04],
                    ),
                    // Read 2, 48 bytes (2 block reads)
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18,
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x20, 0x40, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD,
                            0xDE, 0xAD, 0xD0, 0x0D,
                        ],
                    ),
                    // Read 3, 128 bytes (4 block reads)
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18,
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18,
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18,
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18,
                        ],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                // Write 1
                let write = [0xFEu8, 0xCA, 0xFE, 0xC0];
                bq.write_dataflash(0x4000, &write).await.unwrap();

                // Write 2
                let write = [
                    0x03u8, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0,
                ];
                bq.write_dataflash(0x4000, &write).await.unwrap();

                // Write 3
                let write = [
                    0x03u8, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                ];
                bq.write_dataflash(0x4000, &write).await.unwrap();

                // Read 1
                let mut read = [0u8; 48];
                bq.read_dataflash(0x4000, &mut read[..4]).await.unwrap();
                assert_eq!(read[..4], [0x01, 0x02, 0x03, 0x04]);

                // Read 2
                bq.read_dataflash(0x4000, &mut read).await.unwrap();
                assert_eq!(
                    read,
                    [
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xD0, 0x0D
                    ]
                );

                // Read 3
                let mut read = [0u8; 128];
                bq.read_dataflash(0x4000, &mut read).await.unwrap();

                assert_eq!(
                    read,
                    [
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18
                    ]
                );

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_df_transactions_pec() {
                let expectations = vec![
                    // Write 1, 4 bytes (1 block write)
                    Transaction::write(
                        BQ_ADDR,
                        vec![0x44, 0x06, 0x00, 0x40, 0xFE, 0xCA, 0xFE, 0xC0, 0x41],
                    ),
                    // Write 2, 48 bytes (2 block writes)
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x00, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0xA2, // PEC
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 18, 0x20, 0x40, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE,
                            0xC0, 0xFE, 0xCA, 0xFE, 0xC0, 0x32, // PEC
                        ],
                    ),
                    // Write 3, 128 bytes (4 block writes)
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x00, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0xA2, // PEC
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x20, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x14, // PEC
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x40, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0xC9, // PEC
                        ],
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 34, 0x60, 0x40, 0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E,
                            0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                            0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x7F, // PEC
                        ],
                    ),
                    // PEC reads will always read in 32 byte data chunks.
                    // Read 1, 4 bytes (1 block read)
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40, 0xAB /* PEC */]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04,
                            0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03, 0x04, 0x01, 0x02, 0x03,
                            0x04, 0x01, 0x02, 0x03, 0x04, 0x44, /* PEC */
                        ],
                    ),
                    // Read 2, 48 bytes (2 block reads)
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40, 0xAB /* PEC */]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18, 0x22, // PEC
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x20, 0x40, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD,
                            0xDE, 0xAD, 0xD0, 0x0D, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE,
                            0xAD, 0xDE, 0xAD, 0xD0, 0x0D, 0xE0, // PEC
                        ],
                    ),
                    // Read 3, 128 bytes (4 block reads)
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40, 0xAB /* PEC */]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18, 0x22, // PEC
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18, 0x22, // PEC
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18, 0x22, // PEC
                        ],
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00,
                            0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                            0x38, 0x31, 0xE0, 0x2E, 0x18, 0x22, // PEC
                        ],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new_with_config(
                    i2c,
                    NoopDelay::new(),
                    Config {
                        pec_read: true,
                        pec_write: true,
                        ..Default::default()
                    },
                );

                // Write 1
                let write = [0xFEu8, 0xCA, 0xFE, 0xC0];
                bq.write_dataflash(0x4000, &write).await.unwrap();

                // Write 2
                let write = [
                    0x03u8, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0, 0xFE, 0xCA, 0xFE, 0xC0,
                ];
                bq.write_dataflash(0x4000, &write).await.unwrap();

                // Write 3
                let write = [
                    0x03u8, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                    0x03, 0x56, 0x01, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E,
                    0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0,
                ];
                bq.write_dataflash(0x4000, &write).await.unwrap();

                // Read 1
                let mut read = [0u8; 48];
                bq.read_dataflash(0x4000, &mut read[..4]).await.unwrap();
                assert_eq!(read[..4], [0x01, 0x02, 0x03, 0x04]);

                // Read 2
                bq.read_dataflash(0x4000, &mut read).await.unwrap();
                assert_eq!(
                    read,
                    [
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xD0, 0x0D
                    ]
                );

                // Read 3
                let mut read = [0u8; 128];
                bq.read_dataflash(0x4000, &mut read).await.unwrap();

                assert_eq!(
                    read,
                    [
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18,
                        0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E,
                        0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18
                    ]
                );

                bq.device.interface().i2c.done();
            }
        }
    };
}

pub(crate) use bq40z50_tests;
