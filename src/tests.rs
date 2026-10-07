macro_rules! bq40z50_tests {
    // `$security_keys_len` is the length of the Security Keys block for this revision, taken
    // from the worked example in that revision's TRM. It is passed in rather than read from
    // `consts` so the test pins the TRM value independently of the driver constant.
    //
    // `$lifetime_block_1_len` is the length of LIFETIME_DATA_BLOCK_1 for this revision. r1 and
    // r3 carry six extra temperature bytes at the end of the block. It is a macro parameter
    // rather than a `cfg` because this macro is expanded once per revision module while a `cfg`
    // is evaluated once for the whole build, so a multi-revision build would otherwise give
    // every module the same length.
    ($revision:ident, $security_keys_len:literal, $lifetime_block_1_len:literal) => {
        #[cfg(test)]
        mod tests {
            use device_driver::{
                AsyncBufferInterface, AsyncCommandInterface, AsyncRegisterInterface, FieldsetMetadata,
            };
            use embedded_batteries_async::smart_battery::SmartBattery;
            use embedded_hal_mock::eh1::delay::{CheckedDelay, NoopDelay, Transaction as DelayTransaction};
            use embedded_hal_mock::eh1::i2c::{Mock, Transaction};
            use $revision as Bq40z50;

            use super::*;
            use crate::common::{CapacityModeState, Config};
            use crate::consts::{BQ_ADDR, DEFAULT_BUS_RETRIES, DEFAULT_ERROR_BACKOFF_DELAY_MS};

            fn write_transaction(data: &[u8], use_pec: bool) -> Transaction {
                let mut frame = data.to_vec();
                if use_pec {
                    let preamble = [BQ_ADDR << 1];
                    frame.push(smbus_pec::pec(&[preamble.as_slice(), data].concat()));
                }
                Transaction::write(BQ_ADDR, frame)
            }

            fn read_transaction(address: u8, data: &[u8], use_pec: bool) -> Transaction {
                let mut frame = data.to_vec();
                if use_pec {
                    let preamble = [BQ_ADDR << 1, address, BQ_ADDR << 1 | 1];
                    frame.push(smbus_pec::pec(&[preamble.as_slice(), data].concat()));
                }
                Transaction::write_read(BQ_ADDR, vec![address], frame)
            }

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
            async fn flush_buffer_is_noop() {
                let mut interface = DeviceInterface::new(Mock::new(&[]), CheckedDelay::new(&[]));

                device_driver::AsyncBufferInterface::flush(&mut interface, 0x20)
                    .await
                    .unwrap();

                interface.i2c.done();
                interface.delay.done();
            }

            #[tokio::test]
            async fn test_buffer_write_lengths() {
                for use_pec in [false, true] {
                    for len in [0, 1, 3, 32, 33] {
                        let mut data = vec![0xA5; len];
                        let mut frame = vec![0x70];
                        frame.extend_from_slice(&data);
                        let expectations = [write_transaction(&frame, use_pec)];
                        let mut interface = DeviceInterface::new_with_config(
                            Mock::new(&expectations),
                            CheckedDelay::new(&[]),
                            Config {
                                pec_write: use_pec,
                                ..Default::default()
                            },
                        );

                        assert_eq!(
                            AsyncBufferInterface::write(&mut interface, 0x70, &data).await,
                            Ok(len)
                        );

                        interface.i2c.done();
                        interface.delay.done();
                    }
                }
            }

            #[tokio::test]
            async fn test_register_transfer_lengths() {
                for use_pec in [false, true] {
                    for len in [0, 1, 31, 32] {
                        let mut data = vec![0xA5; len];
                        let mut frame = vec![0x17];
                        frame.extend_from_slice(&data);
                        let expectations = [
                            write_transaction(&frame, use_pec),
                            read_transaction(0x17, &data, use_pec),
                        ];
                        let mut interface = DeviceInterface::new_with_config(
                            Mock::new(&expectations),
                            CheckedDelay::new(&[]),
                            Config {
                                pec_read: use_pec,
                                pec_write: use_pec,
                                ..Default::default()
                            },
                        );

                        AsyncRegisterInterface::write_register(
                            &mut interface,
                            0x17,
                            &mut data,
                            &FieldsetMetadata::DEFAULT,
                        )
                        .await
                        .unwrap();
                        let mut read = vec![0xCC; len];
                        AsyncRegisterInterface::read_register(
                            &mut interface,
                            0x17,
                            &mut read,
                            &FieldsetMetadata::DEFAULT,
                        )
                        .await
                        .unwrap();
                        assert_eq!(read, data);

                        interface.i2c.done();
                        interface.delay.done();
                    }
                }
            }

            #[tokio::test]
            async fn test_command_read_maximum_length() {
                for use_pec in [false, true] {
                    let response = [vec![0x22, 0x70, 0x00], vec![0x5A; 32]].concat();
                    let expectations = [
                        write_transaction(&[0x44, 0x02, 0x70, 0x00], use_pec),
                        read_transaction(0x44, &response, use_pec),
                    ];
                    let mut interface = DeviceInterface::new_with_config(
                        Mock::new(&expectations),
                        CheckedDelay::new(&[]),
                        Config {
                            pec_read: use_pec,
                            ..Default::default()
                        },
                    );
                    let mut data = [0xCC; 32];

                    AsyncCommandInterface::dispatch_command(
                        &mut interface,
                        0x447000,
                        &mut [],
                        &FieldsetMetadata::DEFAULT,
                        &mut data,
                        &FieldsetMetadata::DEFAULT,
                    )
                    .await
                    .unwrap();
                    assert_eq!(data, [0x5A; 32]);

                    interface.i2c.done();
                    interface.delay.done();
                }
            }

            #[tokio::test]
            async fn test_oversized_trait_buffers() {
                for use_pec in [false, true] {
                    let mut interface = DeviceInterface::new_with_config(
                        Mock::new(&[]),
                        CheckedDelay::new(&[]),
                        Config {
                            pec_read: use_pec,
                            pec_write: use_pec,
                            ..Default::default()
                        },
                    );
                    for len in [33, 64] {
                        let mut data = vec![0xA5; len];
                        let mut output = data.clone();

                        assert_eq!(
                            AsyncRegisterInterface::write_register(
                                &mut interface,
                                0x17,
                                &mut data,
                                &FieldsetMetadata::DEFAULT
                            )
                            .await,
                            Err(BQ40Z50Error::DataTooLarge)
                        );
                        assert_eq!(
                            AsyncRegisterInterface::read_register(
                                &mut interface,
                                0x17,
                                &mut output,
                                &FieldsetMetadata::DEFAULT
                            )
                            .await,
                            Err(BQ40Z50Error::DataTooLarge)
                        );
                        assert_eq!(
                            AsyncCommandInterface::dispatch_command(
                                &mut interface,
                                0x447000,
                                &mut [],
                                &FieldsetMetadata::DEFAULT,
                                &mut output,
                                &FieldsetMetadata::DEFAULT,
                            )
                            .await,
                            Err(BQ40Z50Error::DataTooLarge)
                        );
                        assert_eq!(
                            AsyncCommandInterface::dispatch_command(
                                &mut interface,
                                0x447000,
                                &mut data,
                                &FieldsetMetadata::DEFAULT,
                                &mut [],
                                &FieldsetMetadata::DEFAULT,
                            )
                            .await,
                            Err(BQ40Z50Error::DataTooLarge)
                        );
                        assert_eq!(output, data);
                    }
                    for len in [34, 64] {
                        assert_eq!(
                            AsyncBufferInterface::write(&mut interface, 0x70, &vec![0xA5; len]).await,
                            Err(BQ40Z50Error::DataTooLarge)
                        );
                    }

                    interface.i2c.done();
                    interface.delay.done();
                }
            }

            #[tokio::test]
            async fn test_buffer_reads_keep_caller_length() {
                for len in [0, 1, 33, 64] {
                    let data = vec![0x5A; len];
                    let expectations = [read_transaction(0x20, &data, false)];
                    let mut interface = DeviceInterface::new_with_config(
                        Mock::new(&expectations),
                        CheckedDelay::new(&[]),
                        Config {
                            pec_read: true,
                            ..Default::default()
                        },
                    );
                    let mut read = vec![0xCC; len];

                    assert_eq!(
                        AsyncBufferInterface::read(&mut interface, 0x20, &mut read).await,
                        Ok(len)
                    );
                    assert_eq!(read, data);

                    interface.i2c.done();
                    interface.delay.done();
                }
            }

            #[tokio::test]
            async fn test_mfg_info_read_lengths() {
                let data = [vec![32], vec![0x5A; 32]].concat();
                for use_pec in [false, true] {
                    for len in [0, 1, 32, 33] {
                        let response = if use_pec { &data[..] } else { &data[..len] };
                        let expectations = [read_transaction(0x70, response, use_pec)];
                        let mut bq = Bq40z50::new_with_config(
                            Mock::new(&expectations),
                            CheckedDelay::new(&[]),
                            Config {
                                pec_read: use_pec,
                                ..Default::default()
                            },
                        );
                        let mut read = vec![0xCC; len];

                        bq.read_mfg_info(&mut read).await.unwrap();
                        assert_eq!(read, data[..len]);

                        bq.device.interface().i2c.done();
                        bq.device.interface().delay.done();
                    }
                }
            }

            #[tokio::test]
            async fn test_calibration_stop_aliases() {
                for use_pec in [false, true] {
                    let stop = write_transaction(&[0x44, 0x02, 0x80, 0xF0], use_pec);
                    let expectations = [stop.clone(), stop.clone(), stop];
                    let mut bq = Bq40z50::new_with_config(
                        Mock::new(&expectations),
                        CheckedDelay::new(&[]),
                        Config {
                            pec_write: use_pec,
                            ..Default::default()
                        },
                    );

                    bq.device
                        .mac_exit_calibration_output_mode()
                        .dispatch_async()
                        .await
                        .unwrap();
                    bq.device
                        .mac_stop_output_ccadc_cal()
                        .dispatch_async()
                        .await
                        .unwrap();
                    bq.device
                        .mac_stop_output_shorted_ccadc_cal()
                        .dispatch_async()
                        .await
                        .unwrap();

                    bq.device.interface().i2c.done();
                    bq.device.interface().delay.done();
                }
            }

            #[tokio::test]
            async fn test_calibration_enable_commands() {
                for use_pec in [false, true] {
                    let normal = [vec![0x1A, 0x81, 0xF0], vec![0x5A; 24]].concat();
                    let shorted = [vec![0x1A, 0x82, 0xF0], vec![0xA5; 24]].concat();
                    let expectations = [
                        write_transaction(&[0x44, 0x02, 0x81, 0xF0], use_pec),
                        read_transaction(0x44, &normal, use_pec),
                        write_transaction(&[0x44, 0x02, 0x82, 0xF0], use_pec),
                        read_transaction(0x44, &shorted, use_pec),
                    ];
                    let mut bq = Bq40z50::new_with_config(
                        Mock::new(&expectations),
                        CheckedDelay::new(&[]),
                        Config {
                            pec_read: use_pec,
                            pec_write: use_pec,
                            ..Default::default()
                        },
                    );

                    let normal: [u8; 24] = bq
                        .device
                        .mac_output_ccadc_cal()
                        .dispatch_out_async()
                        .await
                        .unwrap()
                        .into();
                    let shorted: [u8; 24] = bq
                        .device
                        .mac_output_shorted_ccadc_cal()
                        .dispatch_out_async()
                        .await
                        .unwrap()
                        .into();
                    assert_eq!(normal, [0x5A; 24]);
                    assert_eq!(shorted, [0xA5; 24]);

                    bq.device.interface().i2c.done();
                    bq.device.interface().delay.done();
                }
            }

            #[tokio::test]
            async fn test_dataflash_partial_write_stops_after_failed_chunk() {
                for use_pec in [false, true] {
                    let first = [vec![0x44, 0x22, 0x00, 0x40], vec![0x11; 32]].concat();
                    let second = [vec![0x44, 0x22, 0x20, 0x40], vec![0x22; 32]].concat();
                    let error =
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address);
                    let failed_write = write_transaction(&second, use_pec).with_error(error);
                    let expectations = [
                        write_transaction(&first, use_pec),
                        failed_write.clone(),
                        failed_write,
                    ];
                    let delays = [DelayTransaction::delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS)];
                    let mut bq = Bq40z50::new_with_config(
                        Mock::new(&expectations),
                        CheckedDelay::new(&delays),
                        Config {
                            max_bus_retries: 1,
                            pec_write: use_pec,
                            ..Default::default()
                        },
                    );
                    let data = [[0x11; 32], [0x22; 32], [0x33; 32]].concat();

                    // The first chunk was committed before the second one failed, so the error
                    // reports that progress rather than the underlying NAK.
                    assert_eq!(
                        bq.write_dataflash(0x4000, &data).await,
                        Err(BQ40Z50Error::PartialDataFlashWrite { committed: 32 })
                    );

                    bq.device.interface().i2c.done();
                    bq.device.interface().delay.done();
                }
            }

            #[tokio::test]
            async fn write_unseal_keys() {
                // The Security Keys block grew with each silicon revision, so its length is taken
                // from the worked example in this revision's TRM section: SLUUA43A 12.1.33,
                // SLUUBU5A 15.1.33, SLUUCH2 16.1.33 or SLUUCN4B 16.1.34.
                const LEN: usize = $security_keys_len;

                let security_keys: [u8; LEN] = core::array::from_fn(|i| u8::try_from(i).unwrap());

                // [ 0x44 | 2 command bytes + key bytes | 0x35 | 0x00 | keys ]
                let mut write_frame = vec![0x44, u8::try_from(LEN + 2).unwrap(), 0x35, 0x00];
                write_frame.extend_from_slice(&security_keys);

                // [ 2 command bytes + key bytes | 0x35 | 0x00 | keys ]
                let mut read_frame = vec![u8::try_from(LEN + 2).unwrap(), 0x35, 0x00];
                read_frame.extend_from_slice(&security_keys);

                let expectations = vec![
                    Transaction::write(BQ_ADDR, write_frame),
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x35, 0x00]),
                    Transaction::write_read(BQ_ADDR, vec![0x44], read_frame),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                bq.write_security_keys(&security_keys).await.unwrap();

                let mut result = [0u8; LEN];
                bq.read_security_keys(&mut result).await.unwrap();

                assert_eq!(result, security_keys);
                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn write_authentication_key() {
                // The authentication key is 128 bits, i.e. 16 bytes, on every revision:
                // SLUUA43A 12.1.34, SLUUBU5A 15.1.34, SLUUCH2 16.1.34 and SLUUCN4B 16.1.35.
                let auth_key: [u8; 16] = core::array::from_fn(|i| u8::try_from(i).unwrap());

                // [ 0x44 | 0x12 (18 = 2 command bytes + 16 key bytes) | 0x37 | 0x00 | 16 key bytes ]
                let mut write_frame = vec![0x44, 0x12, 0x37, 0x00];
                write_frame.extend_from_slice(&auth_key);
                // The length byte must agree with what actually goes on the wire after it.
                assert_eq!(write_frame.len(), 20);

                let expectations = vec![Transaction::write(BQ_ADDR, write_frame)];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                bq.write_authentication_key(&auth_key).await.unwrap();

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
                assert!(status.fully_discharged());
                assert!(status.fully_charged());
                assert!(status.over_temp_alarm());
                assert!(!status.discharging());

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
                assert!(status.fully_discharged());
                assert!(status.fully_charged());
                assert!(status.over_temp_alarm());
                assert!(!status.discharging());

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_battery_status_pec_retry_resets_accumulator() {
                let expectations = vec![
                    Transaction::write_read(BQ_ADDR, vec![0x16], vec![0x40, 0x00, 0xFF]),
                    Transaction::write_read(BQ_ADDR, vec![0x16], vec![0x40, 0x00, 0x85]),
                ];
                let delay_expectations = vec![DelayTransaction::delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS)];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new_with_config(
                    i2c,
                    CheckedDelay::new(&delay_expectations),
                    Config {
                        pec_read: true,
                        ..Default::default()
                    },
                );

                let status = bq.battery_status().await.unwrap();

                assert_eq!(status.error_code(), ErrorCode::Ok);
                assert!(status.discharging());
                bq.device.interface().i2c.done();
                bq.device.interface().delay.done();
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
            async fn test_capacity_mode_not_cached_on_failed_write() {
                // SLUUCN4B 16.4 0x03 BatteryMode(): the reporting unit follows CAPM as latched in the
                // part. A write that never completes latches nothing, so the cache must not move.
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x80]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x80]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x80]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x80]).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    Transaction::write_read(BQ_ADDR, vec![0x0F], vec![80, 0x00]),
                ];
                let i2c = Mock::new(&expectations);
                let delay_expectations = vec![
                    DelayTransaction::delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS),
                    DelayTransaction::delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS),
                    DelayTransaction::delay_ms(DEFAULT_ERROR_BACKOFF_DELAY_MS),
                ];
                let mut bq = Bq40z50::new(i2c, CheckedDelay::new(&delay_expectations));

                assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Milliamps);

                let mode = BatteryModeFields::new().with_capacity_mode(true);
                let res = bq.set_battery_mode(mode).await;
                assert_eq!(
                    res,
                    Err(BQ40Z50Error::I2c(embedded_hal::i2c::ErrorKind::NoAcknowledge(
                        embedded_hal::i2c::NoAcknowledgeSource::Address
                    )))
                );

                // The write failed, so the part is still in mA mode and so is the cache.
                assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Milliamps);
                let rem_cap = bq.remaining_capacity().await.unwrap();
                assert!(matches!(rem_cap, CapacityModeValue::MilliAmpUnsigned(80)));

                bq.device.interface().i2c.done();
                bq.device.interface().delay.done();
            }

            #[tokio::test]
            async fn test_battery_mode_read_refreshes_capacity_mode() {
                // SLUUCN4B 16.4 0x03 BatteryMode(): a read returns the authoritative CAPM, so the
                // cached reporting unit must follow it.
                let expectations = vec![
                    Transaction::write_read(BQ_ADDR, vec![0x03], vec![0x00, 0x80]),
                    Transaction::write_read(BQ_ADDR, vec![0x0F], vec![100, 0x00]),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Milliamps);

                let mode = bq.battery_mode().await.unwrap();
                assert!(mode.capacity_mode());
                assert_eq!(bq.capacity_mode_state.get(), CapacityModeState::Centiwatt);

                let rem_cap = bq.remaining_capacity().await.unwrap();
                assert!(matches!(rem_cap, CapacityModeValue::CentiWattUnsigned(100)));

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_set_remaining_capacity_alarm_unit_mismatch() {
                // SLUUCN4B 16.2 0x01 RemainingCapacityAlarm(): the register unit is selected by
                // BatteryMode()[CAPM], not by the caller, so a mismatched tag must be rejected.
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x01, 0x2C, 0x01]),
                    Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x80]),
                    Transaction::write(BQ_ADDR, vec![0x01, 0x2C, 0x01]),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                // Cache defaults to mA: the matching tag is accepted.
                bq.set_remaining_capacity_alarm(CapacityModeValue::MilliAmpUnsigned(300))
                    .await
                    .unwrap();

                // The mismatched tag is rejected without touching the bus.
                assert_eq!(
                    bq.set_remaining_capacity_alarm(CapacityModeValue::CentiWattUnsigned(300))
                        .await,
                    Err(BQ40Z50Error::CapacityModeMismatch)
                );

                bq.set_battery_mode(BatteryModeFields::new().with_capacity_mode(true))
                    .await
                    .unwrap();

                // Now in cWh mode, the units swap over.
                assert_eq!(
                    bq.set_remaining_capacity_alarm(CapacityModeValue::MilliAmpUnsigned(300))
                        .await,
                    Err(BQ40Z50Error::CapacityModeMismatch)
                );
                bq.set_remaining_capacity_alarm(CapacityModeValue::CentiWattUnsigned(300))
                    .await
                    .unwrap();

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_set_at_rate_unit_mismatch() {
                // SLUUCN4B 16.5 0x04 AtRate(): the register unit is selected by BatteryMode()[CAPM],
                // not by the caller, so a mismatched tag must be rejected.
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x04, 0x9C, 0xFF]),
                    Transaction::write(BQ_ADDR, vec![0x03, 0x00, 0x80]),
                    Transaction::write(BQ_ADDR, vec![0x04, 0x9C, 0xFF]),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                bq.set_at_rate(CapacityModeSignedValue::MilliAmpSigned(-100))
                    .await
                    .unwrap();

                assert_eq!(
                    bq.set_at_rate(CapacityModeSignedValue::CentiWattSigned(-100))
                        .await,
                    Err(BQ40Z50Error::CapacityModeMismatch)
                );

                bq.set_battery_mode(BatteryModeFields::new().with_capacity_mode(true))
                    .await
                    .unwrap();

                assert_eq!(
                    bq.set_at_rate(CapacityModeSignedValue::MilliAmpSigned(-100))
                        .await,
                    Err(BQ40Z50Error::CapacityModeMismatch)
                );
                bq.set_at_rate(CapacityModeSignedValue::CentiWattSigned(-100))
                    .await
                    .unwrap();

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

            #[cfg(not(feature = "r1"))]
            #[tokio::test]
            async fn test_charging_override_voltage_signed() {
                // SLUUCN4B Table 16-1 types 0x00B0 ChargingVoltageOverride as "Signed Int", so a raw
                // word with bit 15 set must round-trip as a negative value, not a large positive one.
                let expectations = vec![
                    Transaction::write(
                        BQ_ADDR,
                        vec![
                            0x44, 0x0C, 0xB0, 0x00, 0xFF, 0xFF, 0x00, 0x80, 0xFF, 0x7F, 0xE8, 0xD1, 0x18, 0x2E,
                        ],
                    ),
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0xB0, 0x00]),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        vec![
                            0x0C, 0xB0, 0x00, 0xFF, 0xFF, 0x00, 0x80, 0xFF, 0x7F, 0xE8, 0xD1, 0x18, 0x2E,
                        ],
                    ),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                bq.write_charging_voltage_override(&ChargingVoltageOverride {
                    low_temp_chrg_mv: -1,
                    std_low_temp_chrg_mv: -32768,
                    std_hi_temp_chrg_mv: 32767,
                    hi_temp_chrg_mv: -11800,
                    recommended_temp_chrg_mv: 11800,
                })
                .await
                .unwrap();

                let override_struct = bq.read_charging_voltage_override().await.unwrap();

                assert_eq!(override_struct.low_temp_chrg_mv, -1);
                assert_eq!(override_struct.std_low_temp_chrg_mv, -32768);
                assert_eq!(override_struct.std_hi_temp_chrg_mv, 32767);
                assert_eq!(override_struct.hi_temp_chrg_mv, -11800);
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
                assert_eq!(
                    buf,
                    [
                        0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31,
                        0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x44, 0x32,
                    ]
                );

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
                assert_eq!(
                    buf,
                    [
                        0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31,
                        0xE0, 0x2E, 0x18, 0x2E, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x44, 0x32,
                    ]
                );

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
                    Transaction::write(BQ_ADDR, [vec![0x44, 34, 0x00, 0x40], vec![0x11; 32]].concat()),
                    Transaction::write(BQ_ADDR, [vec![0x44, 34, 0x20, 0x40], vec![0x22; 32]].concat()),
                    Transaction::write(BQ_ADDR, [vec![0x44, 34, 0x40, 0x40], vec![0x33; 32]].concat()),
                    Transaction::write(BQ_ADDR, [vec![0x44, 34, 0x60, 0x40], vec![0x44; 32]].concat()),
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
                        [vec![0x22, 0x00, 0x40], vec![0x11; 32]].concat(),
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        [vec![0x22, 0x20, 0x40], vec![0x22; 32]].concat(),
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        [vec![0x22, 0x40, 0x40], vec![0x33; 32]].concat(),
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        [vec![0x22, 0x60, 0x40], vec![0x44; 32]].concat(),
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
                let write = [[0x11; 32], [0x22; 32], [0x33; 32], [0x44; 32]].concat();
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

                let expected = [[0x11; 32], [0x22; 32], [0x33; 32], [0x44; 32]].concat();
                assert_eq!(read.as_slice(), expected.as_slice());

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
                        [vec![0x44, 34, 0x00, 0x40], vec![0x11; 32], vec![0x66]].concat(),
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        [vec![0x44, 34, 0x20, 0x40], vec![0x22; 32], vec![0x86]].concat(),
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        [vec![0x44, 34, 0x40, 0x40], vec![0x33; 32], vec![0x94]].concat(),
                    ),
                    Transaction::write(
                        BQ_ADDR,
                        [vec![0x44, 34, 0x60, 0x40], vec![0x44; 32], vec![0x41]].concat(),
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
                        [vec![0x22, 0x00, 0x40], vec![0x11; 32], vec![0x28]].concat(),
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        [vec![0x22, 0x20, 0x40], vec![0x22; 32], vec![0xC8]].concat(),
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        [vec![0x22, 0x40, 0x40], vec![0x33; 32], vec![0xDA]].concat(),
                    ),
                    Transaction::write_read(
                        BQ_ADDR,
                        vec![0x44],
                        [vec![0x22, 0x60, 0x40], vec![0x44; 32], vec![0x0F]].concat(),
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
                let write = [[0x11; 32], [0x22; 32], [0x33; 32], [0x44; 32]].concat();
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

                let expected = [[0x11; 32], [0x22; 32], [0x33; 32], [0x44; 32]].concat();
                assert_eq!(read.as_slice(), expected.as_slice());

                bq.device.interface().i2c.done();
            }

            /// Issue #65: a PEC failure on a DF read chunk must re-send that chunk's starting
            /// address before re-reading, otherwise the gauge's address auto-increment hands
            /// back the *next* block and the driver silently returns the wrong data.
            ///
            /// SLUUCN4B 16.1.101: "To read the DF, send an SMBus block write to the
            /// ManufacturerBlockAccess(), followed by the starting address, then send an SMBus
            /// block read to the ManufacturerBlockAccess()." and "If another SMBus read block is
            /// sent with command 0x44, the gauge returns another 32 bytes of DF data, starting
            /// with address 0x4020."
            #[tokio::test]
            async fn test_df_read_pec_retry_resends_address() {
                const BLOCK_A: [u8; 35] = [
                    0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                    0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31,
                    0xE0, 0x2E, 0x18,
                ];
                const BLOCK_B: [u8; 35] = [
                    0x22, 0x20, 0x40, // length + starting address 0x4020
                    0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xD0, 0x0D,
                    0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xDE, 0xAD, 0xD0, 0x0D,
                ];

                let mut block_a_good = BLOCK_A.to_vec();
                block_a_good.push(0x22); // correct PEC
                let mut block_b_good = BLOCK_B.to_vec();
                block_b_good.push(0xE0); // correct PEC
                let mut block_b_bad = BLOCK_B.to_vec();
                block_b_bad.push(0x00); // corrupted PEC

                let expectations = vec![
                    // Address write for the whole transfer, starting at 0x4000.
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40, 0xAB /* PEC */]),
                    // Chunk 1 at 0x4000, good PEC.
                    Transaction::write_read(BQ_ADDR, vec![0x44], block_a_good),
                    // Chunk 2 at 0x4020, corrupted PEC.
                    Transaction::write_read(BQ_ADDR, vec![0x44], block_b_bad),
                    // The retry MUST re-send chunk 2's own starting address (0x4020), not rely on
                    // the auto-increment, which would otherwise serve up the block at 0x4040.
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x20, 0x40, 0x05 /* PEC */]),
                    Transaction::write_read(BQ_ADDR, vec![0x44], block_b_good),
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

                let mut read = [0u8; 64];
                bq.read_dataflash(0x4000, &mut read).await.unwrap();

                let mut expected = [0u8; 64];
                expected[..32].copy_from_slice(&BLOCK_A[3..]);
                expected[32..].copy_from_slice(&BLOCK_B[3..]);
                assert_eq!(read, expected);

                bq.device.interface().i2c.done();
            }

            /// Issue #65: when the retries are exhausted the caller must get `Pec`, never a
            /// silently substituted neighbouring block.
            #[tokio::test]
            async fn test_df_read_pec_retries_exhausted() {
                const BLOCK_A: [u8; 35] = [
                    0x22, 0x00, 0x40, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18,
                    0x2E, 0xE0, 0x2E, 0x38, 0x31, 0xE0, 0x2E, 0x18, 0x2E, 0x00, 0x18, 0x2E, 0xE0, 0x2E, 0x38, 0x31,
                    0xE0, 0x2E, 0x18,
                ];
                let mut block_a_bad = BLOCK_A.to_vec();
                block_a_bad.push(0x00); // corrupted PEC

                let mut expectations = vec![Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40, 0xAB])];
                // Initial attempt plus DEFAULT_BUS_RETRIES retries, each re-addressing 0x4000.
                for i in 0..=DEFAULT_BUS_RETRIES {
                    if i > 0 {
                        expectations.push(Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40, 0xAB]));
                    }
                    expectations.push(Transaction::write_read(BQ_ADDR, vec![0x44], block_a_bad.clone()));
                }

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

                let mut read = [0u8; 32];
                assert_eq!(bq.read_dataflash(0x4000, &mut read).await, Err(BQ40Z50Error::Pec));

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_df_read_bus_error_retry_resends_address() {
                // The non-PEC read path retries bus errors. SLUUCN4B 16.1.101 DataFlashAccess
                // specifies a read as an address write followed by a block read, and the gauge
                // auto-increments its read pointer afterwards. After a failed block read the
                // pointer's position is not defined by the TRM, so the retry must re-send this
                // chunk's own starting address rather than rely on the auto-increment.
                const BLOCK_A: [u8; 35] = [
                    0x22, 0x00, 0x40, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                    0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11,
                    0x11, 0x11, 0x11,
                ];
                const BLOCK_B: [u8; 35] = [
                    0x22, 0x20, 0x40, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                    0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22, 0x22,
                    0x22, 0x22, 0x22,
                ];

                let expectations = vec![
                    // Address write for the whole transfer, starting at 0x4000.
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x00, 0x40]),
                    // Chunk 1 at 0x4000 succeeds; the pointer auto-increments to 0x4020.
                    Transaction::write_read(BQ_ADDR, vec![0x44], BLOCK_A.to_vec()),
                    // Chunk 2 fails on the bus.
                    Transaction::write_read(BQ_ADDR, vec![0x44], BLOCK_B.to_vec()).with_error(
                        embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address),
                    ),
                    // The retry MUST re-address 0x4020. Without this the next block read would
                    // return whatever the pointer now refers to, and the driver would hand that
                    // back as the block the caller asked for.
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x20, 0x40]),
                    Transaction::write_read(BQ_ADDR, vec![0x44], BLOCK_B.to_vec()),
                ];
                let i2c = Mock::new(&expectations);
                let delay_expectations = vec![DelayTransaction::delay_ms(10)];
                let mut bq = Bq40z50::new_with_config(
                    i2c,
                    CheckedDelay::new(&delay_expectations),
                    Config {
                        pec_read: false,
                        pec_write: false,
                        ..Default::default()
                    },
                );

                let mut buf = [0u8; 64];
                bq.read_dataflash(0x4000, &mut buf).await.unwrap();

                assert_eq!(buf[..32], [0x11u8; 32]);
                assert_eq!(buf[32..], [0x22u8; 32]);

                bq.device.interface().i2c.done();
                bq.device.interface().delay.done();
            }

            /// Issue #76: the DF window is 0x4000-0x5FFF on every revision. SLUUCN4B 16.1.101 is
            /// titled "ManufacturerAccess() 0x4000-0x5FFF DataFlashAccess"; SLUUCH2 16.1.98,
            /// SLUUBU5A 15.1.83 and SLUUA43A 12.1.60 carry the same range in their titles.
            ///
            /// Out-of-window addresses, and transfers whose *end* leaves the window, must be
            /// rejected before any bus traffic.
            #[tokio::test]
            async fn test_df_address_bounds() {
                // No transactions at all: every one of these must be rejected up front.
                let i2c = Mock::new(&[]);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                let mut read = [0u8; 4];

                // Below the window.
                assert_eq!(
                    bq.read_dataflash(0x3FFF, &mut read).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );
                assert_eq!(
                    bq.write_dataflash(0x0000, &[0u8; 4]).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );

                // Above the window.
                assert_eq!(
                    bq.read_dataflash(0x6000, &mut read).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );
                assert_eq!(
                    bq.write_dataflash(0x6000, &[0u8; 4]).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );

                // Starts inside the window but runs off the end of it.
                assert_eq!(
                    bq.read_dataflash(0x5FFE, &mut read).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );
                assert_eq!(
                    bq.write_dataflash(0x5FFE, &[0u8; 4]).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );

                // Multi-chunk write whose later chunk addresses would run past the window.
                assert_eq!(
                    bq.write_dataflash(0x5FF0, &[0u8; 64]).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );

                // Near the top of the u16 space: the per-chunk address add would wrap.
                assert_eq!(
                    bq.write_dataflash(0xFFF0, &[0u8; 64]).await,
                    Err(BQ40Z50Error::DataFlashAddressOutOfRange)
                );

                bq.device.interface().i2c.done();
            }

            /// Issue #79: a DF write that fails part way through has already committed the earlier
            /// chunks to flash. The error must report exactly how many caller-payload bytes the
            /// gauge accepted.
            #[tokio::test]
            async fn test_df_write_partial_commit_reports_committed_bytes() {
                let write = [0xAAu8; 96]; // 3 chunks
                let nak = embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address);

                let mut chunk1 = vec![0x44, 34, 0x00, 0x40];
                chunk1.extend_from_slice(&write[..32]);
                let mut chunk2 = vec![0x44, 34, 0x20, 0x40];
                chunk2.extend_from_slice(&write[32..64]);

                let mut expectations = vec![Transaction::write(BQ_ADDR, chunk1)];
                // Chunk 2 fails on the initial attempt and on every retry.
                for _ in 0..=DEFAULT_BUS_RETRIES {
                    expectations.push(Transaction::write(BQ_ADDR, chunk2.clone()).with_error(nak));
                }
                // Chunk 3 is never attempted.

                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                // Exactly one 32-byte chunk reached flash.
                assert_eq!(
                    bq.write_dataflash(0x4000, &write).await,
                    Err(BQ40Z50Error::PartialDataFlashWrite { committed: 32 })
                );

                bq.device.interface().i2c.done();
            }

            /// Issue #79: when the *first* chunk fails nothing was committed, so there is no
            /// partial state to report. The caller gets the underlying cause instead, and
            /// `PartialDataFlashWrite` is reserved for `committed > 0`.
            #[tokio::test]
            async fn test_df_write_first_chunk_failure_reports_underlying_error() {
                let write = [0xAAu8; 96];
                let nak = embedded_hal::i2c::ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Address);

                let mut chunk1 = vec![0x44, 34, 0x00, 0x40];
                chunk1.extend_from_slice(&write[..32]);

                let mut expectations = Vec::new();
                for _ in 0..=DEFAULT_BUS_RETRIES {
                    expectations.push(Transaction::write(BQ_ADDR, chunk1.clone()).with_error(nak));
                }

                let i2c = Mock::new(&expectations);
                let mut bq = Bq40z50::new(i2c, NoopDelay::new());

                assert_eq!(
                    bq.write_dataflash(0x4000, &write).await,
                    Err(BQ40Z50Error::I2c(nak))
                );

                bq.device.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_read_chem_id() {
                // SLUUCN4B 16.1: "The second 2 bytes, "00 01", is the chem ID returning in little
                // endian. That is 0x0100, chem ID 100." ChemID is therefore a 16-bit value.
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x06, 0x00]),
                    Transaction::write_read(BQ_ADDR, vec![0x44], vec![0x04, 0x06, 0x00, 0x00, 0x01]),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                let chem_id = bq.mac_chem_id().dispatch_out_async().await.unwrap();
                assert_eq!(chem_id.chem_id(), 0x0100);

                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_lifetime_data_block_1_discharge_is_signed() {
                // SLUUCN4B 17.17 Data Flash Summary types Max Discharge Current, Max Avg Dsg
                // Current and Max Avg Dsg Power as I2 with range -32768..0, while the adjacent
                // Max Charge Current row is I2 with range 0..32767. A raw 0xFC18 must therefore
                // decode as -1000, not 64536.
                // r1 and r3 carry six extra temperature bytes at the end of this block, so the
                // length comes in as a macro parameter.
                let mut block = vec![
                    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                    0x00, 0x00, 0xE8, 0x03, 0x18, 0xFC, 0x7C, 0xFD, 0x9C, 0xFF,
                ];
                block.resize($lifetime_block_1_len, 0x00);
                let mut mac_block = vec![(block.len() + 2) as u8, 0x60, 0x00];
                mac_block.extend_from_slice(&block);
                let expectations = vec![
                    Transaction::write_read(BQ_ADDR, vec![0x60], block.clone()),
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x60, 0x00]),
                    Transaction::write_read(BQ_ADDR, vec![0x44], mac_block),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                let reg = bq.lifetime_data_block_1().read_async().await.unwrap();
                assert_eq!(reg.max_charge_a(), 1000);
                assert_eq!(reg.max_discharge_a(), -1000);
                assert_eq!(reg.max_avg_discharge_a(), -644);
                assert_eq!(reg.max_avg_discharge_pwr(), -100);

                let mac = bq.mac_lifetime_data_block_1().dispatch_out_async().await.unwrap();
                assert_eq!(mac.max_charge_a(), 1000);
                assert_eq!(mac.max_discharge_a(), -1000);
                assert_eq!(mac.max_avg_discharge_a(), -644);
                assert_eq!(mac.max_avg_discharge_pwr(), -100);

                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_stop_output_ccadc_cal_sends_disable_subcommand() {
                // SLUUCN4B 16.1.102 and 16.1.103 both give the Disable condition as
                // "ManufacturingStatus()[CAL_EN] = 1 AND 0xF080 to ManufacturerAccess()", so both
                // stop commands must put 0xF080 (little endian on the wire) on the bus, not the
                // 0xF081/0xF082 enable subcommands.
                let expectations = vec![
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x80, 0xF0]),
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x80, 0xF0]),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                bq.mac_stop_output_ccadc_cal().dispatch_async().await.unwrap();
                bq.mac_stop_output_shorted_ccadc_cal()
                    .dispatch_async()
                    .await
                    .unwrap();

                bq.interface().i2c.done();
            }

            #[tokio::test]
            async fn test_gauge_status_3_reads_documented_length() {
                // SLUUCN4B 16.1.69 ManufacturerAccess() 0x0075 GaugeStatus3: "Action: Output 24
                // bytes of IT data values on ManufacturerBlockAccess() or ManufacturerData()".
                // The MAC entry and the direct 0x75 register describe the same data, so both must
                // put 24 bytes on the wire. The mock fails the transaction if the driver asks for
                // any other length.
                let mut block = vec![0x00u8; 24];
                block[0] = 0x34;
                block[1] = 0x12;
                block[22] = 0x78;
                block[23] = 0x56;
                let mut mac_block = vec![(block.len() + 2) as u8, 0x75, 0x00];
                mac_block.extend_from_slice(&block);
                let expectations = vec![
                    Transaction::write_read(BQ_ADDR, vec![0x75], block.clone()),
                    Transaction::write(BQ_ADDR, vec![0x44, 0x02, 0x75, 0x00]),
                    Transaction::write_read(BQ_ADDR, vec![0x44], mac_block),
                ];
                let i2c = Mock::new(&expectations);
                let mut bq = Device::new(DeviceInterface::new(i2c, NoopDelay::new()));

                let reg = bq.gauge_status_3().read_async().await.unwrap();
                assert_eq!(reg.qmax_0(), 0x1234);
                assert_eq!(reg.temp_a_factor(), 0x5678);

                let mac = bq.mac_gauge_status_3().dispatch_out_async().await.unwrap();
                assert_eq!(mac.qmax_0(), 0x1234);
                assert_eq!(mac.temp_a_factor(), 0x5678);

                bq.interface().i2c.done();
            }
        }
    };
}

pub(crate) use bq40z50_tests;
