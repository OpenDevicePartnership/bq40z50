// This code was generated using device-driver `2.1.0` (),
// a tool distributed under MIT OR Apache-2.0 by Dion Dokter <dev@diondokter.nl>
//
// For more information about device-driver, visit the website: https://device-driver.com

/// Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    interface: I,
    #[doc(hidden)]
    #[allow(unused)]
    base_address: u32,
}
impl<I> Device<I> {
    /// Create a new instance of the device
    pub const fn new(interface: I) -> Self {
        Self {
            interface,
            base_address: 0,
        }
    }
    /// Drop the driver instance and reclaim the interface
    pub fn free(self) -> I {
        self.interface
    }
    /// The device can be checked for the IC part number.
    ///
    /// Command operation:
    /// - Address: `4456704`
    #[doc(alias = "MAC_DEVICE_TYPE")]
    pub fn mac_device_type(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacDeviceType, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4456704;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// The device can be checked for the firmware version of the IC.
    ///
    /// Command operation:
    /// - Address: `4456960`
    #[doc(alias = "MAC_FIRMWARE_VERSION")]
    pub fn mac_firmware_version(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacFirmwareVersion, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4456960;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4457216`
    #[doc(alias = "MAC_HARDWARE_VERSION")]
    pub fn mac_hardware_version(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacHardwareVersion, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4457216;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4457472`
    #[doc(alias = "MAC_INSTRUCTION_FLASH_SIGNATURE")]
    pub fn mac_instruction_flash_signature(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacInstructionFlashSignature, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4457472;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4457728`
    #[doc(alias = "MAC_STATIC_DF_SIGNATURE")]
    pub fn mac_static_df_signature(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacStaticDfSignature, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4457728;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4457984`
    #[doc(alias = "MAC_CHEM_ID")]
    pub fn mac_chem_id(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacChemId, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4457984;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4458496`
    #[doc(alias = "MAC_STATIC_CHEM_DF_SIG")]
    pub fn mac_static_chem_df_sig(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacStaticChemDfSig, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4458496;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4458752`
    #[doc(alias = "MAC_ALL_DF_SIGNATURE")]
    pub fn mac_all_df_signature(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacAllDfSignature, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4458752;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4460544`
    #[doc(alias = "MAC_SHUTDOWN_MODE")]
    pub fn mac_shutdown_mode(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4460544;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4460800`
    #[doc(alias = "MAC_SLEEP_MODE")]
    pub fn mac_sleep_mode(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4460800;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4461312`
    #[doc(alias = "MAC_AUTO_CC_OFFSET")]
    pub fn mac_auto_cc_offset(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4461312;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4463872`
    #[doc(alias = "MAC_FUSE_TOGGLE")]
    pub fn mac_fuse_toggle(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4463872;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4464128`
    #[doc(alias = "MAC_PCHG_FET_TOGGLE")]
    pub fn mac_pchg_fet_toggle(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4464128;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4464384`
    #[doc(alias = "MAC_CHG_FET_TOGGLE")]
    pub fn mac_chg_fet_toggle(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4464384;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4464640`
    #[doc(alias = "MAC_DSG_FET_TOGGLE")]
    pub fn mac_dsg_fet_toggle(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4464640;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// This command enables or disables the gauging function to ease
    /// testing during manufacturing.
    ///
    /// Command operation:
    /// - Address: `4464896`
    #[doc(alias = "MAC_GAUGING")]
    pub fn mac_gauging(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4464896;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4465152`
    #[doc(alias = "MAC_FET_CTRL")]
    pub fn mac_fet_ctrl(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4465152;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4465408`
    #[doc(alias = "MAC_LIFETIME_DATA_COLLECTION")]
    pub fn mac_lifetime_data_collection(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4465408;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4465664`
    #[doc(alias = "MAC_PERMANENT_FAILURE")]
    pub fn mac_permanent_failure(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4465664;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4465920`
    #[doc(alias = "MAC_BLACK_BLOCK_RECORDER")]
    pub fn mac_black_block_recorder(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4465920;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4466176`
    #[doc(alias = "MAC_FUSE")]
    pub fn mac_fuse(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4466176;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4466432`
    #[doc(alias = "MAC_LED_DISP_EN")]
    pub fn mac_led_disp_en(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4466432;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4466688`
    #[doc(alias = "MAC_LIFETIME_DATA_RST")]
    pub fn mac_lifetime_data_rst(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4466688;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4466944`
    #[doc(alias = "MAC_PF_DATA_RST")]
    pub fn mac_pf_data_rst(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4466944;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4467200`
    #[doc(alias = "MAC_BLK_BOX_REC_RESET")]
    pub fn mac_blk_box_rec_reset(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4467200;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4467456`
    #[doc(alias = "MAC_LED_TOGGLE")]
    pub fn mac_led_toggle(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4467456;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4467712`
    #[doc(alias = "MAC_LED_DISP_PRESS")]
    pub fn mac_led_disp_press(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4467712;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4467968`
    #[doc(alias = "MAC_CALIBRATION_MODE")]
    pub fn mac_calibration_mode(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4467968;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4468224`
    #[doc(alias = "MAC_LIFETIME_DATA_FLUSH")]
    pub fn mac_lifetime_data_flush(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4468224;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4468480`
    #[doc(alias = "MAC_LIFETIME_DATA_SPEED_UP_MODE")]
    pub fn mac_lifetime_data_speed_up_mode(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4468480;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4468736`
    #[doc(alias = "MAC_SEAL")]
    pub fn mac_seal(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4468736;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4473088`
    #[doc(alias = "MAC_DEVICE_RESET")]
    pub fn mac_device_reset(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4473088;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4476928`
    #[doc(alias = "MAC_SAFETY_ALERT")]
    pub fn mac_safety_alert(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacSafetyAlert, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4476928;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4477184`
    #[doc(alias = "MAC_SAFETY_STATUS")]
    pub fn mac_safety_status(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacSafetyStatus, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4477184;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4477440`
    #[doc(alias = "MAC_PF_ALERT")]
    pub fn mac_pf_alert(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacPfAlert, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4477440;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4477696`
    #[doc(alias = "MAC_PF_STATUS")]
    pub fn mac_pf_status(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacPfStatus, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4477696;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4477952`
    #[doc(alias = "MAC_OPERATION_STATUS")]
    pub fn mac_operation_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacOperationStatus, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4477952;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4478208`
    #[doc(alias = "MAC_CHARGING_STATUS")]
    pub fn mac_charging_status(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacChargingStatus, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4478208;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4478464`
    #[doc(alias = "MAC_GAUGING_STATUS")]
    pub fn mac_gauging_status(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacGaugingStatus, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4478464;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4478720`
    #[doc(alias = "MAC_MANUFACTURING_STATUS")]
    pub fn mac_manufacturing_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacManufacturingStatus, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4478720;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4478976`
    #[doc(alias = "MAC_AFE_REG")]
    pub fn mac_afe_reg(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacAfeReg, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4478976;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4481024`
    #[doc(alias = "MAC_LIFETIME_DATA_BLOCK_1")]
    pub fn mac_lifetime_data_block_1(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacLifetimeDataBlock1, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4481024;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4481280`
    #[doc(alias = "MAC_LIFETIME_DATA_BLOCK_2")]
    pub fn mac_lifetime_data_block_2(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacLifetimeDataBlock2, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4481280;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4481536`
    #[doc(alias = "MAC_LIFETIME_DATA_BLOCK_3")]
    pub fn mac_lifetime_data_block_3(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacLifetimeDataBlock3, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4481536;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4481792`
    #[doc(alias = "MAC_LIFETIME_DATA_BLOCK_4")]
    pub fn mac_lifetime_data_block_4(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacLifetimeDataBlock4, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4481792;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4482048`
    #[doc(alias = "MAC_LIFETIME_DATA_BLOCK_5")]
    pub fn mac_lifetime_data_block_5(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacLifetimeDataBlock5, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4482048;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4485120`
    #[doc(alias = "MAC_MANUFACTURE_INFO")]
    pub fn mac_manufacture_info(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacManufactureInfo, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4485120;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4485376`
    #[doc(alias = "MAC_DA_STATUS_1")]
    pub fn mac_da_status_1(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacDaStatus1, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4485376;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4485632`
    #[doc(alias = "MAC_DA_STATUS_2")]
    pub fn mac_da_status_2(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacDaStatus2, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4485632;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4485888`
    #[doc(alias = "MAC_GAUGE_STATUS_1")]
    pub fn mac_gauge_status_1(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacGaugeStatus1, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4485888;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4486144`
    #[doc(alias = "MAC_GAUGE_STATUS_2")]
    pub fn mac_gauge_status_2(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacGaugeStatus2, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4486144;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4486400`
    #[doc(alias = "MAC_GAUGE_STATUS_3")]
    pub fn mac_gauge_status_3(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacGaugeStatus3, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4486400;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4486656`
    #[doc(alias = "MAC_CB_STATUS")]
    pub fn mac_cb_status(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacCbStatus, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4486656;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4486912`
    #[doc(alias = "MAC_STATE_OF_HEALTH")]
    pub fn mac_state_of_health(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacStateOfHealth, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4486912;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4487168`
    #[doc(alias = "MAC_FILTER_CAPACITY")]
    pub fn mac_filter_capacity(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacFilterCapacity, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4487168;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4456463`
    #[doc(alias = "MAC_ROM_MODE")]
    pub fn mac_rom_mode(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4456463;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4489456`
    #[doc(alias = "MAC_EXIT_CALIBRATION_OUTPUT_MODE")]
    pub fn mac_exit_calibration_output_mode(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4489456;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4489712`
    #[doc(alias = "MAC_STOP_OUTPUT_CCADC_CAL")]
    pub fn mac_stop_output_ccadc_cal(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4489712;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4489712`
    #[doc(alias = "MAC_OUTPUT_CCADC_CAL")]
    pub fn mac_output_ccadc_cal(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacOutputCcadcCal, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4489712;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4489968`
    #[doc(alias = "MAC_STOP_OUTPUT_SHORTED_CCADC_CAL")]
    pub fn mac_stop_output_shorted_ccadc_cal(&mut self) -> ::device_driver::CommandOperation<'_, Self, u32, (), (), ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4489968;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// Command operation:
    /// - Address: `4489968`
    #[doc(alias = "MAC_OUTPUT_SHORTED_CCADC_CAL")]
    pub fn mac_output_shorted_ccadc_cal(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, Self, u32, (), MacOutputShortedCcadcCal, ()>
    where
        I: ::device_driver::CommandInterfaceBase<AddressType = u32>,
    {
        let address = self.base_address + 4489968;
        ::device_driver::CommandOperation::new(self, address as u32)
    }
    /// This read/write word function sets a low capacity alarm threshold for the cell stack.
    ///
    /// Register operation:
    /// - Address: `1`
    /// - Reset value: `0x12c`
    #[doc(alias = "REMAINING_CAPACITY_ALARM")]
    pub fn remaining_capacity_alarm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, RemainingCapacityAlarm, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::new(self, address as u8, || RemainingCapacityAlarm::from([44, 1]))
    }
    /// This read/write word function sets a low remaining time-to-fully discharge alarm threshold for the cell stack.
    ///
    /// Register operation:
    /// - Address: `2`
    /// - Reset value: `0x0a`
    #[doc(alias = "REMAINING_TIME_ALARM")]
    pub fn remaining_time_alarm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, RemainingTimeAlarm, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::new(self, address as u8, || RemainingTimeAlarm::from([10, 0]))
    }
    /// Register operation:
    /// - Address: `3`
    /// - Reset value: `0x4000`
    #[doc(alias = "BATTERY_MODE")]
    pub fn battery_mode(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BatteryMode, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::new(self, address as u8, || BatteryMode::from([0, 64]))
    }
    /// Register operation:
    /// - Address: `4`
    /// - Reset value: `0x00`
    #[doc(alias = "AT_RATE")]
    pub fn at_rate(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AtRate, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::new(self, address as u8, || AtRate::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `5`
    /// - Reset value: `0x00`
    #[doc(alias = "AT_RATE_TIME_TO_FULL")]
    pub fn at_rate_time_to_full(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AtRateTimeToFull, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 5;
        ::device_driver::RegisterOperation::new(self, address as u8, || AtRateTimeToFull::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `6`
    /// - Reset value: `0x00`
    #[doc(alias = "AT_RATE_TIME_TO_EMPTY")]
    pub fn at_rate_time_to_empty(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AtRateTimeToEmpty, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::new(self, address as u8, || AtRateTimeToEmpty::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `7`
    /// - Reset value: `0x00`
    #[doc(alias = "AT_RATE_OK")]
    pub fn at_rate_ok(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AtRateOk, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 7;
        ::device_driver::RegisterOperation::new(self, address as u8, || AtRateOk::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `8`
    /// - Reset value: `0x00`
    #[doc(alias = "TEMPERATURE")]
    pub fn temperature(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, Temperature, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::new(self, address as u8, || Temperature::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `9`
    /// - Reset value: `0x00`
    #[doc(alias = "VOLTAGE")]
    pub fn voltage(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Voltage, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::new(self, address as u8, || Voltage::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `10`
    /// - Reset value: `0x00`
    #[doc(alias = "CURRENT")]
    pub fn current(&mut self) -> ::device_driver::RegisterOperation<'_, Self, Current, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::new(self, address as u8, || Current::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `11`
    /// - Reset value: `0x00`
    #[doc(alias = "AVG_CURRENT")]
    pub fn avg_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AvgCurrent, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 11;
        ::device_driver::RegisterOperation::new(self, address as u8, || AvgCurrent::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `12`
    /// - Reset value: `0x00`
    #[doc(alias = "MAX_ERROR")]
    pub fn max_error(&mut self) -> ::device_driver::RegisterOperation<'_, Self, MaxError, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::new(self, address as u8, || MaxError::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `13`
    /// - Reset value: `0x00`
    #[doc(alias = "RELATIVE_STATE_OF_CHARGE")]
    pub fn relative_state_of_charge(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, RelativeStateOfCharge, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 13;
        ::device_driver::RegisterOperation::new(self, address as u8, || RelativeStateOfCharge::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `14`
    /// - Reset value: `0x00`
    #[doc(alias = "ABSOLUTE_STATE_OF_CHARGE")]
    pub fn absolute_state_of_charge(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AbsoluteStateOfCharge, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::new(self, address as u8, || AbsoluteStateOfCharge::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `15`
    /// - Reset value: `0x00`
    #[doc(alias = "REMAINING_CAPACITY")]
    pub fn remaining_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, RemainingCapacity, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 15;
        ::device_driver::RegisterOperation::new(self, address as u8, || RemainingCapacity::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `16`
    /// - Reset value: `0x00`
    #[doc(alias = "FULL_CHARGE_CAPACITY")]
    pub fn full_charge_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, FullChargeCapacity, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::new(self, address as u8, || FullChargeCapacity::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `17`
    /// - Reset value: `0x00`
    #[doc(alias = "RUN_TIME_TO_EMPTY")]
    pub fn run_time_to_empty(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, RunTimeToEmpty, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 17;
        ::device_driver::RegisterOperation::new(self, address as u8, || RunTimeToEmpty::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `18`
    /// - Reset value: `0x00`
    #[doc(alias = "AVERAGE_TIME_TO_EMPTY")]
    pub fn average_time_to_empty(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AverageTimeToEmpty, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 18;
        ::device_driver::RegisterOperation::new(self, address as u8, || AverageTimeToEmpty::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `19`
    /// - Reset value: `0x00`
    #[doc(alias = "AVERAGE_TIME_TO_FULL")]
    pub fn average_time_to_full(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, AverageTimeToFull, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 19;
        ::device_driver::RegisterOperation::new(self, address as u8, || AverageTimeToFull::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `20`
    /// - Reset value: `0x00`
    #[doc(alias = "CHARGING_CURRENT")]
    pub fn charging_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargingCurrent, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargingCurrent::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `21`
    /// - Reset value: `0x00`
    #[doc(alias = "CHARGING_VOLTAGE")]
    pub fn charging_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargingVoltage, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 21;
        ::device_driver::RegisterOperation::new(self, address as u8, || ChargingVoltage::from([0, 0]))
    }
    /// Register operation:
    /// - Address: `22`
    /// - Reset value: `0`
    #[doc(alias = "BATTERY_STATUS")]
    pub fn battery_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BatteryStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::new(self, address as u8, BatteryStatus::default)
    }
    /// Register operation:
    /// - Address: `23`
    /// - Reset value: `0`
    #[doc(alias = "CYCLE_COUNT")]
    pub fn cycle_count(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CycleCount, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 23;
        ::device_driver::RegisterOperation::new(self, address as u8, CycleCount::default)
    }
    /// Register operation:
    /// - Address: `24`
    /// - Reset value: `0`
    #[doc(alias = "DESIGN_CAPACITY")]
    pub fn design_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, DesignCapacity, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::new(self, address as u8, DesignCapacity::default)
    }
    /// Register operation:
    /// - Address: `25`
    /// - Reset value: `0`
    #[doc(alias = "DESIGN_VOLTAGE")]
    pub fn design_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, DesignVoltage, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 25;
        ::device_driver::RegisterOperation::new(self, address as u8, DesignVoltage::default)
    }
    /// Register operation:
    /// - Address: `26`
    /// - Reset value: `0`
    #[doc(alias = "SPECIFICATION_INFO")]
    pub fn specification_info(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, SpecificationInfo, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::new(self, address as u8, SpecificationInfo::default)
    }
    /// Register operation:
    /// - Address: `27`
    /// - Reset value: `0`
    #[doc(alias = "MANUFACTURE_DATE")]
    pub fn manufacture_date(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ManufactureDate, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 27;
        ::device_driver::RegisterOperation::new(self, address as u8, ManufactureDate::default)
    }
    /// Register operation:
    /// - Address: `28`
    /// - Reset value: `0x01`
    #[doc(alias = "SERIAL_NUMBER")]
    pub fn serial_number(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, SerialNumber, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 28;
        ::device_driver::RegisterOperation::new(self, address as u8, || SerialNumber::from([1, 0]))
    }
    /// Buffer operation:
    /// - Address: `32`
    #[doc(alias = "MANUFACTURE_NAME")]
    pub fn manufacture_name(&mut self) -> ::device_driver::BufferOperation<'_, Self, u8, ::device_driver::RO>
    where
        I: ::device_driver::BufferInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 32;
        ::device_driver::BufferOperation::new(self, address as u8)
    }
    /// Buffer operation:
    /// - Address: `33`
    #[doc(alias = "DEVICE_NAME")]
    pub fn device_name(&mut self) -> ::device_driver::BufferOperation<'_, Self, u8, ::device_driver::RO>
    where
        I: ::device_driver::BufferInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 33;
        ::device_driver::BufferOperation::new(self, address as u8)
    }
    /// Buffer operation:
    /// - Address: `34`
    #[doc(alias = "DEVICE_CHEMISTRY")]
    pub fn device_chemistry(&mut self) -> ::device_driver::BufferOperation<'_, Self, u8, ::device_driver::RO>
    where
        I: ::device_driver::BufferInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 34;
        ::device_driver::BufferOperation::new(self, address as u8)
    }
    /// Buffer operation:
    /// - Address: `35`
    #[doc(alias = "MANUFACTURER_DATA")]
    pub fn manufacturer_data(&mut self) -> ::device_driver::BufferOperation<'_, Self, u8, ::device_driver::RO>
    where
        I: ::device_driver::BufferInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 35;
        ::device_driver::BufferOperation::new(self, address as u8)
    }
    /// Buffer operation:
    /// - Address: `47`
    #[doc(alias = "AUTHENTICATE")]
    pub fn authenticate(&mut self) -> ::device_driver::BufferOperation<'_, Self, u8, ::device_driver::RW>
    where
        I: ::device_driver::BufferInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 47;
        ::device_driver::BufferOperation::new(self, address as u8)
    }
    /// Register operation:
    /// - Address: `60`
    /// - Reset value: `0`
    #[doc(alias = "CELL_VOLTAGE_4")]
    pub fn cell_voltage_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CellVoltage4, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::new(self, address as u8, CellVoltage4::default)
    }
    /// Register operation:
    /// - Address: `61`
    /// - Reset value: `0`
    #[doc(alias = "CELL_VOLTAGE_3")]
    pub fn cell_voltage_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CellVoltage3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 61;
        ::device_driver::RegisterOperation::new(self, address as u8, CellVoltage3::default)
    }
    /// Register operation:
    /// - Address: `62`
    /// - Reset value: `0`
    #[doc(alias = "CELL_VOLTAGE_2")]
    pub fn cell_voltage_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CellVoltage2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 62;
        ::device_driver::RegisterOperation::new(self, address as u8, CellVoltage2::default)
    }
    /// Register operation:
    /// - Address: `63`
    /// - Reset value: `0`
    #[doc(alias = "CELL_VOLTAGE_1")]
    pub fn cell_voltage_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, CellVoltage1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 63;
        ::device_driver::RegisterOperation::new(self, address as u8, CellVoltage1::default)
    }
    /// Register operation:
    /// - Address: `74`
    /// - Reset value: `0x96`
    #[doc(alias = "BTP_DISCHARGE_SET")]
    pub fn btp_discharge_set(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BtpDischargeSet, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 74;
        ::device_driver::RegisterOperation::new(self, address as u8, || BtpDischargeSet::from([150, 0]))
    }
    /// Register operation:
    /// - Address: `75`
    /// - Reset value: `0xaf`
    #[doc(alias = "BTP_CHARGE_SET")]
    pub fn btp_charge_set(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, BtpChargeSet, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 75;
        ::device_driver::RegisterOperation::new(self, address as u8, || BtpChargeSet::from([175, 0]))
    }
    /// Register operation:
    /// - Address: `79`
    /// - Reset value: `0`
    #[doc(alias = "STATE_OF_HEALTH_SOH")]
    pub fn state_of_health_soh(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, StateOfHealthSoh, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 79;
        ::device_driver::RegisterOperation::new(self, address as u8, StateOfHealthSoh::default)
    }
    /// Register operation:
    /// - Address: `80`
    /// - Reset value: `0`
    #[doc(alias = "SAFETY_ALERT")]
    pub fn safety_alert(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, SafetyAlert, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 80;
        ::device_driver::RegisterOperation::new(self, address as u8, SafetyAlert::default)
    }
    /// Register operation:
    /// - Address: `81`
    /// - Reset value: `0`
    #[doc(alias = "SAFETY_STATUS")]
    pub fn safety_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, SafetyStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 81;
        ::device_driver::RegisterOperation::new(self, address as u8, SafetyStatus::default)
    }
    /// Register operation:
    /// - Address: `82`
    /// - Reset value: `0`
    #[doc(alias = "PF_ALERT")]
    pub fn pf_alert(&mut self) -> ::device_driver::RegisterOperation<'_, Self, PfAlert, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 82;
        ::device_driver::RegisterOperation::new(self, address as u8, PfAlert::default)
    }
    /// Register operation:
    /// - Address: `83`
    /// - Reset value: `0`
    #[doc(alias = "PF_STATUS")]
    pub fn pf_status(&mut self) -> ::device_driver::RegisterOperation<'_, Self, PfStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 83;
        ::device_driver::RegisterOperation::new(self, address as u8, PfStatus::default)
    }
    /// Register operation:
    /// - Address: `84`
    /// - Reset value: `0`
    #[doc(alias = "OPERATION_STATUS")]
    pub fn operation_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, OperationStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 84;
        ::device_driver::RegisterOperation::new(self, address as u8, OperationStatus::default)
    }
    /// Register operation:
    /// - Address: `85`
    /// - Reset value: `0`
    #[doc(alias = "CHARGING_STATUS")]
    pub fn charging_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ChargingStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 85;
        ::device_driver::RegisterOperation::new(self, address as u8, ChargingStatus::default)
    }
    /// Register operation:
    /// - Address: `86`
    /// - Reset value: `0`
    #[doc(alias = "GAUGING_STATUS")]
    pub fn gauging_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, GaugingStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 86;
        ::device_driver::RegisterOperation::new(self, address as u8, GaugingStatus::default)
    }
    /// Register operation:
    /// - Address: `87`
    /// - Reset value: `0`
    #[doc(alias = "MANUFACTURING_STATUS")]
    pub fn manufacturing_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, ManufacturingStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 87;
        ::device_driver::RegisterOperation::new(self, address as u8, ManufacturingStatus::default)
    }
    /// Register operation:
    /// - Address: `88`
    /// - Reset value: `0`
    #[doc(alias = "AFE_REG")]
    pub fn afe_reg(&mut self) -> ::device_driver::RegisterOperation<'_, Self, AfeReg, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 88;
        ::device_driver::RegisterOperation::new(self, address as u8, AfeReg::default)
    }
    /// Register operation:
    /// - Address: `89`
    /// - Reset value: `0`
    #[doc(alias = "TURBO_POWER")]
    pub fn turbo_power(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, TurboPower, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 89;
        ::device_driver::RegisterOperation::new(self, address as u8, TurboPower::default)
    }
    /// Register operation:
    /// - Address: `90`
    /// - Reset value: `0`
    #[doc(alias = "TURBO_FINAL")]
    pub fn turbo_final(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, TurboFinal, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 90;
        ::device_driver::RegisterOperation::new(self, address as u8, TurboFinal::default)
    }
    /// Register operation:
    /// - Address: `91`
    /// - Reset value: `0`
    #[doc(alias = "TURBO_PACK_R")]
    pub fn turbo_pack_r(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, TurboPackR, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 91;
        ::device_driver::RegisterOperation::new(self, address as u8, TurboPackR::default)
    }
    /// Register operation:
    /// - Address: `92`
    /// - Reset value: `0`
    #[doc(alias = "TURBO_SYS_R")]
    pub fn turbo_sys_r(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, TurboSysR, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 92;
        ::device_driver::RegisterOperation::new(self, address as u8, TurboSysR::default)
    }
    /// Register operation:
    /// - Address: `93`
    /// - Reset value: `0`
    #[doc(alias = "TURBO_EDV")]
    pub fn turbo_edv(&mut self) -> ::device_driver::RegisterOperation<'_, Self, TurboEdv, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 93;
        ::device_driver::RegisterOperation::new(self, address as u8, TurboEdv::default)
    }
    /// Register operation:
    /// - Address: `94`
    /// - Reset value: `0`
    #[doc(alias = "TURBO_CURRENT")]
    pub fn turbo_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, TurboCurrent, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 94;
        ::device_driver::RegisterOperation::new(self, address as u8, TurboCurrent::default)
    }
    /// Register operation:
    /// - Address: `96`
    /// - Reset value: `0`
    #[doc(alias = "LIFETIME_DATA_BLOCK_1")]
    pub fn lifetime_data_block_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, LifetimeDataBlock1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 96;
        ::device_driver::RegisterOperation::new(self, address as u8, LifetimeDataBlock1::default)
    }
    /// Register operation:
    /// - Address: `97`
    /// - Reset value: `0`
    #[doc(alias = "LIFETIME_DATA_BLOCK_2")]
    pub fn lifetime_data_block_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, LifetimeDataBlock2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 97;
        ::device_driver::RegisterOperation::new(self, address as u8, LifetimeDataBlock2::default)
    }
    /// Register operation:
    /// - Address: `98`
    /// - Reset value: `0`
    #[doc(alias = "LIFETIME_DATA_BLOCK_3")]
    pub fn lifetime_data_block_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, LifetimeDataBlock3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 98;
        ::device_driver::RegisterOperation::new(self, address as u8, LifetimeDataBlock3::default)
    }
    /// Register operation:
    /// - Address: `99`
    /// - Reset value: `0`
    #[doc(alias = "LIFETIME_DATA_BLOCK_4")]
    pub fn lifetime_data_block_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, LifetimeDataBlock4, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 99;
        ::device_driver::RegisterOperation::new(self, address as u8, LifetimeDataBlock4::default)
    }
    /// Register operation:
    /// - Address: `100`
    /// - Reset value: `0`
    #[doc(alias = "LIFETIME_DATA_BLOCK_5")]
    pub fn lifetime_data_block_5(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, LifetimeDataBlock5, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 100;
        ::device_driver::RegisterOperation::new(self, address as u8, LifetimeDataBlock5::default)
    }
    /// Buffer operation:
    /// - Address: `112`
    #[doc(alias = "MANUFACTURE_INFO")]
    pub fn manufacture_info(&mut self) -> ::device_driver::BufferOperation<'_, Self, u8, ::device_driver::RW>
    where
        I: ::device_driver::BufferInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 112;
        ::device_driver::BufferOperation::new(self, address as u8)
    }
    /// Register operation:
    /// - Address: `113`
    /// - Reset value: `0`
    #[doc(alias = "DA_STATUS_1")]
    pub fn da_status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, DaStatus1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 113;
        ::device_driver::RegisterOperation::new(self, address as u8, DaStatus1::default)
    }
    /// Register operation:
    /// - Address: `114`
    /// - Reset value: `0`
    #[doc(alias = "DA_STATUS_2")]
    pub fn da_status_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, DaStatus2, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 114;
        ::device_driver::RegisterOperation::new(self, address as u8, DaStatus2::default)
    }
    /// Register operation:
    /// - Address: `115`
    /// - Reset value: `0`
    #[doc(alias = "GAUGE_STATUS_1")]
    pub fn gauge_status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, GaugeStatus1, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 115;
        ::device_driver::RegisterOperation::new(self, address as u8, GaugeStatus1::default)
    }
    /// Register operation:
    /// - Address: `116`
    /// - Reset value: `0`
    #[doc(alias = "GAUGE_STATUS_2")]
    pub fn gauge_status_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, GaugeStatus2, u8, ::device_driver::RW, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 116;
        ::device_driver::RegisterOperation::new(self, address as u8, GaugeStatus2::default)
    }
    /// Register operation:
    /// - Address: `117`
    /// - Reset value: `0`
    #[doc(alias = "GAUGE_STATUS_3")]
    pub fn gauge_status_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, GaugeStatus3, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 117;
        ::device_driver::RegisterOperation::new(self, address as u8, GaugeStatus3::default)
    }
    /// Register operation:
    /// - Address: `118`
    /// - Reset value: `0`
    #[doc(alias = "CB_STATUS")]
    pub fn cb_status(&mut self) -> ::device_driver::RegisterOperation<'_, Self, CbStatus, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 118;
        ::device_driver::RegisterOperation::new(self, address as u8, CbStatus::default)
    }
    /// Register operation:
    /// - Address: `119`
    /// - Reset value: `0`
    #[doc(alias = "STATE_OF_HEALTH")]
    pub fn state_of_health(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, StateOfHealth, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 119;
        ::device_driver::RegisterOperation::new(self, address as u8, StateOfHealth::default)
    }
    /// Register operation:
    /// - Address: `120`
    /// - Reset value: `0`
    #[doc(alias = "FILTER_CAPACITY")]
    pub fn filter_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, Self, FilterCapacity, u8, ::device_driver::RO, ()>
    where
        I: ::device_driver::RegisterInterfaceBase<AddressType = u8>,
    {
        let address = self.base_address + 120;
        ::device_driver::RegisterOperation::new(self, address as u8, FilterCapacity::default)
    }
}
impl<I> ::device_driver::Block for Device<I> {
    type Interface = I;
    type RegisterAddressType = u8;
    type CommandAddressType = u32;
    type BufferAddressType = u8;
    type RegisterAddressMode = ();
    fn interface(&mut self) -> &mut Self::Interface {
        &mut self.interface
    }
}
#[doc(alias = "FILTER_CAPACITY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct FilterCapacity {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for FilterCapacity {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl FilterCapacity {
    /// `15:0` - Read the `filt_rem_cap` field.
    ///
    #[doc(alias = "FILT_REM_CAP")]
    #[must_use]
    pub fn filt_rem_cap(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `filt_rem_energy` field.
    ///
    #[doc(alias = "FILT_REM_ENERGY")]
    #[must_use]
    pub fn filt_rem_energy(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `filt_full_chg_cap` field.
    ///
    #[doc(alias = "FILT_FULL_CHG_CAP")]
    #[must_use]
    pub fn filt_full_chg_cap(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `filt_full_chg_energy` field.
    ///
    #[doc(alias = "FILT_FULL_CHG_ENERGY")]
    #[must_use]
    pub fn filt_full_chg_energy(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for FilterCapacity {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for FilterCapacity {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<FilterCapacity> for [u8; 8] {
    fn from(val: FilterCapacity) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for FilterCapacity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("FilterCapacity");
        d.field("filt_rem_cap", &self.filt_rem_cap());
        d.field("filt_rem_energy", &self.filt_rem_energy());
        d.field("filt_full_chg_cap", &self.filt_full_chg_cap());
        d.field("filt_full_chg_energy", &self.filt_full_chg_energy());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for FilterCapacity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FilterCapacity {{ ");
        defmt::write!(f, "filt_rem_cap: {=u16}, ", &self.filt_rem_cap());
        defmt::write!(f, "filt_rem_energy: {=u16}, ", &self.filt_rem_energy());
        defmt::write!(f, "filt_full_chg_cap: {=u16}, ", &self.filt_full_chg_cap());
        defmt::write!(f, "filt_full_chg_energy: {=u16}, ", &self.filt_full_chg_energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for FilterCapacity {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for FilterCapacity {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for FilterCapacity {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for FilterCapacity {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for FilterCapacity {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for FilterCapacity {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for FilterCapacity {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "STATE_OF_HEALTH")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct StateOfHealth {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for StateOfHealth {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl StateOfHealth {
    /// `15:0` - Read the `soh_fcc` field.
    ///
    #[doc(alias = "SOH_FCC")]
    #[must_use]
    pub fn soh_fcc(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `soh_energy` field.
    ///
    #[doc(alias = "SOH_ENERGY")]
    #[must_use]
    pub fn soh_energy(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for StateOfHealth {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for StateOfHealth {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<StateOfHealth> for [u8; 4] {
    fn from(val: StateOfHealth) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for StateOfHealth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("StateOfHealth");
        d.field("soh_fcc", &self.soh_fcc());
        d.field("soh_energy", &self.soh_energy());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for StateOfHealth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "StateOfHealth {{ ");
        defmt::write!(f, "soh_fcc: {=u16}, ", &self.soh_fcc());
        defmt::write!(f, "soh_energy: {=u16}, ", &self.soh_energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for StateOfHealth {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for StateOfHealth {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for StateOfHealth {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for StateOfHealth {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for StateOfHealth {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for StateOfHealth {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for StateOfHealth {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CB_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CbStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for CbStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl CbStatus {
    /// `15:0` - Read the `cb_time_0` field.
    ///
    #[doc(alias = "CB_TIME_0")]
    #[must_use]
    pub fn cb_time_0(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `cb_time_1` field.
    ///
    #[doc(alias = "CB_TIME_1")]
    #[must_use]
    pub fn cb_time_1(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cb_time_2` field.
    ///
    #[doc(alias = "CB_TIME_2")]
    #[must_use]
    pub fn cb_time_2(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cb_time_3` field.
    ///
    #[doc(alias = "CB_TIME_3")]
    #[must_use]
    pub fn cb_time_3(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for CbStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for CbStatus {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<CbStatus> for [u8; 8] {
    fn from(val: CbStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CbStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CbStatus");
        d.field("cb_time_0", &self.cb_time_0());
        d.field("cb_time_1", &self.cb_time_1());
        d.field("cb_time_2", &self.cb_time_2());
        d.field("cb_time_3", &self.cb_time_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for CbStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CbStatus {{ ");
        defmt::write!(f, "cb_time_0: {=u16}, ", &self.cb_time_0());
        defmt::write!(f, "cb_time_1: {=u16}, ", &self.cb_time_1());
        defmt::write!(f, "cb_time_2: {=u16}, ", &self.cb_time_2());
        defmt::write!(f, "cb_time_3: {=u16}, ", &self.cb_time_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CbStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CbStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CbStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CbStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CbStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CbStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CbStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "GAUGE_STATUS_3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct GaugeStatus3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 24],
}
unsafe impl ::device_driver::Fieldset for GaugeStatus3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 24] };
}
impl GaugeStatus3 {
    /// `15:0` - Read the `qmax_0` field.
    ///
    #[doc(alias = "QMAX_0")]
    #[must_use]
    pub fn qmax_0(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `qmax_1` field.
    ///
    #[doc(alias = "QMAX_1")]
    #[must_use]
    pub fn qmax_1(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `qmax_2` field.
    ///
    #[doc(alias = "QMAX_2")]
    #[must_use]
    pub fn qmax_2(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `qmax_3` field.
    ///
    #[doc(alias = "QMAX_3")]
    #[must_use]
    pub fn qmax_3(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `qmax_dod_0_0` field.
    ///
    #[doc(alias = "QMAX_DOD0_0")]
    #[must_use]
    pub fn qmax_dod_0_0(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `qmax_dod_0_1` field.
    ///
    #[doc(alias = "QMAX_DOD0_1")]
    #[must_use]
    pub fn qmax_dod_0_1(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `qmax_dod_0_2` field.
    ///
    #[doc(alias = "QMAX_DOD0_2")]
    #[must_use]
    pub fn qmax_dod_0_2(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `qmax_dod_0_3` field.
    ///
    #[doc(alias = "QMAX_DOD0_3")]
    #[must_use]
    pub fn qmax_dod_0_3(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `qmax_passed_q` field.
    ///
    #[doc(alias = "QMAX_PASSED_Q")]
    #[must_use]
    pub fn qmax_passed_q(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `qmax_time` field.
    ///
    #[doc(alias = "QMAX_TIME")]
    #[must_use]
    pub fn qmax_time(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `temp_k_factor` field.
    ///
    #[doc(alias = "TEMP_K_FACTOR")]
    #[must_use]
    pub fn temp_k_factor(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `temp_a_factor` field.
    ///
    #[doc(alias = "TEMP_A_FACTOR")]
    #[must_use]
    pub fn temp_a_factor(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for GaugeStatus3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 24]> for GaugeStatus3 {
    fn from(bits: [u8; 24]) -> Self {
        Self { bits }
    }
}
impl From<GaugeStatus3> for [u8; 24] {
    fn from(val: GaugeStatus3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for GaugeStatus3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("GaugeStatus3");
        d.field("qmax_0", &self.qmax_0());
        d.field("qmax_1", &self.qmax_1());
        d.field("qmax_2", &self.qmax_2());
        d.field("qmax_3", &self.qmax_3());
        d.field("qmax_dod_0_0", &self.qmax_dod_0_0());
        d.field("qmax_dod_0_1", &self.qmax_dod_0_1());
        d.field("qmax_dod_0_2", &self.qmax_dod_0_2());
        d.field("qmax_dod_0_3", &self.qmax_dod_0_3());
        d.field("qmax_passed_q", &self.qmax_passed_q());
        d.field("qmax_time", &self.qmax_time());
        d.field("temp_k_factor", &self.temp_k_factor());
        d.field("temp_a_factor", &self.temp_a_factor());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for GaugeStatus3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GaugeStatus3 {{ ");
        defmt::write!(f, "qmax_0: {=u16}, ", &self.qmax_0());
        defmt::write!(f, "qmax_1: {=u16}, ", &self.qmax_1());
        defmt::write!(f, "qmax_2: {=u16}, ", &self.qmax_2());
        defmt::write!(f, "qmax_3: {=u16}, ", &self.qmax_3());
        defmt::write!(f, "qmax_dod_0_0: {=u16}, ", &self.qmax_dod_0_0());
        defmt::write!(f, "qmax_dod_0_1: {=u16}, ", &self.qmax_dod_0_1());
        defmt::write!(f, "qmax_dod_0_2: {=u16}, ", &self.qmax_dod_0_2());
        defmt::write!(f, "qmax_dod_0_3: {=u16}, ", &self.qmax_dod_0_3());
        defmt::write!(f, "qmax_passed_q: {=u16}, ", &self.qmax_passed_q());
        defmt::write!(f, "qmax_time: {=u16}, ", &self.qmax_time());
        defmt::write!(f, "temp_k_factor: {=u16}, ", &self.temp_k_factor());
        defmt::write!(f, "temp_a_factor: {=u16}, ", &self.temp_a_factor());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for GaugeStatus3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for GaugeStatus3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for GaugeStatus3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for GaugeStatus3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for GaugeStatus3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for GaugeStatus3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for GaugeStatus3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "GAUGE_STATUS_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct GaugeStatus2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for GaugeStatus2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl GaugeStatus2 {
    /// `7:0` - Read the `pack_grid` field.
    ///
    #[doc(alias = "PACK_GRID")]
    #[must_use]
    pub fn pack_grid(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `9:8` - Read the `q_max_status` field.
    ///
    #[doc(alias = "Q_MAX_STATUS")]
    #[must_use]
    pub fn q_max_status(&self) -> QMaxStatus {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `iten` field.
    ///
    #[doc(alias = "ITEN")]
    #[must_use]
    pub fn iten(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `qmax_field_updated` field.
    ///
    #[doc(alias = "QMAX_FIELD_UPDATED")]
    #[must_use]
    pub fn qmax_field_updated(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `23:16` - Read the `cell_grid_0` field.
    ///
    #[doc(alias = "CELL_GRID_0")]
    #[must_use]
    pub fn cell_grid_0(&self) -> u8 {
        let start = 16;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:24` - Read the `cell_grid_1` field.
    ///
    #[doc(alias = "CELL_GRID_1")]
    #[must_use]
    pub fn cell_grid_1(&self) -> u8 {
        let start = 24;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `39:32` - Read the `cell_grid_2` field.
    ///
    #[doc(alias = "CELL_GRID_2")]
    #[must_use]
    pub fn cell_grid_2(&self) -> u8 {
        let start = 32;
        let end = 39;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:40` - Read the `cell_grid_3` field.
    ///
    #[doc(alias = "CELL_GRID_3")]
    #[must_use]
    pub fn cell_grid_3(&self) -> u8 {
        let start = 40;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:48` - Read the `state_time` field.
    ///
    #[doc(alias = "STATE_TIME")]
    #[must_use]
    pub fn state_time(&self) -> u32 {
        let start = 48;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u32, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `dod_0_0` field.
    ///
    #[doc(alias = "DOD0_0")]
    #[must_use]
    pub fn dod_0_0(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `dod_0_1` field.
    ///
    #[doc(alias = "DOD0_1")]
    #[must_use]
    pub fn dod_0_1(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `dod_0_2` field.
    ///
    #[doc(alias = "DOD0_2")]
    #[must_use]
    pub fn dod_0_2(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `dod_0_3` field.
    ///
    #[doc(alias = "DOD0_3")]
    #[must_use]
    pub fn dod_0_3(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `dod_0_passed_q` field.
    ///
    #[doc(alias = "DOD0_PASSED_Q")]
    #[must_use]
    pub fn dod_0_passed_q(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `dod_0_passed_e` field.
    ///
    #[doc(alias = "DOD0_PASSED_E")]
    #[must_use]
    pub fn dod_0_passed_e(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `dod_0_time` field.
    ///
    #[doc(alias = "DOD0_TIME")]
    #[must_use]
    pub fn dod_0_time(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `dodeoc_0` field.
    ///
    #[doc(alias = "DODEOC_0")]
    #[must_use]
    pub fn dodeoc_0(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `dodeoc_1` field.
    ///
    #[doc(alias = "DODEOC_1")]
    #[must_use]
    pub fn dodeoc_1(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `dodeoc_2` field.
    ///
    #[doc(alias = "DODEOC_2")]
    #[must_use]
    pub fn dodeoc_2(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `dodeoc_3` field.
    ///
    #[doc(alias = "DODEOC_3")]
    #[must_use]
    pub fn dodeoc_3(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Set the `dod_0_0` field.
    ///
    #[doc(alias = "DOD0_0")]
    pub fn set_dod_0_0(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for GaugeStatus2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for GaugeStatus2 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<GaugeStatus2> for [u8; 32] {
    fn from(val: GaugeStatus2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for GaugeStatus2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("GaugeStatus2");
        d.field("pack_grid", &self.pack_grid());
        d.field("q_max_status", &self.q_max_status());
        d.field("iten", &self.iten());
        d.field("qmax_field_updated", &self.qmax_field_updated());
        d.field("cell_grid_0", &self.cell_grid_0());
        d.field("cell_grid_1", &self.cell_grid_1());
        d.field("cell_grid_2", &self.cell_grid_2());
        d.field("cell_grid_3", &self.cell_grid_3());
        d.field("state_time", &self.state_time());
        d.field("dod_0_0", &self.dod_0_0());
        d.field("dod_0_1", &self.dod_0_1());
        d.field("dod_0_2", &self.dod_0_2());
        d.field("dod_0_3", &self.dod_0_3());
        d.field("dod_0_passed_q", &self.dod_0_passed_q());
        d.field("dod_0_passed_e", &self.dod_0_passed_e());
        d.field("dod_0_time", &self.dod_0_time());
        d.field("dodeoc_0", &self.dodeoc_0());
        d.field("dodeoc_1", &self.dodeoc_1());
        d.field("dodeoc_2", &self.dodeoc_2());
        d.field("dodeoc_3", &self.dodeoc_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for GaugeStatus2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GaugeStatus2 {{ ");
        defmt::write!(f, "pack_grid: {=u8}, ", &self.pack_grid());
        defmt::write!(f, "q_max_status: {}, ", &self.q_max_status());
        defmt::write!(f, "iten: {=bool}, ", &self.iten());
        defmt::write!(f, "qmax_field_updated: {=bool}, ", &self.qmax_field_updated());
        defmt::write!(f, "cell_grid_0: {=u8}, ", &self.cell_grid_0());
        defmt::write!(f, "cell_grid_1: {=u8}, ", &self.cell_grid_1());
        defmt::write!(f, "cell_grid_2: {=u8}, ", &self.cell_grid_2());
        defmt::write!(f, "cell_grid_3: {=u8}, ", &self.cell_grid_3());
        defmt::write!(f, "state_time: {=u32}, ", &self.state_time());
        defmt::write!(f, "dod_0_0: {=u16}, ", &self.dod_0_0());
        defmt::write!(f, "dod_0_1: {=u16}, ", &self.dod_0_1());
        defmt::write!(f, "dod_0_2: {=u16}, ", &self.dod_0_2());
        defmt::write!(f, "dod_0_3: {=u16}, ", &self.dod_0_3());
        defmt::write!(f, "dod_0_passed_q: {=u16}, ", &self.dod_0_passed_q());
        defmt::write!(f, "dod_0_passed_e: {=u16}, ", &self.dod_0_passed_e());
        defmt::write!(f, "dod_0_time: {=u16}, ", &self.dod_0_time());
        defmt::write!(f, "dodeoc_0: {=u16}, ", &self.dodeoc_0());
        defmt::write!(f, "dodeoc_1: {=u16}, ", &self.dodeoc_1());
        defmt::write!(f, "dodeoc_2: {=u16}, ", &self.dodeoc_2());
        defmt::write!(f, "dodeoc_3: {=u16}, ", &self.dodeoc_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for GaugeStatus2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for GaugeStatus2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for GaugeStatus2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for GaugeStatus2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for GaugeStatus2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for GaugeStatus2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for GaugeStatus2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "GAUGE_STATUS_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct GaugeStatus1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for GaugeStatus1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl GaugeStatus1 {
    /// `15:0` - Read the `true_rem_q` field.
    ///
    #[doc(alias = "TRUE_REM_Q")]
    #[must_use]
    pub fn true_rem_q(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `true_rem_e` field.
    ///
    #[doc(alias = "TRUE_REM_E")]
    #[must_use]
    pub fn true_rem_e(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `initial_q` field.
    ///
    #[doc(alias = "INITIAL_Q")]
    #[must_use]
    pub fn initial_q(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `initial_e` field.
    ///
    #[doc(alias = "INITIAL_E")]
    #[must_use]
    pub fn initial_e(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `true_fcc_q` field.
    ///
    #[doc(alias = "TRUE_FCC_Q")]
    #[must_use]
    pub fn true_fcc_q(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `true_fcc_e` field.
    ///
    #[doc(alias = "TRUE_FCC_E")]
    #[must_use]
    pub fn true_fcc_e(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `t_sim` field.
    ///
    #[doc(alias = "T_SIM")]
    #[must_use]
    pub fn t_sim(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `t_ambient` field.
    ///
    #[doc(alias = "T_AMBIENT")]
    #[must_use]
    pub fn t_ambient(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `ra_scale_0` field.
    ///
    #[doc(alias = "RA_SCALE_0")]
    #[must_use]
    pub fn ra_scale_0(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `ra_scale_1` field.
    ///
    #[doc(alias = "RA_SCALE_1")]
    #[must_use]
    pub fn ra_scale_1(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `ra_scale_2` field.
    ///
    #[doc(alias = "RA_SCALE_2")]
    #[must_use]
    pub fn ra_scale_2(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `ra_scale_3` field.
    ///
    #[doc(alias = "RA_SCALE_3")]
    #[must_use]
    pub fn ra_scale_3(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `comp_res_0` field.
    ///
    #[doc(alias = "COMP_RES_0")]
    #[must_use]
    pub fn comp_res_0(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `comp_res_1` field.
    ///
    #[doc(alias = "COMP_RES_1")]
    #[must_use]
    pub fn comp_res_1(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `comp_res_2` field.
    ///
    #[doc(alias = "COMP_RES_2")]
    #[must_use]
    pub fn comp_res_2(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `comp_res_3` field.
    ///
    #[doc(alias = "COMP_RES_3")]
    #[must_use]
    pub fn comp_res_3(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for GaugeStatus1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for GaugeStatus1 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<GaugeStatus1> for [u8; 32] {
    fn from(val: GaugeStatus1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for GaugeStatus1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("GaugeStatus1");
        d.field("true_rem_q", &self.true_rem_q());
        d.field("true_rem_e", &self.true_rem_e());
        d.field("initial_q", &self.initial_q());
        d.field("initial_e", &self.initial_e());
        d.field("true_fcc_q", &self.true_fcc_q());
        d.field("true_fcc_e", &self.true_fcc_e());
        d.field("t_sim", &self.t_sim());
        d.field("t_ambient", &self.t_ambient());
        d.field("ra_scale_0", &self.ra_scale_0());
        d.field("ra_scale_1", &self.ra_scale_1());
        d.field("ra_scale_2", &self.ra_scale_2());
        d.field("ra_scale_3", &self.ra_scale_3());
        d.field("comp_res_0", &self.comp_res_0());
        d.field("comp_res_1", &self.comp_res_1());
        d.field("comp_res_2", &self.comp_res_2());
        d.field("comp_res_3", &self.comp_res_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for GaugeStatus1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GaugeStatus1 {{ ");
        defmt::write!(f, "true_rem_q: {=u16}, ", &self.true_rem_q());
        defmt::write!(f, "true_rem_e: {=u16}, ", &self.true_rem_e());
        defmt::write!(f, "initial_q: {=u16}, ", &self.initial_q());
        defmt::write!(f, "initial_e: {=u16}, ", &self.initial_e());
        defmt::write!(f, "true_fcc_q: {=u16}, ", &self.true_fcc_q());
        defmt::write!(f, "true_fcc_e: {=u16}, ", &self.true_fcc_e());
        defmt::write!(f, "t_sim: {=u16}, ", &self.t_sim());
        defmt::write!(f, "t_ambient: {=u16}, ", &self.t_ambient());
        defmt::write!(f, "ra_scale_0: {=u16}, ", &self.ra_scale_0());
        defmt::write!(f, "ra_scale_1: {=u16}, ", &self.ra_scale_1());
        defmt::write!(f, "ra_scale_2: {=u16}, ", &self.ra_scale_2());
        defmt::write!(f, "ra_scale_3: {=u16}, ", &self.ra_scale_3());
        defmt::write!(f, "comp_res_0: {=u16}, ", &self.comp_res_0());
        defmt::write!(f, "comp_res_1: {=u16}, ", &self.comp_res_1());
        defmt::write!(f, "comp_res_2: {=u16}, ", &self.comp_res_2());
        defmt::write!(f, "comp_res_3: {=u16}, ", &self.comp_res_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for GaugeStatus1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for GaugeStatus1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for GaugeStatus1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for GaugeStatus1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for GaugeStatus1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for GaugeStatus1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for GaugeStatus1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "DA_STATUS_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DaStatus2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 14],
}
unsafe impl ::device_driver::Fieldset for DaStatus2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 14] };
}
impl DaStatus2 {
    /// `15:0` - Read the `int_temp` field.
    ///
    #[doc(alias = "INT_TEMP")]
    #[must_use]
    pub fn int_temp(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `ts_1_temp` field.
    ///
    #[doc(alias = "TS1_TEMP")]
    #[must_use]
    pub fn ts_1_temp(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `ts_2_temp` field.
    ///
    #[doc(alias = "TS2_TEMP")]
    #[must_use]
    pub fn ts_2_temp(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `ts_3_temp` field.
    ///
    #[doc(alias = "TS3_TEMP")]
    #[must_use]
    pub fn ts_3_temp(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `ts_4_temp` field.
    ///
    #[doc(alias = "TS4_TEMP")]
    #[must_use]
    pub fn ts_4_temp(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `cell_temp` field.
    ///
    #[doc(alias = "CELL_TEMP")]
    #[must_use]
    pub fn cell_temp(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `fet_temp` field.
    ///
    #[doc(alias = "FET_TEMP")]
    #[must_use]
    pub fn fet_temp(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for DaStatus2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 14]> for DaStatus2 {
    fn from(bits: [u8; 14]) -> Self {
        Self { bits }
    }
}
impl From<DaStatus2> for [u8; 14] {
    fn from(val: DaStatus2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DaStatus2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DaStatus2");
        d.field("int_temp", &self.int_temp());
        d.field("ts_1_temp", &self.ts_1_temp());
        d.field("ts_2_temp", &self.ts_2_temp());
        d.field("ts_3_temp", &self.ts_3_temp());
        d.field("ts_4_temp", &self.ts_4_temp());
        d.field("cell_temp", &self.cell_temp());
        d.field("fet_temp", &self.fet_temp());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for DaStatus2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DaStatus2 {{ ");
        defmt::write!(f, "int_temp: {=u16}, ", &self.int_temp());
        defmt::write!(f, "ts_1_temp: {=u16}, ", &self.ts_1_temp());
        defmt::write!(f, "ts_2_temp: {=u16}, ", &self.ts_2_temp());
        defmt::write!(f, "ts_3_temp: {=u16}, ", &self.ts_3_temp());
        defmt::write!(f, "ts_4_temp: {=u16}, ", &self.ts_4_temp());
        defmt::write!(f, "cell_temp: {=u16}, ", &self.cell_temp());
        defmt::write!(f, "fet_temp: {=u16}, ", &self.fet_temp());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DaStatus2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DaStatus2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DaStatus2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DaStatus2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DaStatus2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DaStatus2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DaStatus2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "DA_STATUS_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DaStatus1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for DaStatus1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl DaStatus1 {
    /// `15:0` - Read the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    #[must_use]
    pub fn cell_voltage_1(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    #[must_use]
    pub fn cell_voltage_2(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    #[must_use]
    pub fn cell_voltage_3(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    #[must_use]
    pub fn cell_voltage_4(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `bat_voltage` field.
    ///
    #[doc(alias = "BAT_VOLTAGE")]
    #[must_use]
    pub fn bat_voltage(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `pack_voltage` field.
    ///
    #[doc(alias = "PACK_VOLTAGE")]
    #[must_use]
    pub fn pack_voltage(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `cell_current_1` field.
    ///
    #[doc(alias = "CELL_CURRENT_1")]
    #[must_use]
    pub fn cell_current_1(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `cell_current_2` field.
    ///
    #[doc(alias = "CELL_CURRENT_2")]
    #[must_use]
    pub fn cell_current_2(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `cell_current_3` field.
    ///
    #[doc(alias = "CELL_CURRENT_3")]
    #[must_use]
    pub fn cell_current_3(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `cell_current_4` field.
    ///
    #[doc(alias = "CELL_CURRENT_4")]
    #[must_use]
    pub fn cell_current_4(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `cell_pwr_1` field.
    ///
    #[doc(alias = "CELL_PWR_1")]
    #[must_use]
    pub fn cell_pwr_1(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `cell_pwr_2` field.
    ///
    #[doc(alias = "CELL_PWR_2")]
    #[must_use]
    pub fn cell_pwr_2(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `cell_pwr_3` field.
    ///
    #[doc(alias = "CELL_PWR_3")]
    #[must_use]
    pub fn cell_pwr_3(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `cell_pwr_4` field.
    ///
    #[doc(alias = "CELL_PWR_4")]
    #[must_use]
    pub fn cell_pwr_4(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `total_pwr` field.
    ///
    #[doc(alias = "TOTAL_PWR")]
    #[must_use]
    pub fn total_pwr(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `avg_pwr` field.
    ///
    #[doc(alias = "AVG_PWR")]
    #[must_use]
    pub fn avg_pwr(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for DaStatus1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for DaStatus1 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<DaStatus1> for [u8; 32] {
    fn from(val: DaStatus1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DaStatus1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DaStatus1");
        d.field("cell_voltage_1", &self.cell_voltage_1());
        d.field("cell_voltage_2", &self.cell_voltage_2());
        d.field("cell_voltage_3", &self.cell_voltage_3());
        d.field("cell_voltage_4", &self.cell_voltage_4());
        d.field("bat_voltage", &self.bat_voltage());
        d.field("pack_voltage", &self.pack_voltage());
        d.field("cell_current_1", &self.cell_current_1());
        d.field("cell_current_2", &self.cell_current_2());
        d.field("cell_current_3", &self.cell_current_3());
        d.field("cell_current_4", &self.cell_current_4());
        d.field("cell_pwr_1", &self.cell_pwr_1());
        d.field("cell_pwr_2", &self.cell_pwr_2());
        d.field("cell_pwr_3", &self.cell_pwr_3());
        d.field("cell_pwr_4", &self.cell_pwr_4());
        d.field("total_pwr", &self.total_pwr());
        d.field("avg_pwr", &self.avg_pwr());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for DaStatus1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DaStatus1 {{ ");
        defmt::write!(f, "cell_voltage_1: {=u16}, ", &self.cell_voltage_1());
        defmt::write!(f, "cell_voltage_2: {=u16}, ", &self.cell_voltage_2());
        defmt::write!(f, "cell_voltage_3: {=u16}, ", &self.cell_voltage_3());
        defmt::write!(f, "cell_voltage_4: {=u16}, ", &self.cell_voltage_4());
        defmt::write!(f, "bat_voltage: {=u16}, ", &self.bat_voltage());
        defmt::write!(f, "pack_voltage: {=u16}, ", &self.pack_voltage());
        defmt::write!(f, "cell_current_1: {=u16}, ", &self.cell_current_1());
        defmt::write!(f, "cell_current_2: {=u16}, ", &self.cell_current_2());
        defmt::write!(f, "cell_current_3: {=u16}, ", &self.cell_current_3());
        defmt::write!(f, "cell_current_4: {=u16}, ", &self.cell_current_4());
        defmt::write!(f, "cell_pwr_1: {=u16}, ", &self.cell_pwr_1());
        defmt::write!(f, "cell_pwr_2: {=u16}, ", &self.cell_pwr_2());
        defmt::write!(f, "cell_pwr_3: {=u16}, ", &self.cell_pwr_3());
        defmt::write!(f, "cell_pwr_4: {=u16}, ", &self.cell_pwr_4());
        defmt::write!(f, "total_pwr: {=u16}, ", &self.total_pwr());
        defmt::write!(f, "avg_pwr: {=u16}, ", &self.avg_pwr());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DaStatus1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DaStatus1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DaStatus1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DaStatus1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DaStatus1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DaStatus1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DaStatus1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "LIFETIME_DATA_BLOCK_5")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct LifetimeDataBlock5 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for LifetimeDataBlock5 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl LifetimeDataBlock5 {
    /// `15:0` - Read the `num_ascc_events` field.
    ///
    #[doc(alias = "NUM_ASCC_EVENTS")]
    #[must_use]
    pub fn num_ascc_events(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `last_ascc_event` field.
    ///
    #[doc(alias = "LAST_ASCC_EVENT")]
    #[must_use]
    pub fn last_ascc_event(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `num_otc_events` field.
    ///
    #[doc(alias = "NUM_OTC_EVENTS")]
    #[must_use]
    pub fn num_otc_events(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `last_otc_event` field.
    ///
    #[doc(alias = "LAST_OTC_EVENT")]
    #[must_use]
    pub fn last_otc_event(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `num_otd_event` field.
    ///
    #[doc(alias = "NUM_OTD_EVENT")]
    #[must_use]
    pub fn num_otd_event(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `last_otd_event` field.
    ///
    #[doc(alias = "LAST_OTD_EVENT")]
    #[must_use]
    pub fn last_otd_event(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `num_otf_events` field.
    ///
    #[doc(alias = "NUM_OTF_EVENTS")]
    #[must_use]
    pub fn num_otf_events(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `last_otf_event` field.
    ///
    #[doc(alias = "LAST_OTF_EVENT")]
    #[must_use]
    pub fn last_otf_event(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `num_valid_chg_term` field.
    ///
    #[doc(alias = "NUM_VALID_CHG_TERM")]
    #[must_use]
    pub fn num_valid_chg_term(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `last_valid_chg_term` field.
    ///
    #[doc(alias = "LAST_VALID_CHG_TERM")]
    #[must_use]
    pub fn last_valid_chg_term(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `num_qmax_updates` field.
    ///
    #[doc(alias = "NUM_QMAX_UPDATES")]
    #[must_use]
    pub fn num_qmax_updates(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `last_qmax_update` field.
    ///
    #[doc(alias = "LAST_QMAX_UPDATE")]
    #[must_use]
    pub fn last_qmax_update(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `num_ra_updates` field.
    ///
    #[doc(alias = "NUM_RA_UPDATES")]
    #[must_use]
    pub fn num_ra_updates(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `last_ra_update` field.
    ///
    #[doc(alias = "LAST_RA_UPDATE")]
    #[must_use]
    pub fn last_ra_update(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `num_ra_disable` field.
    ///
    #[doc(alias = "NUM_RA_DISABLE")]
    #[must_use]
    pub fn num_ra_disable(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `last_ra_disable` field.
    ///
    #[doc(alias = "LAST_RA_DISABLE")]
    #[must_use]
    pub fn last_ra_disable(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for LifetimeDataBlock5 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for LifetimeDataBlock5 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<LifetimeDataBlock5> for [u8; 32] {
    fn from(val: LifetimeDataBlock5) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for LifetimeDataBlock5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("LifetimeDataBlock5");
        d.field("num_ascc_events", &self.num_ascc_events());
        d.field("last_ascc_event", &self.last_ascc_event());
        d.field("num_otc_events", &self.num_otc_events());
        d.field("last_otc_event", &self.last_otc_event());
        d.field("num_otd_event", &self.num_otd_event());
        d.field("last_otd_event", &self.last_otd_event());
        d.field("num_otf_events", &self.num_otf_events());
        d.field("last_otf_event", &self.last_otf_event());
        d.field("num_valid_chg_term", &self.num_valid_chg_term());
        d.field("last_valid_chg_term", &self.last_valid_chg_term());
        d.field("num_qmax_updates", &self.num_qmax_updates());
        d.field("last_qmax_update", &self.last_qmax_update());
        d.field("num_ra_updates", &self.num_ra_updates());
        d.field("last_ra_update", &self.last_ra_update());
        d.field("num_ra_disable", &self.num_ra_disable());
        d.field("last_ra_disable", &self.last_ra_disable());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for LifetimeDataBlock5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LifetimeDataBlock5 {{ ");
        defmt::write!(f, "num_ascc_events: {=u16}, ", &self.num_ascc_events());
        defmt::write!(f, "last_ascc_event: {=u16}, ", &self.last_ascc_event());
        defmt::write!(f, "num_otc_events: {=u16}, ", &self.num_otc_events());
        defmt::write!(f, "last_otc_event: {=u16}, ", &self.last_otc_event());
        defmt::write!(f, "num_otd_event: {=u16}, ", &self.num_otd_event());
        defmt::write!(f, "last_otd_event: {=u16}, ", &self.last_otd_event());
        defmt::write!(f, "num_otf_events: {=u16}, ", &self.num_otf_events());
        defmt::write!(f, "last_otf_event: {=u16}, ", &self.last_otf_event());
        defmt::write!(f, "num_valid_chg_term: {=u16}, ", &self.num_valid_chg_term());
        defmt::write!(f, "last_valid_chg_term: {=u16}, ", &self.last_valid_chg_term());
        defmt::write!(f, "num_qmax_updates: {=u16}, ", &self.num_qmax_updates());
        defmt::write!(f, "last_qmax_update: {=u16}, ", &self.last_qmax_update());
        defmt::write!(f, "num_ra_updates: {=u16}, ", &self.num_ra_updates());
        defmt::write!(f, "last_ra_update: {=u16}, ", &self.last_ra_update());
        defmt::write!(f, "num_ra_disable: {=u16}, ", &self.num_ra_disable());
        defmt::write!(f, "last_ra_disable: {=u16}, ", &self.last_ra_disable());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for LifetimeDataBlock5 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for LifetimeDataBlock5 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for LifetimeDataBlock5 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for LifetimeDataBlock5 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for LifetimeDataBlock5 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for LifetimeDataBlock5 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for LifetimeDataBlock5 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "LIFETIME_DATA_BLOCK_4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct LifetimeDataBlock4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for LifetimeDataBlock4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl LifetimeDataBlock4 {
    /// `15:0` - Read the `num_cov_events` field.
    ///
    #[doc(alias = "NUM_COV_EVENTS")]
    #[must_use]
    pub fn num_cov_events(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `last_cov_event` field.
    ///
    #[doc(alias = "LAST_COV_EVENT")]
    #[must_use]
    pub fn last_cov_event(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `num_cuv_events` field.
    ///
    #[doc(alias = "NUM_CUV_EVENTS")]
    #[must_use]
    pub fn num_cuv_events(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `last_cuv_event` field.
    ///
    #[doc(alias = "LAST_CUV_EVENT")]
    #[must_use]
    pub fn last_cuv_event(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `num_ocd_1_event` field.
    ///
    #[doc(alias = "NUM_OCD1_EVENT")]
    #[must_use]
    pub fn num_ocd_1_event(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `last_ocd_1_event` field.
    ///
    #[doc(alias = "LAST_OCD1_EVENT")]
    #[must_use]
    pub fn last_ocd_1_event(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `num_ocd_2_events` field.
    ///
    #[doc(alias = "NUM_OCD2_EVENTS")]
    #[must_use]
    pub fn num_ocd_2_events(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `last_ocd_2_event` field.
    ///
    #[doc(alias = "LAST_OCD2_EVENT")]
    #[must_use]
    pub fn last_ocd_2_event(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `num_occ_1_events` field.
    ///
    #[doc(alias = "NUM_OCC1_EVENTS")]
    #[must_use]
    pub fn num_occ_1_events(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `last_occ_1_event` field.
    ///
    #[doc(alias = "LAST_OCC1_EVENT")]
    #[must_use]
    pub fn last_occ_1_event(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `num_occ_2_events` field.
    ///
    #[doc(alias = "NUM_OCC2_EVENTS")]
    #[must_use]
    pub fn num_occ_2_events(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `last_occ_2_event` field.
    ///
    #[doc(alias = "LAST_OCC2_EVENT")]
    #[must_use]
    pub fn last_occ_2_event(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `num_aold_events` field.
    ///
    #[doc(alias = "NUM_AOLD_EVENTS")]
    #[must_use]
    pub fn num_aold_events(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `last_aold_event` field.
    ///
    #[doc(alias = "LAST_AOLD_EVENT")]
    #[must_use]
    pub fn last_aold_event(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `num_ascd_events` field.
    ///
    #[doc(alias = "NUM_ASCD_EVENTS")]
    #[must_use]
    pub fn num_ascd_events(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `last_ascd_event` field.
    ///
    #[doc(alias = "LAST_ASCD_EVENT")]
    #[must_use]
    pub fn last_ascd_event(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for LifetimeDataBlock4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for LifetimeDataBlock4 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<LifetimeDataBlock4> for [u8; 32] {
    fn from(val: LifetimeDataBlock4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for LifetimeDataBlock4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("LifetimeDataBlock4");
        d.field("num_cov_events", &self.num_cov_events());
        d.field("last_cov_event", &self.last_cov_event());
        d.field("num_cuv_events", &self.num_cuv_events());
        d.field("last_cuv_event", &self.last_cuv_event());
        d.field("num_ocd_1_event", &self.num_ocd_1_event());
        d.field("last_ocd_1_event", &self.last_ocd_1_event());
        d.field("num_ocd_2_events", &self.num_ocd_2_events());
        d.field("last_ocd_2_event", &self.last_ocd_2_event());
        d.field("num_occ_1_events", &self.num_occ_1_events());
        d.field("last_occ_1_event", &self.last_occ_1_event());
        d.field("num_occ_2_events", &self.num_occ_2_events());
        d.field("last_occ_2_event", &self.last_occ_2_event());
        d.field("num_aold_events", &self.num_aold_events());
        d.field("last_aold_event", &self.last_aold_event());
        d.field("num_ascd_events", &self.num_ascd_events());
        d.field("last_ascd_event", &self.last_ascd_event());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for LifetimeDataBlock4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LifetimeDataBlock4 {{ ");
        defmt::write!(f, "num_cov_events: {=u16}, ", &self.num_cov_events());
        defmt::write!(f, "last_cov_event: {=u16}, ", &self.last_cov_event());
        defmt::write!(f, "num_cuv_events: {=u16}, ", &self.num_cuv_events());
        defmt::write!(f, "last_cuv_event: {=u16}, ", &self.last_cuv_event());
        defmt::write!(f, "num_ocd_1_event: {=u16}, ", &self.num_ocd_1_event());
        defmt::write!(f, "last_ocd_1_event: {=u16}, ", &self.last_ocd_1_event());
        defmt::write!(f, "num_ocd_2_events: {=u16}, ", &self.num_ocd_2_events());
        defmt::write!(f, "last_ocd_2_event: {=u16}, ", &self.last_ocd_2_event());
        defmt::write!(f, "num_occ_1_events: {=u16}, ", &self.num_occ_1_events());
        defmt::write!(f, "last_occ_1_event: {=u16}, ", &self.last_occ_1_event());
        defmt::write!(f, "num_occ_2_events: {=u16}, ", &self.num_occ_2_events());
        defmt::write!(f, "last_occ_2_event: {=u16}, ", &self.last_occ_2_event());
        defmt::write!(f, "num_aold_events: {=u16}, ", &self.num_aold_events());
        defmt::write!(f, "last_aold_event: {=u16}, ", &self.last_aold_event());
        defmt::write!(f, "num_ascd_events: {=u16}, ", &self.num_ascd_events());
        defmt::write!(f, "last_ascd_event: {=u16}, ", &self.last_ascd_event());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for LifetimeDataBlock4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for LifetimeDataBlock4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for LifetimeDataBlock4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for LifetimeDataBlock4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for LifetimeDataBlock4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for LifetimeDataBlock4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for LifetimeDataBlock4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "LIFETIME_DATA_BLOCK_3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct LifetimeDataBlock3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 16],
}
unsafe impl ::device_driver::Fieldset for LifetimeDataBlock3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 16] };
}
impl LifetimeDataBlock3 {
    /// `15:0` - Read the `total_fw_runtime` field.
    ///
    #[doc(alias = "TOTAL_FW_RUNTIME")]
    #[must_use]
    pub fn total_fw_runtime(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `time_spent_in_ut` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_UT")]
    #[must_use]
    pub fn time_spent_in_ut(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `time_spent_in_lt` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_LT")]
    #[must_use]
    pub fn time_spent_in_lt(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `time_spent_in_stl` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_STL")]
    #[must_use]
    pub fn time_spent_in_stl(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `time_spent_in_rt` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_RT")]
    #[must_use]
    pub fn time_spent_in_rt(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `time_spent_in_sth` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_STH")]
    #[must_use]
    pub fn time_spent_in_sth(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `time_spent_in_ht` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_HT")]
    #[must_use]
    pub fn time_spent_in_ht(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `time_spent_in_ot` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_OT")]
    #[must_use]
    pub fn time_spent_in_ot(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for LifetimeDataBlock3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 16]> for LifetimeDataBlock3 {
    fn from(bits: [u8; 16]) -> Self {
        Self { bits }
    }
}
impl From<LifetimeDataBlock3> for [u8; 16] {
    fn from(val: LifetimeDataBlock3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for LifetimeDataBlock3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("LifetimeDataBlock3");
        d.field("total_fw_runtime", &self.total_fw_runtime());
        d.field("time_spent_in_ut", &self.time_spent_in_ut());
        d.field("time_spent_in_lt", &self.time_spent_in_lt());
        d.field("time_spent_in_stl", &self.time_spent_in_stl());
        d.field("time_spent_in_rt", &self.time_spent_in_rt());
        d.field("time_spent_in_sth", &self.time_spent_in_sth());
        d.field("time_spent_in_ht", &self.time_spent_in_ht());
        d.field("time_spent_in_ot", &self.time_spent_in_ot());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for LifetimeDataBlock3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LifetimeDataBlock3 {{ ");
        defmt::write!(f, "total_fw_runtime: {=u16}, ", &self.total_fw_runtime());
        defmt::write!(f, "time_spent_in_ut: {=u16}, ", &self.time_spent_in_ut());
        defmt::write!(f, "time_spent_in_lt: {=u16}, ", &self.time_spent_in_lt());
        defmt::write!(f, "time_spent_in_stl: {=u16}, ", &self.time_spent_in_stl());
        defmt::write!(f, "time_spent_in_rt: {=u16}, ", &self.time_spent_in_rt());
        defmt::write!(f, "time_spent_in_sth: {=u16}, ", &self.time_spent_in_sth());
        defmt::write!(f, "time_spent_in_ht: {=u16}, ", &self.time_spent_in_ht());
        defmt::write!(f, "time_spent_in_ot: {=u16}, ", &self.time_spent_in_ot());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for LifetimeDataBlock3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for LifetimeDataBlock3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for LifetimeDataBlock3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for LifetimeDataBlock3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for LifetimeDataBlock3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for LifetimeDataBlock3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for LifetimeDataBlock3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "LIFETIME_DATA_BLOCK_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct LifetimeDataBlock2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for LifetimeDataBlock2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl LifetimeDataBlock2 {
    /// `7:0` - Read the `num_shutdowns` field.
    ///
    #[doc(alias = "NUM_SHUTDOWNS")]
    #[must_use]
    pub fn num_shutdowns(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `num_part_resets` field.
    ///
    #[doc(alias = "NUM_PART_RESETS")]
    #[must_use]
    pub fn num_part_resets(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `23:16` - Read the `num_full_resets` field.
    ///
    #[doc(alias = "NUM_FULL_RESETS")]
    #[must_use]
    pub fn num_full_resets(&self) -> u8 {
        let start = 16;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:24` - Read the `num_wdt_resets` field.
    ///
    #[doc(alias = "NUM_WDT_RESETS")]
    #[must_use]
    pub fn num_wdt_resets(&self) -> u8 {
        let start = 24;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `39:32` - Read the `cb_time_cell_1` field.
    ///
    #[doc(alias = "CB_TIME_CELL_1")]
    #[must_use]
    pub fn cb_time_cell_1(&self) -> u8 {
        let start = 32;
        let end = 39;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:40` - Read the `cb_time_cell_2` field.
    ///
    #[doc(alias = "CB_TIME_CELL_2")]
    #[must_use]
    pub fn cb_time_cell_2(&self) -> u8 {
        let start = 40;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `55:48` - Read the `cb_time_cell_3` field.
    ///
    #[doc(alias = "CB_TIME_CELL_3")]
    #[must_use]
    pub fn cb_time_cell_3(&self) -> u8 {
        let start = 48;
        let end = 55;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:56` - Read the `cb_time_cell_4` field.
    ///
    #[doc(alias = "CB_TIME_CELL_4")]
    #[must_use]
    pub fn cb_time_cell_4(&self) -> u8 {
        let start = 56;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for LifetimeDataBlock2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for LifetimeDataBlock2 {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<LifetimeDataBlock2> for [u8; 8] {
    fn from(val: LifetimeDataBlock2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for LifetimeDataBlock2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("LifetimeDataBlock2");
        d.field("num_shutdowns", &self.num_shutdowns());
        d.field("num_part_resets", &self.num_part_resets());
        d.field("num_full_resets", &self.num_full_resets());
        d.field("num_wdt_resets", &self.num_wdt_resets());
        d.field("cb_time_cell_1", &self.cb_time_cell_1());
        d.field("cb_time_cell_2", &self.cb_time_cell_2());
        d.field("cb_time_cell_3", &self.cb_time_cell_3());
        d.field("cb_time_cell_4", &self.cb_time_cell_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for LifetimeDataBlock2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LifetimeDataBlock2 {{ ");
        defmt::write!(f, "num_shutdowns: {=u8}, ", &self.num_shutdowns());
        defmt::write!(f, "num_part_resets: {=u8}, ", &self.num_part_resets());
        defmt::write!(f, "num_full_resets: {=u8}, ", &self.num_full_resets());
        defmt::write!(f, "num_wdt_resets: {=u8}, ", &self.num_wdt_resets());
        defmt::write!(f, "cb_time_cell_1: {=u8}, ", &self.cb_time_cell_1());
        defmt::write!(f, "cb_time_cell_2: {=u8}, ", &self.cb_time_cell_2());
        defmt::write!(f, "cb_time_cell_3: {=u8}, ", &self.cb_time_cell_3());
        defmt::write!(f, "cb_time_cell_4: {=u8}, ", &self.cb_time_cell_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for LifetimeDataBlock2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for LifetimeDataBlock2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for LifetimeDataBlock2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for LifetimeDataBlock2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for LifetimeDataBlock2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for LifetimeDataBlock2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for LifetimeDataBlock2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "LIFETIME_DATA_BLOCK_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct LifetimeDataBlock1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for LifetimeDataBlock1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl LifetimeDataBlock1 {
    /// `15:0` - Read the `cell_1_max_v` field.
    ///
    #[doc(alias = "CELL_1_MAX_V")]
    #[must_use]
    pub fn cell_1_max_v(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `cell_2_max_v` field.
    ///
    #[doc(alias = "CELL_2_MAX_V")]
    #[must_use]
    pub fn cell_2_max_v(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cell_3_max_v` field.
    ///
    #[doc(alias = "CELL_3_MAX_V")]
    #[must_use]
    pub fn cell_3_max_v(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cell_4_max_v` field.
    ///
    #[doc(alias = "CELL_4_MAX_V")]
    #[must_use]
    pub fn cell_4_max_v(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `cell_1_min_v` field.
    ///
    #[doc(alias = "CELL_1_MIN_V")]
    #[must_use]
    pub fn cell_1_min_v(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `cell_2_min_v` field.
    ///
    #[doc(alias = "CELL_2_MIN_V")]
    #[must_use]
    pub fn cell_2_min_v(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `cell_3_min_v` field.
    ///
    #[doc(alias = "CELL_3_MIN_V")]
    #[must_use]
    pub fn cell_3_min_v(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `cell_4_min_v` field.
    ///
    #[doc(alias = "CELL_4_MIN_V")]
    #[must_use]
    pub fn cell_4_min_v(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `max_delta_cell_v` field.
    ///
    #[doc(alias = "MAX_DELTA_CELL_V")]
    #[must_use]
    pub fn max_delta_cell_v(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `max_charge_a` field.
    ///
    #[doc(alias = "MAX_CHARGE_A")]
    #[must_use]
    pub fn max_charge_a(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `max_discharge_a` field.
    ///
    #[doc(alias = "MAX_DISCHARGE_A")]
    #[must_use]
    pub fn max_discharge_a(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `max_avg_discharge_a` field.
    ///
    #[doc(alias = "MAX_AVG_DISCHARGE_A")]
    #[must_use]
    pub fn max_avg_discharge_a(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `max_avg_discharge_pwr` field.
    ///
    #[doc(alias = "MAX_AVG_DISCHARGE_PWR")]
    #[must_use]
    pub fn max_avg_discharge_pwr(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `215:208` - Read the `max_temp_cell` field.
    ///
    #[doc(alias = "MAX_TEMP_CELL")]
    #[must_use]
    pub fn max_temp_cell(&self) -> u8 {
        let start = 208;
        let end = 215;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:216` - Read the `min_temp_cell` field.
    ///
    #[doc(alias = "MIN_TEMP_CELL")]
    #[must_use]
    pub fn min_temp_cell(&self) -> u8 {
        let start = 216;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `231:224` - Read the `max_delta_cell_temp` field.
    ///
    #[doc(alias = "MAX_DELTA_CELL_TEMP")]
    #[must_use]
    pub fn max_delta_cell_temp(&self) -> u8 {
        let start = 224;
        let end = 231;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:232` - Read the `max_temp_int_sensor` field.
    ///
    #[doc(alias = "MAX_TEMP_INT_SENSOR")]
    #[must_use]
    pub fn max_temp_int_sensor(&self) -> u8 {
        let start = 232;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `247:240` - Read the `min_temp_int_sensor` field.
    ///
    #[doc(alias = "MIN_TEMP_INT_SENSOR")]
    #[must_use]
    pub fn min_temp_int_sensor(&self) -> u8 {
        let start = 240;
        let end = 247;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:248` - Read the `max_temp_fet` field.
    ///
    #[doc(alias = "MAX_TEMP_FET")]
    #[must_use]
    pub fn max_temp_fet(&self) -> u8 {
        let start = 248;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for LifetimeDataBlock1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for LifetimeDataBlock1 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<LifetimeDataBlock1> for [u8; 32] {
    fn from(val: LifetimeDataBlock1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for LifetimeDataBlock1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("LifetimeDataBlock1");
        d.field("cell_1_max_v", &self.cell_1_max_v());
        d.field("cell_2_max_v", &self.cell_2_max_v());
        d.field("cell_3_max_v", &self.cell_3_max_v());
        d.field("cell_4_max_v", &self.cell_4_max_v());
        d.field("cell_1_min_v", &self.cell_1_min_v());
        d.field("cell_2_min_v", &self.cell_2_min_v());
        d.field("cell_3_min_v", &self.cell_3_min_v());
        d.field("cell_4_min_v", &self.cell_4_min_v());
        d.field("max_delta_cell_v", &self.max_delta_cell_v());
        d.field("max_charge_a", &self.max_charge_a());
        d.field("max_discharge_a", &self.max_discharge_a());
        d.field("max_avg_discharge_a", &self.max_avg_discharge_a());
        d.field("max_avg_discharge_pwr", &self.max_avg_discharge_pwr());
        d.field("max_temp_cell", &self.max_temp_cell());
        d.field("min_temp_cell", &self.min_temp_cell());
        d.field("max_delta_cell_temp", &self.max_delta_cell_temp());
        d.field("max_temp_int_sensor", &self.max_temp_int_sensor());
        d.field("min_temp_int_sensor", &self.min_temp_int_sensor());
        d.field("max_temp_fet", &self.max_temp_fet());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for LifetimeDataBlock1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "LifetimeDataBlock1 {{ ");
        defmt::write!(f, "cell_1_max_v: {=u16}, ", &self.cell_1_max_v());
        defmt::write!(f, "cell_2_max_v: {=u16}, ", &self.cell_2_max_v());
        defmt::write!(f, "cell_3_max_v: {=u16}, ", &self.cell_3_max_v());
        defmt::write!(f, "cell_4_max_v: {=u16}, ", &self.cell_4_max_v());
        defmt::write!(f, "cell_1_min_v: {=u16}, ", &self.cell_1_min_v());
        defmt::write!(f, "cell_2_min_v: {=u16}, ", &self.cell_2_min_v());
        defmt::write!(f, "cell_3_min_v: {=u16}, ", &self.cell_3_min_v());
        defmt::write!(f, "cell_4_min_v: {=u16}, ", &self.cell_4_min_v());
        defmt::write!(f, "max_delta_cell_v: {=u16}, ", &self.max_delta_cell_v());
        defmt::write!(f, "max_charge_a: {=u16}, ", &self.max_charge_a());
        defmt::write!(f, "max_discharge_a: {=u16}, ", &self.max_discharge_a());
        defmt::write!(f, "max_avg_discharge_a: {=u16}, ", &self.max_avg_discharge_a());
        defmt::write!(f, "max_avg_discharge_pwr: {=u16}, ", &self.max_avg_discharge_pwr());
        defmt::write!(f, "max_temp_cell: {=u8}, ", &self.max_temp_cell());
        defmt::write!(f, "min_temp_cell: {=u8}, ", &self.min_temp_cell());
        defmt::write!(f, "max_delta_cell_temp: {=u8}, ", &self.max_delta_cell_temp());
        defmt::write!(f, "max_temp_int_sensor: {=u8}, ", &self.max_temp_int_sensor());
        defmt::write!(f, "min_temp_int_sensor: {=u8}, ", &self.min_temp_int_sensor());
        defmt::write!(f, "max_temp_fet: {=u8}, ", &self.max_temp_fet());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for LifetimeDataBlock1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for LifetimeDataBlock1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for LifetimeDataBlock1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for LifetimeDataBlock1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for LifetimeDataBlock1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for LifetimeDataBlock1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for LifetimeDataBlock1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "TURBO_CURRENT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct TurboCurrent {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for TurboCurrent {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl TurboCurrent {
    /// `15:0` - Read the `turbo_current` field.
    ///
    #[doc(alias = "TURBO_CURRENT")]
    #[must_use]
    pub fn turbo_current(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `turbo_current` field.
    ///
    #[doc(alias = "TURBO_CURRENT")]
    pub fn set_turbo_current(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for TurboCurrent {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for TurboCurrent {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<TurboCurrent> for [u8; 2] {
    fn from(val: TurboCurrent) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TurboCurrent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TurboCurrent");
        d.field("turbo_current", &self.turbo_current());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for TurboCurrent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TurboCurrent {{ ");
        defmt::write!(f, "turbo_current: {=u16}, ", &self.turbo_current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TurboCurrent {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TurboCurrent {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TurboCurrent {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TurboCurrent {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TurboCurrent {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TurboCurrent {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TurboCurrent {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "TURBO_EDV")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct TurboEdv {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for TurboEdv {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl TurboEdv {
    /// `15:0` - Read the `turbo_edv` field.
    ///
    #[doc(alias = "TURBO_EDV")]
    #[must_use]
    pub fn turbo_edv(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `turbo_edv` field.
    ///
    #[doc(alias = "TURBO_EDV")]
    pub fn set_turbo_edv(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for TurboEdv {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for TurboEdv {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<TurboEdv> for [u8; 2] {
    fn from(val: TurboEdv) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TurboEdv {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TurboEdv");
        d.field("turbo_edv", &self.turbo_edv());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for TurboEdv {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TurboEdv {{ ");
        defmt::write!(f, "turbo_edv: {=u16}, ", &self.turbo_edv());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TurboEdv {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TurboEdv {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TurboEdv {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TurboEdv {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TurboEdv {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TurboEdv {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TurboEdv {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "TURBO_SYS_R")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct TurboSysR {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for TurboSysR {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl TurboSysR {
    /// `15:0` - Read the `turbo_sys_r` field.
    ///
    #[doc(alias = "TURBO_SYS_R")]
    #[must_use]
    pub fn turbo_sys_r(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `turbo_sys_r` field.
    ///
    #[doc(alias = "TURBO_SYS_R")]
    pub fn set_turbo_sys_r(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for TurboSysR {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for TurboSysR {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<TurboSysR> for [u8; 2] {
    fn from(val: TurboSysR) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TurboSysR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TurboSysR");
        d.field("turbo_sys_r", &self.turbo_sys_r());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for TurboSysR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TurboSysR {{ ");
        defmt::write!(f, "turbo_sys_r: {=u16}, ", &self.turbo_sys_r());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TurboSysR {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TurboSysR {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TurboSysR {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TurboSysR {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TurboSysR {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TurboSysR {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TurboSysR {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "TURBO_PACK_R")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct TurboPackR {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for TurboPackR {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl TurboPackR {
    /// `15:0` - Read the `turbo_pack_r` field.
    ///
    #[doc(alias = "TURBO_PACK_R")]
    #[must_use]
    pub fn turbo_pack_r(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `turbo_pack_r` field.
    ///
    #[doc(alias = "TURBO_PACK_R")]
    pub fn set_turbo_pack_r(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for TurboPackR {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for TurboPackR {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<TurboPackR> for [u8; 2] {
    fn from(val: TurboPackR) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TurboPackR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TurboPackR");
        d.field("turbo_pack_r", &self.turbo_pack_r());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for TurboPackR {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TurboPackR {{ ");
        defmt::write!(f, "turbo_pack_r: {=u16}, ", &self.turbo_pack_r());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TurboPackR {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TurboPackR {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TurboPackR {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TurboPackR {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TurboPackR {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TurboPackR {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TurboPackR {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "TURBO_FINAL")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct TurboFinal {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for TurboFinal {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl TurboFinal {
    /// `15:0` - Read the `turbo_final` field.
    ///
    #[doc(alias = "TURBO_FINAL")]
    #[must_use]
    pub fn turbo_final(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `turbo_final` field.
    ///
    #[doc(alias = "TURBO_FINAL")]
    pub fn set_turbo_final(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for TurboFinal {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for TurboFinal {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<TurboFinal> for [u8; 2] {
    fn from(val: TurboFinal) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TurboFinal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TurboFinal");
        d.field("turbo_final", &self.turbo_final());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for TurboFinal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TurboFinal {{ ");
        defmt::write!(f, "turbo_final: {=u16}, ", &self.turbo_final());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TurboFinal {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TurboFinal {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TurboFinal {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TurboFinal {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TurboFinal {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TurboFinal {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TurboFinal {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "TURBO_POWER")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct TurboPower {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for TurboPower {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl TurboPower {
    /// `15:0` - Read the `turbo_power` field.
    ///
    #[doc(alias = "TURBO_POWER")]
    #[must_use]
    pub fn turbo_power(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `turbo_power` field.
    ///
    #[doc(alias = "TURBO_POWER")]
    pub fn set_turbo_power(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for TurboPower {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for TurboPower {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<TurboPower> for [u8; 2] {
    fn from(val: TurboPower) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for TurboPower {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("TurboPower");
        d.field("turbo_power", &self.turbo_power());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for TurboPower {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "TurboPower {{ ");
        defmt::write!(f, "turbo_power: {=u16}, ", &self.turbo_power());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for TurboPower {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for TurboPower {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for TurboPower {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for TurboPower {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for TurboPower {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for TurboPower {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for TurboPower {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AFE_REG")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AfeReg {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 21],
}
unsafe impl ::device_driver::Fieldset for AfeReg {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 21] };
}
impl AfeReg {
    /// `7:0` - Read the `afe_int_status` field.
    ///
    #[doc(alias = "AFE_INT_STATUS")]
    #[must_use]
    pub fn afe_int_status(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `afe_fet_status` field.
    ///
    #[doc(alias = "AFE_FET_STATUS")]
    #[must_use]
    pub fn afe_fet_status(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `23:16` - Read the `afe_rxin` field.
    ///
    #[doc(alias = "AFE_RXIN")]
    #[must_use]
    pub fn afe_rxin(&self) -> u8 {
        let start = 16;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:24` - Read the `afe_latch_status` field.
    ///
    #[doc(alias = "AFE_LATCH_STATUS")]
    #[must_use]
    pub fn afe_latch_status(&self) -> u8 {
        let start = 24;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `39:32` - Read the `afe_int_en` field.
    ///
    #[doc(alias = "AFE_INT_EN")]
    #[must_use]
    pub fn afe_int_en(&self) -> u8 {
        let start = 32;
        let end = 39;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:40` - Read the `afe_ctrl` field.
    ///
    #[doc(alias = "AFE_CTRL")]
    #[must_use]
    pub fn afe_ctrl(&self) -> u8 {
        let start = 40;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `55:48` - Read the `afe_rxien` field.
    ///
    #[doc(alias = "AFE_RXIEN")]
    #[must_use]
    pub fn afe_rxien(&self) -> u8 {
        let start = 48;
        let end = 55;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:56` - Read the `afe_rlout` field.
    ///
    #[doc(alias = "AFE_RLOUT")]
    #[must_use]
    pub fn afe_rlout(&self) -> u8 {
        let start = 56;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `71:64` - Read the `afe_rhout` field.
    ///
    #[doc(alias = "AFE_RHOUT")]
    #[must_use]
    pub fn afe_rhout(&self) -> u8 {
        let start = 64;
        let end = 71;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:72` - Read the `afe_rhint` field.
    ///
    #[doc(alias = "AFE_RHINT")]
    #[must_use]
    pub fn afe_rhint(&self) -> u8 {
        let start = 72;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `87:80` - Read the `afe_cell_balance` field.
    ///
    #[doc(alias = "AFE_CELL_BALANCE")]
    #[must_use]
    pub fn afe_cell_balance(&self) -> u8 {
        let start = 80;
        let end = 87;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:88` - Read the `afe_adc_cc_ctrl` field.
    ///
    #[doc(alias = "AFE_ADC_CC_CTRL")]
    #[must_use]
    pub fn afe_adc_cc_ctrl(&self) -> u8 {
        let start = 88;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `103:96` - Read the `afe_adc_mux_ctrl` field.
    ///
    #[doc(alias = "AFE_ADC_MUX_CTRL")]
    #[must_use]
    pub fn afe_adc_mux_ctrl(&self) -> u8 {
        let start = 96;
        let end = 103;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:104` - Read the `afe_led_ctrl` field.
    ///
    #[doc(alias = "AFE_LED_CTRL")]
    #[must_use]
    pub fn afe_led_ctrl(&self) -> u8 {
        let start = 104;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `119:112` - Read the `afe_hw_ctrl` field.
    ///
    #[doc(alias = "AFE_HW_CTRL")]
    #[must_use]
    pub fn afe_hw_ctrl(&self) -> u8 {
        let start = 112;
        let end = 119;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:120` - Read the `afe_tmr_ctrl` field.
    ///
    #[doc(alias = "AFE_TMR_CTRL")]
    #[must_use]
    pub fn afe_tmr_ctrl(&self) -> u8 {
        let start = 120;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `135:128` - Read the `afe_protection` field.
    ///
    #[doc(alias = "AFE_PROTECTION")]
    #[must_use]
    pub fn afe_protection(&self) -> u8 {
        let start = 128;
        let end = 135;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:136` - Read the `afe_ocd` field.
    ///
    #[doc(alias = "AFE_OCD")]
    #[must_use]
    pub fn afe_ocd(&self) -> u8 {
        let start = 136;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `151:144` - Read the `afe_scc` field.
    ///
    #[doc(alias = "AFE_SCC")]
    #[must_use]
    pub fn afe_scc(&self) -> u8 {
        let start = 144;
        let end = 151;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:152` - Read the `afe_scd_1` field.
    ///
    #[doc(alias = "AFE_SCD1")]
    #[must_use]
    pub fn afe_scd_1(&self) -> u8 {
        let start = 152;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `167:160` - Read the `afe_scd_2` field.
    ///
    #[doc(alias = "AFE_SCD2")]
    #[must_use]
    pub fn afe_scd_2(&self) -> u8 {
        let start = 160;
        let end = 167;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AfeReg {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 21]> for AfeReg {
    fn from(bits: [u8; 21]) -> Self {
        Self { bits }
    }
}
impl From<AfeReg> for [u8; 21] {
    fn from(val: AfeReg) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AfeReg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AfeReg");
        d.field("afe_int_status", &self.afe_int_status());
        d.field("afe_fet_status", &self.afe_fet_status());
        d.field("afe_rxin", &self.afe_rxin());
        d.field("afe_latch_status", &self.afe_latch_status());
        d.field("afe_int_en", &self.afe_int_en());
        d.field("afe_ctrl", &self.afe_ctrl());
        d.field("afe_rxien", &self.afe_rxien());
        d.field("afe_rlout", &self.afe_rlout());
        d.field("afe_rhout", &self.afe_rhout());
        d.field("afe_rhint", &self.afe_rhint());
        d.field("afe_cell_balance", &self.afe_cell_balance());
        d.field("afe_adc_cc_ctrl", &self.afe_adc_cc_ctrl());
        d.field("afe_adc_mux_ctrl", &self.afe_adc_mux_ctrl());
        d.field("afe_led_ctrl", &self.afe_led_ctrl());
        d.field("afe_hw_ctrl", &self.afe_hw_ctrl());
        d.field("afe_tmr_ctrl", &self.afe_tmr_ctrl());
        d.field("afe_protection", &self.afe_protection());
        d.field("afe_ocd", &self.afe_ocd());
        d.field("afe_scc", &self.afe_scc());
        d.field("afe_scd_1", &self.afe_scd_1());
        d.field("afe_scd_2", &self.afe_scd_2());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AfeReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AfeReg {{ ");
        defmt::write!(f, "afe_int_status: {=u8}, ", &self.afe_int_status());
        defmt::write!(f, "afe_fet_status: {=u8}, ", &self.afe_fet_status());
        defmt::write!(f, "afe_rxin: {=u8}, ", &self.afe_rxin());
        defmt::write!(f, "afe_latch_status: {=u8}, ", &self.afe_latch_status());
        defmt::write!(f, "afe_int_en: {=u8}, ", &self.afe_int_en());
        defmt::write!(f, "afe_ctrl: {=u8}, ", &self.afe_ctrl());
        defmt::write!(f, "afe_rxien: {=u8}, ", &self.afe_rxien());
        defmt::write!(f, "afe_rlout: {=u8}, ", &self.afe_rlout());
        defmt::write!(f, "afe_rhout: {=u8}, ", &self.afe_rhout());
        defmt::write!(f, "afe_rhint: {=u8}, ", &self.afe_rhint());
        defmt::write!(f, "afe_cell_balance: {=u8}, ", &self.afe_cell_balance());
        defmt::write!(f, "afe_adc_cc_ctrl: {=u8}, ", &self.afe_adc_cc_ctrl());
        defmt::write!(f, "afe_adc_mux_ctrl: {=u8}, ", &self.afe_adc_mux_ctrl());
        defmt::write!(f, "afe_led_ctrl: {=u8}, ", &self.afe_led_ctrl());
        defmt::write!(f, "afe_hw_ctrl: {=u8}, ", &self.afe_hw_ctrl());
        defmt::write!(f, "afe_tmr_ctrl: {=u8}, ", &self.afe_tmr_ctrl());
        defmt::write!(f, "afe_protection: {=u8}, ", &self.afe_protection());
        defmt::write!(f, "afe_ocd: {=u8}, ", &self.afe_ocd());
        defmt::write!(f, "afe_scc: {=u8}, ", &self.afe_scc());
        defmt::write!(f, "afe_scd_1: {=u8}, ", &self.afe_scd_1());
        defmt::write!(f, "afe_scd_2: {=u8}, ", &self.afe_scd_2());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AfeReg {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AfeReg {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AfeReg {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AfeReg {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AfeReg {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AfeReg {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AfeReg {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MANUFACTURING_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ManufacturingStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ManufacturingStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ManufacturingStatus {
    /// `bit 0` - Read the `pchg_en` field.
    ///
    #[doc(alias = "PCHG_EN")]
    #[must_use]
    pub fn pchg_en(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `chg_en` field.
    ///
    #[doc(alias = "CHG_EN")]
    #[must_use]
    pub fn chg_en(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `dsg_en` field.
    ///
    #[doc(alias = "DSG_EN")]
    #[must_use]
    pub fn dsg_en(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `gauge_en` field.
    ///
    #[doc(alias = "GAUGE_EN")]
    #[must_use]
    pub fn gauge_en(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `fet_en` field.
    ///
    #[doc(alias = "FET_EN")]
    #[must_use]
    pub fn fet_en(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `lf_en` field.
    ///
    #[doc(alias = "LF_EN")]
    #[must_use]
    pub fn lf_en(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `pf_en` field.
    ///
    #[doc(alias = "PF_EN")]
    #[must_use]
    pub fn pf_en(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `bbr_en` field.
    ///
    #[doc(alias = "BBR_EN")]
    #[must_use]
    pub fn bbr_en(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `fuse_en` field.
    ///
    #[doc(alias = "FUSE_EN")]
    #[must_use]
    pub fn fuse_en(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `led_en` field.
    ///
    #[doc(alias = "LED_EN")]
    #[must_use]
    pub fn led_en(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `lt_test` field.
    ///
    #[doc(alias = "LT_TEST")]
    #[must_use]
    pub fn lt_test(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `cal_test` field.
    ///
    #[doc(alias = "CAL_TEST")]
    #[must_use]
    pub fn cal_test(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for ManufacturingStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ManufacturingStatus {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ManufacturingStatus> for [u8; 2] {
    fn from(val: ManufacturingStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ManufacturingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ManufacturingStatus");
        d.field("pchg_en", &self.pchg_en());
        d.field("chg_en", &self.chg_en());
        d.field("dsg_en", &self.dsg_en());
        d.field("gauge_en", &self.gauge_en());
        d.field("fet_en", &self.fet_en());
        d.field("lf_en", &self.lf_en());
        d.field("pf_en", &self.pf_en());
        d.field("bbr_en", &self.bbr_en());
        d.field("fuse_en", &self.fuse_en());
        d.field("led_en", &self.led_en());
        d.field("lt_test", &self.lt_test());
        d.field("cal_test", &self.cal_test());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for ManufacturingStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ManufacturingStatus {{ ");
        defmt::write!(f, "pchg_en: {=bool}, ", &self.pchg_en());
        defmt::write!(f, "chg_en: {=bool}, ", &self.chg_en());
        defmt::write!(f, "dsg_en: {=bool}, ", &self.dsg_en());
        defmt::write!(f, "gauge_en: {=bool}, ", &self.gauge_en());
        defmt::write!(f, "fet_en: {=bool}, ", &self.fet_en());
        defmt::write!(f, "lf_en: {=bool}, ", &self.lf_en());
        defmt::write!(f, "pf_en: {=bool}, ", &self.pf_en());
        defmt::write!(f, "bbr_en: {=bool}, ", &self.bbr_en());
        defmt::write!(f, "fuse_en: {=bool}, ", &self.fuse_en());
        defmt::write!(f, "led_en: {=bool}, ", &self.led_en());
        defmt::write!(f, "lt_test: {=bool}, ", &self.lt_test());
        defmt::write!(f, "cal_test: {=bool}, ", &self.cal_test());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ManufacturingStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ManufacturingStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ManufacturingStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ManufacturingStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ManufacturingStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ManufacturingStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ManufacturingStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "GAUGING_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct GaugingStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for GaugingStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl GaugingStatus {
    /// `bit 0` - Read the `fd` field.
    ///
    #[doc(alias = "FD")]
    #[must_use]
    pub fn fd(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `fc` field.
    ///
    #[doc(alias = "FC")]
    #[must_use]
    pub fn fc(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `td` field.
    ///
    #[doc(alias = "TD")]
    #[must_use]
    pub fn td(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `tc` field.
    ///
    #[doc(alias = "TC")]
    #[must_use]
    pub fn tc(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `bal_en` field.
    ///
    #[doc(alias = "BAL_EN")]
    #[must_use]
    pub fn bal_en(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `edv` field.
    ///
    #[doc(alias = "EDV")]
    #[must_use]
    pub fn edv(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `dsg` field.
    ///
    #[doc(alias = "DSG")]
    #[must_use]
    pub fn dsg(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `cf` field.
    ///
    #[doc(alias = "CF")]
    #[must_use]
    pub fn cf(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `rest` field.
    ///
    #[doc(alias = "REST")]
    #[must_use]
    pub fn rest(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `r_dis` field.
    ///
    #[doc(alias = "R_DIS")]
    #[must_use]
    pub fn r_dis(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `vok` field.
    ///
    #[doc(alias = "VOK")]
    #[must_use]
    pub fn vok(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `qen` field.
    ///
    #[doc(alias = "QEN")]
    #[must_use]
    pub fn qen(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `slpqmax` field.
    ///
    #[doc(alias = "SLPQMAX")]
    #[must_use]
    pub fn slpqmax(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `nsfm` field.
    ///
    #[doc(alias = "NSFM")]
    #[must_use]
    pub fn nsfm(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `vdq` field.
    ///
    #[doc(alias = "VDQ")]
    #[must_use]
    pub fn vdq(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `qmax` field.
    ///
    #[doc(alias = "QMAX")]
    #[must_use]
    pub fn qmax(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `rx` field.
    ///
    #[doc(alias = "RX")]
    #[must_use]
    pub fn rx(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `ldmd` field.
    ///
    #[doc(alias = "LDMD")]
    #[must_use]
    pub fn ldmd(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `ocvfr` field.
    ///
    #[doc(alias = "OCVFR")]
    #[must_use]
    pub fn ocvfr(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for GaugingStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for GaugingStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<GaugingStatus> for [u8; 4] {
    fn from(val: GaugingStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for GaugingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("GaugingStatus");
        d.field("fd", &self.fd());
        d.field("fc", &self.fc());
        d.field("td", &self.td());
        d.field("tc", &self.tc());
        d.field("bal_en", &self.bal_en());
        d.field("edv", &self.edv());
        d.field("dsg", &self.dsg());
        d.field("cf", &self.cf());
        d.field("rest", &self.rest());
        d.field("r_dis", &self.r_dis());
        d.field("vok", &self.vok());
        d.field("qen", &self.qen());
        d.field("slpqmax", &self.slpqmax());
        d.field("nsfm", &self.nsfm());
        d.field("vdq", &self.vdq());
        d.field("qmax", &self.qmax());
        d.field("rx", &self.rx());
        d.field("ldmd", &self.ldmd());
        d.field("ocvfr", &self.ocvfr());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for GaugingStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "GaugingStatus {{ ");
        defmt::write!(f, "fd: {=bool}, ", &self.fd());
        defmt::write!(f, "fc: {=bool}, ", &self.fc());
        defmt::write!(f, "td: {=bool}, ", &self.td());
        defmt::write!(f, "tc: {=bool}, ", &self.tc());
        defmt::write!(f, "bal_en: {=bool}, ", &self.bal_en());
        defmt::write!(f, "edv: {=bool}, ", &self.edv());
        defmt::write!(f, "dsg: {=bool}, ", &self.dsg());
        defmt::write!(f, "cf: {=bool}, ", &self.cf());
        defmt::write!(f, "rest: {=bool}, ", &self.rest());
        defmt::write!(f, "r_dis: {=bool}, ", &self.r_dis());
        defmt::write!(f, "vok: {=bool}, ", &self.vok());
        defmt::write!(f, "qen: {=bool}, ", &self.qen());
        defmt::write!(f, "slpqmax: {=bool}, ", &self.slpqmax());
        defmt::write!(f, "nsfm: {=bool}, ", &self.nsfm());
        defmt::write!(f, "vdq: {=bool}, ", &self.vdq());
        defmt::write!(f, "qmax: {=bool}, ", &self.qmax());
        defmt::write!(f, "rx: {=bool}, ", &self.rx());
        defmt::write!(f, "ldmd: {=bool}, ", &self.ldmd());
        defmt::write!(f, "ocvfr: {=bool}, ", &self.ocvfr());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for GaugingStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for GaugingStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for GaugingStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for GaugingStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for GaugingStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for GaugingStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for GaugingStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGING_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargingStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for ChargingStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl ChargingStatus {
    /// `bit 0` - Read the `ut` field.
    ///
    #[doc(alias = "UT")]
    #[must_use]
    pub fn ut(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `lt` field.
    ///
    #[doc(alias = "LT")]
    #[must_use]
    pub fn lt(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `stl` field.
    ///
    #[doc(alias = "STL")]
    #[must_use]
    pub fn stl(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `rt` field.
    ///
    #[doc(alias = "RT")]
    #[must_use]
    pub fn rt(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `sth` field.
    ///
    #[doc(alias = "STH")]
    #[must_use]
    pub fn sth(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `ht` field.
    ///
    #[doc(alias = "HT")]
    #[must_use]
    pub fn ht(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `ot` field.
    ///
    #[doc(alias = "OT")]
    #[must_use]
    pub fn ot(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `pv` field.
    ///
    #[doc(alias = "PV")]
    #[must_use]
    pub fn pv(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `lv` field.
    ///
    #[doc(alias = "LV")]
    #[must_use]
    pub fn lv(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `mv` field.
    ///
    #[doc(alias = "MV")]
    #[must_use]
    pub fn mv(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `hv` field.
    ///
    #[doc(alias = "HV")]
    #[must_use]
    pub fn hv(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `chg_in` field.
    ///
    #[doc(alias = "CHG_IN")]
    #[must_use]
    pub fn chg_in(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `mchg` field.
    ///
    #[doc(alias = "MCHG")]
    #[must_use]
    pub fn mchg(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `vct` field.
    ///
    #[doc(alias = "VCT")]
    #[must_use]
    pub fn vct(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `ccr` field.
    ///
    #[doc(alias = "CCR")]
    #[must_use]
    pub fn ccr(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `cvr` field.
    ///
    #[doc(alias = "CVR")]
    #[must_use]
    pub fn cvr(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `ccc` field.
    ///
    #[doc(alias = "CCC")]
    #[must_use]
    pub fn ccc(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for ChargingStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for ChargingStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<ChargingStatus> for [u8; 4] {
    fn from(val: ChargingStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargingStatus");
        d.field("ut", &self.ut());
        d.field("lt", &self.lt());
        d.field("stl", &self.stl());
        d.field("rt", &self.rt());
        d.field("sth", &self.sth());
        d.field("ht", &self.ht());
        d.field("ot", &self.ot());
        d.field("pv", &self.pv());
        d.field("lv", &self.lv());
        d.field("mv", &self.mv());
        d.field("hv", &self.hv());
        d.field("chg_in", &self.chg_in());
        d.field("mchg", &self.mchg());
        d.field("vct", &self.vct());
        d.field("ccr", &self.ccr());
        d.field("cvr", &self.cvr());
        d.field("ccc", &self.ccc());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for ChargingStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargingStatus {{ ");
        defmt::write!(f, "ut: {=bool}, ", &self.ut());
        defmt::write!(f, "lt: {=bool}, ", &self.lt());
        defmt::write!(f, "stl: {=bool}, ", &self.stl());
        defmt::write!(f, "rt: {=bool}, ", &self.rt());
        defmt::write!(f, "sth: {=bool}, ", &self.sth());
        defmt::write!(f, "ht: {=bool}, ", &self.ht());
        defmt::write!(f, "ot: {=bool}, ", &self.ot());
        defmt::write!(f, "pv: {=bool}, ", &self.pv());
        defmt::write!(f, "lv: {=bool}, ", &self.lv());
        defmt::write!(f, "mv: {=bool}, ", &self.mv());
        defmt::write!(f, "hv: {=bool}, ", &self.hv());
        defmt::write!(f, "chg_in: {=bool}, ", &self.chg_in());
        defmt::write!(f, "mchg: {=bool}, ", &self.mchg());
        defmt::write!(f, "vct: {=bool}, ", &self.vct());
        defmt::write!(f, "ccr: {=bool}, ", &self.ccr());
        defmt::write!(f, "cvr: {=bool}, ", &self.cvr());
        defmt::write!(f, "ccc: {=bool}, ", &self.ccc());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargingStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargingStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargingStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargingStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargingStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargingStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargingStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "OPERATION_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct OperationStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for OperationStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl OperationStatus {
    /// `bit 0` - Read the `pres` field.
    ///
    #[doc(alias = "PRES")]
    #[must_use]
    pub fn pres(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `dsg` field.
    ///
    #[doc(alias = "DSG")]
    #[must_use]
    pub fn dsg(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `chg` field.
    ///
    #[doc(alias = "CHG")]
    #[must_use]
    pub fn chg(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `pchg` field.
    ///
    #[doc(alias = "PCHG")]
    #[must_use]
    pub fn pchg(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    #[must_use]
    pub fn fuse(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `btp_int` field.
    ///
    #[doc(alias = "BTP_INT")]
    #[must_use]
    pub fn btp_int(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `9:8` - Read the `sec` field.
    ///
    #[doc(alias = "SEC")]
    #[must_use]
    pub fn sec(&self) -> SecurityMode {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `sdv` field.
    ///
    #[doc(alias = "SDV")]
    #[must_use]
    pub fn sdv(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `ss` field.
    ///
    #[doc(alias = "SS")]
    #[must_use]
    pub fn ss(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `pf` field.
    ///
    #[doc(alias = "PF")]
    #[must_use]
    pub fn pf(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `xdsg` field.
    ///
    #[doc(alias = "XDSG")]
    #[must_use]
    pub fn xdsg(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `xchg` field.
    ///
    #[doc(alias = "XCHG")]
    #[must_use]
    pub fn xchg(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `sleep` field.
    ///
    #[doc(alias = "SLEEP")]
    #[must_use]
    pub fn sleep(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `sdm` field.
    ///
    #[doc(alias = "SDM")]
    #[must_use]
    pub fn sdm(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `led` field.
    ///
    #[doc(alias = "LED")]
    #[must_use]
    pub fn led(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `auth` field.
    ///
    #[doc(alias = "AUTH")]
    #[must_use]
    pub fn auth(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `autocalm` field.
    ///
    #[doc(alias = "AUTOCALM")]
    #[must_use]
    pub fn autocalm(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `cal` field.
    ///
    #[doc(alias = "CAL")]
    #[must_use]
    pub fn cal(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `cal_offset` field.
    ///
    #[doc(alias = "CAL_OFFSET")]
    #[must_use]
    pub fn cal_offset(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `xl` field.
    ///
    #[doc(alias = "XL")]
    #[must_use]
    pub fn xl(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `sleepm` field.
    ///
    #[doc(alias = "SLEEPM")]
    #[must_use]
    pub fn sleepm(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `init` field.
    ///
    #[doc(alias = "INIT")]
    #[must_use]
    pub fn init(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `smblcal` field.
    ///
    #[doc(alias = "SMBLCAL")]
    #[must_use]
    pub fn smblcal(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `slpad` field.
    ///
    #[doc(alias = "SLPAD")]
    #[must_use]
    pub fn slpad(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 27` - Read the `slpcc` field.
    ///
    #[doc(alias = "SLPCC")]
    #[must_use]
    pub fn slpcc(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 28` - Read the `cb` field.
    ///
    #[doc(alias = "CB")]
    #[must_use]
    pub fn cb(&self) -> bool {
        let start = 28;
        let end = 28;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 29` - Read the `emshut` field.
    ///
    #[doc(alias = "EMSHUT")]
    #[must_use]
    pub fn emshut(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for OperationStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for OperationStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<OperationStatus> for [u8; 4] {
    fn from(val: OperationStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for OperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("OperationStatus");
        d.field("pres", &self.pres());
        d.field("dsg", &self.dsg());
        d.field("chg", &self.chg());
        d.field("pchg", &self.pchg());
        d.field("fuse", &self.fuse());
        d.field("btp_int", &self.btp_int());
        d.field("sec", &self.sec());
        d.field("sdv", &self.sdv());
        d.field("ss", &self.ss());
        d.field("pf", &self.pf());
        d.field("xdsg", &self.xdsg());
        d.field("xchg", &self.xchg());
        d.field("sleep", &self.sleep());
        d.field("sdm", &self.sdm());
        d.field("led", &self.led());
        d.field("auth", &self.auth());
        d.field("autocalm", &self.autocalm());
        d.field("cal", &self.cal());
        d.field("cal_offset", &self.cal_offset());
        d.field("xl", &self.xl());
        d.field("sleepm", &self.sleepm());
        d.field("init", &self.init());
        d.field("smblcal", &self.smblcal());
        d.field("slpad", &self.slpad());
        d.field("slpcc", &self.slpcc());
        d.field("cb", &self.cb());
        d.field("emshut", &self.emshut());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for OperationStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "OperationStatus {{ ");
        defmt::write!(f, "pres: {=bool}, ", &self.pres());
        defmt::write!(f, "dsg: {=bool}, ", &self.dsg());
        defmt::write!(f, "chg: {=bool}, ", &self.chg());
        defmt::write!(f, "pchg: {=bool}, ", &self.pchg());
        defmt::write!(f, "fuse: {=bool}, ", &self.fuse());
        defmt::write!(f, "btp_int: {=bool}, ", &self.btp_int());
        defmt::write!(f, "sec: {}, ", &self.sec());
        defmt::write!(f, "sdv: {=bool}, ", &self.sdv());
        defmt::write!(f, "ss: {=bool}, ", &self.ss());
        defmt::write!(f, "pf: {=bool}, ", &self.pf());
        defmt::write!(f, "xdsg: {=bool}, ", &self.xdsg());
        defmt::write!(f, "xchg: {=bool}, ", &self.xchg());
        defmt::write!(f, "sleep: {=bool}, ", &self.sleep());
        defmt::write!(f, "sdm: {=bool}, ", &self.sdm());
        defmt::write!(f, "led: {=bool}, ", &self.led());
        defmt::write!(f, "auth: {=bool}, ", &self.auth());
        defmt::write!(f, "autocalm: {=bool}, ", &self.autocalm());
        defmt::write!(f, "cal: {=bool}, ", &self.cal());
        defmt::write!(f, "cal_offset: {=bool}, ", &self.cal_offset());
        defmt::write!(f, "xl: {=bool}, ", &self.xl());
        defmt::write!(f, "sleepm: {=bool}, ", &self.sleepm());
        defmt::write!(f, "init: {=bool}, ", &self.init());
        defmt::write!(f, "smblcal: {=bool}, ", &self.smblcal());
        defmt::write!(f, "slpad: {=bool}, ", &self.slpad());
        defmt::write!(f, "slpcc: {=bool}, ", &self.slpcc());
        defmt::write!(f, "cb: {=bool}, ", &self.cb());
        defmt::write!(f, "emshut: {=bool}, ", &self.emshut());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for OperationStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for OperationStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for OperationStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for OperationStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for OperationStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for OperationStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for OperationStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "PF_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PfStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for PfStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl PfStatus {
    /// `bit 0` - Read the `suv` field.
    ///
    #[doc(alias = "SUV")]
    #[must_use]
    pub fn suv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `sov` field.
    ///
    #[doc(alias = "SOV")]
    #[must_use]
    pub fn sov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `socc` field.
    ///
    #[doc(alias = "SOCC")]
    #[must_use]
    pub fn socc(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `socd` field.
    ///
    #[doc(alias = "SOCD")]
    #[must_use]
    pub fn socd(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `sot` field.
    ///
    #[doc(alias = "SOT")]
    #[must_use]
    pub fn sot(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `sotf` field.
    ///
    #[doc(alias = "SOTF")]
    #[must_use]
    pub fn sotf(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `qim` field.
    ///
    #[doc(alias = "QIM")]
    #[must_use]
    pub fn qim(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `cb` field.
    ///
    #[doc(alias = "CB")]
    #[must_use]
    pub fn cb(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `imp` field.
    ///
    #[doc(alias = "IMP")]
    #[must_use]
    pub fn imp(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `cd` field.
    ///
    #[doc(alias = "CD")]
    #[must_use]
    pub fn cd(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `vimr` field.
    ///
    #[doc(alias = "VIMR")]
    #[must_use]
    pub fn vimr(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `vima` field.
    ///
    #[doc(alias = "VIMA")]
    #[must_use]
    pub fn vima(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `cfetf` field.
    ///
    #[doc(alias = "CFETF")]
    #[must_use]
    pub fn cfetf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `dfetf` field.
    ///
    #[doc(alias = "DFETF")]
    #[must_use]
    pub fn dfetf(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    #[must_use]
    pub fn fuse(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `afer` field.
    ///
    #[doc(alias = "AFER")]
    #[must_use]
    pub fn afer(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `afec` field.
    ///
    #[doc(alias = "AFEC")]
    #[must_use]
    pub fn afec(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `second_lvl` field.
    ///
    #[doc(alias = "SECOND_LVL")]
    #[must_use]
    pub fn second_lvl(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `ptc` field.
    ///
    #[doc(alias = "PTC")]
    #[must_use]
    pub fn ptc(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `ifc` field.
    ///
    #[doc(alias = "IFC")]
    #[must_use]
    pub fn ifc(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `opncell` field.
    ///
    #[doc(alias = "OPNCELL")]
    #[must_use]
    pub fn opncell(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `dfw` field.
    ///
    #[doc(alias = "DFW")]
    #[must_use]
    pub fn dfw(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 28` - Read the `ts_1` field.
    ///
    #[doc(alias = "TS1")]
    #[must_use]
    pub fn ts_1(&self) -> bool {
        let start = 28;
        let end = 28;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 29` - Read the `ts_2` field.
    ///
    #[doc(alias = "TS2")]
    #[must_use]
    pub fn ts_2(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 30` - Read the `ts_3` field.
    ///
    #[doc(alias = "TS3")]
    #[must_use]
    pub fn ts_3(&self) -> bool {
        let start = 30;
        let end = 30;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 31` - Read the `ts_4` field.
    ///
    #[doc(alias = "TS4")]
    #[must_use]
    pub fn ts_4(&self) -> bool {
        let start = 31;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for PfStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for PfStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<PfStatus> for [u8; 4] {
    fn from(val: PfStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PfStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PfStatus");
        d.field("suv", &self.suv());
        d.field("sov", &self.sov());
        d.field("socc", &self.socc());
        d.field("socd", &self.socd());
        d.field("sot", &self.sot());
        d.field("sotf", &self.sotf());
        d.field("qim", &self.qim());
        d.field("cb", &self.cb());
        d.field("imp", &self.imp());
        d.field("cd", &self.cd());
        d.field("vimr", &self.vimr());
        d.field("vima", &self.vima());
        d.field("cfetf", &self.cfetf());
        d.field("dfetf", &self.dfetf());
        d.field("fuse", &self.fuse());
        d.field("afer", &self.afer());
        d.field("afec", &self.afec());
        d.field("second_lvl", &self.second_lvl());
        d.field("ptc", &self.ptc());
        d.field("ifc", &self.ifc());
        d.field("opncell", &self.opncell());
        d.field("dfw", &self.dfw());
        d.field("ts_1", &self.ts_1());
        d.field("ts_2", &self.ts_2());
        d.field("ts_3", &self.ts_3());
        d.field("ts_4", &self.ts_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for PfStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PfStatus {{ ");
        defmt::write!(f, "suv: {=bool}, ", &self.suv());
        defmt::write!(f, "sov: {=bool}, ", &self.sov());
        defmt::write!(f, "socc: {=bool}, ", &self.socc());
        defmt::write!(f, "socd: {=bool}, ", &self.socd());
        defmt::write!(f, "sot: {=bool}, ", &self.sot());
        defmt::write!(f, "sotf: {=bool}, ", &self.sotf());
        defmt::write!(f, "qim: {=bool}, ", &self.qim());
        defmt::write!(f, "cb: {=bool}, ", &self.cb());
        defmt::write!(f, "imp: {=bool}, ", &self.imp());
        defmt::write!(f, "cd: {=bool}, ", &self.cd());
        defmt::write!(f, "vimr: {=bool}, ", &self.vimr());
        defmt::write!(f, "vima: {=bool}, ", &self.vima());
        defmt::write!(f, "cfetf: {=bool}, ", &self.cfetf());
        defmt::write!(f, "dfetf: {=bool}, ", &self.dfetf());
        defmt::write!(f, "fuse: {=bool}, ", &self.fuse());
        defmt::write!(f, "afer: {=bool}, ", &self.afer());
        defmt::write!(f, "afec: {=bool}, ", &self.afec());
        defmt::write!(f, "second_lvl: {=bool}, ", &self.second_lvl());
        defmt::write!(f, "ptc: {=bool}, ", &self.ptc());
        defmt::write!(f, "ifc: {=bool}, ", &self.ifc());
        defmt::write!(f, "opncell: {=bool}, ", &self.opncell());
        defmt::write!(f, "dfw: {=bool}, ", &self.dfw());
        defmt::write!(f, "ts_1: {=bool}, ", &self.ts_1());
        defmt::write!(f, "ts_2: {=bool}, ", &self.ts_2());
        defmt::write!(f, "ts_3: {=bool}, ", &self.ts_3());
        defmt::write!(f, "ts_4: {=bool}, ", &self.ts_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PfStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PfStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PfStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PfStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PfStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PfStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PfStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "PF_ALERT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct PfAlert {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for PfAlert {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl PfAlert {
    /// `bit 0` - Read the `suv` field.
    ///
    #[doc(alias = "SUV")]
    #[must_use]
    pub fn suv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `sov` field.
    ///
    #[doc(alias = "SOV")]
    #[must_use]
    pub fn sov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `socc` field.
    ///
    #[doc(alias = "SOCC")]
    #[must_use]
    pub fn socc(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `socd` field.
    ///
    #[doc(alias = "SOCD")]
    #[must_use]
    pub fn socd(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `sot` field.
    ///
    #[doc(alias = "SOT")]
    #[must_use]
    pub fn sot(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `sotf` field.
    ///
    #[doc(alias = "SOTF")]
    #[must_use]
    pub fn sotf(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `qim` field.
    ///
    #[doc(alias = "QIM")]
    #[must_use]
    pub fn qim(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `cb` field.
    ///
    #[doc(alias = "CB")]
    #[must_use]
    pub fn cb(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `imp` field.
    ///
    #[doc(alias = "IMP")]
    #[must_use]
    pub fn imp(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `cd` field.
    ///
    #[doc(alias = "CD")]
    #[must_use]
    pub fn cd(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `vimr` field.
    ///
    #[doc(alias = "VIMR")]
    #[must_use]
    pub fn vimr(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `vima` field.
    ///
    #[doc(alias = "VIMA")]
    #[must_use]
    pub fn vima(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `cfetf` field.
    ///
    #[doc(alias = "CFETF")]
    #[must_use]
    pub fn cfetf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `dfetf` field.
    ///
    #[doc(alias = "DFETF")]
    #[must_use]
    pub fn dfetf(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    #[must_use]
    pub fn fuse(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `afer` field.
    ///
    #[doc(alias = "AFER")]
    #[must_use]
    pub fn afer(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `afec` field.
    ///
    #[doc(alias = "AFEC")]
    #[must_use]
    pub fn afec(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `second_lvl` field.
    ///
    #[doc(alias = "SECOND_LVL")]
    #[must_use]
    pub fn second_lvl(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `opnc` field.
    ///
    #[doc(alias = "OPNC")]
    #[must_use]
    pub fn opnc(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 28` - Read the `ts_1` field.
    ///
    #[doc(alias = "TS1")]
    #[must_use]
    pub fn ts_1(&self) -> bool {
        let start = 28;
        let end = 28;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 29` - Read the `ts_2` field.
    ///
    #[doc(alias = "TS2")]
    #[must_use]
    pub fn ts_2(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 30` - Read the `ts_3` field.
    ///
    #[doc(alias = "TS3")]
    #[must_use]
    pub fn ts_3(&self) -> bool {
        let start = 30;
        let end = 30;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 31` - Read the `ts_4` field.
    ///
    #[doc(alias = "TS4")]
    #[must_use]
    pub fn ts_4(&self) -> bool {
        let start = 31;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for PfAlert {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for PfAlert {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<PfAlert> for [u8; 4] {
    fn from(val: PfAlert) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for PfAlert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("PfAlert");
        d.field("suv", &self.suv());
        d.field("sov", &self.sov());
        d.field("socc", &self.socc());
        d.field("socd", &self.socd());
        d.field("sot", &self.sot());
        d.field("sotf", &self.sotf());
        d.field("qim", &self.qim());
        d.field("cb", &self.cb());
        d.field("imp", &self.imp());
        d.field("cd", &self.cd());
        d.field("vimr", &self.vimr());
        d.field("vima", &self.vima());
        d.field("cfetf", &self.cfetf());
        d.field("dfetf", &self.dfetf());
        d.field("fuse", &self.fuse());
        d.field("afer", &self.afer());
        d.field("afec", &self.afec());
        d.field("second_lvl", &self.second_lvl());
        d.field("opnc", &self.opnc());
        d.field("ts_1", &self.ts_1());
        d.field("ts_2", &self.ts_2());
        d.field("ts_3", &self.ts_3());
        d.field("ts_4", &self.ts_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for PfAlert {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "PfAlert {{ ");
        defmt::write!(f, "suv: {=bool}, ", &self.suv());
        defmt::write!(f, "sov: {=bool}, ", &self.sov());
        defmt::write!(f, "socc: {=bool}, ", &self.socc());
        defmt::write!(f, "socd: {=bool}, ", &self.socd());
        defmt::write!(f, "sot: {=bool}, ", &self.sot());
        defmt::write!(f, "sotf: {=bool}, ", &self.sotf());
        defmt::write!(f, "qim: {=bool}, ", &self.qim());
        defmt::write!(f, "cb: {=bool}, ", &self.cb());
        defmt::write!(f, "imp: {=bool}, ", &self.imp());
        defmt::write!(f, "cd: {=bool}, ", &self.cd());
        defmt::write!(f, "vimr: {=bool}, ", &self.vimr());
        defmt::write!(f, "vima: {=bool}, ", &self.vima());
        defmt::write!(f, "cfetf: {=bool}, ", &self.cfetf());
        defmt::write!(f, "dfetf: {=bool}, ", &self.dfetf());
        defmt::write!(f, "fuse: {=bool}, ", &self.fuse());
        defmt::write!(f, "afer: {=bool}, ", &self.afer());
        defmt::write!(f, "afec: {=bool}, ", &self.afec());
        defmt::write!(f, "second_lvl: {=bool}, ", &self.second_lvl());
        defmt::write!(f, "opnc: {=bool}, ", &self.opnc());
        defmt::write!(f, "ts_1: {=bool}, ", &self.ts_1());
        defmt::write!(f, "ts_2: {=bool}, ", &self.ts_2());
        defmt::write!(f, "ts_3: {=bool}, ", &self.ts_3());
        defmt::write!(f, "ts_4: {=bool}, ", &self.ts_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for PfAlert {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for PfAlert {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for PfAlert {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for PfAlert {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for PfAlert {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for PfAlert {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for PfAlert {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "SAFETY_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct SafetyStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for SafetyStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl SafetyStatus {
    /// `bit 0` - Read the `cuv` field.
    ///
    #[doc(alias = "CUV")]
    #[must_use]
    pub fn cuv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `cov` field.
    ///
    #[doc(alias = "COV")]
    #[must_use]
    pub fn cov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `occ_1` field.
    ///
    #[doc(alias = "OCC1")]
    #[must_use]
    pub fn occ_1(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `occ_2` field.
    ///
    #[doc(alias = "OCC2")]
    #[must_use]
    pub fn occ_2(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `ocd_1` field.
    ///
    #[doc(alias = "OCD1")]
    #[must_use]
    pub fn ocd_1(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `ocd_2` field.
    ///
    #[doc(alias = "OCD2")]
    #[must_use]
    pub fn ocd_2(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `aold` field.
    ///
    #[doc(alias = "AOLD")]
    #[must_use]
    pub fn aold(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `aoldl` field.
    ///
    #[doc(alias = "AOLDL")]
    #[must_use]
    pub fn aoldl(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `ascc` field.
    ///
    #[doc(alias = "ASCC")]
    #[must_use]
    pub fn ascc(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `asccl` field.
    ///
    #[doc(alias = "ASCCL")]
    #[must_use]
    pub fn asccl(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `ascd` field.
    ///
    #[doc(alias = "ASCD")]
    #[must_use]
    pub fn ascd(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `ascdl` field.
    ///
    #[doc(alias = "ASCDL")]
    #[must_use]
    pub fn ascdl(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `otc` field.
    ///
    #[doc(alias = "OTC")]
    #[must_use]
    pub fn otc(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `otd` field.
    ///
    #[doc(alias = "OTD")]
    #[must_use]
    pub fn otd(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `cuvc` field.
    ///
    #[doc(alias = "CUVC")]
    #[must_use]
    pub fn cuvc(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `otf` field.
    ///
    #[doc(alias = "OTF")]
    #[must_use]
    pub fn otf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `pto` field.
    ///
    #[doc(alias = "PTO")]
    #[must_use]
    pub fn pto(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `ptos` field.
    ///
    #[doc(alias = "PTOS")]
    #[must_use]
    pub fn ptos(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `cto` field.
    ///
    #[doc(alias = "CTO")]
    #[must_use]
    pub fn cto(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `oc` field.
    ///
    #[doc(alias = "OC")]
    #[must_use]
    pub fn oc(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `chgc` field.
    ///
    #[doc(alias = "CHGC")]
    #[must_use]
    pub fn chgc(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `chgv` field.
    ///
    #[doc(alias = "CHGV")]
    #[must_use]
    pub fn chgv(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `pchgc` field.
    ///
    #[doc(alias = "PCHGC")]
    #[must_use]
    pub fn pchgc(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `utc` field.
    ///
    #[doc(alias = "UTC")]
    #[must_use]
    pub fn utc(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 27` - Read the `utd` field.
    ///
    #[doc(alias = "UTD")]
    #[must_use]
    pub fn utd(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for SafetyStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for SafetyStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<SafetyStatus> for [u8; 4] {
    fn from(val: SafetyStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for SafetyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("SafetyStatus");
        d.field("cuv", &self.cuv());
        d.field("cov", &self.cov());
        d.field("occ_1", &self.occ_1());
        d.field("occ_2", &self.occ_2());
        d.field("ocd_1", &self.ocd_1());
        d.field("ocd_2", &self.ocd_2());
        d.field("aold", &self.aold());
        d.field("aoldl", &self.aoldl());
        d.field("ascc", &self.ascc());
        d.field("asccl", &self.asccl());
        d.field("ascd", &self.ascd());
        d.field("ascdl", &self.ascdl());
        d.field("otc", &self.otc());
        d.field("otd", &self.otd());
        d.field("cuvc", &self.cuvc());
        d.field("otf", &self.otf());
        d.field("pto", &self.pto());
        d.field("ptos", &self.ptos());
        d.field("cto", &self.cto());
        d.field("oc", &self.oc());
        d.field("chgc", &self.chgc());
        d.field("chgv", &self.chgv());
        d.field("pchgc", &self.pchgc());
        d.field("utc", &self.utc());
        d.field("utd", &self.utd());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for SafetyStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SafetyStatus {{ ");
        defmt::write!(f, "cuv: {=bool}, ", &self.cuv());
        defmt::write!(f, "cov: {=bool}, ", &self.cov());
        defmt::write!(f, "occ_1: {=bool}, ", &self.occ_1());
        defmt::write!(f, "occ_2: {=bool}, ", &self.occ_2());
        defmt::write!(f, "ocd_1: {=bool}, ", &self.ocd_1());
        defmt::write!(f, "ocd_2: {=bool}, ", &self.ocd_2());
        defmt::write!(f, "aold: {=bool}, ", &self.aold());
        defmt::write!(f, "aoldl: {=bool}, ", &self.aoldl());
        defmt::write!(f, "ascc: {=bool}, ", &self.ascc());
        defmt::write!(f, "asccl: {=bool}, ", &self.asccl());
        defmt::write!(f, "ascd: {=bool}, ", &self.ascd());
        defmt::write!(f, "ascdl: {=bool}, ", &self.ascdl());
        defmt::write!(f, "otc: {=bool}, ", &self.otc());
        defmt::write!(f, "otd: {=bool}, ", &self.otd());
        defmt::write!(f, "cuvc: {=bool}, ", &self.cuvc());
        defmt::write!(f, "otf: {=bool}, ", &self.otf());
        defmt::write!(f, "pto: {=bool}, ", &self.pto());
        defmt::write!(f, "ptos: {=bool}, ", &self.ptos());
        defmt::write!(f, "cto: {=bool}, ", &self.cto());
        defmt::write!(f, "oc: {=bool}, ", &self.oc());
        defmt::write!(f, "chgc: {=bool}, ", &self.chgc());
        defmt::write!(f, "chgv: {=bool}, ", &self.chgv());
        defmt::write!(f, "pchgc: {=bool}, ", &self.pchgc());
        defmt::write!(f, "utc: {=bool}, ", &self.utc());
        defmt::write!(f, "utd: {=bool}, ", &self.utd());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for SafetyStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for SafetyStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for SafetyStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for SafetyStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for SafetyStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for SafetyStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for SafetyStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "SAFETY_ALERT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct SafetyAlert {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for SafetyAlert {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl SafetyAlert {
    /// `bit 0` - Read the `cuv` field.
    ///
    #[doc(alias = "CUV")]
    #[must_use]
    pub fn cuv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `cov` field.
    ///
    #[doc(alias = "COV")]
    #[must_use]
    pub fn cov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `occ_1` field.
    ///
    #[doc(alias = "OCC1")]
    #[must_use]
    pub fn occ_1(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `occ_2` field.
    ///
    #[doc(alias = "OCC2")]
    #[must_use]
    pub fn occ_2(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `ocd_1` field.
    ///
    #[doc(alias = "OCD1")]
    #[must_use]
    pub fn ocd_1(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `ocd_2` field.
    ///
    #[doc(alias = "OCD2")]
    #[must_use]
    pub fn ocd_2(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `aoldl` field.
    ///
    #[doc(alias = "AOLDL")]
    #[must_use]
    pub fn aoldl(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `asccl` field.
    ///
    #[doc(alias = "ASCCL")]
    #[must_use]
    pub fn asccl(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `ascdl` field.
    ///
    #[doc(alias = "ASCDL")]
    #[must_use]
    pub fn ascdl(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `otc` field.
    ///
    #[doc(alias = "OTC")]
    #[must_use]
    pub fn otc(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `otd` field.
    ///
    #[doc(alias = "OTD")]
    #[must_use]
    pub fn otd(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `cuvc` field.
    ///
    #[doc(alias = "CUVC")]
    #[must_use]
    pub fn cuvc(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `otf` field.
    ///
    #[doc(alias = "OTF")]
    #[must_use]
    pub fn otf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `pto` field.
    ///
    #[doc(alias = "PTO")]
    #[must_use]
    pub fn pto(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `ptos` field.
    ///
    #[doc(alias = "PTOS")]
    #[must_use]
    pub fn ptos(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `cto` field.
    ///
    #[doc(alias = "CTO")]
    #[must_use]
    pub fn cto(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `ctos` field.
    ///
    #[doc(alias = "CTOS")]
    #[must_use]
    pub fn ctos(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `oc` field.
    ///
    #[doc(alias = "OC")]
    #[must_use]
    pub fn oc(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `chgc` field.
    ///
    #[doc(alias = "CHGC")]
    #[must_use]
    pub fn chgc(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `chgv` field.
    ///
    #[doc(alias = "CHGV")]
    #[must_use]
    pub fn chgv(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `pchgc` field.
    ///
    #[doc(alias = "PCHGC")]
    #[must_use]
    pub fn pchgc(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `utc` field.
    ///
    #[doc(alias = "UTC")]
    #[must_use]
    pub fn utc(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 27` - Read the `utd` field.
    ///
    #[doc(alias = "UTD")]
    #[must_use]
    pub fn utd(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for SafetyAlert {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for SafetyAlert {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<SafetyAlert> for [u8; 4] {
    fn from(val: SafetyAlert) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for SafetyAlert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("SafetyAlert");
        d.field("cuv", &self.cuv());
        d.field("cov", &self.cov());
        d.field("occ_1", &self.occ_1());
        d.field("occ_2", &self.occ_2());
        d.field("ocd_1", &self.ocd_1());
        d.field("ocd_2", &self.ocd_2());
        d.field("aoldl", &self.aoldl());
        d.field("asccl", &self.asccl());
        d.field("ascdl", &self.ascdl());
        d.field("otc", &self.otc());
        d.field("otd", &self.otd());
        d.field("cuvc", &self.cuvc());
        d.field("otf", &self.otf());
        d.field("pto", &self.pto());
        d.field("ptos", &self.ptos());
        d.field("cto", &self.cto());
        d.field("ctos", &self.ctos());
        d.field("oc", &self.oc());
        d.field("chgc", &self.chgc());
        d.field("chgv", &self.chgv());
        d.field("pchgc", &self.pchgc());
        d.field("utc", &self.utc());
        d.field("utd", &self.utd());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for SafetyAlert {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SafetyAlert {{ ");
        defmt::write!(f, "cuv: {=bool}, ", &self.cuv());
        defmt::write!(f, "cov: {=bool}, ", &self.cov());
        defmt::write!(f, "occ_1: {=bool}, ", &self.occ_1());
        defmt::write!(f, "occ_2: {=bool}, ", &self.occ_2());
        defmt::write!(f, "ocd_1: {=bool}, ", &self.ocd_1());
        defmt::write!(f, "ocd_2: {=bool}, ", &self.ocd_2());
        defmt::write!(f, "aoldl: {=bool}, ", &self.aoldl());
        defmt::write!(f, "asccl: {=bool}, ", &self.asccl());
        defmt::write!(f, "ascdl: {=bool}, ", &self.ascdl());
        defmt::write!(f, "otc: {=bool}, ", &self.otc());
        defmt::write!(f, "otd: {=bool}, ", &self.otd());
        defmt::write!(f, "cuvc: {=bool}, ", &self.cuvc());
        defmt::write!(f, "otf: {=bool}, ", &self.otf());
        defmt::write!(f, "pto: {=bool}, ", &self.pto());
        defmt::write!(f, "ptos: {=bool}, ", &self.ptos());
        defmt::write!(f, "cto: {=bool}, ", &self.cto());
        defmt::write!(f, "ctos: {=bool}, ", &self.ctos());
        defmt::write!(f, "oc: {=bool}, ", &self.oc());
        defmt::write!(f, "chgc: {=bool}, ", &self.chgc());
        defmt::write!(f, "chgv: {=bool}, ", &self.chgv());
        defmt::write!(f, "pchgc: {=bool}, ", &self.pchgc());
        defmt::write!(f, "utc: {=bool}, ", &self.utc());
        defmt::write!(f, "utd: {=bool}, ", &self.utd());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for SafetyAlert {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for SafetyAlert {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for SafetyAlert {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for SafetyAlert {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for SafetyAlert {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for SafetyAlert {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for SafetyAlert {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "STATE_OF_HEALTH_SOH")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct StateOfHealthSoh {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for StateOfHealthSoh {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl StateOfHealthSoh {
    /// `15:0` - Read the `state_of_health_soh` field.
    ///
    #[doc(alias = "STATE_OF_HEALTH_SOH")]
    #[must_use]
    pub fn state_of_health_soh(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for StateOfHealthSoh {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for StateOfHealthSoh {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<StateOfHealthSoh> for [u8; 2] {
    fn from(val: StateOfHealthSoh) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for StateOfHealthSoh {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("StateOfHealthSoh");
        d.field("state_of_health_soh", &self.state_of_health_soh());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for StateOfHealthSoh {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "StateOfHealthSoh {{ ");
        defmt::write!(f, "state_of_health_soh: {=i16}, ", &self.state_of_health_soh());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for StateOfHealthSoh {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for StateOfHealthSoh {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for StateOfHealthSoh {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for StateOfHealthSoh {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for StateOfHealthSoh {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for StateOfHealthSoh {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for StateOfHealthSoh {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "BTP_CHARGE_SET")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BtpChargeSet {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BtpChargeSet {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BtpChargeSet {
    /// `15:0` - Read the `btp_charge_set` field.
    ///
    #[doc(alias = "BTP_CHARGE_SET")]
    #[must_use]
    pub fn btp_charge_set(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `btp_charge_set` field.
    ///
    #[doc(alias = "BTP_CHARGE_SET")]
    pub fn set_btp_charge_set(&mut self, value: i16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<i16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BtpChargeSet {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BtpChargeSet {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BtpChargeSet> for [u8; 2] {
    fn from(val: BtpChargeSet) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BtpChargeSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BtpChargeSet");
        d.field("btp_charge_set", &self.btp_charge_set());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for BtpChargeSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BtpChargeSet {{ ");
        defmt::write!(f, "btp_charge_set: {=i16}, ", &self.btp_charge_set());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BtpChargeSet {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BtpChargeSet {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BtpChargeSet {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BtpChargeSet {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BtpChargeSet {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BtpChargeSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BtpChargeSet {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "BTP_DISCHARGE_SET")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BtpDischargeSet {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BtpDischargeSet {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BtpDischargeSet {
    /// `15:0` - Read the `btp_discharge_set` field.
    ///
    #[doc(alias = "BTP_DISCHARGE_SET")]
    #[must_use]
    pub fn btp_discharge_set(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `btp_discharge_set` field.
    ///
    #[doc(alias = "BTP_DISCHARGE_SET")]
    pub fn set_btp_discharge_set(&mut self, value: i16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<i16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BtpDischargeSet {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BtpDischargeSet {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BtpDischargeSet> for [u8; 2] {
    fn from(val: BtpDischargeSet) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BtpDischargeSet {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BtpDischargeSet");
        d.field("btp_discharge_set", &self.btp_discharge_set());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for BtpDischargeSet {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BtpDischargeSet {{ ");
        defmt::write!(f, "btp_discharge_set: {=i16}, ", &self.btp_discharge_set());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BtpDischargeSet {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BtpDischargeSet {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BtpDischargeSet {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BtpDischargeSet {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BtpDischargeSet {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BtpDischargeSet {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BtpDischargeSet {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CELL_VOLTAGE_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CellVoltage1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CellVoltage1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CellVoltage1 {
    /// `15:0` - Read the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    #[must_use]
    pub fn cell_voltage_1(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for CellVoltage1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CellVoltage1 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CellVoltage1> for [u8; 2] {
    fn from(val: CellVoltage1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CellVoltage1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CellVoltage1");
        d.field("cell_voltage_1", &self.cell_voltage_1());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for CellVoltage1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CellVoltage1 {{ ");
        defmt::write!(f, "cell_voltage_1: {=u16}, ", &self.cell_voltage_1());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CellVoltage1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CellVoltage1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CellVoltage1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CellVoltage1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CellVoltage1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CellVoltage1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CellVoltage1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CELL_VOLTAGE_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CellVoltage2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CellVoltage2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CellVoltage2 {
    /// `15:0` - Read the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    #[must_use]
    pub fn cell_voltage_2(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for CellVoltage2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CellVoltage2 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CellVoltage2> for [u8; 2] {
    fn from(val: CellVoltage2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CellVoltage2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CellVoltage2");
        d.field("cell_voltage_2", &self.cell_voltage_2());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for CellVoltage2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CellVoltage2 {{ ");
        defmt::write!(f, "cell_voltage_2: {=u16}, ", &self.cell_voltage_2());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CellVoltage2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CellVoltage2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CellVoltage2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CellVoltage2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CellVoltage2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CellVoltage2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CellVoltage2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CELL_VOLTAGE_3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CellVoltage3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CellVoltage3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CellVoltage3 {
    /// `15:0` - Read the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    #[must_use]
    pub fn cell_voltage_3(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for CellVoltage3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CellVoltage3 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CellVoltage3> for [u8; 2] {
    fn from(val: CellVoltage3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CellVoltage3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CellVoltage3");
        d.field("cell_voltage_3", &self.cell_voltage_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for CellVoltage3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CellVoltage3 {{ ");
        defmt::write!(f, "cell_voltage_3: {=u16}, ", &self.cell_voltage_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CellVoltage3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CellVoltage3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CellVoltage3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CellVoltage3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CellVoltage3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CellVoltage3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CellVoltage3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CELL_VOLTAGE_4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CellVoltage4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CellVoltage4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CellVoltage4 {
    /// `15:0` - Read the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    #[must_use]
    pub fn cell_voltage_4(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for CellVoltage4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CellVoltage4 {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CellVoltage4> for [u8; 2] {
    fn from(val: CellVoltage4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CellVoltage4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CellVoltage4");
        d.field("cell_voltage_4", &self.cell_voltage_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for CellVoltage4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CellVoltage4 {{ ");
        defmt::write!(f, "cell_voltage_4: {=u16}, ", &self.cell_voltage_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CellVoltage4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CellVoltage4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CellVoltage4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CellVoltage4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CellVoltage4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CellVoltage4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CellVoltage4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "SERIAL_NUMBER")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct SerialNumber {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for SerialNumber {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl SerialNumber {
    /// `15:0` - Read the `serial_number` field.
    ///
    #[doc(alias = "SERIAL_NUMBER")]
    #[must_use]
    pub fn serial_number(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `serial_number` field.
    ///
    #[doc(alias = "SERIAL_NUMBER")]
    pub fn set_serial_number(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for SerialNumber {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for SerialNumber {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<SerialNumber> for [u8; 2] {
    fn from(val: SerialNumber) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for SerialNumber {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("SerialNumber");
        d.field("serial_number", &self.serial_number());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for SerialNumber {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SerialNumber {{ ");
        defmt::write!(f, "serial_number: {=u16}, ", &self.serial_number());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for SerialNumber {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for SerialNumber {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for SerialNumber {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for SerialNumber {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for SerialNumber {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for SerialNumber {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for SerialNumber {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MANUFACTURE_DATE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ManufactureDate {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ManufactureDate {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ManufactureDate {
    /// `15:0` - Read the `manufacture_date` field.
    ///
    #[doc(alias = "MANUFACTURE_DATE")]
    #[must_use]
    pub fn manufacture_date(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `manufacture_date` field.
    ///
    #[doc(alias = "MANUFACTURE_DATE")]
    pub fn set_manufacture_date(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for ManufactureDate {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ManufactureDate {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ManufactureDate> for [u8; 2] {
    fn from(val: ManufactureDate) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ManufactureDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ManufactureDate");
        d.field("manufacture_date", &self.manufacture_date());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for ManufactureDate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ManufactureDate {{ ");
        defmt::write!(f, "manufacture_date: {=u16}, ", &self.manufacture_date());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ManufactureDate {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ManufactureDate {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ManufactureDate {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ManufactureDate {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ManufactureDate {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ManufactureDate {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ManufactureDate {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "SPECIFICATION_INFO")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct SpecificationInfo {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for SpecificationInfo {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl SpecificationInfo {
    /// `3:0` - Read the `revision` field.
    ///
    #[doc(alias = "REVISION")]
    #[must_use]
    pub fn revision(&self) -> u8 {
        let start = 0;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:4` - Read the `version` field.
    ///
    #[doc(alias = "VERSION")]
    #[must_use]
    pub fn version(&self) -> u8 {
        let start = 4;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `11:8` - Read the `vscale` field.
    ///
    #[doc(alias = "VSCALE")]
    #[must_use]
    pub fn vscale(&self) -> u8 {
        let start = 8;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:12` - Read the `ipscale` field.
    ///
    #[doc(alias = "IPSCALE")]
    #[must_use]
    pub fn ipscale(&self) -> u8 {
        let start = 12;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `3:0` - Set the `revision` field.
    ///
    #[doc(alias = "REVISION")]
    pub fn set_revision(&mut self, value: u8) {
        let start = 0;
        let end = 3;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `7:4` - Set the `version` field.
    ///
    #[doc(alias = "VERSION")]
    pub fn set_version(&mut self, value: u8) {
        let start = 4;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for SpecificationInfo {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for SpecificationInfo {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<SpecificationInfo> for [u8; 2] {
    fn from(val: SpecificationInfo) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for SpecificationInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("SpecificationInfo");
        d.field("revision", &self.revision());
        d.field("version", &self.version());
        d.field("vscale", &self.vscale());
        d.field("ipscale", &self.ipscale());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for SpecificationInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "SpecificationInfo {{ ");
        defmt::write!(f, "revision: {=u8}, ", &self.revision());
        defmt::write!(f, "version: {=u8}, ", &self.version());
        defmt::write!(f, "vscale: {=u8}, ", &self.vscale());
        defmt::write!(f, "ipscale: {=u8}, ", &self.ipscale());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for SpecificationInfo {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for SpecificationInfo {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for SpecificationInfo {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for SpecificationInfo {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for SpecificationInfo {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for SpecificationInfo {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for SpecificationInfo {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "DESIGN_VOLTAGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DesignVoltage {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for DesignVoltage {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl DesignVoltage {
    /// `15:0` - Read the `design_voltage` field.
    ///
    #[doc(alias = "DESIGN_VOLTAGE")]
    #[must_use]
    pub fn design_voltage(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `design_voltage` field.
    ///
    #[doc(alias = "DESIGN_VOLTAGE")]
    pub fn set_design_voltage(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for DesignVoltage {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for DesignVoltage {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<DesignVoltage> for [u8; 2] {
    fn from(val: DesignVoltage) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DesignVoltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DesignVoltage");
        d.field("design_voltage", &self.design_voltage());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for DesignVoltage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DesignVoltage {{ ");
        defmt::write!(f, "design_voltage: {=u16}, ", &self.design_voltage());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DesignVoltage {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DesignVoltage {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DesignVoltage {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DesignVoltage {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DesignVoltage {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DesignVoltage {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DesignVoltage {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "DESIGN_CAPACITY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct DesignCapacity {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for DesignCapacity {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl DesignCapacity {
    /// `15:0` - Read the `design_capacity` field.
    ///
    #[doc(alias = "DESIGN_CAPACITY")]
    #[must_use]
    pub fn design_capacity(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `design_capacity` field.
    ///
    #[doc(alias = "DESIGN_CAPACITY")]
    pub fn set_design_capacity(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for DesignCapacity {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for DesignCapacity {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<DesignCapacity> for [u8; 2] {
    fn from(val: DesignCapacity) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for DesignCapacity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("DesignCapacity");
        d.field("design_capacity", &self.design_capacity());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for DesignCapacity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "DesignCapacity {{ ");
        defmt::write!(f, "design_capacity: {=u16}, ", &self.design_capacity());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for DesignCapacity {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for DesignCapacity {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for DesignCapacity {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for DesignCapacity {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for DesignCapacity {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for DesignCapacity {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for DesignCapacity {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CYCLE_COUNT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct CycleCount {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for CycleCount {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl CycleCount {
    /// `15:0` - Read the `cycle_count` field.
    ///
    #[doc(alias = "CYCLE_COUNT")]
    #[must_use]
    pub fn cycle_count(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `cycle_count` field.
    ///
    #[doc(alias = "CYCLE_COUNT")]
    pub fn set_cycle_count(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for CycleCount {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for CycleCount {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<CycleCount> for [u8; 2] {
    fn from(val: CycleCount) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for CycleCount {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("CycleCount");
        d.field("cycle_count", &self.cycle_count());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for CycleCount {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "CycleCount {{ ");
        defmt::write!(f, "cycle_count: {=u16}, ", &self.cycle_count());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for CycleCount {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for CycleCount {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for CycleCount {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for CycleCount {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for CycleCount {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for CycleCount {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for CycleCount {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "BATTERY_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BatteryStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BatteryStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BatteryStatus {
    /// `2:0` - Read the `ec` field.
    ///
    #[doc(alias = "EC")]
    #[must_use]
    pub fn ec(&self) -> ErrorCode {
        let start = 0;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw.into()
    }
    /// `bit 4` - Read the `fd` field.
    ///
    #[doc(alias = "FD")]
    #[must_use]
    pub fn fd(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `fc` field.
    ///
    #[doc(alias = "FC")]
    #[must_use]
    pub fn fc(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `dsg` field.
    ///
    #[doc(alias = "DSG")]
    #[must_use]
    pub fn dsg(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `init` field.
    ///
    #[doc(alias = "INIT")]
    #[must_use]
    pub fn init(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `rta` field.
    ///
    #[doc(alias = "RTA")]
    #[must_use]
    pub fn rta(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `rca` field.
    ///
    #[doc(alias = "RCA")]
    #[must_use]
    pub fn rca(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `tda` field.
    ///
    #[doc(alias = "TDA")]
    #[must_use]
    pub fn tda(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `ota` field.
    ///
    #[doc(alias = "OTA")]
    #[must_use]
    pub fn ota(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `tca` field.
    ///
    #[doc(alias = "TCA")]
    #[must_use]
    pub fn tca(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `oca` field.
    ///
    #[doc(alias = "OCA")]
    #[must_use]
    pub fn oca(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
}
impl Default for BatteryStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BatteryStatus {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BatteryStatus> for [u8; 2] {
    fn from(val: BatteryStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BatteryStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BatteryStatus");
        d.field("ec", &self.ec());
        d.field("fd", &self.fd());
        d.field("fc", &self.fc());
        d.field("dsg", &self.dsg());
        d.field("init", &self.init());
        d.field("rta", &self.rta());
        d.field("rca", &self.rca());
        d.field("tda", &self.tda());
        d.field("ota", &self.ota());
        d.field("tca", &self.tca());
        d.field("oca", &self.oca());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for BatteryStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BatteryStatus {{ ");
        defmt::write!(f, "ec: {}, ", &self.ec());
        defmt::write!(f, "fd: {=bool}, ", &self.fd());
        defmt::write!(f, "fc: {=bool}, ", &self.fc());
        defmt::write!(f, "dsg: {=bool}, ", &self.dsg());
        defmt::write!(f, "init: {=bool}, ", &self.init());
        defmt::write!(f, "rta: {=bool}, ", &self.rta());
        defmt::write!(f, "rca: {=bool}, ", &self.rca());
        defmt::write!(f, "tda: {=bool}, ", &self.tda());
        defmt::write!(f, "ota: {=bool}, ", &self.ota());
        defmt::write!(f, "tca: {=bool}, ", &self.tca());
        defmt::write!(f, "oca: {=bool}, ", &self.oca());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BatteryStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BatteryStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BatteryStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BatteryStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BatteryStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BatteryStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BatteryStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGING_VOLTAGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargingVoltage {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargingVoltage {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargingVoltage {
    /// `15:0` - Read the `charging_voltage` field.
    ///
    #[doc(alias = "CHARGING_VOLTAGE")]
    #[must_use]
    pub fn charging_voltage(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for ChargingVoltage {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargingVoltage {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargingVoltage> for [u8; 2] {
    fn from(val: ChargingVoltage) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargingVoltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargingVoltage");
        d.field("charging_voltage", &self.charging_voltage());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for ChargingVoltage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargingVoltage {{ ");
        defmt::write!(f, "charging_voltage: {=u16}, ", &self.charging_voltage());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargingVoltage {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargingVoltage {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargingVoltage {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargingVoltage {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargingVoltage {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargingVoltage {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargingVoltage {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CHARGING_CURRENT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct ChargingCurrent {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for ChargingCurrent {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl ChargingCurrent {
    /// `15:0` - Read the `charging_current` field.
    ///
    #[doc(alias = "CHARGING_CURRENT")]
    #[must_use]
    pub fn charging_current(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for ChargingCurrent {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for ChargingCurrent {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<ChargingCurrent> for [u8; 2] {
    fn from(val: ChargingCurrent) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for ChargingCurrent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("ChargingCurrent");
        d.field("charging_current", &self.charging_current());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for ChargingCurrent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "ChargingCurrent {{ ");
        defmt::write!(f, "charging_current: {=u16}, ", &self.charging_current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for ChargingCurrent {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for ChargingCurrent {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for ChargingCurrent {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for ChargingCurrent {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for ChargingCurrent {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for ChargingCurrent {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for ChargingCurrent {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AVERAGE_TIME_TO_FULL")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AverageTimeToFull {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AverageTimeToFull {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AverageTimeToFull {
    /// `15:0` - Read the `average_time_to_full` field.
    ///
    #[doc(alias = "AVERAGE_TIME_TO_FULL")]
    #[must_use]
    pub fn average_time_to_full(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AverageTimeToFull {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AverageTimeToFull {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AverageTimeToFull> for [u8; 2] {
    fn from(val: AverageTimeToFull) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AverageTimeToFull {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AverageTimeToFull");
        d.field("average_time_to_full", &self.average_time_to_full());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AverageTimeToFull {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AverageTimeToFull {{ ");
        defmt::write!(f, "average_time_to_full: {=u16}, ", &self.average_time_to_full());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AverageTimeToFull {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AverageTimeToFull {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AverageTimeToFull {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AverageTimeToFull {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AverageTimeToFull {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AverageTimeToFull {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AverageTimeToFull {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AVERAGE_TIME_TO_EMPTY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AverageTimeToEmpty {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AverageTimeToEmpty {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AverageTimeToEmpty {
    /// `15:0` - Read the `average_time_to_empty` field.
    ///
    #[doc(alias = "AVERAGE_TIME_TO_EMPTY")]
    #[must_use]
    pub fn average_time_to_empty(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AverageTimeToEmpty {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AverageTimeToEmpty {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AverageTimeToEmpty> for [u8; 2] {
    fn from(val: AverageTimeToEmpty) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AverageTimeToEmpty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AverageTimeToEmpty");
        d.field("average_time_to_empty", &self.average_time_to_empty());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AverageTimeToEmpty {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AverageTimeToEmpty {{ ");
        defmt::write!(f, "average_time_to_empty: {=u16}, ", &self.average_time_to_empty());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AverageTimeToEmpty {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AverageTimeToEmpty {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AverageTimeToEmpty {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AverageTimeToEmpty {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AverageTimeToEmpty {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AverageTimeToEmpty {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AverageTimeToEmpty {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "RUN_TIME_TO_EMPTY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct RunTimeToEmpty {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for RunTimeToEmpty {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl RunTimeToEmpty {
    /// `15:0` - Read the `run_time_to_empty` field.
    ///
    #[doc(alias = "RUN_TIME_TO_EMPTY")]
    #[must_use]
    pub fn run_time_to_empty(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for RunTimeToEmpty {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for RunTimeToEmpty {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<RunTimeToEmpty> for [u8; 2] {
    fn from(val: RunTimeToEmpty) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RunTimeToEmpty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RunTimeToEmpty");
        d.field("run_time_to_empty", &self.run_time_to_empty());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for RunTimeToEmpty {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RunTimeToEmpty {{ ");
        defmt::write!(f, "run_time_to_empty: {=u16}, ", &self.run_time_to_empty());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RunTimeToEmpty {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RunTimeToEmpty {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RunTimeToEmpty {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RunTimeToEmpty {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RunTimeToEmpty {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RunTimeToEmpty {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RunTimeToEmpty {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "FULL_CHARGE_CAPACITY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct FullChargeCapacity {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for FullChargeCapacity {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl FullChargeCapacity {
    /// `15:0` - Read the `full_charge_capacity` field.
    ///
    #[doc(alias = "FULL_CHARGE_CAPACITY")]
    #[must_use]
    pub fn full_charge_capacity(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for FullChargeCapacity {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for FullChargeCapacity {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<FullChargeCapacity> for [u8; 2] {
    fn from(val: FullChargeCapacity) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for FullChargeCapacity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("FullChargeCapacity");
        d.field("full_charge_capacity", &self.full_charge_capacity());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for FullChargeCapacity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "FullChargeCapacity {{ ");
        defmt::write!(f, "full_charge_capacity: {=u16}, ", &self.full_charge_capacity());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for FullChargeCapacity {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for FullChargeCapacity {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for FullChargeCapacity {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for FullChargeCapacity {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for FullChargeCapacity {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for FullChargeCapacity {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for FullChargeCapacity {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "REMAINING_CAPACITY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct RemainingCapacity {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for RemainingCapacity {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl RemainingCapacity {
    /// `15:0` - Read the `remaining_capacity` field.
    ///
    #[doc(alias = "REMAINING_CAPACITY")]
    #[must_use]
    pub fn remaining_capacity(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for RemainingCapacity {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for RemainingCapacity {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<RemainingCapacity> for [u8; 2] {
    fn from(val: RemainingCapacity) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RemainingCapacity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RemainingCapacity");
        d.field("remaining_capacity", &self.remaining_capacity());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for RemainingCapacity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RemainingCapacity {{ ");
        defmt::write!(f, "remaining_capacity: {=u16}, ", &self.remaining_capacity());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RemainingCapacity {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RemainingCapacity {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RemainingCapacity {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RemainingCapacity {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RemainingCapacity {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RemainingCapacity {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RemainingCapacity {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "ABSOLUTE_STATE_OF_CHARGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AbsoluteStateOfCharge {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AbsoluteStateOfCharge {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AbsoluteStateOfCharge {
    /// `15:0` - Read the `absolute_state_of_charge` field.
    ///
    #[doc(alias = "ABSOLUTE_STATE_OF_CHARGE")]
    #[must_use]
    pub fn absolute_state_of_charge(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AbsoluteStateOfCharge {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AbsoluteStateOfCharge {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AbsoluteStateOfCharge> for [u8; 2] {
    fn from(val: AbsoluteStateOfCharge) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AbsoluteStateOfCharge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AbsoluteStateOfCharge");
        d.field("absolute_state_of_charge", &self.absolute_state_of_charge());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AbsoluteStateOfCharge {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AbsoluteStateOfCharge {{ ");
        defmt::write!(
            f,
            "absolute_state_of_charge: {=u16}, ",
            &self.absolute_state_of_charge()
        );
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AbsoluteStateOfCharge {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AbsoluteStateOfCharge {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AbsoluteStateOfCharge {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AbsoluteStateOfCharge {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AbsoluteStateOfCharge {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AbsoluteStateOfCharge {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AbsoluteStateOfCharge {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "RELATIVE_STATE_OF_CHARGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct RelativeStateOfCharge {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for RelativeStateOfCharge {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl RelativeStateOfCharge {
    /// `15:0` - Read the `relative_state_of_charge` field.
    ///
    #[doc(alias = "RELATIVE_STATE_OF_CHARGE")]
    #[must_use]
    pub fn relative_state_of_charge(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for RelativeStateOfCharge {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for RelativeStateOfCharge {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<RelativeStateOfCharge> for [u8; 2] {
    fn from(val: RelativeStateOfCharge) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RelativeStateOfCharge {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RelativeStateOfCharge");
        d.field("relative_state_of_charge", &self.relative_state_of_charge());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for RelativeStateOfCharge {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RelativeStateOfCharge {{ ");
        defmt::write!(
            f,
            "relative_state_of_charge: {=u16}, ",
            &self.relative_state_of_charge()
        );
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RelativeStateOfCharge {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RelativeStateOfCharge {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RelativeStateOfCharge {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RelativeStateOfCharge {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RelativeStateOfCharge {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RelativeStateOfCharge {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RelativeStateOfCharge {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAX_ERROR")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MaxError {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for MaxError {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl MaxError {
    /// `15:0` - Read the `max_error` field.
    ///
    #[doc(alias = "MAX_ERROR")]
    #[must_use]
    pub fn max_error(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for MaxError {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for MaxError {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<MaxError> for [u8; 2] {
    fn from(val: MaxError) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MaxError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MaxError");
        d.field("max_error", &self.max_error());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MaxError {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MaxError {{ ");
        defmt::write!(f, "max_error: {=u16}, ", &self.max_error());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MaxError {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MaxError {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MaxError {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MaxError {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MaxError {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MaxError {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MaxError {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AVG_CURRENT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AvgCurrent {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AvgCurrent {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AvgCurrent {
    /// `15:0` - Read the `avg_current` field.
    ///
    #[doc(alias = "AVG_CURRENT")]
    #[must_use]
    pub fn avg_current(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AvgCurrent {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AvgCurrent {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AvgCurrent> for [u8; 2] {
    fn from(val: AvgCurrent) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AvgCurrent {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AvgCurrent");
        d.field("avg_current", &self.avg_current());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AvgCurrent {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AvgCurrent {{ ");
        defmt::write!(f, "avg_current: {=i16}, ", &self.avg_current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AvgCurrent {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AvgCurrent {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AvgCurrent {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AvgCurrent {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AvgCurrent {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AvgCurrent {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AvgCurrent {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "CURRENT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Current {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Current {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Current {
    /// `15:0` - Read the `current` field.
    ///
    #[doc(alias = "CURRENT")]
    #[must_use]
    pub fn current(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for Current {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Current {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Current> for [u8; 2] {
    fn from(val: Current) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Current {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Current");
        d.field("current", &self.current());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for Current {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Current {{ ");
        defmt::write!(f, "current: {=i16}, ", &self.current());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Current {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Current {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Current {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Current {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Current {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Current {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Current {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "VOLTAGE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Voltage {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Voltage {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Voltage {
    /// `15:0` - Read the `voltage` field.
    ///
    #[doc(alias = "VOLTAGE")]
    #[must_use]
    pub fn voltage(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for Voltage {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Voltage {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Voltage> for [u8; 2] {
    fn from(val: Voltage) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Voltage {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Voltage");
        d.field("voltage", &self.voltage());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for Voltage {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Voltage {{ ");
        defmt::write!(f, "voltage: {=u16}, ", &self.voltage());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Voltage {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Voltage {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Voltage {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Voltage {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Voltage {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Voltage {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Voltage {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "TEMPERATURE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct Temperature {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for Temperature {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl Temperature {
    /// `15:0` - Read the `temperature` field.
    ///
    #[doc(alias = "TEMPERATURE")]
    #[must_use]
    pub fn temperature(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for Temperature {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for Temperature {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<Temperature> for [u8; 2] {
    fn from(val: Temperature) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for Temperature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("Temperature");
        d.field("temperature", &self.temperature());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for Temperature {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "Temperature {{ ");
        defmt::write!(f, "temperature: {=u16}, ", &self.temperature());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for Temperature {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for Temperature {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for Temperature {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for Temperature {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for Temperature {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for Temperature {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for Temperature {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AT_RATE_OK")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AtRateOk {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AtRateOk {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AtRateOk {
    /// `15:0` - Read the `at_rate_ok` field.
    ///
    #[doc(alias = "AT_RATE_OK")]
    #[must_use]
    pub fn at_rate_ok(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AtRateOk {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AtRateOk {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AtRateOk> for [u8; 2] {
    fn from(val: AtRateOk) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AtRateOk {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AtRateOk");
        d.field("at_rate_ok", &self.at_rate_ok());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AtRateOk {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AtRateOk {{ ");
        defmt::write!(f, "at_rate_ok: {=u16}, ", &self.at_rate_ok());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AtRateOk {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AtRateOk {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AtRateOk {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AtRateOk {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AtRateOk {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AtRateOk {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AtRateOk {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AT_RATE_TIME_TO_EMPTY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AtRateTimeToEmpty {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AtRateTimeToEmpty {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AtRateTimeToEmpty {
    /// `15:0` - Read the `at_rate_time_to_empty` field.
    ///
    #[doc(alias = "AT_RATE_TIME_TO_EMPTY")]
    #[must_use]
    pub fn at_rate_time_to_empty(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AtRateTimeToEmpty {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AtRateTimeToEmpty {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AtRateTimeToEmpty> for [u8; 2] {
    fn from(val: AtRateTimeToEmpty) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AtRateTimeToEmpty {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AtRateTimeToEmpty");
        d.field("at_rate_time_to_empty", &self.at_rate_time_to_empty());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AtRateTimeToEmpty {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AtRateTimeToEmpty {{ ");
        defmt::write!(f, "at_rate_time_to_empty: {=u16}, ", &self.at_rate_time_to_empty());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AtRateTimeToEmpty {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AtRateTimeToEmpty {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AtRateTimeToEmpty {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AtRateTimeToEmpty {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AtRateTimeToEmpty {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AtRateTimeToEmpty {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AtRateTimeToEmpty {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AT_RATE_TIME_TO_FULL")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AtRateTimeToFull {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AtRateTimeToFull {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AtRateTimeToFull {
    /// `15:0` - Read the `at_rate_time_to_full` field.
    ///
    #[doc(alias = "AT_RATE_TIME_TO_FULL")]
    #[must_use]
    pub fn at_rate_time_to_full(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
}
impl Default for AtRateTimeToFull {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AtRateTimeToFull {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AtRateTimeToFull> for [u8; 2] {
    fn from(val: AtRateTimeToFull) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AtRateTimeToFull {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AtRateTimeToFull");
        d.field("at_rate_time_to_full", &self.at_rate_time_to_full());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AtRateTimeToFull {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AtRateTimeToFull {{ ");
        defmt::write!(f, "at_rate_time_to_full: {=u16}, ", &self.at_rate_time_to_full());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AtRateTimeToFull {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AtRateTimeToFull {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AtRateTimeToFull {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AtRateTimeToFull {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AtRateTimeToFull {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AtRateTimeToFull {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AtRateTimeToFull {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "AT_RATE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct AtRate {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for AtRate {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl AtRate {
    /// `15:0` - Read the `at_rate` field.
    ///
    #[doc(alias = "AT_RATE")]
    #[must_use]
    pub fn at_rate(&self) -> i16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<i16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `at_rate` field.
    ///
    #[doc(alias = "AT_RATE")]
    pub fn set_at_rate(&mut self, value: i16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<i16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for AtRate {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for AtRate {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<AtRate> for [u8; 2] {
    fn from(val: AtRate) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for AtRate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("AtRate");
        d.field("at_rate", &self.at_rate());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for AtRate {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "AtRate {{ ");
        defmt::write!(f, "at_rate: {=i16}, ", &self.at_rate());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for AtRate {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for AtRate {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for AtRate {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for AtRate {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for AtRate {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for AtRate {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for AtRate {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "BATTERY_MODE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct BatteryMode {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for BatteryMode {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl BatteryMode {
    /// `bit 0` - Read the `icc` field.
    ///
    #[doc(alias = "ICC")]
    #[must_use]
    pub fn icc(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `pbs` field.
    ///
    #[doc(alias = "PBS")]
    #[must_use]
    pub fn pbs(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `cf` field.
    ///
    #[doc(alias = "CF")]
    #[must_use]
    pub fn cf(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `cc` field.
    ///
    #[doc(alias = "CC")]
    #[must_use]
    pub fn cc(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `pb` field.
    ///
    #[doc(alias = "PB")]
    #[must_use]
    pub fn pb(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `am` field.
    ///
    #[doc(alias = "AM")]
    #[must_use]
    pub fn am(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `chgm` field.
    ///
    #[doc(alias = "CHGM")]
    #[must_use]
    pub fn chgm(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `capm` field.
    ///
    #[doc(alias = "CAPM")]
    #[must_use]
    pub fn capm(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Set the `cc` field.
    ///
    #[doc(alias = "CC")]
    pub fn set_cc(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `pb` field.
    ///
    #[doc(alias = "PB")]
    pub fn set_pb(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `am` field.
    ///
    #[doc(alias = "AM")]
    pub fn set_am(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `chgm` field.
    ///
    #[doc(alias = "CHGM")]
    pub fn set_chgm(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `capm` field.
    ///
    #[doc(alias = "CAPM")]
    pub fn set_capm(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for BatteryMode {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for BatteryMode {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<BatteryMode> for [u8; 2] {
    fn from(val: BatteryMode) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for BatteryMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("BatteryMode");
        d.field("icc", &self.icc());
        d.field("pbs", &self.pbs());
        d.field("cf", &self.cf());
        d.field("cc", &self.cc());
        d.field("pb", &self.pb());
        d.field("am", &self.am());
        d.field("chgm", &self.chgm());
        d.field("capm", &self.capm());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for BatteryMode {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "BatteryMode {{ ");
        defmt::write!(f, "icc: {=bool}, ", &self.icc());
        defmt::write!(f, "pbs: {=bool}, ", &self.pbs());
        defmt::write!(f, "cf: {=bool}, ", &self.cf());
        defmt::write!(f, "cc: {=bool}, ", &self.cc());
        defmt::write!(f, "pb: {=bool}, ", &self.pb());
        defmt::write!(f, "am: {=bool}, ", &self.am());
        defmt::write!(f, "chgm: {=bool}, ", &self.chgm());
        defmt::write!(f, "capm: {=bool}, ", &self.capm());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for BatteryMode {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for BatteryMode {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for BatteryMode {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for BatteryMode {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for BatteryMode {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for BatteryMode {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for BatteryMode {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "REMAINING_TIME_ALARM")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct RemainingTimeAlarm {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for RemainingTimeAlarm {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl RemainingTimeAlarm {
    /// `15:0` - Read the `remaining_time_alarm` field.
    ///
    #[doc(alias = "REMAINING_TIME_ALARM")]
    #[must_use]
    pub fn remaining_time_alarm(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `remaining_time_alarm` field.
    ///
    #[doc(alias = "REMAINING_TIME_ALARM")]
    pub fn set_remaining_time_alarm(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for RemainingTimeAlarm {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for RemainingTimeAlarm {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<RemainingTimeAlarm> for [u8; 2] {
    fn from(val: RemainingTimeAlarm) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RemainingTimeAlarm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RemainingTimeAlarm");
        d.field("remaining_time_alarm", &self.remaining_time_alarm());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for RemainingTimeAlarm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RemainingTimeAlarm {{ ");
        defmt::write!(f, "remaining_time_alarm: {=u16}, ", &self.remaining_time_alarm());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RemainingTimeAlarm {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RemainingTimeAlarm {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RemainingTimeAlarm {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RemainingTimeAlarm {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RemainingTimeAlarm {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RemainingTimeAlarm {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RemainingTimeAlarm {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "REMAINING_CAPACITY_ALARM")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct RemainingCapacityAlarm {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for RemainingCapacityAlarm {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl RemainingCapacityAlarm {
    /// `15:0` - Read the `remaining_capacity_alarm` field.
    ///
    #[doc(alias = "REMAINING_CAPACITY_ALARM")]
    #[must_use]
    pub fn remaining_capacity_alarm(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `remaining_capacity_alarm` field.
    ///
    #[doc(alias = "REMAINING_CAPACITY_ALARM")]
    pub fn set_remaining_capacity_alarm(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for RemainingCapacityAlarm {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for RemainingCapacityAlarm {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<RemainingCapacityAlarm> for [u8; 2] {
    fn from(val: RemainingCapacityAlarm) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for RemainingCapacityAlarm {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("RemainingCapacityAlarm");
        d.field("remaining_capacity_alarm", &self.remaining_capacity_alarm());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for RemainingCapacityAlarm {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "RemainingCapacityAlarm {{ ");
        defmt::write!(
            f,
            "remaining_capacity_alarm: {=u16}, ",
            &self.remaining_capacity_alarm()
        );
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for RemainingCapacityAlarm {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for RemainingCapacityAlarm {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for RemainingCapacityAlarm {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for RemainingCapacityAlarm {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for RemainingCapacityAlarm {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for RemainingCapacityAlarm {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for RemainingCapacityAlarm {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_OUTPUT_SHORTED_CCADC_CAL")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacOutputShortedCcadcCal {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 24],
}
unsafe impl ::device_driver::Fieldset for MacOutputShortedCcadcCal {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 24] };
}
impl MacOutputShortedCcadcCal {
    /// `7:0` - Read the `refresh_ctr` field.
    ///
    #[doc(alias = "REFRESH_CTR")]
    #[must_use]
    pub fn refresh_ctr(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `status` field.
    ///
    #[doc(alias = "STATUS")]
    #[must_use]
    pub fn status(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `current` field.
    ///
    #[doc(alias = "CURRENT")]
    #[must_use]
    pub fn current(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    #[must_use]
    pub fn cell_voltage_1(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    #[must_use]
    pub fn cell_voltage_2(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    #[must_use]
    pub fn cell_voltage_3(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    #[must_use]
    pub fn cell_voltage_4(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `pack_voltage` field.
    ///
    #[doc(alias = "PACK_VOLTAGE")]
    #[must_use]
    pub fn pack_voltage(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `bat_voltage` field.
    ///
    #[doc(alias = "BAT_VOLTAGE")]
    #[must_use]
    pub fn bat_voltage(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `cell_current_1` field.
    ///
    #[doc(alias = "CELL_CURRENT_1")]
    #[must_use]
    pub fn cell_current_1(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `cell_current_2` field.
    ///
    #[doc(alias = "CELL_CURRENT_2")]
    #[must_use]
    pub fn cell_current_2(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `cell_current_3` field.
    ///
    #[doc(alias = "CELL_CURRENT_3")]
    #[must_use]
    pub fn cell_current_3(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `cell_current_4` field.
    ///
    #[doc(alias = "CELL_CURRENT_4")]
    #[must_use]
    pub fn cell_current_4(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `refresh_ctr` field.
    ///
    #[doc(alias = "REFRESH_CTR")]
    pub fn set_refresh_ctr(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:8` - Set the `status` field.
    ///
    #[doc(alias = "STATUS")]
    pub fn set_status(&mut self, value: u8) {
        let start = 8;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `current` field.
    ///
    #[doc(alias = "CURRENT")]
    pub fn set_current(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    pub fn set_cell_voltage_1(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    pub fn set_cell_voltage_2(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    pub fn set_cell_voltage_3(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    pub fn set_cell_voltage_4(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `pack_voltage` field.
    ///
    #[doc(alias = "PACK_VOLTAGE")]
    pub fn set_pack_voltage(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `bat_voltage` field.
    ///
    #[doc(alias = "BAT_VOLTAGE")]
    pub fn set_bat_voltage(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `cell_current_1` field.
    ///
    #[doc(alias = "CELL_CURRENT_1")]
    pub fn set_cell_current_1(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `cell_current_2` field.
    ///
    #[doc(alias = "CELL_CURRENT_2")]
    pub fn set_cell_current_2(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `cell_current_3` field.
    ///
    #[doc(alias = "CELL_CURRENT_3")]
    pub fn set_cell_current_3(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `cell_current_4` field.
    ///
    #[doc(alias = "CELL_CURRENT_4")]
    pub fn set_cell_current_4(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacOutputShortedCcadcCal {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 24]> for MacOutputShortedCcadcCal {
    fn from(bits: [u8; 24]) -> Self {
        Self { bits }
    }
}
impl From<MacOutputShortedCcadcCal> for [u8; 24] {
    fn from(val: MacOutputShortedCcadcCal) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacOutputShortedCcadcCal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacOutputShortedCcadcCal");
        d.field("refresh_ctr", &self.refresh_ctr());
        d.field("status", &self.status());
        d.field("current", &self.current());
        d.field("cell_voltage_1", &self.cell_voltage_1());
        d.field("cell_voltage_2", &self.cell_voltage_2());
        d.field("cell_voltage_3", &self.cell_voltage_3());
        d.field("cell_voltage_4", &self.cell_voltage_4());
        d.field("pack_voltage", &self.pack_voltage());
        d.field("bat_voltage", &self.bat_voltage());
        d.field("cell_current_1", &self.cell_current_1());
        d.field("cell_current_2", &self.cell_current_2());
        d.field("cell_current_3", &self.cell_current_3());
        d.field("cell_current_4", &self.cell_current_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacOutputShortedCcadcCal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacOutputShortedCcadcCal {{ ");
        defmt::write!(f, "refresh_ctr: {=u8}, ", &self.refresh_ctr());
        defmt::write!(f, "status: {=u8}, ", &self.status());
        defmt::write!(f, "current: {=u16}, ", &self.current());
        defmt::write!(f, "cell_voltage_1: {=u16}, ", &self.cell_voltage_1());
        defmt::write!(f, "cell_voltage_2: {=u16}, ", &self.cell_voltage_2());
        defmt::write!(f, "cell_voltage_3: {=u16}, ", &self.cell_voltage_3());
        defmt::write!(f, "cell_voltage_4: {=u16}, ", &self.cell_voltage_4());
        defmt::write!(f, "pack_voltage: {=u16}, ", &self.pack_voltage());
        defmt::write!(f, "bat_voltage: {=u16}, ", &self.bat_voltage());
        defmt::write!(f, "cell_current_1: {=u16}, ", &self.cell_current_1());
        defmt::write!(f, "cell_current_2: {=u16}, ", &self.cell_current_2());
        defmt::write!(f, "cell_current_3: {=u16}, ", &self.cell_current_3());
        defmt::write!(f, "cell_current_4: {=u16}, ", &self.cell_current_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacOutputShortedCcadcCal {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacOutputShortedCcadcCal {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacOutputShortedCcadcCal {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacOutputShortedCcadcCal {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacOutputShortedCcadcCal {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacOutputShortedCcadcCal {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacOutputShortedCcadcCal {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_OUTPUT_CCADC_CAL")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacOutputCcadcCal {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 24],
}
unsafe impl ::device_driver::Fieldset for MacOutputCcadcCal {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 24] };
}
impl MacOutputCcadcCal {
    /// `7:0` - Read the `refresh_ctr` field.
    ///
    #[doc(alias = "REFRESH_CTR")]
    #[must_use]
    pub fn refresh_ctr(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `status` field.
    ///
    #[doc(alias = "STATUS")]
    #[must_use]
    pub fn status(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `current` field.
    ///
    #[doc(alias = "CURRENT")]
    #[must_use]
    pub fn current(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    #[must_use]
    pub fn cell_voltage_1(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    #[must_use]
    pub fn cell_voltage_2(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    #[must_use]
    pub fn cell_voltage_3(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    #[must_use]
    pub fn cell_voltage_4(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `pack_voltage` field.
    ///
    #[doc(alias = "PACK_VOLTAGE")]
    #[must_use]
    pub fn pack_voltage(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `bat_voltage` field.
    ///
    #[doc(alias = "BAT_VOLTAGE")]
    #[must_use]
    pub fn bat_voltage(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `cell_current_1` field.
    ///
    #[doc(alias = "CELL_CURRENT_1")]
    #[must_use]
    pub fn cell_current_1(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `cell_current_2` field.
    ///
    #[doc(alias = "CELL_CURRENT_2")]
    #[must_use]
    pub fn cell_current_2(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `cell_current_3` field.
    ///
    #[doc(alias = "CELL_CURRENT_3")]
    #[must_use]
    pub fn cell_current_3(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `cell_current_4` field.
    ///
    #[doc(alias = "CELL_CURRENT_4")]
    #[must_use]
    pub fn cell_current_4(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `refresh_ctr` field.
    ///
    #[doc(alias = "REFRESH_CTR")]
    pub fn set_refresh_ctr(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:8` - Set the `status` field.
    ///
    #[doc(alias = "STATUS")]
    pub fn set_status(&mut self, value: u8) {
        let start = 8;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `current` field.
    ///
    #[doc(alias = "CURRENT")]
    pub fn set_current(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    pub fn set_cell_voltage_1(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    pub fn set_cell_voltage_2(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    pub fn set_cell_voltage_3(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    pub fn set_cell_voltage_4(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `pack_voltage` field.
    ///
    #[doc(alias = "PACK_VOLTAGE")]
    pub fn set_pack_voltage(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `bat_voltage` field.
    ///
    #[doc(alias = "BAT_VOLTAGE")]
    pub fn set_bat_voltage(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `cell_current_1` field.
    ///
    #[doc(alias = "CELL_CURRENT_1")]
    pub fn set_cell_current_1(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `cell_current_2` field.
    ///
    #[doc(alias = "CELL_CURRENT_2")]
    pub fn set_cell_current_2(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `cell_current_3` field.
    ///
    #[doc(alias = "CELL_CURRENT_3")]
    pub fn set_cell_current_3(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `cell_current_4` field.
    ///
    #[doc(alias = "CELL_CURRENT_4")]
    pub fn set_cell_current_4(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacOutputCcadcCal {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 24]> for MacOutputCcadcCal {
    fn from(bits: [u8; 24]) -> Self {
        Self { bits }
    }
}
impl From<MacOutputCcadcCal> for [u8; 24] {
    fn from(val: MacOutputCcadcCal) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacOutputCcadcCal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacOutputCcadcCal");
        d.field("refresh_ctr", &self.refresh_ctr());
        d.field("status", &self.status());
        d.field("current", &self.current());
        d.field("cell_voltage_1", &self.cell_voltage_1());
        d.field("cell_voltage_2", &self.cell_voltage_2());
        d.field("cell_voltage_3", &self.cell_voltage_3());
        d.field("cell_voltage_4", &self.cell_voltage_4());
        d.field("pack_voltage", &self.pack_voltage());
        d.field("bat_voltage", &self.bat_voltage());
        d.field("cell_current_1", &self.cell_current_1());
        d.field("cell_current_2", &self.cell_current_2());
        d.field("cell_current_3", &self.cell_current_3());
        d.field("cell_current_4", &self.cell_current_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacOutputCcadcCal {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacOutputCcadcCal {{ ");
        defmt::write!(f, "refresh_ctr: {=u8}, ", &self.refresh_ctr());
        defmt::write!(f, "status: {=u8}, ", &self.status());
        defmt::write!(f, "current: {=u16}, ", &self.current());
        defmt::write!(f, "cell_voltage_1: {=u16}, ", &self.cell_voltage_1());
        defmt::write!(f, "cell_voltage_2: {=u16}, ", &self.cell_voltage_2());
        defmt::write!(f, "cell_voltage_3: {=u16}, ", &self.cell_voltage_3());
        defmt::write!(f, "cell_voltage_4: {=u16}, ", &self.cell_voltage_4());
        defmt::write!(f, "pack_voltage: {=u16}, ", &self.pack_voltage());
        defmt::write!(f, "bat_voltage: {=u16}, ", &self.bat_voltage());
        defmt::write!(f, "cell_current_1: {=u16}, ", &self.cell_current_1());
        defmt::write!(f, "cell_current_2: {=u16}, ", &self.cell_current_2());
        defmt::write!(f, "cell_current_3: {=u16}, ", &self.cell_current_3());
        defmt::write!(f, "cell_current_4: {=u16}, ", &self.cell_current_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacOutputCcadcCal {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacOutputCcadcCal {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacOutputCcadcCal {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacOutputCcadcCal {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacOutputCcadcCal {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacOutputCcadcCal {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacOutputCcadcCal {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_FILTER_CAPACITY")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacFilterCapacity {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for MacFilterCapacity {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl MacFilterCapacity {
    /// `15:0` - Read the `filt_rem_cap` field.
    ///
    #[doc(alias = "FILT_REM_CAP")]
    #[must_use]
    pub fn filt_rem_cap(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `filt_rem_energy` field.
    ///
    #[doc(alias = "FILT_REM_ENERGY")]
    #[must_use]
    pub fn filt_rem_energy(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `filt_full_chg_cap` field.
    ///
    #[doc(alias = "FILT_FULL_CHG_CAP")]
    #[must_use]
    pub fn filt_full_chg_cap(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `filt_full_chg_energy` field.
    ///
    #[doc(alias = "FILT_FULL_CHG_ENERGY")]
    #[must_use]
    pub fn filt_full_chg_energy(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `filt_rem_cap` field.
    ///
    #[doc(alias = "FILT_REM_CAP")]
    pub fn set_filt_rem_cap(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `filt_rem_energy` field.
    ///
    #[doc(alias = "FILT_REM_ENERGY")]
    pub fn set_filt_rem_energy(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `filt_full_chg_cap` field.
    ///
    #[doc(alias = "FILT_FULL_CHG_CAP")]
    pub fn set_filt_full_chg_cap(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `filt_full_chg_energy` field.
    ///
    #[doc(alias = "FILT_FULL_CHG_ENERGY")]
    pub fn set_filt_full_chg_energy(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacFilterCapacity {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for MacFilterCapacity {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<MacFilterCapacity> for [u8; 8] {
    fn from(val: MacFilterCapacity) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacFilterCapacity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacFilterCapacity");
        d.field("filt_rem_cap", &self.filt_rem_cap());
        d.field("filt_rem_energy", &self.filt_rem_energy());
        d.field("filt_full_chg_cap", &self.filt_full_chg_cap());
        d.field("filt_full_chg_energy", &self.filt_full_chg_energy());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacFilterCapacity {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacFilterCapacity {{ ");
        defmt::write!(f, "filt_rem_cap: {=u16}, ", &self.filt_rem_cap());
        defmt::write!(f, "filt_rem_energy: {=u16}, ", &self.filt_rem_energy());
        defmt::write!(f, "filt_full_chg_cap: {=u16}, ", &self.filt_full_chg_cap());
        defmt::write!(f, "filt_full_chg_energy: {=u16}, ", &self.filt_full_chg_energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacFilterCapacity {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacFilterCapacity {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacFilterCapacity {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacFilterCapacity {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacFilterCapacity {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacFilterCapacity {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacFilterCapacity {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_STATE_OF_HEALTH")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacStateOfHealth {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacStateOfHealth {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacStateOfHealth {
    /// `15:0` - Read the `soh_fcc` field.
    ///
    #[doc(alias = "SOH_FCC")]
    #[must_use]
    pub fn soh_fcc(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `soh_energy` field.
    ///
    #[doc(alias = "SOH_ENERGY")]
    #[must_use]
    pub fn soh_energy(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `soh_fcc` field.
    ///
    #[doc(alias = "SOH_FCC")]
    pub fn set_soh_fcc(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `soh_energy` field.
    ///
    #[doc(alias = "SOH_ENERGY")]
    pub fn set_soh_energy(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacStateOfHealth {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacStateOfHealth {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacStateOfHealth> for [u8; 4] {
    fn from(val: MacStateOfHealth) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacStateOfHealth {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacStateOfHealth");
        d.field("soh_fcc", &self.soh_fcc());
        d.field("soh_energy", &self.soh_energy());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacStateOfHealth {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacStateOfHealth {{ ");
        defmt::write!(f, "soh_fcc: {=u16}, ", &self.soh_fcc());
        defmt::write!(f, "soh_energy: {=u16}, ", &self.soh_energy());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacStateOfHealth {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacStateOfHealth {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacStateOfHealth {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacStateOfHealth {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacStateOfHealth {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacStateOfHealth {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacStateOfHealth {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_CB_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacCbStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for MacCbStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl MacCbStatus {
    /// `15:0` - Read the `cb_time_0` field.
    ///
    #[doc(alias = "CB_TIME_0")]
    #[must_use]
    pub fn cb_time_0(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `cb_time_1` field.
    ///
    #[doc(alias = "CB_TIME_1")]
    #[must_use]
    pub fn cb_time_1(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cb_time_2` field.
    ///
    #[doc(alias = "CB_TIME_2")]
    #[must_use]
    pub fn cb_time_2(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cb_time_3` field.
    ///
    #[doc(alias = "CB_TIME_3")]
    #[must_use]
    pub fn cb_time_3(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `cb_time_0` field.
    ///
    #[doc(alias = "CB_TIME_0")]
    pub fn set_cb_time_0(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `cb_time_1` field.
    ///
    #[doc(alias = "CB_TIME_1")]
    pub fn set_cb_time_1(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `cb_time_2` field.
    ///
    #[doc(alias = "CB_TIME_2")]
    pub fn set_cb_time_2(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `cb_time_3` field.
    ///
    #[doc(alias = "CB_TIME_3")]
    pub fn set_cb_time_3(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacCbStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for MacCbStatus {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<MacCbStatus> for [u8; 8] {
    fn from(val: MacCbStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacCbStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacCbStatus");
        d.field("cb_time_0", &self.cb_time_0());
        d.field("cb_time_1", &self.cb_time_1());
        d.field("cb_time_2", &self.cb_time_2());
        d.field("cb_time_3", &self.cb_time_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacCbStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacCbStatus {{ ");
        defmt::write!(f, "cb_time_0: {=u16}, ", &self.cb_time_0());
        defmt::write!(f, "cb_time_1: {=u16}, ", &self.cb_time_1());
        defmt::write!(f, "cb_time_2: {=u16}, ", &self.cb_time_2());
        defmt::write!(f, "cb_time_3: {=u16}, ", &self.cb_time_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacCbStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacCbStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacCbStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacCbStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacCbStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacCbStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacCbStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_GAUGE_STATUS_3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacGaugeStatus3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 24],
}
unsafe impl ::device_driver::Fieldset for MacGaugeStatus3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 24] };
}
impl MacGaugeStatus3 {
    /// `15:0` - Read the `qmax_0` field.
    ///
    #[doc(alias = "QMAX_0")]
    #[must_use]
    pub fn qmax_0(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `qmax_1` field.
    ///
    #[doc(alias = "QMAX_1")]
    #[must_use]
    pub fn qmax_1(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `qmax_2` field.
    ///
    #[doc(alias = "QMAX_2")]
    #[must_use]
    pub fn qmax_2(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `qmax_3` field.
    ///
    #[doc(alias = "QMAX_3")]
    #[must_use]
    pub fn qmax_3(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `qmax_dod_0_0` field.
    ///
    #[doc(alias = "QMAX_DOD0_0")]
    #[must_use]
    pub fn qmax_dod_0_0(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `qmax_dod_0_1` field.
    ///
    #[doc(alias = "QMAX_DOD0_1")]
    #[must_use]
    pub fn qmax_dod_0_1(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `qmax_dod_0_2` field.
    ///
    #[doc(alias = "QMAX_DOD0_2")]
    #[must_use]
    pub fn qmax_dod_0_2(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `qmax_dod_0_3` field.
    ///
    #[doc(alias = "QMAX_DOD0_3")]
    #[must_use]
    pub fn qmax_dod_0_3(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `qmax_passed_q` field.
    ///
    #[doc(alias = "QMAX_PASSED_Q")]
    #[must_use]
    pub fn qmax_passed_q(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `qmax_time` field.
    ///
    #[doc(alias = "QMAX_TIME")]
    #[must_use]
    pub fn qmax_time(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `temp_k_factor` field.
    ///
    #[doc(alias = "TEMP_K_FACTOR")]
    #[must_use]
    pub fn temp_k_factor(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `temp_a_factor` field.
    ///
    #[doc(alias = "TEMP_A_FACTOR")]
    #[must_use]
    pub fn temp_a_factor(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `qmax_0` field.
    ///
    #[doc(alias = "QMAX_0")]
    pub fn set_qmax_0(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `qmax_1` field.
    ///
    #[doc(alias = "QMAX_1")]
    pub fn set_qmax_1(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `qmax_2` field.
    ///
    #[doc(alias = "QMAX_2")]
    pub fn set_qmax_2(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `qmax_3` field.
    ///
    #[doc(alias = "QMAX_3")]
    pub fn set_qmax_3(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `qmax_dod_0_0` field.
    ///
    #[doc(alias = "QMAX_DOD0_0")]
    pub fn set_qmax_dod_0_0(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `qmax_dod_0_1` field.
    ///
    #[doc(alias = "QMAX_DOD0_1")]
    pub fn set_qmax_dod_0_1(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `qmax_dod_0_2` field.
    ///
    #[doc(alias = "QMAX_DOD0_2")]
    pub fn set_qmax_dod_0_2(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `qmax_dod_0_3` field.
    ///
    #[doc(alias = "QMAX_DOD0_3")]
    pub fn set_qmax_dod_0_3(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `qmax_passed_q` field.
    ///
    #[doc(alias = "QMAX_PASSED_Q")]
    pub fn set_qmax_passed_q(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `qmax_time` field.
    ///
    #[doc(alias = "QMAX_TIME")]
    pub fn set_qmax_time(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `temp_k_factor` field.
    ///
    #[doc(alias = "TEMP_K_FACTOR")]
    pub fn set_temp_k_factor(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `temp_a_factor` field.
    ///
    #[doc(alias = "TEMP_A_FACTOR")]
    pub fn set_temp_a_factor(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacGaugeStatus3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 24]> for MacGaugeStatus3 {
    fn from(bits: [u8; 24]) -> Self {
        Self { bits }
    }
}
impl From<MacGaugeStatus3> for [u8; 24] {
    fn from(val: MacGaugeStatus3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacGaugeStatus3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacGaugeStatus3");
        d.field("qmax_0", &self.qmax_0());
        d.field("qmax_1", &self.qmax_1());
        d.field("qmax_2", &self.qmax_2());
        d.field("qmax_3", &self.qmax_3());
        d.field("qmax_dod_0_0", &self.qmax_dod_0_0());
        d.field("qmax_dod_0_1", &self.qmax_dod_0_1());
        d.field("qmax_dod_0_2", &self.qmax_dod_0_2());
        d.field("qmax_dod_0_3", &self.qmax_dod_0_3());
        d.field("qmax_passed_q", &self.qmax_passed_q());
        d.field("qmax_time", &self.qmax_time());
        d.field("temp_k_factor", &self.temp_k_factor());
        d.field("temp_a_factor", &self.temp_a_factor());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacGaugeStatus3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacGaugeStatus3 {{ ");
        defmt::write!(f, "qmax_0: {=u16}, ", &self.qmax_0());
        defmt::write!(f, "qmax_1: {=u16}, ", &self.qmax_1());
        defmt::write!(f, "qmax_2: {=u16}, ", &self.qmax_2());
        defmt::write!(f, "qmax_3: {=u16}, ", &self.qmax_3());
        defmt::write!(f, "qmax_dod_0_0: {=u16}, ", &self.qmax_dod_0_0());
        defmt::write!(f, "qmax_dod_0_1: {=u16}, ", &self.qmax_dod_0_1());
        defmt::write!(f, "qmax_dod_0_2: {=u16}, ", &self.qmax_dod_0_2());
        defmt::write!(f, "qmax_dod_0_3: {=u16}, ", &self.qmax_dod_0_3());
        defmt::write!(f, "qmax_passed_q: {=u16}, ", &self.qmax_passed_q());
        defmt::write!(f, "qmax_time: {=u16}, ", &self.qmax_time());
        defmt::write!(f, "temp_k_factor: {=u16}, ", &self.temp_k_factor());
        defmt::write!(f, "temp_a_factor: {=u16}, ", &self.temp_a_factor());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacGaugeStatus3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacGaugeStatus3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacGaugeStatus3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacGaugeStatus3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacGaugeStatus3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacGaugeStatus3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacGaugeStatus3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_GAUGE_STATUS_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacGaugeStatus2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for MacGaugeStatus2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl MacGaugeStatus2 {
    /// `7:0` - Read the `pack_grid` field.
    ///
    #[doc(alias = "PACK_GRID")]
    #[must_use]
    pub fn pack_grid(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `9:8` - Read the `q_max_status` field.
    ///
    #[doc(alias = "Q_MAX_STATUS")]
    #[must_use]
    pub fn q_max_status(&self) -> MacqMaxStatus {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `iten` field.
    ///
    #[doc(alias = "ITEN")]
    #[must_use]
    pub fn iten(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `qmax_field_updated` field.
    ///
    #[doc(alias = "QMAX_FIELD_UPDATED")]
    #[must_use]
    pub fn qmax_field_updated(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `23:16` - Read the `cell_grid_0` field.
    ///
    #[doc(alias = "CELL_GRID_0")]
    #[must_use]
    pub fn cell_grid_0(&self) -> u8 {
        let start = 16;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:24` - Read the `cell_grid_1` field.
    ///
    #[doc(alias = "CELL_GRID_1")]
    #[must_use]
    pub fn cell_grid_1(&self) -> u8 {
        let start = 24;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `39:32` - Read the `cell_grid_2` field.
    ///
    #[doc(alias = "CELL_GRID_2")]
    #[must_use]
    pub fn cell_grid_2(&self) -> u8 {
        let start = 32;
        let end = 39;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:40` - Read the `cell_grid_3` field.
    ///
    #[doc(alias = "CELL_GRID_3")]
    #[must_use]
    pub fn cell_grid_3(&self) -> u8 {
        let start = 40;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:48` - Read the `state_time` field.
    ///
    #[doc(alias = "STATE_TIME")]
    #[must_use]
    pub fn state_time(&self) -> u32 {
        let start = 48;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u32, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `dod_0_0` field.
    ///
    #[doc(alias = "DOD0_0")]
    #[must_use]
    pub fn dod_0_0(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `dod_0_1` field.
    ///
    #[doc(alias = "DOD0_1")]
    #[must_use]
    pub fn dod_0_1(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `dod_0_2` field.
    ///
    #[doc(alias = "DOD0_2")]
    #[must_use]
    pub fn dod_0_2(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `dod_0_3` field.
    ///
    #[doc(alias = "DOD0_3")]
    #[must_use]
    pub fn dod_0_3(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `dod_0_passed_q` field.
    ///
    #[doc(alias = "DOD0_PASSED_Q")]
    #[must_use]
    pub fn dod_0_passed_q(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `dod_0_passed_e` field.
    ///
    #[doc(alias = "DOD0_PASSED_E")]
    #[must_use]
    pub fn dod_0_passed_e(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `dod_0_time` field.
    ///
    #[doc(alias = "DOD0_TIME")]
    #[must_use]
    pub fn dod_0_time(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `dodeoc_0` field.
    ///
    #[doc(alias = "DODEOC_0")]
    #[must_use]
    pub fn dodeoc_0(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `dodeoc_1` field.
    ///
    #[doc(alias = "DODEOC_1")]
    #[must_use]
    pub fn dodeoc_1(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `dodeoc_2` field.
    ///
    #[doc(alias = "DODEOC_2")]
    #[must_use]
    pub fn dodeoc_2(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `dodeoc_3` field.
    ///
    #[doc(alias = "DODEOC_3")]
    #[must_use]
    pub fn dodeoc_3(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `pack_grid` field.
    ///
    #[doc(alias = "PACK_GRID")]
    pub fn set_pack_grid(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `9:8` - Set the `q_max_status` field.
    ///
    #[doc(alias = "Q_MAX_STATUS")]
    pub fn set_q_max_status(&mut self, value: MacqMaxStatus) {
        let start = 8;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `iten` field.
    ///
    #[doc(alias = "ITEN")]
    pub fn set_iten(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `qmax_field_updated` field.
    ///
    #[doc(alias = "QMAX_FIELD_UPDATED")]
    pub fn set_qmax_field_updated(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `23:16` - Set the `cell_grid_0` field.
    ///
    #[doc(alias = "CELL_GRID_0")]
    pub fn set_cell_grid_0(&mut self, value: u8) {
        let start = 16;
        let end = 23;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:24` - Set the `cell_grid_1` field.
    ///
    #[doc(alias = "CELL_GRID_1")]
    pub fn set_cell_grid_1(&mut self, value: u8) {
        let start = 24;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `39:32` - Set the `cell_grid_2` field.
    ///
    #[doc(alias = "CELL_GRID_2")]
    pub fn set_cell_grid_2(&mut self, value: u8) {
        let start = 32;
        let end = 39;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:40` - Set the `cell_grid_3` field.
    ///
    #[doc(alias = "CELL_GRID_3")]
    pub fn set_cell_grid_3(&mut self, value: u8) {
        let start = 40;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:48` - Set the `state_time` field.
    ///
    #[doc(alias = "STATE_TIME")]
    pub fn set_state_time(&mut self, value: u32) {
        let start = 48;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u32, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `dod_0_0` field.
    ///
    #[doc(alias = "DOD0_0")]
    pub fn set_dod_0_0(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `dod_0_1` field.
    ///
    #[doc(alias = "DOD0_1")]
    pub fn set_dod_0_1(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `dod_0_2` field.
    ///
    #[doc(alias = "DOD0_2")]
    pub fn set_dod_0_2(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `dod_0_3` field.
    ///
    #[doc(alias = "DOD0_3")]
    pub fn set_dod_0_3(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `dod_0_passed_q` field.
    ///
    #[doc(alias = "DOD0_PASSED_Q")]
    pub fn set_dod_0_passed_q(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `dod_0_passed_e` field.
    ///
    #[doc(alias = "DOD0_PASSED_E")]
    pub fn set_dod_0_passed_e(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `dod_0_time` field.
    ///
    #[doc(alias = "DOD0_TIME")]
    pub fn set_dod_0_time(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `207:192` - Set the `dodeoc_0` field.
    ///
    #[doc(alias = "DODEOC_0")]
    pub fn set_dodeoc_0(&mut self, value: u16) {
        let start = 192;
        let end = 207;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `223:208` - Set the `dodeoc_1` field.
    ///
    #[doc(alias = "DODEOC_1")]
    pub fn set_dodeoc_1(&mut self, value: u16) {
        let start = 208;
        let end = 223;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `239:224` - Set the `dodeoc_2` field.
    ///
    #[doc(alias = "DODEOC_2")]
    pub fn set_dodeoc_2(&mut self, value: u16) {
        let start = 224;
        let end = 239;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `255:240` - Set the `dodeoc_3` field.
    ///
    #[doc(alias = "DODEOC_3")]
    pub fn set_dodeoc_3(&mut self, value: u16) {
        let start = 240;
        let end = 255;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacGaugeStatus2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for MacGaugeStatus2 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<MacGaugeStatus2> for [u8; 32] {
    fn from(val: MacGaugeStatus2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacGaugeStatus2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacGaugeStatus2");
        d.field("pack_grid", &self.pack_grid());
        d.field("q_max_status", &self.q_max_status());
        d.field("iten", &self.iten());
        d.field("qmax_field_updated", &self.qmax_field_updated());
        d.field("cell_grid_0", &self.cell_grid_0());
        d.field("cell_grid_1", &self.cell_grid_1());
        d.field("cell_grid_2", &self.cell_grid_2());
        d.field("cell_grid_3", &self.cell_grid_3());
        d.field("state_time", &self.state_time());
        d.field("dod_0_0", &self.dod_0_0());
        d.field("dod_0_1", &self.dod_0_1());
        d.field("dod_0_2", &self.dod_0_2());
        d.field("dod_0_3", &self.dod_0_3());
        d.field("dod_0_passed_q", &self.dod_0_passed_q());
        d.field("dod_0_passed_e", &self.dod_0_passed_e());
        d.field("dod_0_time", &self.dod_0_time());
        d.field("dodeoc_0", &self.dodeoc_0());
        d.field("dodeoc_1", &self.dodeoc_1());
        d.field("dodeoc_2", &self.dodeoc_2());
        d.field("dodeoc_3", &self.dodeoc_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacGaugeStatus2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacGaugeStatus2 {{ ");
        defmt::write!(f, "pack_grid: {=u8}, ", &self.pack_grid());
        defmt::write!(f, "q_max_status: {}, ", &self.q_max_status());
        defmt::write!(f, "iten: {=bool}, ", &self.iten());
        defmt::write!(f, "qmax_field_updated: {=bool}, ", &self.qmax_field_updated());
        defmt::write!(f, "cell_grid_0: {=u8}, ", &self.cell_grid_0());
        defmt::write!(f, "cell_grid_1: {=u8}, ", &self.cell_grid_1());
        defmt::write!(f, "cell_grid_2: {=u8}, ", &self.cell_grid_2());
        defmt::write!(f, "cell_grid_3: {=u8}, ", &self.cell_grid_3());
        defmt::write!(f, "state_time: {=u32}, ", &self.state_time());
        defmt::write!(f, "dod_0_0: {=u16}, ", &self.dod_0_0());
        defmt::write!(f, "dod_0_1: {=u16}, ", &self.dod_0_1());
        defmt::write!(f, "dod_0_2: {=u16}, ", &self.dod_0_2());
        defmt::write!(f, "dod_0_3: {=u16}, ", &self.dod_0_3());
        defmt::write!(f, "dod_0_passed_q: {=u16}, ", &self.dod_0_passed_q());
        defmt::write!(f, "dod_0_passed_e: {=u16}, ", &self.dod_0_passed_e());
        defmt::write!(f, "dod_0_time: {=u16}, ", &self.dod_0_time());
        defmt::write!(f, "dodeoc_0: {=u16}, ", &self.dodeoc_0());
        defmt::write!(f, "dodeoc_1: {=u16}, ", &self.dodeoc_1());
        defmt::write!(f, "dodeoc_2: {=u16}, ", &self.dodeoc_2());
        defmt::write!(f, "dodeoc_3: {=u16}, ", &self.dodeoc_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacGaugeStatus2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacGaugeStatus2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacGaugeStatus2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacGaugeStatus2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacGaugeStatus2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacGaugeStatus2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacGaugeStatus2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_GAUGE_STATUS_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacGaugeStatus1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for MacGaugeStatus1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl MacGaugeStatus1 {
    /// `15:0` - Read the `true_rem_q` field.
    ///
    #[doc(alias = "TRUE_REM_Q")]
    #[must_use]
    pub fn true_rem_q(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `true_rem_e` field.
    ///
    #[doc(alias = "TRUE_REM_E")]
    #[must_use]
    pub fn true_rem_e(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `initial_q` field.
    ///
    #[doc(alias = "INITIAL_Q")]
    #[must_use]
    pub fn initial_q(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `initial_e` field.
    ///
    #[doc(alias = "INITIAL_E")]
    #[must_use]
    pub fn initial_e(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `true_fcc_q` field.
    ///
    #[doc(alias = "TRUE_FCC_Q")]
    #[must_use]
    pub fn true_fcc_q(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `true_fcc_e` field.
    ///
    #[doc(alias = "TRUE_FCC_E")]
    #[must_use]
    pub fn true_fcc_e(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `t_sim` field.
    ///
    #[doc(alias = "T_SIM")]
    #[must_use]
    pub fn t_sim(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `t_ambient` field.
    ///
    #[doc(alias = "T_AMBIENT")]
    #[must_use]
    pub fn t_ambient(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `ra_scale_0` field.
    ///
    #[doc(alias = "RA_SCALE_0")]
    #[must_use]
    pub fn ra_scale_0(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `ra_scale_1` field.
    ///
    #[doc(alias = "RA_SCALE_1")]
    #[must_use]
    pub fn ra_scale_1(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `ra_scale_2` field.
    ///
    #[doc(alias = "RA_SCALE_2")]
    #[must_use]
    pub fn ra_scale_2(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `ra_scale_3` field.
    ///
    #[doc(alias = "RA_SCALE_3")]
    #[must_use]
    pub fn ra_scale_3(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `comp_res_0` field.
    ///
    #[doc(alias = "COMP_RES_0")]
    #[must_use]
    pub fn comp_res_0(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `comp_res_1` field.
    ///
    #[doc(alias = "COMP_RES_1")]
    #[must_use]
    pub fn comp_res_1(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `comp_res_2` field.
    ///
    #[doc(alias = "COMP_RES_2")]
    #[must_use]
    pub fn comp_res_2(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `comp_res_3` field.
    ///
    #[doc(alias = "COMP_RES_3")]
    #[must_use]
    pub fn comp_res_3(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `true_rem_q` field.
    ///
    #[doc(alias = "TRUE_REM_Q")]
    pub fn set_true_rem_q(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `true_rem_e` field.
    ///
    #[doc(alias = "TRUE_REM_E")]
    pub fn set_true_rem_e(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `initial_q` field.
    ///
    #[doc(alias = "INITIAL_Q")]
    pub fn set_initial_q(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `initial_e` field.
    ///
    #[doc(alias = "INITIAL_E")]
    pub fn set_initial_e(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `true_fcc_q` field.
    ///
    #[doc(alias = "TRUE_FCC_Q")]
    pub fn set_true_fcc_q(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `true_fcc_e` field.
    ///
    #[doc(alias = "TRUE_FCC_E")]
    pub fn set_true_fcc_e(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `t_sim` field.
    ///
    #[doc(alias = "T_SIM")]
    pub fn set_t_sim(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `t_ambient` field.
    ///
    #[doc(alias = "T_AMBIENT")]
    pub fn set_t_ambient(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `ra_scale_0` field.
    ///
    #[doc(alias = "RA_SCALE_0")]
    pub fn set_ra_scale_0(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `ra_scale_1` field.
    ///
    #[doc(alias = "RA_SCALE_1")]
    pub fn set_ra_scale_1(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `ra_scale_2` field.
    ///
    #[doc(alias = "RA_SCALE_2")]
    pub fn set_ra_scale_2(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `ra_scale_3` field.
    ///
    #[doc(alias = "RA_SCALE_3")]
    pub fn set_ra_scale_3(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `207:192` - Set the `comp_res_0` field.
    ///
    #[doc(alias = "COMP_RES_0")]
    pub fn set_comp_res_0(&mut self, value: u16) {
        let start = 192;
        let end = 207;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `223:208` - Set the `comp_res_1` field.
    ///
    #[doc(alias = "COMP_RES_1")]
    pub fn set_comp_res_1(&mut self, value: u16) {
        let start = 208;
        let end = 223;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `239:224` - Set the `comp_res_2` field.
    ///
    #[doc(alias = "COMP_RES_2")]
    pub fn set_comp_res_2(&mut self, value: u16) {
        let start = 224;
        let end = 239;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `255:240` - Set the `comp_res_3` field.
    ///
    #[doc(alias = "COMP_RES_3")]
    pub fn set_comp_res_3(&mut self, value: u16) {
        let start = 240;
        let end = 255;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacGaugeStatus1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for MacGaugeStatus1 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<MacGaugeStatus1> for [u8; 32] {
    fn from(val: MacGaugeStatus1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacGaugeStatus1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacGaugeStatus1");
        d.field("true_rem_q", &self.true_rem_q());
        d.field("true_rem_e", &self.true_rem_e());
        d.field("initial_q", &self.initial_q());
        d.field("initial_e", &self.initial_e());
        d.field("true_fcc_q", &self.true_fcc_q());
        d.field("true_fcc_e", &self.true_fcc_e());
        d.field("t_sim", &self.t_sim());
        d.field("t_ambient", &self.t_ambient());
        d.field("ra_scale_0", &self.ra_scale_0());
        d.field("ra_scale_1", &self.ra_scale_1());
        d.field("ra_scale_2", &self.ra_scale_2());
        d.field("ra_scale_3", &self.ra_scale_3());
        d.field("comp_res_0", &self.comp_res_0());
        d.field("comp_res_1", &self.comp_res_1());
        d.field("comp_res_2", &self.comp_res_2());
        d.field("comp_res_3", &self.comp_res_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacGaugeStatus1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacGaugeStatus1 {{ ");
        defmt::write!(f, "true_rem_q: {=u16}, ", &self.true_rem_q());
        defmt::write!(f, "true_rem_e: {=u16}, ", &self.true_rem_e());
        defmt::write!(f, "initial_q: {=u16}, ", &self.initial_q());
        defmt::write!(f, "initial_e: {=u16}, ", &self.initial_e());
        defmt::write!(f, "true_fcc_q: {=u16}, ", &self.true_fcc_q());
        defmt::write!(f, "true_fcc_e: {=u16}, ", &self.true_fcc_e());
        defmt::write!(f, "t_sim: {=u16}, ", &self.t_sim());
        defmt::write!(f, "t_ambient: {=u16}, ", &self.t_ambient());
        defmt::write!(f, "ra_scale_0: {=u16}, ", &self.ra_scale_0());
        defmt::write!(f, "ra_scale_1: {=u16}, ", &self.ra_scale_1());
        defmt::write!(f, "ra_scale_2: {=u16}, ", &self.ra_scale_2());
        defmt::write!(f, "ra_scale_3: {=u16}, ", &self.ra_scale_3());
        defmt::write!(f, "comp_res_0: {=u16}, ", &self.comp_res_0());
        defmt::write!(f, "comp_res_1: {=u16}, ", &self.comp_res_1());
        defmt::write!(f, "comp_res_2: {=u16}, ", &self.comp_res_2());
        defmt::write!(f, "comp_res_3: {=u16}, ", &self.comp_res_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacGaugeStatus1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacGaugeStatus1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacGaugeStatus1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacGaugeStatus1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacGaugeStatus1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacGaugeStatus1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacGaugeStatus1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_DA_STATUS_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacDaStatus2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 14],
}
unsafe impl ::device_driver::Fieldset for MacDaStatus2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 14] };
}
impl MacDaStatus2 {
    /// `15:0` - Read the `int_temp` field.
    ///
    #[doc(alias = "INT_TEMP")]
    #[must_use]
    pub fn int_temp(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `ts_1_temp` field.
    ///
    #[doc(alias = "TS1_TEMP")]
    #[must_use]
    pub fn ts_1_temp(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `ts_2_temp` field.
    ///
    #[doc(alias = "TS2_TEMP")]
    #[must_use]
    pub fn ts_2_temp(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `ts_3_temp` field.
    ///
    #[doc(alias = "TS3_TEMP")]
    #[must_use]
    pub fn ts_3_temp(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `ts_4_temp` field.
    ///
    #[doc(alias = "TS4_TEMP")]
    #[must_use]
    pub fn ts_4_temp(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `cell_temp` field.
    ///
    #[doc(alias = "CELL_TEMP")]
    #[must_use]
    pub fn cell_temp(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `fet_temp` field.
    ///
    #[doc(alias = "FET_TEMP")]
    #[must_use]
    pub fn fet_temp(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `int_temp` field.
    ///
    #[doc(alias = "INT_TEMP")]
    pub fn set_int_temp(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `ts_1_temp` field.
    ///
    #[doc(alias = "TS1_TEMP")]
    pub fn set_ts_1_temp(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `ts_2_temp` field.
    ///
    #[doc(alias = "TS2_TEMP")]
    pub fn set_ts_2_temp(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `ts_3_temp` field.
    ///
    #[doc(alias = "TS3_TEMP")]
    pub fn set_ts_3_temp(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `ts_4_temp` field.
    ///
    #[doc(alias = "TS4_TEMP")]
    pub fn set_ts_4_temp(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `cell_temp` field.
    ///
    #[doc(alias = "CELL_TEMP")]
    pub fn set_cell_temp(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `fet_temp` field.
    ///
    #[doc(alias = "FET_TEMP")]
    pub fn set_fet_temp(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacDaStatus2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 14]> for MacDaStatus2 {
    fn from(bits: [u8; 14]) -> Self {
        Self { bits }
    }
}
impl From<MacDaStatus2> for [u8; 14] {
    fn from(val: MacDaStatus2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacDaStatus2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacDaStatus2");
        d.field("int_temp", &self.int_temp());
        d.field("ts_1_temp", &self.ts_1_temp());
        d.field("ts_2_temp", &self.ts_2_temp());
        d.field("ts_3_temp", &self.ts_3_temp());
        d.field("ts_4_temp", &self.ts_4_temp());
        d.field("cell_temp", &self.cell_temp());
        d.field("fet_temp", &self.fet_temp());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacDaStatus2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacDaStatus2 {{ ");
        defmt::write!(f, "int_temp: {=u16}, ", &self.int_temp());
        defmt::write!(f, "ts_1_temp: {=u16}, ", &self.ts_1_temp());
        defmt::write!(f, "ts_2_temp: {=u16}, ", &self.ts_2_temp());
        defmt::write!(f, "ts_3_temp: {=u16}, ", &self.ts_3_temp());
        defmt::write!(f, "ts_4_temp: {=u16}, ", &self.ts_4_temp());
        defmt::write!(f, "cell_temp: {=u16}, ", &self.cell_temp());
        defmt::write!(f, "fet_temp: {=u16}, ", &self.fet_temp());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacDaStatus2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacDaStatus2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacDaStatus2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacDaStatus2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacDaStatus2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacDaStatus2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacDaStatus2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_DA_STATUS_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacDaStatus1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for MacDaStatus1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl MacDaStatus1 {
    /// `15:0` - Read the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    #[must_use]
    pub fn cell_voltage_1(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    #[must_use]
    pub fn cell_voltage_2(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    #[must_use]
    pub fn cell_voltage_3(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    #[must_use]
    pub fn cell_voltage_4(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `bat_voltage` field.
    ///
    #[doc(alias = "BAT_VOLTAGE")]
    #[must_use]
    pub fn bat_voltage(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `pack_voltage` field.
    ///
    #[doc(alias = "PACK_VOLTAGE")]
    #[must_use]
    pub fn pack_voltage(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `cell_current_1` field.
    ///
    #[doc(alias = "CELL_CURRENT_1")]
    #[must_use]
    pub fn cell_current_1(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `cell_current_2` field.
    ///
    #[doc(alias = "CELL_CURRENT_2")]
    #[must_use]
    pub fn cell_current_2(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `cell_current_3` field.
    ///
    #[doc(alias = "CELL_CURRENT_3")]
    #[must_use]
    pub fn cell_current_3(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `cell_current_4` field.
    ///
    #[doc(alias = "CELL_CURRENT_4")]
    #[must_use]
    pub fn cell_current_4(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `cell_pwr_1` field.
    ///
    #[doc(alias = "CELL_PWR_1")]
    #[must_use]
    pub fn cell_pwr_1(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `cell_pwr_2` field.
    ///
    #[doc(alias = "CELL_PWR_2")]
    #[must_use]
    pub fn cell_pwr_2(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `cell_pwr_3` field.
    ///
    #[doc(alias = "CELL_PWR_3")]
    #[must_use]
    pub fn cell_pwr_3(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `cell_pwr_4` field.
    ///
    #[doc(alias = "CELL_PWR_4")]
    #[must_use]
    pub fn cell_pwr_4(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `total_pwr` field.
    ///
    #[doc(alias = "TOTAL_PWR")]
    #[must_use]
    pub fn total_pwr(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `avg_pwr` field.
    ///
    #[doc(alias = "AVG_PWR")]
    #[must_use]
    pub fn avg_pwr(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `cell_voltage_1` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_1")]
    pub fn set_cell_voltage_1(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `cell_voltage_2` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_2")]
    pub fn set_cell_voltage_2(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `cell_voltage_3` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_3")]
    pub fn set_cell_voltage_3(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `cell_voltage_4` field.
    ///
    #[doc(alias = "CELL_VOLTAGE_4")]
    pub fn set_cell_voltage_4(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `bat_voltage` field.
    ///
    #[doc(alias = "BAT_VOLTAGE")]
    pub fn set_bat_voltage(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `pack_voltage` field.
    ///
    #[doc(alias = "PACK_VOLTAGE")]
    pub fn set_pack_voltage(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `cell_current_1` field.
    ///
    #[doc(alias = "CELL_CURRENT_1")]
    pub fn set_cell_current_1(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `cell_current_2` field.
    ///
    #[doc(alias = "CELL_CURRENT_2")]
    pub fn set_cell_current_2(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `cell_current_3` field.
    ///
    #[doc(alias = "CELL_CURRENT_3")]
    pub fn set_cell_current_3(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `cell_current_4` field.
    ///
    #[doc(alias = "CELL_CURRENT_4")]
    pub fn set_cell_current_4(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `cell_pwr_1` field.
    ///
    #[doc(alias = "CELL_PWR_1")]
    pub fn set_cell_pwr_1(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `cell_pwr_2` field.
    ///
    #[doc(alias = "CELL_PWR_2")]
    pub fn set_cell_pwr_2(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `207:192` - Set the `cell_pwr_3` field.
    ///
    #[doc(alias = "CELL_PWR_3")]
    pub fn set_cell_pwr_3(&mut self, value: u16) {
        let start = 192;
        let end = 207;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `223:208` - Set the `cell_pwr_4` field.
    ///
    #[doc(alias = "CELL_PWR_4")]
    pub fn set_cell_pwr_4(&mut self, value: u16) {
        let start = 208;
        let end = 223;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `239:224` - Set the `total_pwr` field.
    ///
    #[doc(alias = "TOTAL_PWR")]
    pub fn set_total_pwr(&mut self, value: u16) {
        let start = 224;
        let end = 239;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `255:240` - Set the `avg_pwr` field.
    ///
    #[doc(alias = "AVG_PWR")]
    pub fn set_avg_pwr(&mut self, value: u16) {
        let start = 240;
        let end = 255;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacDaStatus1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for MacDaStatus1 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<MacDaStatus1> for [u8; 32] {
    fn from(val: MacDaStatus1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacDaStatus1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacDaStatus1");
        d.field("cell_voltage_1", &self.cell_voltage_1());
        d.field("cell_voltage_2", &self.cell_voltage_2());
        d.field("cell_voltage_3", &self.cell_voltage_3());
        d.field("cell_voltage_4", &self.cell_voltage_4());
        d.field("bat_voltage", &self.bat_voltage());
        d.field("pack_voltage", &self.pack_voltage());
        d.field("cell_current_1", &self.cell_current_1());
        d.field("cell_current_2", &self.cell_current_2());
        d.field("cell_current_3", &self.cell_current_3());
        d.field("cell_current_4", &self.cell_current_4());
        d.field("cell_pwr_1", &self.cell_pwr_1());
        d.field("cell_pwr_2", &self.cell_pwr_2());
        d.field("cell_pwr_3", &self.cell_pwr_3());
        d.field("cell_pwr_4", &self.cell_pwr_4());
        d.field("total_pwr", &self.total_pwr());
        d.field("avg_pwr", &self.avg_pwr());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacDaStatus1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacDaStatus1 {{ ");
        defmt::write!(f, "cell_voltage_1: {=u16}, ", &self.cell_voltage_1());
        defmt::write!(f, "cell_voltage_2: {=u16}, ", &self.cell_voltage_2());
        defmt::write!(f, "cell_voltage_3: {=u16}, ", &self.cell_voltage_3());
        defmt::write!(f, "cell_voltage_4: {=u16}, ", &self.cell_voltage_4());
        defmt::write!(f, "bat_voltage: {=u16}, ", &self.bat_voltage());
        defmt::write!(f, "pack_voltage: {=u16}, ", &self.pack_voltage());
        defmt::write!(f, "cell_current_1: {=u16}, ", &self.cell_current_1());
        defmt::write!(f, "cell_current_2: {=u16}, ", &self.cell_current_2());
        defmt::write!(f, "cell_current_3: {=u16}, ", &self.cell_current_3());
        defmt::write!(f, "cell_current_4: {=u16}, ", &self.cell_current_4());
        defmt::write!(f, "cell_pwr_1: {=u16}, ", &self.cell_pwr_1());
        defmt::write!(f, "cell_pwr_2: {=u16}, ", &self.cell_pwr_2());
        defmt::write!(f, "cell_pwr_3: {=u16}, ", &self.cell_pwr_3());
        defmt::write!(f, "cell_pwr_4: {=u16}, ", &self.cell_pwr_4());
        defmt::write!(f, "total_pwr: {=u16}, ", &self.total_pwr());
        defmt::write!(f, "avg_pwr: {=u16}, ", &self.avg_pwr());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacDaStatus1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacDaStatus1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacDaStatus1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacDaStatus1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacDaStatus1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacDaStatus1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacDaStatus1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_MANUFACTURE_INFO")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacManufactureInfo {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for MacManufactureInfo {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl MacManufactureInfo {
    /// `63:0` - Read the `manufacture_info_0` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_0")]
    #[must_use]
    pub fn manufacture_info_0(&self) -> u64 {
        let start = 0;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u64, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:64` - Read the `manufacture_info_1` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_1")]
    #[must_use]
    pub fn manufacture_info_1(&self) -> u64 {
        let start = 64;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u64, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:128` - Read the `manufacture_info_2` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_2")]
    #[must_use]
    pub fn manufacture_info_2(&self) -> u64 {
        let start = 128;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u64, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:192` - Read the `manufacture_info_3` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_3")]
    #[must_use]
    pub fn manufacture_info_3(&self) -> u64 {
        let start = 192;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u64, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:0` - Set the `manufacture_info_0` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_0")]
    pub fn set_manufacture_info_0(&mut self, value: u64) {
        let start = 0;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u64, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:64` - Set the `manufacture_info_1` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_1")]
    pub fn set_manufacture_info_1(&mut self, value: u64) {
        let start = 64;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u64, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:128` - Set the `manufacture_info_2` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_2")]
    pub fn set_manufacture_info_2(&mut self, value: u64) {
        let start = 128;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u64, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `255:192` - Set the `manufacture_info_3` field.
    ///
    #[doc(alias = "MANUFACTURE_INFO_3")]
    pub fn set_manufacture_info_3(&mut self, value: u64) {
        let start = 192;
        let end = 255;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u64, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacManufactureInfo {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for MacManufactureInfo {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<MacManufactureInfo> for [u8; 32] {
    fn from(val: MacManufactureInfo) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacManufactureInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacManufactureInfo");
        d.field("manufacture_info_0", &self.manufacture_info_0());
        d.field("manufacture_info_1", &self.manufacture_info_1());
        d.field("manufacture_info_2", &self.manufacture_info_2());
        d.field("manufacture_info_3", &self.manufacture_info_3());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacManufactureInfo {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacManufactureInfo {{ ");
        defmt::write!(f, "manufacture_info_0: {=u64}, ", &self.manufacture_info_0());
        defmt::write!(f, "manufacture_info_1: {=u64}, ", &self.manufacture_info_1());
        defmt::write!(f, "manufacture_info_2: {=u64}, ", &self.manufacture_info_2());
        defmt::write!(f, "manufacture_info_3: {=u64}, ", &self.manufacture_info_3());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacManufactureInfo {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacManufactureInfo {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacManufactureInfo {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacManufactureInfo {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacManufactureInfo {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacManufactureInfo {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacManufactureInfo {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_LIFETIME_DATA_BLOCK_5")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacLifetimeDataBlock5 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for MacLifetimeDataBlock5 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl MacLifetimeDataBlock5 {
    /// `15:0` - Read the `num_ascc_events` field.
    ///
    #[doc(alias = "NUM_ASCC_EVENTS")]
    #[must_use]
    pub fn num_ascc_events(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `last_ascc_event` field.
    ///
    #[doc(alias = "LAST_ASCC_EVENT")]
    #[must_use]
    pub fn last_ascc_event(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `num_otc_events` field.
    ///
    #[doc(alias = "NUM_OTC_EVENTS")]
    #[must_use]
    pub fn num_otc_events(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `last_otc_event` field.
    ///
    #[doc(alias = "LAST_OTC_EVENT")]
    #[must_use]
    pub fn last_otc_event(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `num_otd_event` field.
    ///
    #[doc(alias = "NUM_OTD_EVENT")]
    #[must_use]
    pub fn num_otd_event(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `last_otd_event` field.
    ///
    #[doc(alias = "LAST_OTD_EVENT")]
    #[must_use]
    pub fn last_otd_event(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `num_otf_events` field.
    ///
    #[doc(alias = "NUM_OTF_EVENTS")]
    #[must_use]
    pub fn num_otf_events(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `last_otf_event` field.
    ///
    #[doc(alias = "LAST_OTF_EVENT")]
    #[must_use]
    pub fn last_otf_event(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `num_valid_chg_term` field.
    ///
    #[doc(alias = "NUM_VALID_CHG_TERM")]
    #[must_use]
    pub fn num_valid_chg_term(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `last_valid_chg_term` field.
    ///
    #[doc(alias = "LAST_VALID_CHG_TERM")]
    #[must_use]
    pub fn last_valid_chg_term(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `num_qmax_updates` field.
    ///
    #[doc(alias = "NUM_QMAX_UPDATES")]
    #[must_use]
    pub fn num_qmax_updates(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `last_qmax_update` field.
    ///
    #[doc(alias = "LAST_QMAX_UPDATE")]
    #[must_use]
    pub fn last_qmax_update(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `num_ra_updates` field.
    ///
    #[doc(alias = "NUM_RA_UPDATES")]
    #[must_use]
    pub fn num_ra_updates(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `last_ra_update` field.
    ///
    #[doc(alias = "LAST_RA_UPDATE")]
    #[must_use]
    pub fn last_ra_update(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `num_ra_disable` field.
    ///
    #[doc(alias = "NUM_RA_DISABLE")]
    #[must_use]
    pub fn num_ra_disable(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `last_ra_disable` field.
    ///
    #[doc(alias = "LAST_RA_DISABLE")]
    #[must_use]
    pub fn last_ra_disable(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `num_ascc_events` field.
    ///
    #[doc(alias = "NUM_ASCC_EVENTS")]
    pub fn set_num_ascc_events(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `last_ascc_event` field.
    ///
    #[doc(alias = "LAST_ASCC_EVENT")]
    pub fn set_last_ascc_event(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `num_otc_events` field.
    ///
    #[doc(alias = "NUM_OTC_EVENTS")]
    pub fn set_num_otc_events(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `last_otc_event` field.
    ///
    #[doc(alias = "LAST_OTC_EVENT")]
    pub fn set_last_otc_event(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `num_otd_event` field.
    ///
    #[doc(alias = "NUM_OTD_EVENT")]
    pub fn set_num_otd_event(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `last_otd_event` field.
    ///
    #[doc(alias = "LAST_OTD_EVENT")]
    pub fn set_last_otd_event(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `num_otf_events` field.
    ///
    #[doc(alias = "NUM_OTF_EVENTS")]
    pub fn set_num_otf_events(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `last_otf_event` field.
    ///
    #[doc(alias = "LAST_OTF_EVENT")]
    pub fn set_last_otf_event(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `num_valid_chg_term` field.
    ///
    #[doc(alias = "NUM_VALID_CHG_TERM")]
    pub fn set_num_valid_chg_term(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `last_valid_chg_term` field.
    ///
    #[doc(alias = "LAST_VALID_CHG_TERM")]
    pub fn set_last_valid_chg_term(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `num_qmax_updates` field.
    ///
    #[doc(alias = "NUM_QMAX_UPDATES")]
    pub fn set_num_qmax_updates(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `last_qmax_update` field.
    ///
    #[doc(alias = "LAST_QMAX_UPDATE")]
    pub fn set_last_qmax_update(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `207:192` - Set the `num_ra_updates` field.
    ///
    #[doc(alias = "NUM_RA_UPDATES")]
    pub fn set_num_ra_updates(&mut self, value: u16) {
        let start = 192;
        let end = 207;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `223:208` - Set the `last_ra_update` field.
    ///
    #[doc(alias = "LAST_RA_UPDATE")]
    pub fn set_last_ra_update(&mut self, value: u16) {
        let start = 208;
        let end = 223;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `239:224` - Set the `num_ra_disable` field.
    ///
    #[doc(alias = "NUM_RA_DISABLE")]
    pub fn set_num_ra_disable(&mut self, value: u16) {
        let start = 224;
        let end = 239;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `255:240` - Set the `last_ra_disable` field.
    ///
    #[doc(alias = "LAST_RA_DISABLE")]
    pub fn set_last_ra_disable(&mut self, value: u16) {
        let start = 240;
        let end = 255;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacLifetimeDataBlock5 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for MacLifetimeDataBlock5 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<MacLifetimeDataBlock5> for [u8; 32] {
    fn from(val: MacLifetimeDataBlock5) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacLifetimeDataBlock5 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacLifetimeDataBlock5");
        d.field("num_ascc_events", &self.num_ascc_events());
        d.field("last_ascc_event", &self.last_ascc_event());
        d.field("num_otc_events", &self.num_otc_events());
        d.field("last_otc_event", &self.last_otc_event());
        d.field("num_otd_event", &self.num_otd_event());
        d.field("last_otd_event", &self.last_otd_event());
        d.field("num_otf_events", &self.num_otf_events());
        d.field("last_otf_event", &self.last_otf_event());
        d.field("num_valid_chg_term", &self.num_valid_chg_term());
        d.field("last_valid_chg_term", &self.last_valid_chg_term());
        d.field("num_qmax_updates", &self.num_qmax_updates());
        d.field("last_qmax_update", &self.last_qmax_update());
        d.field("num_ra_updates", &self.num_ra_updates());
        d.field("last_ra_update", &self.last_ra_update());
        d.field("num_ra_disable", &self.num_ra_disable());
        d.field("last_ra_disable", &self.last_ra_disable());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacLifetimeDataBlock5 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacLifetimeDataBlock5 {{ ");
        defmt::write!(f, "num_ascc_events: {=u16}, ", &self.num_ascc_events());
        defmt::write!(f, "last_ascc_event: {=u16}, ", &self.last_ascc_event());
        defmt::write!(f, "num_otc_events: {=u16}, ", &self.num_otc_events());
        defmt::write!(f, "last_otc_event: {=u16}, ", &self.last_otc_event());
        defmt::write!(f, "num_otd_event: {=u16}, ", &self.num_otd_event());
        defmt::write!(f, "last_otd_event: {=u16}, ", &self.last_otd_event());
        defmt::write!(f, "num_otf_events: {=u16}, ", &self.num_otf_events());
        defmt::write!(f, "last_otf_event: {=u16}, ", &self.last_otf_event());
        defmt::write!(f, "num_valid_chg_term: {=u16}, ", &self.num_valid_chg_term());
        defmt::write!(f, "last_valid_chg_term: {=u16}, ", &self.last_valid_chg_term());
        defmt::write!(f, "num_qmax_updates: {=u16}, ", &self.num_qmax_updates());
        defmt::write!(f, "last_qmax_update: {=u16}, ", &self.last_qmax_update());
        defmt::write!(f, "num_ra_updates: {=u16}, ", &self.num_ra_updates());
        defmt::write!(f, "last_ra_update: {=u16}, ", &self.last_ra_update());
        defmt::write!(f, "num_ra_disable: {=u16}, ", &self.num_ra_disable());
        defmt::write!(f, "last_ra_disable: {=u16}, ", &self.last_ra_disable());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacLifetimeDataBlock5 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacLifetimeDataBlock5 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacLifetimeDataBlock5 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacLifetimeDataBlock5 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacLifetimeDataBlock5 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacLifetimeDataBlock5 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacLifetimeDataBlock5 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_LIFETIME_DATA_BLOCK_4")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacLifetimeDataBlock4 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for MacLifetimeDataBlock4 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl MacLifetimeDataBlock4 {
    /// `15:0` - Read the `num_cov_events` field.
    ///
    #[doc(alias = "NUM_COV_EVENTS")]
    #[must_use]
    pub fn num_cov_events(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `last_cov_event` field.
    ///
    #[doc(alias = "LAST_COV_EVENT")]
    #[must_use]
    pub fn last_cov_event(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `num_cuv_events` field.
    ///
    #[doc(alias = "NUM_CUV_EVENTS")]
    #[must_use]
    pub fn num_cuv_events(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `last_cuv_event` field.
    ///
    #[doc(alias = "LAST_CUV_EVENT")]
    #[must_use]
    pub fn last_cuv_event(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `num_ocd_1_event` field.
    ///
    #[doc(alias = "NUM_OCD1_EVENT")]
    #[must_use]
    pub fn num_ocd_1_event(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `last_ocd_1_event` field.
    ///
    #[doc(alias = "LAST_OCD1_EVENT")]
    #[must_use]
    pub fn last_ocd_1_event(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `num_ocd_2_events` field.
    ///
    #[doc(alias = "NUM_OCD2_EVENTS")]
    #[must_use]
    pub fn num_ocd_2_events(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `last_ocd_2_event` field.
    ///
    #[doc(alias = "LAST_OCD2_EVENT")]
    #[must_use]
    pub fn last_ocd_2_event(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `num_occ_1_events` field.
    ///
    #[doc(alias = "NUM_OCC1_EVENTS")]
    #[must_use]
    pub fn num_occ_1_events(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `last_occ_1_event` field.
    ///
    #[doc(alias = "LAST_OCC1_EVENT")]
    #[must_use]
    pub fn last_occ_1_event(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `num_occ_2_events` field.
    ///
    #[doc(alias = "NUM_OCC2_EVENTS")]
    #[must_use]
    pub fn num_occ_2_events(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `last_occ_2_event` field.
    ///
    #[doc(alias = "LAST_OCC2_EVENT")]
    #[must_use]
    pub fn last_occ_2_event(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `num_aold_events` field.
    ///
    #[doc(alias = "NUM_AOLD_EVENTS")]
    #[must_use]
    pub fn num_aold_events(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:208` - Read the `last_aold_event` field.
    ///
    #[doc(alias = "LAST_AOLD_EVENT")]
    #[must_use]
    pub fn last_aold_event(&self) -> u16 {
        let start = 208;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:224` - Read the `num_ascd_events` field.
    ///
    #[doc(alias = "NUM_ASCD_EVENTS")]
    #[must_use]
    pub fn num_ascd_events(&self) -> u16 {
        let start = 224;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:240` - Read the `last_ascd_event` field.
    ///
    #[doc(alias = "LAST_ASCD_EVENT")]
    #[must_use]
    pub fn last_ascd_event(&self) -> u16 {
        let start = 240;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `num_cov_events` field.
    ///
    #[doc(alias = "NUM_COV_EVENTS")]
    pub fn set_num_cov_events(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `last_cov_event` field.
    ///
    #[doc(alias = "LAST_COV_EVENT")]
    pub fn set_last_cov_event(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `num_cuv_events` field.
    ///
    #[doc(alias = "NUM_CUV_EVENTS")]
    pub fn set_num_cuv_events(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `last_cuv_event` field.
    ///
    #[doc(alias = "LAST_CUV_EVENT")]
    pub fn set_last_cuv_event(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `num_ocd_1_event` field.
    ///
    #[doc(alias = "NUM_OCD1_EVENT")]
    pub fn set_num_ocd_1_event(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `last_ocd_1_event` field.
    ///
    #[doc(alias = "LAST_OCD1_EVENT")]
    pub fn set_last_ocd_1_event(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `num_ocd_2_events` field.
    ///
    #[doc(alias = "NUM_OCD2_EVENTS")]
    pub fn set_num_ocd_2_events(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `last_ocd_2_event` field.
    ///
    #[doc(alias = "LAST_OCD2_EVENT")]
    pub fn set_last_ocd_2_event(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `num_occ_1_events` field.
    ///
    #[doc(alias = "NUM_OCC1_EVENTS")]
    pub fn set_num_occ_1_events(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `last_occ_1_event` field.
    ///
    #[doc(alias = "LAST_OCC1_EVENT")]
    pub fn set_last_occ_1_event(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `num_occ_2_events` field.
    ///
    #[doc(alias = "NUM_OCC2_EVENTS")]
    pub fn set_num_occ_2_events(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `last_occ_2_event` field.
    ///
    #[doc(alias = "LAST_OCC2_EVENT")]
    pub fn set_last_occ_2_event(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `207:192` - Set the `num_aold_events` field.
    ///
    #[doc(alias = "NUM_AOLD_EVENTS")]
    pub fn set_num_aold_events(&mut self, value: u16) {
        let start = 192;
        let end = 207;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `223:208` - Set the `last_aold_event` field.
    ///
    #[doc(alias = "LAST_AOLD_EVENT")]
    pub fn set_last_aold_event(&mut self, value: u16) {
        let start = 208;
        let end = 223;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `239:224` - Set the `num_ascd_events` field.
    ///
    #[doc(alias = "NUM_ASCD_EVENTS")]
    pub fn set_num_ascd_events(&mut self, value: u16) {
        let start = 224;
        let end = 239;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `255:240` - Set the `last_ascd_event` field.
    ///
    #[doc(alias = "LAST_ASCD_EVENT")]
    pub fn set_last_ascd_event(&mut self, value: u16) {
        let start = 240;
        let end = 255;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacLifetimeDataBlock4 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for MacLifetimeDataBlock4 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<MacLifetimeDataBlock4> for [u8; 32] {
    fn from(val: MacLifetimeDataBlock4) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacLifetimeDataBlock4 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacLifetimeDataBlock4");
        d.field("num_cov_events", &self.num_cov_events());
        d.field("last_cov_event", &self.last_cov_event());
        d.field("num_cuv_events", &self.num_cuv_events());
        d.field("last_cuv_event", &self.last_cuv_event());
        d.field("num_ocd_1_event", &self.num_ocd_1_event());
        d.field("last_ocd_1_event", &self.last_ocd_1_event());
        d.field("num_ocd_2_events", &self.num_ocd_2_events());
        d.field("last_ocd_2_event", &self.last_ocd_2_event());
        d.field("num_occ_1_events", &self.num_occ_1_events());
        d.field("last_occ_1_event", &self.last_occ_1_event());
        d.field("num_occ_2_events", &self.num_occ_2_events());
        d.field("last_occ_2_event", &self.last_occ_2_event());
        d.field("num_aold_events", &self.num_aold_events());
        d.field("last_aold_event", &self.last_aold_event());
        d.field("num_ascd_events", &self.num_ascd_events());
        d.field("last_ascd_event", &self.last_ascd_event());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacLifetimeDataBlock4 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacLifetimeDataBlock4 {{ ");
        defmt::write!(f, "num_cov_events: {=u16}, ", &self.num_cov_events());
        defmt::write!(f, "last_cov_event: {=u16}, ", &self.last_cov_event());
        defmt::write!(f, "num_cuv_events: {=u16}, ", &self.num_cuv_events());
        defmt::write!(f, "last_cuv_event: {=u16}, ", &self.last_cuv_event());
        defmt::write!(f, "num_ocd_1_event: {=u16}, ", &self.num_ocd_1_event());
        defmt::write!(f, "last_ocd_1_event: {=u16}, ", &self.last_ocd_1_event());
        defmt::write!(f, "num_ocd_2_events: {=u16}, ", &self.num_ocd_2_events());
        defmt::write!(f, "last_ocd_2_event: {=u16}, ", &self.last_ocd_2_event());
        defmt::write!(f, "num_occ_1_events: {=u16}, ", &self.num_occ_1_events());
        defmt::write!(f, "last_occ_1_event: {=u16}, ", &self.last_occ_1_event());
        defmt::write!(f, "num_occ_2_events: {=u16}, ", &self.num_occ_2_events());
        defmt::write!(f, "last_occ_2_event: {=u16}, ", &self.last_occ_2_event());
        defmt::write!(f, "num_aold_events: {=u16}, ", &self.num_aold_events());
        defmt::write!(f, "last_aold_event: {=u16}, ", &self.last_aold_event());
        defmt::write!(f, "num_ascd_events: {=u16}, ", &self.num_ascd_events());
        defmt::write!(f, "last_ascd_event: {=u16}, ", &self.last_ascd_event());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacLifetimeDataBlock4 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacLifetimeDataBlock4 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacLifetimeDataBlock4 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacLifetimeDataBlock4 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacLifetimeDataBlock4 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacLifetimeDataBlock4 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacLifetimeDataBlock4 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_LIFETIME_DATA_BLOCK_3")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacLifetimeDataBlock3 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 16],
}
unsafe impl ::device_driver::Fieldset for MacLifetimeDataBlock3 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 16] };
}
impl MacLifetimeDataBlock3 {
    /// `15:0` - Read the `total_fw_runtime` field.
    ///
    #[doc(alias = "TOTAL_FW_RUNTIME")]
    #[must_use]
    pub fn total_fw_runtime(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `time_spent_in_ut` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_UT")]
    #[must_use]
    pub fn time_spent_in_ut(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `time_spent_in_lt` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_LT")]
    #[must_use]
    pub fn time_spent_in_lt(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `time_spent_in_stl` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_STL")]
    #[must_use]
    pub fn time_spent_in_stl(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `time_spent_in_rt` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_RT")]
    #[must_use]
    pub fn time_spent_in_rt(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `time_spent_in_sth` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_STH")]
    #[must_use]
    pub fn time_spent_in_sth(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `time_spent_in_ht` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_HT")]
    #[must_use]
    pub fn time_spent_in_ht(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `time_spent_in_ot` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_OT")]
    #[must_use]
    pub fn time_spent_in_ot(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `total_fw_runtime` field.
    ///
    #[doc(alias = "TOTAL_FW_RUNTIME")]
    pub fn set_total_fw_runtime(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `time_spent_in_ut` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_UT")]
    pub fn set_time_spent_in_ut(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `time_spent_in_lt` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_LT")]
    pub fn set_time_spent_in_lt(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `time_spent_in_stl` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_STL")]
    pub fn set_time_spent_in_stl(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `time_spent_in_rt` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_RT")]
    pub fn set_time_spent_in_rt(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `time_spent_in_sth` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_STH")]
    pub fn set_time_spent_in_sth(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `time_spent_in_ht` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_HT")]
    pub fn set_time_spent_in_ht(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `time_spent_in_ot` field.
    ///
    #[doc(alias = "TIME_SPENT_IN_OT")]
    pub fn set_time_spent_in_ot(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacLifetimeDataBlock3 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 16]> for MacLifetimeDataBlock3 {
    fn from(bits: [u8; 16]) -> Self {
        Self { bits }
    }
}
impl From<MacLifetimeDataBlock3> for [u8; 16] {
    fn from(val: MacLifetimeDataBlock3) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacLifetimeDataBlock3 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacLifetimeDataBlock3");
        d.field("total_fw_runtime", &self.total_fw_runtime());
        d.field("time_spent_in_ut", &self.time_spent_in_ut());
        d.field("time_spent_in_lt", &self.time_spent_in_lt());
        d.field("time_spent_in_stl", &self.time_spent_in_stl());
        d.field("time_spent_in_rt", &self.time_spent_in_rt());
        d.field("time_spent_in_sth", &self.time_spent_in_sth());
        d.field("time_spent_in_ht", &self.time_spent_in_ht());
        d.field("time_spent_in_ot", &self.time_spent_in_ot());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacLifetimeDataBlock3 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacLifetimeDataBlock3 {{ ");
        defmt::write!(f, "total_fw_runtime: {=u16}, ", &self.total_fw_runtime());
        defmt::write!(f, "time_spent_in_ut: {=u16}, ", &self.time_spent_in_ut());
        defmt::write!(f, "time_spent_in_lt: {=u16}, ", &self.time_spent_in_lt());
        defmt::write!(f, "time_spent_in_stl: {=u16}, ", &self.time_spent_in_stl());
        defmt::write!(f, "time_spent_in_rt: {=u16}, ", &self.time_spent_in_rt());
        defmt::write!(f, "time_spent_in_sth: {=u16}, ", &self.time_spent_in_sth());
        defmt::write!(f, "time_spent_in_ht: {=u16}, ", &self.time_spent_in_ht());
        defmt::write!(f, "time_spent_in_ot: {=u16}, ", &self.time_spent_in_ot());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacLifetimeDataBlock3 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacLifetimeDataBlock3 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacLifetimeDataBlock3 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacLifetimeDataBlock3 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacLifetimeDataBlock3 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacLifetimeDataBlock3 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacLifetimeDataBlock3 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_LIFETIME_DATA_BLOCK_2")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacLifetimeDataBlock2 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 8],
}
unsafe impl ::device_driver::Fieldset for MacLifetimeDataBlock2 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 8] };
}
impl MacLifetimeDataBlock2 {
    /// `7:0` - Read the `num_shutdowns` field.
    ///
    #[doc(alias = "NUM_SHUTDOWNS")]
    #[must_use]
    pub fn num_shutdowns(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `num_part_resets` field.
    ///
    #[doc(alias = "NUM_PART_RESETS")]
    #[must_use]
    pub fn num_part_resets(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `23:16` - Read the `num_full_resets` field.
    ///
    #[doc(alias = "NUM_FULL_RESETS")]
    #[must_use]
    pub fn num_full_resets(&self) -> u8 {
        let start = 16;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:24` - Read the `num_wdt_resets` field.
    ///
    #[doc(alias = "NUM_WDT_RESETS")]
    #[must_use]
    pub fn num_wdt_resets(&self) -> u8 {
        let start = 24;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `39:32` - Read the `cb_time_cell_1` field.
    ///
    #[doc(alias = "CB_TIME_CELL_1")]
    #[must_use]
    pub fn cb_time_cell_1(&self) -> u8 {
        let start = 32;
        let end = 39;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:40` - Read the `cb_time_cell_2` field.
    ///
    #[doc(alias = "CB_TIME_CELL_2")]
    #[must_use]
    pub fn cb_time_cell_2(&self) -> u8 {
        let start = 40;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `55:48` - Read the `cb_time_cell_3` field.
    ///
    #[doc(alias = "CB_TIME_CELL_3")]
    #[must_use]
    pub fn cb_time_cell_3(&self) -> u8 {
        let start = 48;
        let end = 55;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:56` - Read the `cb_time_cell_4` field.
    ///
    #[doc(alias = "CB_TIME_CELL_4")]
    #[must_use]
    pub fn cb_time_cell_4(&self) -> u8 {
        let start = 56;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `num_shutdowns` field.
    ///
    #[doc(alias = "NUM_SHUTDOWNS")]
    pub fn set_num_shutdowns(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:8` - Set the `num_part_resets` field.
    ///
    #[doc(alias = "NUM_PART_RESETS")]
    pub fn set_num_part_resets(&mut self, value: u8) {
        let start = 8;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `23:16` - Set the `num_full_resets` field.
    ///
    #[doc(alias = "NUM_FULL_RESETS")]
    pub fn set_num_full_resets(&mut self, value: u8) {
        let start = 16;
        let end = 23;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:24` - Set the `num_wdt_resets` field.
    ///
    #[doc(alias = "NUM_WDT_RESETS")]
    pub fn set_num_wdt_resets(&mut self, value: u8) {
        let start = 24;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `39:32` - Set the `cb_time_cell_1` field.
    ///
    #[doc(alias = "CB_TIME_CELL_1")]
    pub fn set_cb_time_cell_1(&mut self, value: u8) {
        let start = 32;
        let end = 39;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:40` - Set the `cb_time_cell_2` field.
    ///
    #[doc(alias = "CB_TIME_CELL_2")]
    pub fn set_cb_time_cell_2(&mut self, value: u8) {
        let start = 40;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `55:48` - Set the `cb_time_cell_3` field.
    ///
    #[doc(alias = "CB_TIME_CELL_3")]
    pub fn set_cb_time_cell_3(&mut self, value: u8) {
        let start = 48;
        let end = 55;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:56` - Set the `cb_time_cell_4` field.
    ///
    #[doc(alias = "CB_TIME_CELL_4")]
    pub fn set_cb_time_cell_4(&mut self, value: u8) {
        let start = 56;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacLifetimeDataBlock2 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 8]> for MacLifetimeDataBlock2 {
    fn from(bits: [u8; 8]) -> Self {
        Self { bits }
    }
}
impl From<MacLifetimeDataBlock2> for [u8; 8] {
    fn from(val: MacLifetimeDataBlock2) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacLifetimeDataBlock2 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacLifetimeDataBlock2");
        d.field("num_shutdowns", &self.num_shutdowns());
        d.field("num_part_resets", &self.num_part_resets());
        d.field("num_full_resets", &self.num_full_resets());
        d.field("num_wdt_resets", &self.num_wdt_resets());
        d.field("cb_time_cell_1", &self.cb_time_cell_1());
        d.field("cb_time_cell_2", &self.cb_time_cell_2());
        d.field("cb_time_cell_3", &self.cb_time_cell_3());
        d.field("cb_time_cell_4", &self.cb_time_cell_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacLifetimeDataBlock2 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacLifetimeDataBlock2 {{ ");
        defmt::write!(f, "num_shutdowns: {=u8}, ", &self.num_shutdowns());
        defmt::write!(f, "num_part_resets: {=u8}, ", &self.num_part_resets());
        defmt::write!(f, "num_full_resets: {=u8}, ", &self.num_full_resets());
        defmt::write!(f, "num_wdt_resets: {=u8}, ", &self.num_wdt_resets());
        defmt::write!(f, "cb_time_cell_1: {=u8}, ", &self.cb_time_cell_1());
        defmt::write!(f, "cb_time_cell_2: {=u8}, ", &self.cb_time_cell_2());
        defmt::write!(f, "cb_time_cell_3: {=u8}, ", &self.cb_time_cell_3());
        defmt::write!(f, "cb_time_cell_4: {=u8}, ", &self.cb_time_cell_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacLifetimeDataBlock2 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacLifetimeDataBlock2 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacLifetimeDataBlock2 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacLifetimeDataBlock2 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacLifetimeDataBlock2 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacLifetimeDataBlock2 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacLifetimeDataBlock2 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_LIFETIME_DATA_BLOCK_1")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacLifetimeDataBlock1 {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 32],
}
unsafe impl ::device_driver::Fieldset for MacLifetimeDataBlock1 {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 32] };
}
impl MacLifetimeDataBlock1 {
    /// `15:0` - Read the `cell_1_max_v` field.
    ///
    #[doc(alias = "CELL_1_MAX_V")]
    #[must_use]
    pub fn cell_1_max_v(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `cell_2_max_v` field.
    ///
    #[doc(alias = "CELL_2_MAX_V")]
    #[must_use]
    pub fn cell_2_max_v(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `cell_3_max_v` field.
    ///
    #[doc(alias = "CELL_3_MAX_V")]
    #[must_use]
    pub fn cell_3_max_v(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:48` - Read the `cell_4_max_v` field.
    ///
    #[doc(alias = "CELL_4_MAX_V")]
    #[must_use]
    pub fn cell_4_max_v(&self) -> u16 {
        let start = 48;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:64` - Read the `cell_1_min_v` field.
    ///
    #[doc(alias = "CELL_1_MIN_V")]
    #[must_use]
    pub fn cell_1_min_v(&self) -> u16 {
        let start = 64;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:80` - Read the `cell_2_min_v` field.
    ///
    #[doc(alias = "CELL_2_MIN_V")]
    #[must_use]
    pub fn cell_2_min_v(&self) -> u16 {
        let start = 80;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:96` - Read the `cell_3_min_v` field.
    ///
    #[doc(alias = "CELL_3_MIN_V")]
    #[must_use]
    pub fn cell_3_min_v(&self) -> u16 {
        let start = 96;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:112` - Read the `cell_4_min_v` field.
    ///
    #[doc(alias = "CELL_4_MIN_V")]
    #[must_use]
    pub fn cell_4_min_v(&self) -> u16 {
        let start = 112;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:128` - Read the `max_delta_cell_v` field.
    ///
    #[doc(alias = "MAX_DELTA_CELL_V")]
    #[must_use]
    pub fn max_delta_cell_v(&self) -> u16 {
        let start = 128;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:144` - Read the `max_charge_a` field.
    ///
    #[doc(alias = "MAX_CHARGE_A")]
    #[must_use]
    pub fn max_charge_a(&self) -> u16 {
        let start = 144;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `175:160` - Read the `max_discharge_a` field.
    ///
    #[doc(alias = "MAX_DISCHARGE_A")]
    #[must_use]
    pub fn max_discharge_a(&self) -> u16 {
        let start = 160;
        let end = 175;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `191:176` - Read the `max_avg_discharge_a` field.
    ///
    #[doc(alias = "MAX_AVG_DISCHARGE_A")]
    #[must_use]
    pub fn max_avg_discharge_a(&self) -> u16 {
        let start = 176;
        let end = 191;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `207:192` - Read the `max_avg_discharge_pwr` field.
    ///
    #[doc(alias = "MAX_AVG_DISCHARGE_PWR")]
    #[must_use]
    pub fn max_avg_discharge_pwr(&self) -> u16 {
        let start = 192;
        let end = 207;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `215:208` - Read the `max_temp_cell` field.
    ///
    #[doc(alias = "MAX_TEMP_CELL")]
    #[must_use]
    pub fn max_temp_cell(&self) -> u8 {
        let start = 208;
        let end = 215;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `223:216` - Read the `min_temp_cell` field.
    ///
    #[doc(alias = "MIN_TEMP_CELL")]
    #[must_use]
    pub fn min_temp_cell(&self) -> u8 {
        let start = 216;
        let end = 223;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `231:224` - Read the `max_delta_cell_temp` field.
    ///
    #[doc(alias = "MAX_DELTA_CELL_TEMP")]
    #[must_use]
    pub fn max_delta_cell_temp(&self) -> u8 {
        let start = 224;
        let end = 231;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `239:232` - Read the `max_temp_int_sensor` field.
    ///
    #[doc(alias = "MAX_TEMP_INT_SENSOR")]
    #[must_use]
    pub fn max_temp_int_sensor(&self) -> u8 {
        let start = 232;
        let end = 239;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `247:240` - Read the `min_temp_int_sensor` field.
    ///
    #[doc(alias = "MIN_TEMP_INT_SENSOR")]
    #[must_use]
    pub fn min_temp_int_sensor(&self) -> u8 {
        let start = 240;
        let end = 247;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `255:248` - Read the `max_temp_fet` field.
    ///
    #[doc(alias = "MAX_TEMP_FET")]
    #[must_use]
    pub fn max_temp_fet(&self) -> u8 {
        let start = 248;
        let end = 255;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `cell_1_max_v` field.
    ///
    #[doc(alias = "CELL_1_MAX_V")]
    pub fn set_cell_1_max_v(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `cell_2_max_v` field.
    ///
    #[doc(alias = "CELL_2_MAX_V")]
    pub fn set_cell_2_max_v(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `cell_3_max_v` field.
    ///
    #[doc(alias = "CELL_3_MAX_V")]
    pub fn set_cell_3_max_v(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:48` - Set the `cell_4_max_v` field.
    ///
    #[doc(alias = "CELL_4_MAX_V")]
    pub fn set_cell_4_max_v(&mut self, value: u16) {
        let start = 48;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:64` - Set the `cell_1_min_v` field.
    ///
    #[doc(alias = "CELL_1_MIN_V")]
    pub fn set_cell_1_min_v(&mut self, value: u16) {
        let start = 64;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:80` - Set the `cell_2_min_v` field.
    ///
    #[doc(alias = "CELL_2_MIN_V")]
    pub fn set_cell_2_min_v(&mut self, value: u16) {
        let start = 80;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:96` - Set the `cell_3_min_v` field.
    ///
    #[doc(alias = "CELL_3_MIN_V")]
    pub fn set_cell_3_min_v(&mut self, value: u16) {
        let start = 96;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:112` - Set the `cell_4_min_v` field.
    ///
    #[doc(alias = "CELL_4_MIN_V")]
    pub fn set_cell_4_min_v(&mut self, value: u16) {
        let start = 112;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:128` - Set the `max_delta_cell_v` field.
    ///
    #[doc(alias = "MAX_DELTA_CELL_V")]
    pub fn set_max_delta_cell_v(&mut self, value: u16) {
        let start = 128;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:144` - Set the `max_charge_a` field.
    ///
    #[doc(alias = "MAX_CHARGE_A")]
    pub fn set_max_charge_a(&mut self, value: u16) {
        let start = 144;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `175:160` - Set the `max_discharge_a` field.
    ///
    #[doc(alias = "MAX_DISCHARGE_A")]
    pub fn set_max_discharge_a(&mut self, value: u16) {
        let start = 160;
        let end = 175;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `191:176` - Set the `max_avg_discharge_a` field.
    ///
    #[doc(alias = "MAX_AVG_DISCHARGE_A")]
    pub fn set_max_avg_discharge_a(&mut self, value: u16) {
        let start = 176;
        let end = 191;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `207:192` - Set the `max_avg_discharge_pwr` field.
    ///
    #[doc(alias = "MAX_AVG_DISCHARGE_PWR")]
    pub fn set_max_avg_discharge_pwr(&mut self, value: u16) {
        let start = 192;
        let end = 207;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `215:208` - Set the `max_temp_cell` field.
    ///
    #[doc(alias = "MAX_TEMP_CELL")]
    pub fn set_max_temp_cell(&mut self, value: u8) {
        let start = 208;
        let end = 215;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `223:216` - Set the `min_temp_cell` field.
    ///
    #[doc(alias = "MIN_TEMP_CELL")]
    pub fn set_min_temp_cell(&mut self, value: u8) {
        let start = 216;
        let end = 223;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `231:224` - Set the `max_delta_cell_temp` field.
    ///
    #[doc(alias = "MAX_DELTA_CELL_TEMP")]
    pub fn set_max_delta_cell_temp(&mut self, value: u8) {
        let start = 224;
        let end = 231;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `239:232` - Set the `max_temp_int_sensor` field.
    ///
    #[doc(alias = "MAX_TEMP_INT_SENSOR")]
    pub fn set_max_temp_int_sensor(&mut self, value: u8) {
        let start = 232;
        let end = 239;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `247:240` - Set the `min_temp_int_sensor` field.
    ///
    #[doc(alias = "MIN_TEMP_INT_SENSOR")]
    pub fn set_min_temp_int_sensor(&mut self, value: u8) {
        let start = 240;
        let end = 247;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `255:248` - Set the `max_temp_fet` field.
    ///
    #[doc(alias = "MAX_TEMP_FET")]
    pub fn set_max_temp_fet(&mut self, value: u8) {
        let start = 248;
        let end = 255;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacLifetimeDataBlock1 {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 32]> for MacLifetimeDataBlock1 {
    fn from(bits: [u8; 32]) -> Self {
        Self { bits }
    }
}
impl From<MacLifetimeDataBlock1> for [u8; 32] {
    fn from(val: MacLifetimeDataBlock1) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacLifetimeDataBlock1 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacLifetimeDataBlock1");
        d.field("cell_1_max_v", &self.cell_1_max_v());
        d.field("cell_2_max_v", &self.cell_2_max_v());
        d.field("cell_3_max_v", &self.cell_3_max_v());
        d.field("cell_4_max_v", &self.cell_4_max_v());
        d.field("cell_1_min_v", &self.cell_1_min_v());
        d.field("cell_2_min_v", &self.cell_2_min_v());
        d.field("cell_3_min_v", &self.cell_3_min_v());
        d.field("cell_4_min_v", &self.cell_4_min_v());
        d.field("max_delta_cell_v", &self.max_delta_cell_v());
        d.field("max_charge_a", &self.max_charge_a());
        d.field("max_discharge_a", &self.max_discharge_a());
        d.field("max_avg_discharge_a", &self.max_avg_discharge_a());
        d.field("max_avg_discharge_pwr", &self.max_avg_discharge_pwr());
        d.field("max_temp_cell", &self.max_temp_cell());
        d.field("min_temp_cell", &self.min_temp_cell());
        d.field("max_delta_cell_temp", &self.max_delta_cell_temp());
        d.field("max_temp_int_sensor", &self.max_temp_int_sensor());
        d.field("min_temp_int_sensor", &self.min_temp_int_sensor());
        d.field("max_temp_fet", &self.max_temp_fet());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacLifetimeDataBlock1 {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacLifetimeDataBlock1 {{ ");
        defmt::write!(f, "cell_1_max_v: {=u16}, ", &self.cell_1_max_v());
        defmt::write!(f, "cell_2_max_v: {=u16}, ", &self.cell_2_max_v());
        defmt::write!(f, "cell_3_max_v: {=u16}, ", &self.cell_3_max_v());
        defmt::write!(f, "cell_4_max_v: {=u16}, ", &self.cell_4_max_v());
        defmt::write!(f, "cell_1_min_v: {=u16}, ", &self.cell_1_min_v());
        defmt::write!(f, "cell_2_min_v: {=u16}, ", &self.cell_2_min_v());
        defmt::write!(f, "cell_3_min_v: {=u16}, ", &self.cell_3_min_v());
        defmt::write!(f, "cell_4_min_v: {=u16}, ", &self.cell_4_min_v());
        defmt::write!(f, "max_delta_cell_v: {=u16}, ", &self.max_delta_cell_v());
        defmt::write!(f, "max_charge_a: {=u16}, ", &self.max_charge_a());
        defmt::write!(f, "max_discharge_a: {=u16}, ", &self.max_discharge_a());
        defmt::write!(f, "max_avg_discharge_a: {=u16}, ", &self.max_avg_discharge_a());
        defmt::write!(f, "max_avg_discharge_pwr: {=u16}, ", &self.max_avg_discharge_pwr());
        defmt::write!(f, "max_temp_cell: {=u8}, ", &self.max_temp_cell());
        defmt::write!(f, "min_temp_cell: {=u8}, ", &self.min_temp_cell());
        defmt::write!(f, "max_delta_cell_temp: {=u8}, ", &self.max_delta_cell_temp());
        defmt::write!(f, "max_temp_int_sensor: {=u8}, ", &self.max_temp_int_sensor());
        defmt::write!(f, "min_temp_int_sensor: {=u8}, ", &self.min_temp_int_sensor());
        defmt::write!(f, "max_temp_fet: {=u8}, ", &self.max_temp_fet());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacLifetimeDataBlock1 {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacLifetimeDataBlock1 {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacLifetimeDataBlock1 {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacLifetimeDataBlock1 {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacLifetimeDataBlock1 {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacLifetimeDataBlock1 {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacLifetimeDataBlock1 {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_AFE_REG")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacAfeReg {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 21],
}
unsafe impl ::device_driver::Fieldset for MacAfeReg {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 21] };
}
impl MacAfeReg {
    /// `7:0` - Read the `afe_int_status` field.
    ///
    #[doc(alias = "AFE_INT_STATUS")]
    #[must_use]
    pub fn afe_int_status(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:8` - Read the `afe_fet_status` field.
    ///
    #[doc(alias = "AFE_FET_STATUS")]
    #[must_use]
    pub fn afe_fet_status(&self) -> u8 {
        let start = 8;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `23:16` - Read the `afe_rxin` field.
    ///
    #[doc(alias = "AFE_RXIN")]
    #[must_use]
    pub fn afe_rxin(&self) -> u8 {
        let start = 16;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:24` - Read the `afe_latch_status` field.
    ///
    #[doc(alias = "AFE_LATCH_STATUS")]
    #[must_use]
    pub fn afe_latch_status(&self) -> u8 {
        let start = 24;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `39:32` - Read the `afe_int_en` field.
    ///
    #[doc(alias = "AFE_INT_EN")]
    #[must_use]
    pub fn afe_int_en(&self) -> u8 {
        let start = 32;
        let end = 39;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:40` - Read the `afe_ctrl` field.
    ///
    #[doc(alias = "AFE_CTRL")]
    #[must_use]
    pub fn afe_ctrl(&self) -> u8 {
        let start = 40;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `55:48` - Read the `afe_rxien` field.
    ///
    #[doc(alias = "AFE_RXIEN")]
    #[must_use]
    pub fn afe_rxien(&self) -> u8 {
        let start = 48;
        let end = 55;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `63:56` - Read the `afe_rlout` field.
    ///
    #[doc(alias = "AFE_RLOUT")]
    #[must_use]
    pub fn afe_rlout(&self) -> u8 {
        let start = 56;
        let end = 63;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `71:64` - Read the `afe_rhout` field.
    ///
    #[doc(alias = "AFE_RHOUT")]
    #[must_use]
    pub fn afe_rhout(&self) -> u8 {
        let start = 64;
        let end = 71;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `79:72` - Read the `afe_rhint` field.
    ///
    #[doc(alias = "AFE_RHINT")]
    #[must_use]
    pub fn afe_rhint(&self) -> u8 {
        let start = 72;
        let end = 79;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `87:80` - Read the `afe_cell_balance` field.
    ///
    #[doc(alias = "AFE_CELL_BALANCE")]
    #[must_use]
    pub fn afe_cell_balance(&self) -> u8 {
        let start = 80;
        let end = 87;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `95:88` - Read the `afe_adc_cc_ctrl` field.
    ///
    #[doc(alias = "AFE_ADC_CC_CTRL")]
    #[must_use]
    pub fn afe_adc_cc_ctrl(&self) -> u8 {
        let start = 88;
        let end = 95;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `103:96` - Read the `afe_adc_mux_ctrl` field.
    ///
    #[doc(alias = "AFE_ADC_MUX_CTRL")]
    #[must_use]
    pub fn afe_adc_mux_ctrl(&self) -> u8 {
        let start = 96;
        let end = 103;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `111:104` - Read the `afe_led_ctrl` field.
    ///
    #[doc(alias = "AFE_LED_CTRL")]
    #[must_use]
    pub fn afe_led_ctrl(&self) -> u8 {
        let start = 104;
        let end = 111;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `119:112` - Read the `afe_hw_ctrl` field.
    ///
    #[doc(alias = "AFE_HW_CTRL")]
    #[must_use]
    pub fn afe_hw_ctrl(&self) -> u8 {
        let start = 112;
        let end = 119;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `127:120` - Read the `afe_tmr_ctrl` field.
    ///
    #[doc(alias = "AFE_TMR_CTRL")]
    #[must_use]
    pub fn afe_tmr_ctrl(&self) -> u8 {
        let start = 120;
        let end = 127;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `135:128` - Read the `afe_protection` field.
    ///
    #[doc(alias = "AFE_PROTECTION")]
    #[must_use]
    pub fn afe_protection(&self) -> u8 {
        let start = 128;
        let end = 135;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `143:136` - Read the `afe_ocd` field.
    ///
    #[doc(alias = "AFE_OCD")]
    #[must_use]
    pub fn afe_ocd(&self) -> u8 {
        let start = 136;
        let end = 143;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `151:144` - Read the `afe_scc` field.
    ///
    #[doc(alias = "AFE_SCC")]
    #[must_use]
    pub fn afe_scc(&self) -> u8 {
        let start = 144;
        let end = 151;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `159:152` - Read the `afe_scd_1` field.
    ///
    #[doc(alias = "AFE_SCD1")]
    #[must_use]
    pub fn afe_scd_1(&self) -> u8 {
        let start = 152;
        let end = 159;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `167:160` - Read the `afe_scd_2` field.
    ///
    #[doc(alias = "AFE_SCD2")]
    #[must_use]
    pub fn afe_scd_2(&self) -> u8 {
        let start = 160;
        let end = 167;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `afe_int_status` field.
    ///
    #[doc(alias = "AFE_INT_STATUS")]
    pub fn set_afe_int_status(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `15:8` - Set the `afe_fet_status` field.
    ///
    #[doc(alias = "AFE_FET_STATUS")]
    pub fn set_afe_fet_status(&mut self, value: u8) {
        let start = 8;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `23:16` - Set the `afe_rxin` field.
    ///
    #[doc(alias = "AFE_RXIN")]
    pub fn set_afe_rxin(&mut self, value: u8) {
        let start = 16;
        let end = 23;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:24` - Set the `afe_latch_status` field.
    ///
    #[doc(alias = "AFE_LATCH_STATUS")]
    pub fn set_afe_latch_status(&mut self, value: u8) {
        let start = 24;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `39:32` - Set the `afe_int_en` field.
    ///
    #[doc(alias = "AFE_INT_EN")]
    pub fn set_afe_int_en(&mut self, value: u8) {
        let start = 32;
        let end = 39;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:40` - Set the `afe_ctrl` field.
    ///
    #[doc(alias = "AFE_CTRL")]
    pub fn set_afe_ctrl(&mut self, value: u8) {
        let start = 40;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `55:48` - Set the `afe_rxien` field.
    ///
    #[doc(alias = "AFE_RXIEN")]
    pub fn set_afe_rxien(&mut self, value: u8) {
        let start = 48;
        let end = 55;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `63:56` - Set the `afe_rlout` field.
    ///
    #[doc(alias = "AFE_RLOUT")]
    pub fn set_afe_rlout(&mut self, value: u8) {
        let start = 56;
        let end = 63;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `71:64` - Set the `afe_rhout` field.
    ///
    #[doc(alias = "AFE_RHOUT")]
    pub fn set_afe_rhout(&mut self, value: u8) {
        let start = 64;
        let end = 71;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `79:72` - Set the `afe_rhint` field.
    ///
    #[doc(alias = "AFE_RHINT")]
    pub fn set_afe_rhint(&mut self, value: u8) {
        let start = 72;
        let end = 79;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `87:80` - Set the `afe_cell_balance` field.
    ///
    #[doc(alias = "AFE_CELL_BALANCE")]
    pub fn set_afe_cell_balance(&mut self, value: u8) {
        let start = 80;
        let end = 87;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `95:88` - Set the `afe_adc_cc_ctrl` field.
    ///
    #[doc(alias = "AFE_ADC_CC_CTRL")]
    pub fn set_afe_adc_cc_ctrl(&mut self, value: u8) {
        let start = 88;
        let end = 95;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `103:96` - Set the `afe_adc_mux_ctrl` field.
    ///
    #[doc(alias = "AFE_ADC_MUX_CTRL")]
    pub fn set_afe_adc_mux_ctrl(&mut self, value: u8) {
        let start = 96;
        let end = 103;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `111:104` - Set the `afe_led_ctrl` field.
    ///
    #[doc(alias = "AFE_LED_CTRL")]
    pub fn set_afe_led_ctrl(&mut self, value: u8) {
        let start = 104;
        let end = 111;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `119:112` - Set the `afe_hw_ctrl` field.
    ///
    #[doc(alias = "AFE_HW_CTRL")]
    pub fn set_afe_hw_ctrl(&mut self, value: u8) {
        let start = 112;
        let end = 119;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `127:120` - Set the `afe_tmr_ctrl` field.
    ///
    #[doc(alias = "AFE_TMR_CTRL")]
    pub fn set_afe_tmr_ctrl(&mut self, value: u8) {
        let start = 120;
        let end = 127;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `135:128` - Set the `afe_protection` field.
    ///
    #[doc(alias = "AFE_PROTECTION")]
    pub fn set_afe_protection(&mut self, value: u8) {
        let start = 128;
        let end = 135;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `143:136` - Set the `afe_ocd` field.
    ///
    #[doc(alias = "AFE_OCD")]
    pub fn set_afe_ocd(&mut self, value: u8) {
        let start = 136;
        let end = 143;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `151:144` - Set the `afe_scc` field.
    ///
    #[doc(alias = "AFE_SCC")]
    pub fn set_afe_scc(&mut self, value: u8) {
        let start = 144;
        let end = 151;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `159:152` - Set the `afe_scd_1` field.
    ///
    #[doc(alias = "AFE_SCD1")]
    pub fn set_afe_scd_1(&mut self, value: u8) {
        let start = 152;
        let end = 159;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `167:160` - Set the `afe_scd_2` field.
    ///
    #[doc(alias = "AFE_SCD2")]
    pub fn set_afe_scd_2(&mut self, value: u8) {
        let start = 160;
        let end = 167;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacAfeReg {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 21]> for MacAfeReg {
    fn from(bits: [u8; 21]) -> Self {
        Self { bits }
    }
}
impl From<MacAfeReg> for [u8; 21] {
    fn from(val: MacAfeReg) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacAfeReg {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacAfeReg");
        d.field("afe_int_status", &self.afe_int_status());
        d.field("afe_fet_status", &self.afe_fet_status());
        d.field("afe_rxin", &self.afe_rxin());
        d.field("afe_latch_status", &self.afe_latch_status());
        d.field("afe_int_en", &self.afe_int_en());
        d.field("afe_ctrl", &self.afe_ctrl());
        d.field("afe_rxien", &self.afe_rxien());
        d.field("afe_rlout", &self.afe_rlout());
        d.field("afe_rhout", &self.afe_rhout());
        d.field("afe_rhint", &self.afe_rhint());
        d.field("afe_cell_balance", &self.afe_cell_balance());
        d.field("afe_adc_cc_ctrl", &self.afe_adc_cc_ctrl());
        d.field("afe_adc_mux_ctrl", &self.afe_adc_mux_ctrl());
        d.field("afe_led_ctrl", &self.afe_led_ctrl());
        d.field("afe_hw_ctrl", &self.afe_hw_ctrl());
        d.field("afe_tmr_ctrl", &self.afe_tmr_ctrl());
        d.field("afe_protection", &self.afe_protection());
        d.field("afe_ocd", &self.afe_ocd());
        d.field("afe_scc", &self.afe_scc());
        d.field("afe_scd_1", &self.afe_scd_1());
        d.field("afe_scd_2", &self.afe_scd_2());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacAfeReg {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacAfeReg {{ ");
        defmt::write!(f, "afe_int_status: {=u8}, ", &self.afe_int_status());
        defmt::write!(f, "afe_fet_status: {=u8}, ", &self.afe_fet_status());
        defmt::write!(f, "afe_rxin: {=u8}, ", &self.afe_rxin());
        defmt::write!(f, "afe_latch_status: {=u8}, ", &self.afe_latch_status());
        defmt::write!(f, "afe_int_en: {=u8}, ", &self.afe_int_en());
        defmt::write!(f, "afe_ctrl: {=u8}, ", &self.afe_ctrl());
        defmt::write!(f, "afe_rxien: {=u8}, ", &self.afe_rxien());
        defmt::write!(f, "afe_rlout: {=u8}, ", &self.afe_rlout());
        defmt::write!(f, "afe_rhout: {=u8}, ", &self.afe_rhout());
        defmt::write!(f, "afe_rhint: {=u8}, ", &self.afe_rhint());
        defmt::write!(f, "afe_cell_balance: {=u8}, ", &self.afe_cell_balance());
        defmt::write!(f, "afe_adc_cc_ctrl: {=u8}, ", &self.afe_adc_cc_ctrl());
        defmt::write!(f, "afe_adc_mux_ctrl: {=u8}, ", &self.afe_adc_mux_ctrl());
        defmt::write!(f, "afe_led_ctrl: {=u8}, ", &self.afe_led_ctrl());
        defmt::write!(f, "afe_hw_ctrl: {=u8}, ", &self.afe_hw_ctrl());
        defmt::write!(f, "afe_tmr_ctrl: {=u8}, ", &self.afe_tmr_ctrl());
        defmt::write!(f, "afe_protection: {=u8}, ", &self.afe_protection());
        defmt::write!(f, "afe_ocd: {=u8}, ", &self.afe_ocd());
        defmt::write!(f, "afe_scc: {=u8}, ", &self.afe_scc());
        defmt::write!(f, "afe_scd_1: {=u8}, ", &self.afe_scd_1());
        defmt::write!(f, "afe_scd_2: {=u8}, ", &self.afe_scd_2());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacAfeReg {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacAfeReg {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacAfeReg {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacAfeReg {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacAfeReg {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacAfeReg {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacAfeReg {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_MANUFACTURING_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacManufacturingStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for MacManufacturingStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl MacManufacturingStatus {
    /// `bit 0` - Read the `pchg_en` field.
    ///
    #[doc(alias = "PCHG_EN")]
    #[must_use]
    pub fn pchg_en(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `chg_en` field.
    ///
    #[doc(alias = "CHG_EN")]
    #[must_use]
    pub fn chg_en(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `dsg_en` field.
    ///
    #[doc(alias = "DSG_EN")]
    #[must_use]
    pub fn dsg_en(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `gauge_en` field.
    ///
    #[doc(alias = "GAUGE_EN")]
    #[must_use]
    pub fn gauge_en(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `fet_en` field.
    ///
    #[doc(alias = "FET_EN")]
    #[must_use]
    pub fn fet_en(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `lf_en` field.
    ///
    #[doc(alias = "LF_EN")]
    #[must_use]
    pub fn lf_en(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `pf_en` field.
    ///
    #[doc(alias = "PF_EN")]
    #[must_use]
    pub fn pf_en(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `bbr_en` field.
    ///
    #[doc(alias = "BBR_EN")]
    #[must_use]
    pub fn bbr_en(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `fuse_en` field.
    ///
    #[doc(alias = "FUSE_EN")]
    #[must_use]
    pub fn fuse_en(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `led_en` field.
    ///
    #[doc(alias = "LED_EN")]
    #[must_use]
    pub fn led_en(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `lt_test` field.
    ///
    #[doc(alias = "LT_TEST")]
    #[must_use]
    pub fn lt_test(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `cal_test` field.
    ///
    #[doc(alias = "CAL_TEST")]
    #[must_use]
    pub fn cal_test(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `pchg_en` field.
    ///
    #[doc(alias = "PCHG_EN")]
    pub fn set_pchg_en(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `chg_en` field.
    ///
    #[doc(alias = "CHG_EN")]
    pub fn set_chg_en(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `dsg_en` field.
    ///
    #[doc(alias = "DSG_EN")]
    pub fn set_dsg_en(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `gauge_en` field.
    ///
    #[doc(alias = "GAUGE_EN")]
    pub fn set_gauge_en(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `fet_en` field.
    ///
    #[doc(alias = "FET_EN")]
    pub fn set_fet_en(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `lf_en` field.
    ///
    #[doc(alias = "LF_EN")]
    pub fn set_lf_en(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `pf_en` field.
    ///
    #[doc(alias = "PF_EN")]
    pub fn set_pf_en(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `bbr_en` field.
    ///
    #[doc(alias = "BBR_EN")]
    pub fn set_bbr_en(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `fuse_en` field.
    ///
    #[doc(alias = "FUSE_EN")]
    pub fn set_fuse_en(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `led_en` field.
    ///
    #[doc(alias = "LED_EN")]
    pub fn set_led_en(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `lt_test` field.
    ///
    #[doc(alias = "LT_TEST")]
    pub fn set_lt_test(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `cal_test` field.
    ///
    #[doc(alias = "CAL_TEST")]
    pub fn set_cal_test(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacManufacturingStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for MacManufacturingStatus {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<MacManufacturingStatus> for [u8; 2] {
    fn from(val: MacManufacturingStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacManufacturingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacManufacturingStatus");
        d.field("pchg_en", &self.pchg_en());
        d.field("chg_en", &self.chg_en());
        d.field("dsg_en", &self.dsg_en());
        d.field("gauge_en", &self.gauge_en());
        d.field("fet_en", &self.fet_en());
        d.field("lf_en", &self.lf_en());
        d.field("pf_en", &self.pf_en());
        d.field("bbr_en", &self.bbr_en());
        d.field("fuse_en", &self.fuse_en());
        d.field("led_en", &self.led_en());
        d.field("lt_test", &self.lt_test());
        d.field("cal_test", &self.cal_test());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacManufacturingStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacManufacturingStatus {{ ");
        defmt::write!(f, "pchg_en: {=bool}, ", &self.pchg_en());
        defmt::write!(f, "chg_en: {=bool}, ", &self.chg_en());
        defmt::write!(f, "dsg_en: {=bool}, ", &self.dsg_en());
        defmt::write!(f, "gauge_en: {=bool}, ", &self.gauge_en());
        defmt::write!(f, "fet_en: {=bool}, ", &self.fet_en());
        defmt::write!(f, "lf_en: {=bool}, ", &self.lf_en());
        defmt::write!(f, "pf_en: {=bool}, ", &self.pf_en());
        defmt::write!(f, "bbr_en: {=bool}, ", &self.bbr_en());
        defmt::write!(f, "fuse_en: {=bool}, ", &self.fuse_en());
        defmt::write!(f, "led_en: {=bool}, ", &self.led_en());
        defmt::write!(f, "lt_test: {=bool}, ", &self.lt_test());
        defmt::write!(f, "cal_test: {=bool}, ", &self.cal_test());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacManufacturingStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacManufacturingStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacManufacturingStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacManufacturingStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacManufacturingStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacManufacturingStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacManufacturingStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_GAUGING_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacGaugingStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacGaugingStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacGaugingStatus {
    /// `bit 0` - Read the `fd` field.
    ///
    #[doc(alias = "FD")]
    #[must_use]
    pub fn fd(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `fc` field.
    ///
    #[doc(alias = "FC")]
    #[must_use]
    pub fn fc(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `td` field.
    ///
    #[doc(alias = "TD")]
    #[must_use]
    pub fn td(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `tc` field.
    ///
    #[doc(alias = "TC")]
    #[must_use]
    pub fn tc(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `bal_en` field.
    ///
    #[doc(alias = "BAL_EN")]
    #[must_use]
    pub fn bal_en(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `edv` field.
    ///
    #[doc(alias = "EDV")]
    #[must_use]
    pub fn edv(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `dsg` field.
    ///
    #[doc(alias = "DSG")]
    #[must_use]
    pub fn dsg(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `cf` field.
    ///
    #[doc(alias = "CF")]
    #[must_use]
    pub fn cf(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `rest` field.
    ///
    #[doc(alias = "REST")]
    #[must_use]
    pub fn rest(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `r_dis` field.
    ///
    #[doc(alias = "R_DIS")]
    #[must_use]
    pub fn r_dis(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `vok` field.
    ///
    #[doc(alias = "VOK")]
    #[must_use]
    pub fn vok(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `qen` field.
    ///
    #[doc(alias = "QEN")]
    #[must_use]
    pub fn qen(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `slpqmax` field.
    ///
    #[doc(alias = "SLPQMAX")]
    #[must_use]
    pub fn slpqmax(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `nsfm` field.
    ///
    #[doc(alias = "NSFM")]
    #[must_use]
    pub fn nsfm(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `vdq` field.
    ///
    #[doc(alias = "VDQ")]
    #[must_use]
    pub fn vdq(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `qmax` field.
    ///
    #[doc(alias = "QMAX")]
    #[must_use]
    pub fn qmax(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `rx` field.
    ///
    #[doc(alias = "RX")]
    #[must_use]
    pub fn rx(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `ldmd` field.
    ///
    #[doc(alias = "LDMD")]
    #[must_use]
    pub fn ldmd(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `ocvfr` field.
    ///
    #[doc(alias = "OCVFR")]
    #[must_use]
    pub fn ocvfr(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `fd` field.
    ///
    #[doc(alias = "FD")]
    pub fn set_fd(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `fc` field.
    ///
    #[doc(alias = "FC")]
    pub fn set_fc(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `td` field.
    ///
    #[doc(alias = "TD")]
    pub fn set_td(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `tc` field.
    ///
    #[doc(alias = "TC")]
    pub fn set_tc(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `bal_en` field.
    ///
    #[doc(alias = "BAL_EN")]
    pub fn set_bal_en(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `edv` field.
    ///
    #[doc(alias = "EDV")]
    pub fn set_edv(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `dsg` field.
    ///
    #[doc(alias = "DSG")]
    pub fn set_dsg(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `cf` field.
    ///
    #[doc(alias = "CF")]
    pub fn set_cf(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `rest` field.
    ///
    #[doc(alias = "REST")]
    pub fn set_rest(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `r_dis` field.
    ///
    #[doc(alias = "R_DIS")]
    pub fn set_r_dis(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `vok` field.
    ///
    #[doc(alias = "VOK")]
    pub fn set_vok(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `qen` field.
    ///
    #[doc(alias = "QEN")]
    pub fn set_qen(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `slpqmax` field.
    ///
    #[doc(alias = "SLPQMAX")]
    pub fn set_slpqmax(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `nsfm` field.
    ///
    #[doc(alias = "NSFM")]
    pub fn set_nsfm(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 16` - Set the `vdq` field.
    ///
    #[doc(alias = "VDQ")]
    pub fn set_vdq(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 17` - Set the `qmax` field.
    ///
    #[doc(alias = "QMAX")]
    pub fn set_qmax(&mut self, value: bool) {
        let start = 17;
        let end = 17;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 18` - Set the `rx` field.
    ///
    #[doc(alias = "RX")]
    pub fn set_rx(&mut self, value: bool) {
        let start = 18;
        let end = 18;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 19` - Set the `ldmd` field.
    ///
    #[doc(alias = "LDMD")]
    pub fn set_ldmd(&mut self, value: bool) {
        let start = 19;
        let end = 19;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 20` - Set the `ocvfr` field.
    ///
    #[doc(alias = "OCVFR")]
    pub fn set_ocvfr(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacGaugingStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacGaugingStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacGaugingStatus> for [u8; 4] {
    fn from(val: MacGaugingStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacGaugingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacGaugingStatus");
        d.field("fd", &self.fd());
        d.field("fc", &self.fc());
        d.field("td", &self.td());
        d.field("tc", &self.tc());
        d.field("bal_en", &self.bal_en());
        d.field("edv", &self.edv());
        d.field("dsg", &self.dsg());
        d.field("cf", &self.cf());
        d.field("rest", &self.rest());
        d.field("r_dis", &self.r_dis());
        d.field("vok", &self.vok());
        d.field("qen", &self.qen());
        d.field("slpqmax", &self.slpqmax());
        d.field("nsfm", &self.nsfm());
        d.field("vdq", &self.vdq());
        d.field("qmax", &self.qmax());
        d.field("rx", &self.rx());
        d.field("ldmd", &self.ldmd());
        d.field("ocvfr", &self.ocvfr());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacGaugingStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacGaugingStatus {{ ");
        defmt::write!(f, "fd: {=bool}, ", &self.fd());
        defmt::write!(f, "fc: {=bool}, ", &self.fc());
        defmt::write!(f, "td: {=bool}, ", &self.td());
        defmt::write!(f, "tc: {=bool}, ", &self.tc());
        defmt::write!(f, "bal_en: {=bool}, ", &self.bal_en());
        defmt::write!(f, "edv: {=bool}, ", &self.edv());
        defmt::write!(f, "dsg: {=bool}, ", &self.dsg());
        defmt::write!(f, "cf: {=bool}, ", &self.cf());
        defmt::write!(f, "rest: {=bool}, ", &self.rest());
        defmt::write!(f, "r_dis: {=bool}, ", &self.r_dis());
        defmt::write!(f, "vok: {=bool}, ", &self.vok());
        defmt::write!(f, "qen: {=bool}, ", &self.qen());
        defmt::write!(f, "slpqmax: {=bool}, ", &self.slpqmax());
        defmt::write!(f, "nsfm: {=bool}, ", &self.nsfm());
        defmt::write!(f, "vdq: {=bool}, ", &self.vdq());
        defmt::write!(f, "qmax: {=bool}, ", &self.qmax());
        defmt::write!(f, "rx: {=bool}, ", &self.rx());
        defmt::write!(f, "ldmd: {=bool}, ", &self.ldmd());
        defmt::write!(f, "ocvfr: {=bool}, ", &self.ocvfr());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacGaugingStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacGaugingStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacGaugingStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacGaugingStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacGaugingStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacGaugingStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacGaugingStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_CHARGING_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacChargingStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacChargingStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacChargingStatus {
    /// `bit 0` - Read the `ut` field.
    ///
    #[doc(alias = "UT")]
    #[must_use]
    pub fn ut(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `lt` field.
    ///
    #[doc(alias = "LT")]
    #[must_use]
    pub fn lt(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `stl` field.
    ///
    #[doc(alias = "STL")]
    #[must_use]
    pub fn stl(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `rt` field.
    ///
    #[doc(alias = "RT")]
    #[must_use]
    pub fn rt(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `sth` field.
    ///
    #[doc(alias = "STH")]
    #[must_use]
    pub fn sth(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `ht` field.
    ///
    #[doc(alias = "HT")]
    #[must_use]
    pub fn ht(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `ot` field.
    ///
    #[doc(alias = "OT")]
    #[must_use]
    pub fn ot(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `pv` field.
    ///
    #[doc(alias = "PV")]
    #[must_use]
    pub fn pv(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `lv` field.
    ///
    #[doc(alias = "LV")]
    #[must_use]
    pub fn lv(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `mv` field.
    ///
    #[doc(alias = "MV")]
    #[must_use]
    pub fn mv(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `hv` field.
    ///
    #[doc(alias = "HV")]
    #[must_use]
    pub fn hv(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `chg_in` field.
    ///
    #[doc(alias = "CHG_IN")]
    #[must_use]
    pub fn chg_in(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `mchg` field.
    ///
    #[doc(alias = "MCHG")]
    #[must_use]
    pub fn mchg(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `vct` field.
    ///
    #[doc(alias = "VCT")]
    #[must_use]
    pub fn vct(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `ccr` field.
    ///
    #[doc(alias = "CCR")]
    #[must_use]
    pub fn ccr(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `cvr` field.
    ///
    #[doc(alias = "CVR")]
    #[must_use]
    pub fn cvr(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `ccc` field.
    ///
    #[doc(alias = "CCC")]
    #[must_use]
    pub fn ccc(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `ut` field.
    ///
    #[doc(alias = "UT")]
    pub fn set_ut(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `lt` field.
    ///
    #[doc(alias = "LT")]
    pub fn set_lt(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `stl` field.
    ///
    #[doc(alias = "STL")]
    pub fn set_stl(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `rt` field.
    ///
    #[doc(alias = "RT")]
    pub fn set_rt(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `sth` field.
    ///
    #[doc(alias = "STH")]
    pub fn set_sth(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `ht` field.
    ///
    #[doc(alias = "HT")]
    pub fn set_ht(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `ot` field.
    ///
    #[doc(alias = "OT")]
    pub fn set_ot(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `pv` field.
    ///
    #[doc(alias = "PV")]
    pub fn set_pv(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `lv` field.
    ///
    #[doc(alias = "LV")]
    pub fn set_lv(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `mv` field.
    ///
    #[doc(alias = "MV")]
    pub fn set_mv(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `hv` field.
    ///
    #[doc(alias = "HV")]
    pub fn set_hv(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `chg_in` field.
    ///
    #[doc(alias = "CHG_IN")]
    pub fn set_chg_in(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `mchg` field.
    ///
    #[doc(alias = "MCHG")]
    pub fn set_mchg(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `vct` field.
    ///
    #[doc(alias = "VCT")]
    pub fn set_vct(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `ccr` field.
    ///
    #[doc(alias = "CCR")]
    pub fn set_ccr(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 16` - Set the `cvr` field.
    ///
    #[doc(alias = "CVR")]
    pub fn set_cvr(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 17` - Set the `ccc` field.
    ///
    #[doc(alias = "CCC")]
    pub fn set_ccc(&mut self, value: bool) {
        let start = 17;
        let end = 17;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacChargingStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacChargingStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacChargingStatus> for [u8; 4] {
    fn from(val: MacChargingStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacChargingStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacChargingStatus");
        d.field("ut", &self.ut());
        d.field("lt", &self.lt());
        d.field("stl", &self.stl());
        d.field("rt", &self.rt());
        d.field("sth", &self.sth());
        d.field("ht", &self.ht());
        d.field("ot", &self.ot());
        d.field("pv", &self.pv());
        d.field("lv", &self.lv());
        d.field("mv", &self.mv());
        d.field("hv", &self.hv());
        d.field("chg_in", &self.chg_in());
        d.field("mchg", &self.mchg());
        d.field("vct", &self.vct());
        d.field("ccr", &self.ccr());
        d.field("cvr", &self.cvr());
        d.field("ccc", &self.ccc());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacChargingStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacChargingStatus {{ ");
        defmt::write!(f, "ut: {=bool}, ", &self.ut());
        defmt::write!(f, "lt: {=bool}, ", &self.lt());
        defmt::write!(f, "stl: {=bool}, ", &self.stl());
        defmt::write!(f, "rt: {=bool}, ", &self.rt());
        defmt::write!(f, "sth: {=bool}, ", &self.sth());
        defmt::write!(f, "ht: {=bool}, ", &self.ht());
        defmt::write!(f, "ot: {=bool}, ", &self.ot());
        defmt::write!(f, "pv: {=bool}, ", &self.pv());
        defmt::write!(f, "lv: {=bool}, ", &self.lv());
        defmt::write!(f, "mv: {=bool}, ", &self.mv());
        defmt::write!(f, "hv: {=bool}, ", &self.hv());
        defmt::write!(f, "chg_in: {=bool}, ", &self.chg_in());
        defmt::write!(f, "mchg: {=bool}, ", &self.mchg());
        defmt::write!(f, "vct: {=bool}, ", &self.vct());
        defmt::write!(f, "ccr: {=bool}, ", &self.ccr());
        defmt::write!(f, "cvr: {=bool}, ", &self.cvr());
        defmt::write!(f, "ccc: {=bool}, ", &self.ccc());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacChargingStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacChargingStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacChargingStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacChargingStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacChargingStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacChargingStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacChargingStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_OPERATION_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacOperationStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacOperationStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacOperationStatus {
    /// `bit 0` - Read the `pres` field.
    ///
    #[doc(alias = "PRES")]
    #[must_use]
    pub fn pres(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `dsg` field.
    ///
    #[doc(alias = "DSG")]
    #[must_use]
    pub fn dsg(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `chg` field.
    ///
    #[doc(alias = "CHG")]
    #[must_use]
    pub fn chg(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `pchg` field.
    ///
    #[doc(alias = "PCHG")]
    #[must_use]
    pub fn pchg(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    #[must_use]
    pub fn fuse(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `btp_int` field.
    ///
    #[doc(alias = "BTP_INT")]
    #[must_use]
    pub fn btp_int(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `9:8` - Read the `sec` field.
    ///
    #[doc(alias = "SEC")]
    #[must_use]
    pub fn sec(&self) -> MacSecurityMode {
        let start = 8;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        unsafe { raw.try_into().unwrap_unchecked() }
    }
    /// `bit 10` - Read the `sdv` field.
    ///
    #[doc(alias = "SDV")]
    #[must_use]
    pub fn sdv(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `ss` field.
    ///
    #[doc(alias = "SS")]
    #[must_use]
    pub fn ss(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `pf` field.
    ///
    #[doc(alias = "PF")]
    #[must_use]
    pub fn pf(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `xdsg` field.
    ///
    #[doc(alias = "XDSG")]
    #[must_use]
    pub fn xdsg(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `xchg` field.
    ///
    #[doc(alias = "XCHG")]
    #[must_use]
    pub fn xchg(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 15` - Read the `sleep` field.
    ///
    #[doc(alias = "SLEEP")]
    #[must_use]
    pub fn sleep(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `sdm` field.
    ///
    #[doc(alias = "SDM")]
    #[must_use]
    pub fn sdm(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `led` field.
    ///
    #[doc(alias = "LED")]
    #[must_use]
    pub fn led(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `auth` field.
    ///
    #[doc(alias = "AUTH")]
    #[must_use]
    pub fn auth(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `autocalm` field.
    ///
    #[doc(alias = "AUTOCALM")]
    #[must_use]
    pub fn autocalm(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `cal` field.
    ///
    #[doc(alias = "CAL")]
    #[must_use]
    pub fn cal(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `cal_offset` field.
    ///
    #[doc(alias = "CAL_OFFSET")]
    #[must_use]
    pub fn cal_offset(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `xl` field.
    ///
    #[doc(alias = "XL")]
    #[must_use]
    pub fn xl(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `sleepm` field.
    ///
    #[doc(alias = "SLEEPM")]
    #[must_use]
    pub fn sleepm(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `init` field.
    ///
    #[doc(alias = "INIT")]
    #[must_use]
    pub fn init(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `smblcal` field.
    ///
    #[doc(alias = "SMBLCAL")]
    #[must_use]
    pub fn smblcal(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `slpad` field.
    ///
    #[doc(alias = "SLPAD")]
    #[must_use]
    pub fn slpad(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 27` - Read the `slpcc` field.
    ///
    #[doc(alias = "SLPCC")]
    #[must_use]
    pub fn slpcc(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 28` - Read the `cb` field.
    ///
    #[doc(alias = "CB")]
    #[must_use]
    pub fn cb(&self) -> bool {
        let start = 28;
        let end = 28;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 29` - Read the `emshut` field.
    ///
    #[doc(alias = "EMSHUT")]
    #[must_use]
    pub fn emshut(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `pres` field.
    ///
    #[doc(alias = "PRES")]
    pub fn set_pres(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `dsg` field.
    ///
    #[doc(alias = "DSG")]
    pub fn set_dsg(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `chg` field.
    ///
    #[doc(alias = "CHG")]
    pub fn set_chg(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `pchg` field.
    ///
    #[doc(alias = "PCHG")]
    pub fn set_pchg(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    pub fn set_fuse(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `btp_int` field.
    ///
    #[doc(alias = "BTP_INT")]
    pub fn set_btp_int(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `9:8` - Set the `sec` field.
    ///
    #[doc(alias = "SEC")]
    pub fn set_sec(&mut self, value: MacSecurityMode) {
        let start = 8;
        let end = 9;
        let raw = value.into();
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `sdv` field.
    ///
    #[doc(alias = "SDV")]
    pub fn set_sdv(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `ss` field.
    ///
    #[doc(alias = "SS")]
    pub fn set_ss(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `pf` field.
    ///
    #[doc(alias = "PF")]
    pub fn set_pf(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `xdsg` field.
    ///
    #[doc(alias = "XDSG")]
    pub fn set_xdsg(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `xchg` field.
    ///
    #[doc(alias = "XCHG")]
    pub fn set_xchg(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `sleep` field.
    ///
    #[doc(alias = "SLEEP")]
    pub fn set_sleep(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 16` - Set the `sdm` field.
    ///
    #[doc(alias = "SDM")]
    pub fn set_sdm(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 17` - Set the `led` field.
    ///
    #[doc(alias = "LED")]
    pub fn set_led(&mut self, value: bool) {
        let start = 17;
        let end = 17;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 18` - Set the `auth` field.
    ///
    #[doc(alias = "AUTH")]
    pub fn set_auth(&mut self, value: bool) {
        let start = 18;
        let end = 18;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 19` - Set the `autocalm` field.
    ///
    #[doc(alias = "AUTOCALM")]
    pub fn set_autocalm(&mut self, value: bool) {
        let start = 19;
        let end = 19;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 20` - Set the `cal` field.
    ///
    #[doc(alias = "CAL")]
    pub fn set_cal(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 21` - Set the `cal_offset` field.
    ///
    #[doc(alias = "CAL_OFFSET")]
    pub fn set_cal_offset(&mut self, value: bool) {
        let start = 21;
        let end = 21;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 22` - Set the `xl` field.
    ///
    #[doc(alias = "XL")]
    pub fn set_xl(&mut self, value: bool) {
        let start = 22;
        let end = 22;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 23` - Set the `sleepm` field.
    ///
    #[doc(alias = "SLEEPM")]
    pub fn set_sleepm(&mut self, value: bool) {
        let start = 23;
        let end = 23;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 24` - Set the `init` field.
    ///
    #[doc(alias = "INIT")]
    pub fn set_init(&mut self, value: bool) {
        let start = 24;
        let end = 24;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 25` - Set the `smblcal` field.
    ///
    #[doc(alias = "SMBLCAL")]
    pub fn set_smblcal(&mut self, value: bool) {
        let start = 25;
        let end = 25;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 26` - Set the `slpad` field.
    ///
    #[doc(alias = "SLPAD")]
    pub fn set_slpad(&mut self, value: bool) {
        let start = 26;
        let end = 26;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 27` - Set the `slpcc` field.
    ///
    #[doc(alias = "SLPCC")]
    pub fn set_slpcc(&mut self, value: bool) {
        let start = 27;
        let end = 27;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 28` - Set the `cb` field.
    ///
    #[doc(alias = "CB")]
    pub fn set_cb(&mut self, value: bool) {
        let start = 28;
        let end = 28;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 29` - Set the `emshut` field.
    ///
    #[doc(alias = "EMSHUT")]
    pub fn set_emshut(&mut self, value: bool) {
        let start = 29;
        let end = 29;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacOperationStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacOperationStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacOperationStatus> for [u8; 4] {
    fn from(val: MacOperationStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacOperationStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacOperationStatus");
        d.field("pres", &self.pres());
        d.field("dsg", &self.dsg());
        d.field("chg", &self.chg());
        d.field("pchg", &self.pchg());
        d.field("fuse", &self.fuse());
        d.field("btp_int", &self.btp_int());
        d.field("sec", &self.sec());
        d.field("sdv", &self.sdv());
        d.field("ss", &self.ss());
        d.field("pf", &self.pf());
        d.field("xdsg", &self.xdsg());
        d.field("xchg", &self.xchg());
        d.field("sleep", &self.sleep());
        d.field("sdm", &self.sdm());
        d.field("led", &self.led());
        d.field("auth", &self.auth());
        d.field("autocalm", &self.autocalm());
        d.field("cal", &self.cal());
        d.field("cal_offset", &self.cal_offset());
        d.field("xl", &self.xl());
        d.field("sleepm", &self.sleepm());
        d.field("init", &self.init());
        d.field("smblcal", &self.smblcal());
        d.field("slpad", &self.slpad());
        d.field("slpcc", &self.slpcc());
        d.field("cb", &self.cb());
        d.field("emshut", &self.emshut());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacOperationStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacOperationStatus {{ ");
        defmt::write!(f, "pres: {=bool}, ", &self.pres());
        defmt::write!(f, "dsg: {=bool}, ", &self.dsg());
        defmt::write!(f, "chg: {=bool}, ", &self.chg());
        defmt::write!(f, "pchg: {=bool}, ", &self.pchg());
        defmt::write!(f, "fuse: {=bool}, ", &self.fuse());
        defmt::write!(f, "btp_int: {=bool}, ", &self.btp_int());
        defmt::write!(f, "sec: {}, ", &self.sec());
        defmt::write!(f, "sdv: {=bool}, ", &self.sdv());
        defmt::write!(f, "ss: {=bool}, ", &self.ss());
        defmt::write!(f, "pf: {=bool}, ", &self.pf());
        defmt::write!(f, "xdsg: {=bool}, ", &self.xdsg());
        defmt::write!(f, "xchg: {=bool}, ", &self.xchg());
        defmt::write!(f, "sleep: {=bool}, ", &self.sleep());
        defmt::write!(f, "sdm: {=bool}, ", &self.sdm());
        defmt::write!(f, "led: {=bool}, ", &self.led());
        defmt::write!(f, "auth: {=bool}, ", &self.auth());
        defmt::write!(f, "autocalm: {=bool}, ", &self.autocalm());
        defmt::write!(f, "cal: {=bool}, ", &self.cal());
        defmt::write!(f, "cal_offset: {=bool}, ", &self.cal_offset());
        defmt::write!(f, "xl: {=bool}, ", &self.xl());
        defmt::write!(f, "sleepm: {=bool}, ", &self.sleepm());
        defmt::write!(f, "init: {=bool}, ", &self.init());
        defmt::write!(f, "smblcal: {=bool}, ", &self.smblcal());
        defmt::write!(f, "slpad: {=bool}, ", &self.slpad());
        defmt::write!(f, "slpcc: {=bool}, ", &self.slpcc());
        defmt::write!(f, "cb: {=bool}, ", &self.cb());
        defmt::write!(f, "emshut: {=bool}, ", &self.emshut());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacOperationStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacOperationStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacOperationStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacOperationStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacOperationStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacOperationStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacOperationStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_PF_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacPfStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacPfStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacPfStatus {
    /// `bit 0` - Read the `suv` field.
    ///
    #[doc(alias = "SUV")]
    #[must_use]
    pub fn suv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `sov` field.
    ///
    #[doc(alias = "SOV")]
    #[must_use]
    pub fn sov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `socc` field.
    ///
    #[doc(alias = "SOCC")]
    #[must_use]
    pub fn socc(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `socd` field.
    ///
    #[doc(alias = "SOCD")]
    #[must_use]
    pub fn socd(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `sot` field.
    ///
    #[doc(alias = "SOT")]
    #[must_use]
    pub fn sot(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `sotf` field.
    ///
    #[doc(alias = "SOTF")]
    #[must_use]
    pub fn sotf(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `qim` field.
    ///
    #[doc(alias = "QIM")]
    #[must_use]
    pub fn qim(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `cb` field.
    ///
    #[doc(alias = "CB")]
    #[must_use]
    pub fn cb(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `imp` field.
    ///
    #[doc(alias = "IMP")]
    #[must_use]
    pub fn imp(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `cd` field.
    ///
    #[doc(alias = "CD")]
    #[must_use]
    pub fn cd(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `vimr` field.
    ///
    #[doc(alias = "VIMR")]
    #[must_use]
    pub fn vimr(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `vima` field.
    ///
    #[doc(alias = "VIMA")]
    #[must_use]
    pub fn vima(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `cfetf` field.
    ///
    #[doc(alias = "CFETF")]
    #[must_use]
    pub fn cfetf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `dfetf` field.
    ///
    #[doc(alias = "DFETF")]
    #[must_use]
    pub fn dfetf(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    #[must_use]
    pub fn fuse(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `afer` field.
    ///
    #[doc(alias = "AFER")]
    #[must_use]
    pub fn afer(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `afec` field.
    ///
    #[doc(alias = "AFEC")]
    #[must_use]
    pub fn afec(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `second_lvl` field.
    ///
    #[doc(alias = "SECOND_LVL")]
    #[must_use]
    pub fn second_lvl(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `ptc` field.
    ///
    #[doc(alias = "PTC")]
    #[must_use]
    pub fn ptc(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `ifc` field.
    ///
    #[doc(alias = "IFC")]
    #[must_use]
    pub fn ifc(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `opncell` field.
    ///
    #[doc(alias = "OPNCELL")]
    #[must_use]
    pub fn opncell(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `dfw` field.
    ///
    #[doc(alias = "DFW")]
    #[must_use]
    pub fn dfw(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 28` - Read the `ts_1` field.
    ///
    #[doc(alias = "TS1")]
    #[must_use]
    pub fn ts_1(&self) -> bool {
        let start = 28;
        let end = 28;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 29` - Read the `ts_2` field.
    ///
    #[doc(alias = "TS2")]
    #[must_use]
    pub fn ts_2(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 30` - Read the `ts_3` field.
    ///
    #[doc(alias = "TS3")]
    #[must_use]
    pub fn ts_3(&self) -> bool {
        let start = 30;
        let end = 30;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 31` - Read the `ts_4` field.
    ///
    #[doc(alias = "TS4")]
    #[must_use]
    pub fn ts_4(&self) -> bool {
        let start = 31;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `suv` field.
    ///
    #[doc(alias = "SUV")]
    pub fn set_suv(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `sov` field.
    ///
    #[doc(alias = "SOV")]
    pub fn set_sov(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `socc` field.
    ///
    #[doc(alias = "SOCC")]
    pub fn set_socc(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `socd` field.
    ///
    #[doc(alias = "SOCD")]
    pub fn set_socd(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `sot` field.
    ///
    #[doc(alias = "SOT")]
    pub fn set_sot(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `sotf` field.
    ///
    #[doc(alias = "SOTF")]
    pub fn set_sotf(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `qim` field.
    ///
    #[doc(alias = "QIM")]
    pub fn set_qim(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `cb` field.
    ///
    #[doc(alias = "CB")]
    pub fn set_cb(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `imp` field.
    ///
    #[doc(alias = "IMP")]
    pub fn set_imp(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `cd` field.
    ///
    #[doc(alias = "CD")]
    pub fn set_cd(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `vimr` field.
    ///
    #[doc(alias = "VIMR")]
    pub fn set_vimr(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `vima` field.
    ///
    #[doc(alias = "VIMA")]
    pub fn set_vima(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 16` - Set the `cfetf` field.
    ///
    #[doc(alias = "CFETF")]
    pub fn set_cfetf(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 17` - Set the `dfetf` field.
    ///
    #[doc(alias = "DFETF")]
    pub fn set_dfetf(&mut self, value: bool) {
        let start = 17;
        let end = 17;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 19` - Set the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    pub fn set_fuse(&mut self, value: bool) {
        let start = 19;
        let end = 19;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 20` - Set the `afer` field.
    ///
    #[doc(alias = "AFER")]
    pub fn set_afer(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 21` - Set the `afec` field.
    ///
    #[doc(alias = "AFEC")]
    pub fn set_afec(&mut self, value: bool) {
        let start = 21;
        let end = 21;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 22` - Set the `second_lvl` field.
    ///
    #[doc(alias = "SECOND_LVL")]
    pub fn set_second_lvl(&mut self, value: bool) {
        let start = 22;
        let end = 22;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 23` - Set the `ptc` field.
    ///
    #[doc(alias = "PTC")]
    pub fn set_ptc(&mut self, value: bool) {
        let start = 23;
        let end = 23;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 24` - Set the `ifc` field.
    ///
    #[doc(alias = "IFC")]
    pub fn set_ifc(&mut self, value: bool) {
        let start = 24;
        let end = 24;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 25` - Set the `opncell` field.
    ///
    #[doc(alias = "OPNCELL")]
    pub fn set_opncell(&mut self, value: bool) {
        let start = 25;
        let end = 25;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 26` - Set the `dfw` field.
    ///
    #[doc(alias = "DFW")]
    pub fn set_dfw(&mut self, value: bool) {
        let start = 26;
        let end = 26;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 28` - Set the `ts_1` field.
    ///
    #[doc(alias = "TS1")]
    pub fn set_ts_1(&mut self, value: bool) {
        let start = 28;
        let end = 28;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 29` - Set the `ts_2` field.
    ///
    #[doc(alias = "TS2")]
    pub fn set_ts_2(&mut self, value: bool) {
        let start = 29;
        let end = 29;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 30` - Set the `ts_3` field.
    ///
    #[doc(alias = "TS3")]
    pub fn set_ts_3(&mut self, value: bool) {
        let start = 30;
        let end = 30;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 31` - Set the `ts_4` field.
    ///
    #[doc(alias = "TS4")]
    pub fn set_ts_4(&mut self, value: bool) {
        let start = 31;
        let end = 31;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacPfStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacPfStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacPfStatus> for [u8; 4] {
    fn from(val: MacPfStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacPfStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacPfStatus");
        d.field("suv", &self.suv());
        d.field("sov", &self.sov());
        d.field("socc", &self.socc());
        d.field("socd", &self.socd());
        d.field("sot", &self.sot());
        d.field("sotf", &self.sotf());
        d.field("qim", &self.qim());
        d.field("cb", &self.cb());
        d.field("imp", &self.imp());
        d.field("cd", &self.cd());
        d.field("vimr", &self.vimr());
        d.field("vima", &self.vima());
        d.field("cfetf", &self.cfetf());
        d.field("dfetf", &self.dfetf());
        d.field("fuse", &self.fuse());
        d.field("afer", &self.afer());
        d.field("afec", &self.afec());
        d.field("second_lvl", &self.second_lvl());
        d.field("ptc", &self.ptc());
        d.field("ifc", &self.ifc());
        d.field("opncell", &self.opncell());
        d.field("dfw", &self.dfw());
        d.field("ts_1", &self.ts_1());
        d.field("ts_2", &self.ts_2());
        d.field("ts_3", &self.ts_3());
        d.field("ts_4", &self.ts_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacPfStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacPfStatus {{ ");
        defmt::write!(f, "suv: {=bool}, ", &self.suv());
        defmt::write!(f, "sov: {=bool}, ", &self.sov());
        defmt::write!(f, "socc: {=bool}, ", &self.socc());
        defmt::write!(f, "socd: {=bool}, ", &self.socd());
        defmt::write!(f, "sot: {=bool}, ", &self.sot());
        defmt::write!(f, "sotf: {=bool}, ", &self.sotf());
        defmt::write!(f, "qim: {=bool}, ", &self.qim());
        defmt::write!(f, "cb: {=bool}, ", &self.cb());
        defmt::write!(f, "imp: {=bool}, ", &self.imp());
        defmt::write!(f, "cd: {=bool}, ", &self.cd());
        defmt::write!(f, "vimr: {=bool}, ", &self.vimr());
        defmt::write!(f, "vima: {=bool}, ", &self.vima());
        defmt::write!(f, "cfetf: {=bool}, ", &self.cfetf());
        defmt::write!(f, "dfetf: {=bool}, ", &self.dfetf());
        defmt::write!(f, "fuse: {=bool}, ", &self.fuse());
        defmt::write!(f, "afer: {=bool}, ", &self.afer());
        defmt::write!(f, "afec: {=bool}, ", &self.afec());
        defmt::write!(f, "second_lvl: {=bool}, ", &self.second_lvl());
        defmt::write!(f, "ptc: {=bool}, ", &self.ptc());
        defmt::write!(f, "ifc: {=bool}, ", &self.ifc());
        defmt::write!(f, "opncell: {=bool}, ", &self.opncell());
        defmt::write!(f, "dfw: {=bool}, ", &self.dfw());
        defmt::write!(f, "ts_1: {=bool}, ", &self.ts_1());
        defmt::write!(f, "ts_2: {=bool}, ", &self.ts_2());
        defmt::write!(f, "ts_3: {=bool}, ", &self.ts_3());
        defmt::write!(f, "ts_4: {=bool}, ", &self.ts_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacPfStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacPfStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacPfStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacPfStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacPfStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacPfStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacPfStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_PF_ALERT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacPfAlert {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacPfAlert {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacPfAlert {
    /// `bit 0` - Read the `suv` field.
    ///
    #[doc(alias = "SUV")]
    #[must_use]
    pub fn suv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `sov` field.
    ///
    #[doc(alias = "SOV")]
    #[must_use]
    pub fn sov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `socc` field.
    ///
    #[doc(alias = "SOCC")]
    #[must_use]
    pub fn socc(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `socd` field.
    ///
    #[doc(alias = "SOCD")]
    #[must_use]
    pub fn socd(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `sot` field.
    ///
    #[doc(alias = "SOT")]
    #[must_use]
    pub fn sot(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `sotf` field.
    ///
    #[doc(alias = "SOTF")]
    #[must_use]
    pub fn sotf(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `qim` field.
    ///
    #[doc(alias = "QIM")]
    #[must_use]
    pub fn qim(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `cb` field.
    ///
    #[doc(alias = "CB")]
    #[must_use]
    pub fn cb(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `imp` field.
    ///
    #[doc(alias = "IMP")]
    #[must_use]
    pub fn imp(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `cd` field.
    ///
    #[doc(alias = "CD")]
    #[must_use]
    pub fn cd(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `vimr` field.
    ///
    #[doc(alias = "VIMR")]
    #[must_use]
    pub fn vimr(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `vima` field.
    ///
    #[doc(alias = "VIMA")]
    #[must_use]
    pub fn vima(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `cfetf` field.
    ///
    #[doc(alias = "CFETF")]
    #[must_use]
    pub fn cfetf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 17` - Read the `dfetf` field.
    ///
    #[doc(alias = "DFETF")]
    #[must_use]
    pub fn dfetf(&self) -> bool {
        let start = 17;
        let end = 17;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    #[must_use]
    pub fn fuse(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `afer` field.
    ///
    #[doc(alias = "AFER")]
    #[must_use]
    pub fn afer(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `afec` field.
    ///
    #[doc(alias = "AFEC")]
    #[must_use]
    pub fn afec(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `second_lvl` field.
    ///
    #[doc(alias = "SECOND_LVL")]
    #[must_use]
    pub fn second_lvl(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `opnc` field.
    ///
    #[doc(alias = "OPNC")]
    #[must_use]
    pub fn opnc(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 28` - Read the `ts_1` field.
    ///
    #[doc(alias = "TS1")]
    #[must_use]
    pub fn ts_1(&self) -> bool {
        let start = 28;
        let end = 28;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 29` - Read the `ts_2` field.
    ///
    #[doc(alias = "TS2")]
    #[must_use]
    pub fn ts_2(&self) -> bool {
        let start = 29;
        let end = 29;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 30` - Read the `ts_3` field.
    ///
    #[doc(alias = "TS3")]
    #[must_use]
    pub fn ts_3(&self) -> bool {
        let start = 30;
        let end = 30;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 31` - Read the `ts_4` field.
    ///
    #[doc(alias = "TS4")]
    #[must_use]
    pub fn ts_4(&self) -> bool {
        let start = 31;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `suv` field.
    ///
    #[doc(alias = "SUV")]
    pub fn set_suv(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `sov` field.
    ///
    #[doc(alias = "SOV")]
    pub fn set_sov(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `socc` field.
    ///
    #[doc(alias = "SOCC")]
    pub fn set_socc(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `socd` field.
    ///
    #[doc(alias = "SOCD")]
    pub fn set_socd(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `sot` field.
    ///
    #[doc(alias = "SOT")]
    pub fn set_sot(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `sotf` field.
    ///
    #[doc(alias = "SOTF")]
    pub fn set_sotf(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `qim` field.
    ///
    #[doc(alias = "QIM")]
    pub fn set_qim(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `cb` field.
    ///
    #[doc(alias = "CB")]
    pub fn set_cb(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `imp` field.
    ///
    #[doc(alias = "IMP")]
    pub fn set_imp(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `cd` field.
    ///
    #[doc(alias = "CD")]
    pub fn set_cd(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `vimr` field.
    ///
    #[doc(alias = "VIMR")]
    pub fn set_vimr(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `vima` field.
    ///
    #[doc(alias = "VIMA")]
    pub fn set_vima(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 16` - Set the `cfetf` field.
    ///
    #[doc(alias = "CFETF")]
    pub fn set_cfetf(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 17` - Set the `dfetf` field.
    ///
    #[doc(alias = "DFETF")]
    pub fn set_dfetf(&mut self, value: bool) {
        let start = 17;
        let end = 17;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 19` - Set the `fuse` field.
    ///
    #[doc(alias = "FUSE")]
    pub fn set_fuse(&mut self, value: bool) {
        let start = 19;
        let end = 19;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 20` - Set the `afer` field.
    ///
    #[doc(alias = "AFER")]
    pub fn set_afer(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 21` - Set the `afec` field.
    ///
    #[doc(alias = "AFEC")]
    pub fn set_afec(&mut self, value: bool) {
        let start = 21;
        let end = 21;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 22` - Set the `second_lvl` field.
    ///
    #[doc(alias = "SECOND_LVL")]
    pub fn set_second_lvl(&mut self, value: bool) {
        let start = 22;
        let end = 22;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 25` - Set the `opnc` field.
    ///
    #[doc(alias = "OPNC")]
    pub fn set_opnc(&mut self, value: bool) {
        let start = 25;
        let end = 25;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 28` - Set the `ts_1` field.
    ///
    #[doc(alias = "TS1")]
    pub fn set_ts_1(&mut self, value: bool) {
        let start = 28;
        let end = 28;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 29` - Set the `ts_2` field.
    ///
    #[doc(alias = "TS2")]
    pub fn set_ts_2(&mut self, value: bool) {
        let start = 29;
        let end = 29;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 30` - Set the `ts_3` field.
    ///
    #[doc(alias = "TS3")]
    pub fn set_ts_3(&mut self, value: bool) {
        let start = 30;
        let end = 30;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 31` - Set the `ts_4` field.
    ///
    #[doc(alias = "TS4")]
    pub fn set_ts_4(&mut self, value: bool) {
        let start = 31;
        let end = 31;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacPfAlert {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacPfAlert {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacPfAlert> for [u8; 4] {
    fn from(val: MacPfAlert) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacPfAlert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacPfAlert");
        d.field("suv", &self.suv());
        d.field("sov", &self.sov());
        d.field("socc", &self.socc());
        d.field("socd", &self.socd());
        d.field("sot", &self.sot());
        d.field("sotf", &self.sotf());
        d.field("qim", &self.qim());
        d.field("cb", &self.cb());
        d.field("imp", &self.imp());
        d.field("cd", &self.cd());
        d.field("vimr", &self.vimr());
        d.field("vima", &self.vima());
        d.field("cfetf", &self.cfetf());
        d.field("dfetf", &self.dfetf());
        d.field("fuse", &self.fuse());
        d.field("afer", &self.afer());
        d.field("afec", &self.afec());
        d.field("second_lvl", &self.second_lvl());
        d.field("opnc", &self.opnc());
        d.field("ts_1", &self.ts_1());
        d.field("ts_2", &self.ts_2());
        d.field("ts_3", &self.ts_3());
        d.field("ts_4", &self.ts_4());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacPfAlert {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacPfAlert {{ ");
        defmt::write!(f, "suv: {=bool}, ", &self.suv());
        defmt::write!(f, "sov: {=bool}, ", &self.sov());
        defmt::write!(f, "socc: {=bool}, ", &self.socc());
        defmt::write!(f, "socd: {=bool}, ", &self.socd());
        defmt::write!(f, "sot: {=bool}, ", &self.sot());
        defmt::write!(f, "sotf: {=bool}, ", &self.sotf());
        defmt::write!(f, "qim: {=bool}, ", &self.qim());
        defmt::write!(f, "cb: {=bool}, ", &self.cb());
        defmt::write!(f, "imp: {=bool}, ", &self.imp());
        defmt::write!(f, "cd: {=bool}, ", &self.cd());
        defmt::write!(f, "vimr: {=bool}, ", &self.vimr());
        defmt::write!(f, "vima: {=bool}, ", &self.vima());
        defmt::write!(f, "cfetf: {=bool}, ", &self.cfetf());
        defmt::write!(f, "dfetf: {=bool}, ", &self.dfetf());
        defmt::write!(f, "fuse: {=bool}, ", &self.fuse());
        defmt::write!(f, "afer: {=bool}, ", &self.afer());
        defmt::write!(f, "afec: {=bool}, ", &self.afec());
        defmt::write!(f, "second_lvl: {=bool}, ", &self.second_lvl());
        defmt::write!(f, "opnc: {=bool}, ", &self.opnc());
        defmt::write!(f, "ts_1: {=bool}, ", &self.ts_1());
        defmt::write!(f, "ts_2: {=bool}, ", &self.ts_2());
        defmt::write!(f, "ts_3: {=bool}, ", &self.ts_3());
        defmt::write!(f, "ts_4: {=bool}, ", &self.ts_4());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacPfAlert {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacPfAlert {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacPfAlert {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacPfAlert {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacPfAlert {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacPfAlert {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacPfAlert {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_SAFETY_STATUS")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacSafetyStatus {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacSafetyStatus {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacSafetyStatus {
    /// `bit 0` - Read the `cuv` field.
    ///
    #[doc(alias = "CUV")]
    #[must_use]
    pub fn cuv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `cov` field.
    ///
    #[doc(alias = "COV")]
    #[must_use]
    pub fn cov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `occ_1` field.
    ///
    #[doc(alias = "OCC1")]
    #[must_use]
    pub fn occ_1(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `occ_2` field.
    ///
    #[doc(alias = "OCC2")]
    #[must_use]
    pub fn occ_2(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `ocd_1` field.
    ///
    #[doc(alias = "OCD1")]
    #[must_use]
    pub fn ocd_1(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `ocd_2` field.
    ///
    #[doc(alias = "OCD2")]
    #[must_use]
    pub fn ocd_2(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 6` - Read the `aold` field.
    ///
    #[doc(alias = "AOLD")]
    #[must_use]
    pub fn aold(&self) -> bool {
        let start = 6;
        let end = 6;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `aoldl` field.
    ///
    #[doc(alias = "AOLDL")]
    #[must_use]
    pub fn aoldl(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 8` - Read the `ascc` field.
    ///
    #[doc(alias = "ASCC")]
    #[must_use]
    pub fn ascc(&self) -> bool {
        let start = 8;
        let end = 8;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `asccl` field.
    ///
    #[doc(alias = "ASCCL")]
    #[must_use]
    pub fn asccl(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 10` - Read the `ascd` field.
    ///
    #[doc(alias = "ASCD")]
    #[must_use]
    pub fn ascd(&self) -> bool {
        let start = 10;
        let end = 10;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `ascdl` field.
    ///
    #[doc(alias = "ASCDL")]
    #[must_use]
    pub fn ascdl(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `otc` field.
    ///
    #[doc(alias = "OTC")]
    #[must_use]
    pub fn otc(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `otd` field.
    ///
    #[doc(alias = "OTD")]
    #[must_use]
    pub fn otd(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `cuvc` field.
    ///
    #[doc(alias = "CUVC")]
    #[must_use]
    pub fn cuvc(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `otf` field.
    ///
    #[doc(alias = "OTF")]
    #[must_use]
    pub fn otf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `pto` field.
    ///
    #[doc(alias = "PTO")]
    #[must_use]
    pub fn pto(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `ptos` field.
    ///
    #[doc(alias = "PTOS")]
    #[must_use]
    pub fn ptos(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `cto` field.
    ///
    #[doc(alias = "CTO")]
    #[must_use]
    pub fn cto(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `oc` field.
    ///
    #[doc(alias = "OC")]
    #[must_use]
    pub fn oc(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `chgc` field.
    ///
    #[doc(alias = "CHGC")]
    #[must_use]
    pub fn chgc(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `chgv` field.
    ///
    #[doc(alias = "CHGV")]
    #[must_use]
    pub fn chgv(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `pchgc` field.
    ///
    #[doc(alias = "PCHGC")]
    #[must_use]
    pub fn pchgc(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `utc` field.
    ///
    #[doc(alias = "UTC")]
    #[must_use]
    pub fn utc(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 27` - Read the `utd` field.
    ///
    #[doc(alias = "UTD")]
    #[must_use]
    pub fn utd(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `cuv` field.
    ///
    #[doc(alias = "CUV")]
    pub fn set_cuv(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `cov` field.
    ///
    #[doc(alias = "COV")]
    pub fn set_cov(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `occ_1` field.
    ///
    #[doc(alias = "OCC1")]
    pub fn set_occ_1(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `occ_2` field.
    ///
    #[doc(alias = "OCC2")]
    pub fn set_occ_2(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `ocd_1` field.
    ///
    #[doc(alias = "OCD1")]
    pub fn set_ocd_1(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `ocd_2` field.
    ///
    #[doc(alias = "OCD2")]
    pub fn set_ocd_2(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 6` - Set the `aold` field.
    ///
    #[doc(alias = "AOLD")]
    pub fn set_aold(&mut self, value: bool) {
        let start = 6;
        let end = 6;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `aoldl` field.
    ///
    #[doc(alias = "AOLDL")]
    pub fn set_aoldl(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 8` - Set the `ascc` field.
    ///
    #[doc(alias = "ASCC")]
    pub fn set_ascc(&mut self, value: bool) {
        let start = 8;
        let end = 8;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `asccl` field.
    ///
    #[doc(alias = "ASCCL")]
    pub fn set_asccl(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 10` - Set the `ascd` field.
    ///
    #[doc(alias = "ASCD")]
    pub fn set_ascd(&mut self, value: bool) {
        let start = 10;
        let end = 10;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `ascdl` field.
    ///
    #[doc(alias = "ASCDL")]
    pub fn set_ascdl(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `otc` field.
    ///
    #[doc(alias = "OTC")]
    pub fn set_otc(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `otd` field.
    ///
    #[doc(alias = "OTD")]
    pub fn set_otd(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `cuvc` field.
    ///
    #[doc(alias = "CUVC")]
    pub fn set_cuvc(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 16` - Set the `otf` field.
    ///
    #[doc(alias = "OTF")]
    pub fn set_otf(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 18` - Set the `pto` field.
    ///
    #[doc(alias = "PTO")]
    pub fn set_pto(&mut self, value: bool) {
        let start = 18;
        let end = 18;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 19` - Set the `ptos` field.
    ///
    #[doc(alias = "PTOS")]
    pub fn set_ptos(&mut self, value: bool) {
        let start = 19;
        let end = 19;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 20` - Set the `cto` field.
    ///
    #[doc(alias = "CTO")]
    pub fn set_cto(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 22` - Set the `oc` field.
    ///
    #[doc(alias = "OC")]
    pub fn set_oc(&mut self, value: bool) {
        let start = 22;
        let end = 22;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 23` - Set the `chgc` field.
    ///
    #[doc(alias = "CHGC")]
    pub fn set_chgc(&mut self, value: bool) {
        let start = 23;
        let end = 23;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 24` - Set the `chgv` field.
    ///
    #[doc(alias = "CHGV")]
    pub fn set_chgv(&mut self, value: bool) {
        let start = 24;
        let end = 24;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 25` - Set the `pchgc` field.
    ///
    #[doc(alias = "PCHGC")]
    pub fn set_pchgc(&mut self, value: bool) {
        let start = 25;
        let end = 25;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 26` - Set the `utc` field.
    ///
    #[doc(alias = "UTC")]
    pub fn set_utc(&mut self, value: bool) {
        let start = 26;
        let end = 26;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 27` - Set the `utd` field.
    ///
    #[doc(alias = "UTD")]
    pub fn set_utd(&mut self, value: bool) {
        let start = 27;
        let end = 27;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacSafetyStatus {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacSafetyStatus {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacSafetyStatus> for [u8; 4] {
    fn from(val: MacSafetyStatus) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacSafetyStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacSafetyStatus");
        d.field("cuv", &self.cuv());
        d.field("cov", &self.cov());
        d.field("occ_1", &self.occ_1());
        d.field("occ_2", &self.occ_2());
        d.field("ocd_1", &self.ocd_1());
        d.field("ocd_2", &self.ocd_2());
        d.field("aold", &self.aold());
        d.field("aoldl", &self.aoldl());
        d.field("ascc", &self.ascc());
        d.field("asccl", &self.asccl());
        d.field("ascd", &self.ascd());
        d.field("ascdl", &self.ascdl());
        d.field("otc", &self.otc());
        d.field("otd", &self.otd());
        d.field("cuvc", &self.cuvc());
        d.field("otf", &self.otf());
        d.field("pto", &self.pto());
        d.field("ptos", &self.ptos());
        d.field("cto", &self.cto());
        d.field("oc", &self.oc());
        d.field("chgc", &self.chgc());
        d.field("chgv", &self.chgv());
        d.field("pchgc", &self.pchgc());
        d.field("utc", &self.utc());
        d.field("utd", &self.utd());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacSafetyStatus {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacSafetyStatus {{ ");
        defmt::write!(f, "cuv: {=bool}, ", &self.cuv());
        defmt::write!(f, "cov: {=bool}, ", &self.cov());
        defmt::write!(f, "occ_1: {=bool}, ", &self.occ_1());
        defmt::write!(f, "occ_2: {=bool}, ", &self.occ_2());
        defmt::write!(f, "ocd_1: {=bool}, ", &self.ocd_1());
        defmt::write!(f, "ocd_2: {=bool}, ", &self.ocd_2());
        defmt::write!(f, "aold: {=bool}, ", &self.aold());
        defmt::write!(f, "aoldl: {=bool}, ", &self.aoldl());
        defmt::write!(f, "ascc: {=bool}, ", &self.ascc());
        defmt::write!(f, "asccl: {=bool}, ", &self.asccl());
        defmt::write!(f, "ascd: {=bool}, ", &self.ascd());
        defmt::write!(f, "ascdl: {=bool}, ", &self.ascdl());
        defmt::write!(f, "otc: {=bool}, ", &self.otc());
        defmt::write!(f, "otd: {=bool}, ", &self.otd());
        defmt::write!(f, "cuvc: {=bool}, ", &self.cuvc());
        defmt::write!(f, "otf: {=bool}, ", &self.otf());
        defmt::write!(f, "pto: {=bool}, ", &self.pto());
        defmt::write!(f, "ptos: {=bool}, ", &self.ptos());
        defmt::write!(f, "cto: {=bool}, ", &self.cto());
        defmt::write!(f, "oc: {=bool}, ", &self.oc());
        defmt::write!(f, "chgc: {=bool}, ", &self.chgc());
        defmt::write!(f, "chgv: {=bool}, ", &self.chgv());
        defmt::write!(f, "pchgc: {=bool}, ", &self.pchgc());
        defmt::write!(f, "utc: {=bool}, ", &self.utc());
        defmt::write!(f, "utd: {=bool}, ", &self.utd());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacSafetyStatus {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacSafetyStatus {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacSafetyStatus {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacSafetyStatus {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacSafetyStatus {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacSafetyStatus {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacSafetyStatus {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_SAFETY_ALERT")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacSafetyAlert {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 4],
}
unsafe impl ::device_driver::Fieldset for MacSafetyAlert {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 4] };
}
impl MacSafetyAlert {
    /// `bit 0` - Read the `cuv` field.
    ///
    #[doc(alias = "CUV")]
    #[must_use]
    pub fn cuv(&self) -> bool {
        let start = 0;
        let end = 0;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 1` - Read the `cov` field.
    ///
    #[doc(alias = "COV")]
    #[must_use]
    pub fn cov(&self) -> bool {
        let start = 1;
        let end = 1;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 2` - Read the `occ_1` field.
    ///
    #[doc(alias = "OCC1")]
    #[must_use]
    pub fn occ_1(&self) -> bool {
        let start = 2;
        let end = 2;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 3` - Read the `occ_2` field.
    ///
    #[doc(alias = "OCC2")]
    #[must_use]
    pub fn occ_2(&self) -> bool {
        let start = 3;
        let end = 3;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 4` - Read the `ocd_1` field.
    ///
    #[doc(alias = "OCD1")]
    #[must_use]
    pub fn ocd_1(&self) -> bool {
        let start = 4;
        let end = 4;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 5` - Read the `ocd_2` field.
    ///
    #[doc(alias = "OCD2")]
    #[must_use]
    pub fn ocd_2(&self) -> bool {
        let start = 5;
        let end = 5;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 7` - Read the `aoldl` field.
    ///
    #[doc(alias = "AOLDL")]
    #[must_use]
    pub fn aoldl(&self) -> bool {
        let start = 7;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 9` - Read the `asccl` field.
    ///
    #[doc(alias = "ASCCL")]
    #[must_use]
    pub fn asccl(&self) -> bool {
        let start = 9;
        let end = 9;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 11` - Read the `ascdl` field.
    ///
    #[doc(alias = "ASCDL")]
    #[must_use]
    pub fn ascdl(&self) -> bool {
        let start = 11;
        let end = 11;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 12` - Read the `otc` field.
    ///
    #[doc(alias = "OTC")]
    #[must_use]
    pub fn otc(&self) -> bool {
        let start = 12;
        let end = 12;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 13` - Read the `otd` field.
    ///
    #[doc(alias = "OTD")]
    #[must_use]
    pub fn otd(&self) -> bool {
        let start = 13;
        let end = 13;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 14` - Read the `cuvc` field.
    ///
    #[doc(alias = "CUVC")]
    #[must_use]
    pub fn cuvc(&self) -> bool {
        let start = 14;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 16` - Read the `otf` field.
    ///
    #[doc(alias = "OTF")]
    #[must_use]
    pub fn otf(&self) -> bool {
        let start = 16;
        let end = 16;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 18` - Read the `pto` field.
    ///
    #[doc(alias = "PTO")]
    #[must_use]
    pub fn pto(&self) -> bool {
        let start = 18;
        let end = 18;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 19` - Read the `ptos` field.
    ///
    #[doc(alias = "PTOS")]
    #[must_use]
    pub fn ptos(&self) -> bool {
        let start = 19;
        let end = 19;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 20` - Read the `cto` field.
    ///
    #[doc(alias = "CTO")]
    #[must_use]
    pub fn cto(&self) -> bool {
        let start = 20;
        let end = 20;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 21` - Read the `ctos` field.
    ///
    #[doc(alias = "CTOS")]
    #[must_use]
    pub fn ctos(&self) -> bool {
        let start = 21;
        let end = 21;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 22` - Read the `oc` field.
    ///
    #[doc(alias = "OC")]
    #[must_use]
    pub fn oc(&self) -> bool {
        let start = 22;
        let end = 22;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 23` - Read the `chgc` field.
    ///
    #[doc(alias = "CHGC")]
    #[must_use]
    pub fn chgc(&self) -> bool {
        let start = 23;
        let end = 23;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 24` - Read the `chgv` field.
    ///
    #[doc(alias = "CHGV")]
    #[must_use]
    pub fn chgv(&self) -> bool {
        let start = 24;
        let end = 24;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 25` - Read the `pchgc` field.
    ///
    #[doc(alias = "PCHGC")]
    #[must_use]
    pub fn pchgc(&self) -> bool {
        let start = 25;
        let end = 25;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 26` - Read the `utc` field.
    ///
    #[doc(alias = "UTC")]
    #[must_use]
    pub fn utc(&self) -> bool {
        let start = 26;
        let end = 26;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 27` - Read the `utd` field.
    ///
    #[doc(alias = "UTD")]
    #[must_use]
    pub fn utd(&self) -> bool {
        let start = 27;
        let end = 27;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `bit 0` - Set the `cuv` field.
    ///
    #[doc(alias = "CUV")]
    pub fn set_cuv(&mut self, value: bool) {
        let start = 0;
        let end = 0;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 1` - Set the `cov` field.
    ///
    #[doc(alias = "COV")]
    pub fn set_cov(&mut self, value: bool) {
        let start = 1;
        let end = 1;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 2` - Set the `occ_1` field.
    ///
    #[doc(alias = "OCC1")]
    pub fn set_occ_1(&mut self, value: bool) {
        let start = 2;
        let end = 2;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 3` - Set the `occ_2` field.
    ///
    #[doc(alias = "OCC2")]
    pub fn set_occ_2(&mut self, value: bool) {
        let start = 3;
        let end = 3;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 4` - Set the `ocd_1` field.
    ///
    #[doc(alias = "OCD1")]
    pub fn set_ocd_1(&mut self, value: bool) {
        let start = 4;
        let end = 4;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 5` - Set the `ocd_2` field.
    ///
    #[doc(alias = "OCD2")]
    pub fn set_ocd_2(&mut self, value: bool) {
        let start = 5;
        let end = 5;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 7` - Set the `aoldl` field.
    ///
    #[doc(alias = "AOLDL")]
    pub fn set_aoldl(&mut self, value: bool) {
        let start = 7;
        let end = 7;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 9` - Set the `asccl` field.
    ///
    #[doc(alias = "ASCCL")]
    pub fn set_asccl(&mut self, value: bool) {
        let start = 9;
        let end = 9;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 11` - Set the `ascdl` field.
    ///
    #[doc(alias = "ASCDL")]
    pub fn set_ascdl(&mut self, value: bool) {
        let start = 11;
        let end = 11;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 12` - Set the `otc` field.
    ///
    #[doc(alias = "OTC")]
    pub fn set_otc(&mut self, value: bool) {
        let start = 12;
        let end = 12;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 13` - Set the `otd` field.
    ///
    #[doc(alias = "OTD")]
    pub fn set_otd(&mut self, value: bool) {
        let start = 13;
        let end = 13;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 14` - Set the `cuvc` field.
    ///
    #[doc(alias = "CUVC")]
    pub fn set_cuvc(&mut self, value: bool) {
        let start = 14;
        let end = 14;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 16` - Set the `otf` field.
    ///
    #[doc(alias = "OTF")]
    pub fn set_otf(&mut self, value: bool) {
        let start = 16;
        let end = 16;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 18` - Set the `pto` field.
    ///
    #[doc(alias = "PTO")]
    pub fn set_pto(&mut self, value: bool) {
        let start = 18;
        let end = 18;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 19` - Set the `ptos` field.
    ///
    #[doc(alias = "PTOS")]
    pub fn set_ptos(&mut self, value: bool) {
        let start = 19;
        let end = 19;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 20` - Set the `cto` field.
    ///
    #[doc(alias = "CTO")]
    pub fn set_cto(&mut self, value: bool) {
        let start = 20;
        let end = 20;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 21` - Set the `ctos` field.
    ///
    #[doc(alias = "CTOS")]
    pub fn set_ctos(&mut self, value: bool) {
        let start = 21;
        let end = 21;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 22` - Set the `oc` field.
    ///
    #[doc(alias = "OC")]
    pub fn set_oc(&mut self, value: bool) {
        let start = 22;
        let end = 22;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 23` - Set the `chgc` field.
    ///
    #[doc(alias = "CHGC")]
    pub fn set_chgc(&mut self, value: bool) {
        let start = 23;
        let end = 23;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 24` - Set the `chgv` field.
    ///
    #[doc(alias = "CHGV")]
    pub fn set_chgv(&mut self, value: bool) {
        let start = 24;
        let end = 24;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 25` - Set the `pchgc` field.
    ///
    #[doc(alias = "PCHGC")]
    pub fn set_pchgc(&mut self, value: bool) {
        let start = 25;
        let end = 25;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 26` - Set the `utc` field.
    ///
    #[doc(alias = "UTC")]
    pub fn set_utc(&mut self, value: bool) {
        let start = 26;
        let end = 26;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 27` - Set the `utd` field.
    ///
    #[doc(alias = "UTD")]
    pub fn set_utd(&mut self, value: bool) {
        let start = 27;
        let end = 27;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacSafetyAlert {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 4]> for MacSafetyAlert {
    fn from(bits: [u8; 4]) -> Self {
        Self { bits }
    }
}
impl From<MacSafetyAlert> for [u8; 4] {
    fn from(val: MacSafetyAlert) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacSafetyAlert {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacSafetyAlert");
        d.field("cuv", &self.cuv());
        d.field("cov", &self.cov());
        d.field("occ_1", &self.occ_1());
        d.field("occ_2", &self.occ_2());
        d.field("ocd_1", &self.ocd_1());
        d.field("ocd_2", &self.ocd_2());
        d.field("aoldl", &self.aoldl());
        d.field("asccl", &self.asccl());
        d.field("ascdl", &self.ascdl());
        d.field("otc", &self.otc());
        d.field("otd", &self.otd());
        d.field("cuvc", &self.cuvc());
        d.field("otf", &self.otf());
        d.field("pto", &self.pto());
        d.field("ptos", &self.ptos());
        d.field("cto", &self.cto());
        d.field("ctos", &self.ctos());
        d.field("oc", &self.oc());
        d.field("chgc", &self.chgc());
        d.field("chgv", &self.chgv());
        d.field("pchgc", &self.pchgc());
        d.field("utc", &self.utc());
        d.field("utd", &self.utd());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacSafetyAlert {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacSafetyAlert {{ ");
        defmt::write!(f, "cuv: {=bool}, ", &self.cuv());
        defmt::write!(f, "cov: {=bool}, ", &self.cov());
        defmt::write!(f, "occ_1: {=bool}, ", &self.occ_1());
        defmt::write!(f, "occ_2: {=bool}, ", &self.occ_2());
        defmt::write!(f, "ocd_1: {=bool}, ", &self.ocd_1());
        defmt::write!(f, "ocd_2: {=bool}, ", &self.ocd_2());
        defmt::write!(f, "aoldl: {=bool}, ", &self.aoldl());
        defmt::write!(f, "asccl: {=bool}, ", &self.asccl());
        defmt::write!(f, "ascdl: {=bool}, ", &self.ascdl());
        defmt::write!(f, "otc: {=bool}, ", &self.otc());
        defmt::write!(f, "otd: {=bool}, ", &self.otd());
        defmt::write!(f, "cuvc: {=bool}, ", &self.cuvc());
        defmt::write!(f, "otf: {=bool}, ", &self.otf());
        defmt::write!(f, "pto: {=bool}, ", &self.pto());
        defmt::write!(f, "ptos: {=bool}, ", &self.ptos());
        defmt::write!(f, "cto: {=bool}, ", &self.cto());
        defmt::write!(f, "ctos: {=bool}, ", &self.ctos());
        defmt::write!(f, "oc: {=bool}, ", &self.oc());
        defmt::write!(f, "chgc: {=bool}, ", &self.chgc());
        defmt::write!(f, "chgv: {=bool}, ", &self.chgv());
        defmt::write!(f, "pchgc: {=bool}, ", &self.pchgc());
        defmt::write!(f, "utc: {=bool}, ", &self.utc());
        defmt::write!(f, "utd: {=bool}, ", &self.utd());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacSafetyAlert {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacSafetyAlert {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacSafetyAlert {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacSafetyAlert {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacSafetyAlert {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacSafetyAlert {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacSafetyAlert {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_ALL_DF_SIGNATURE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacAllDfSignature {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for MacAllDfSignature {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl MacAllDfSignature {
    /// `14:0` - Read the `static_chem_df_sig` field.
    ///
    #[doc(alias = "STATIC_CHEM_DF_SIG")]
    #[must_use]
    pub fn static_chem_df_sig(&self) -> u16 {
        let start = 0;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 15` - Read the `sig_mismatch` field.
    ///
    #[doc(alias = "SIG_MISMATCH")]
    #[must_use]
    pub fn sig_mismatch(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `14:0` - Set the `static_chem_df_sig` field.
    ///
    #[doc(alias = "STATIC_CHEM_DF_SIG")]
    pub fn set_static_chem_df_sig(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `sig_mismatch` field.
    ///
    #[doc(alias = "SIG_MISMATCH")]
    pub fn set_sig_mismatch(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacAllDfSignature {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for MacAllDfSignature {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<MacAllDfSignature> for [u8; 2] {
    fn from(val: MacAllDfSignature) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacAllDfSignature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacAllDfSignature");
        d.field("static_chem_df_sig", &self.static_chem_df_sig());
        d.field("sig_mismatch", &self.sig_mismatch());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacAllDfSignature {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacAllDfSignature {{ ");
        defmt::write!(f, "static_chem_df_sig: {=u16}, ", &self.static_chem_df_sig());
        defmt::write!(f, "sig_mismatch: {=bool}, ", &self.sig_mismatch());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacAllDfSignature {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacAllDfSignature {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacAllDfSignature {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacAllDfSignature {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacAllDfSignature {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacAllDfSignature {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacAllDfSignature {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_STATIC_CHEM_DF_SIG")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacStaticChemDfSig {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for MacStaticChemDfSig {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl MacStaticChemDfSig {
    /// `14:0` - Read the `static_chem_df_sig` field.
    ///
    #[doc(alias = "STATIC_CHEM_DF_SIG")]
    #[must_use]
    pub fn static_chem_df_sig(&self) -> u16 {
        let start = 0;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 15` - Read the `sig_mismatch` field.
    ///
    #[doc(alias = "SIG_MISMATCH")]
    #[must_use]
    pub fn sig_mismatch(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `14:0` - Set the `static_chem_df_sig` field.
    ///
    #[doc(alias = "STATIC_CHEM_DF_SIG")]
    pub fn set_static_chem_df_sig(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `sig_mismatch` field.
    ///
    #[doc(alias = "SIG_MISMATCH")]
    pub fn set_sig_mismatch(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacStaticChemDfSig {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for MacStaticChemDfSig {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<MacStaticChemDfSig> for [u8; 2] {
    fn from(val: MacStaticChemDfSig) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacStaticChemDfSig {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacStaticChemDfSig");
        d.field("static_chem_df_sig", &self.static_chem_df_sig());
        d.field("sig_mismatch", &self.sig_mismatch());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacStaticChemDfSig {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacStaticChemDfSig {{ ");
        defmt::write!(f, "static_chem_df_sig: {=u16}, ", &self.static_chem_df_sig());
        defmt::write!(f, "sig_mismatch: {=bool}, ", &self.sig_mismatch());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacStaticChemDfSig {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacStaticChemDfSig {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacStaticChemDfSig {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacStaticChemDfSig {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacStaticChemDfSig {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacStaticChemDfSig {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacStaticChemDfSig {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_CHEM_ID")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacChemId {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for MacChemId {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl MacChemId {
    /// `7:0` - Read the `chem_id` field.
    ///
    #[doc(alias = "CHEM_ID")]
    #[must_use]
    pub fn chem_id(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `chem_id` field.
    ///
    #[doc(alias = "CHEM_ID")]
    pub fn set_chem_id(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacChemId {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for MacChemId {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<MacChemId> for [u8; 1] {
    fn from(val: MacChemId) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacChemId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacChemId");
        d.field("chem_id", &self.chem_id());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacChemId {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacChemId {{ ");
        defmt::write!(f, "chem_id: {=u8}, ", &self.chem_id());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacChemId {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacChemId {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacChemId {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacChemId {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacChemId {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacChemId {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacChemId {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_STATIC_DF_SIGNATURE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacStaticDfSignature {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for MacStaticDfSignature {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl MacStaticDfSignature {
    /// `14:0` - Read the `static_df_sig` field.
    ///
    #[doc(alias = "STATIC_DF_SIG")]
    #[must_use]
    pub fn static_df_sig(&self) -> u16 {
        let start = 0;
        let end = 14;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `bit 15` - Read the `sig_mismatch` field.
    ///
    #[doc(alias = "SIG_MISMATCH")]
    #[must_use]
    pub fn sig_mismatch(&self) -> bool {
        let start = 15;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw > 0
    }
    /// `14:0` - Set the `static_df_sig` field.
    ///
    #[doc(alias = "STATIC_DF_SIG")]
    pub fn set_static_df_sig(&mut self, value: u16) {
        let start = 0;
        let end = 14;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `bit 15` - Set the `sig_mismatch` field.
    ///
    #[doc(alias = "SIG_MISMATCH")]
    pub fn set_sig_mismatch(&mut self, value: bool) {
        let start = 15;
        let end = 15;
        let raw = value as _;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacStaticDfSignature {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for MacStaticDfSignature {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<MacStaticDfSignature> for [u8; 2] {
    fn from(val: MacStaticDfSignature) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacStaticDfSignature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacStaticDfSignature");
        d.field("static_df_sig", &self.static_df_sig());
        d.field("sig_mismatch", &self.sig_mismatch());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacStaticDfSignature {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacStaticDfSignature {{ ");
        defmt::write!(f, "static_df_sig: {=u16}, ", &self.static_df_sig());
        defmt::write!(f, "sig_mismatch: {=bool}, ", &self.sig_mismatch());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacStaticDfSignature {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacStaticDfSignature {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacStaticDfSignature {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacStaticDfSignature {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacStaticDfSignature {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacStaticDfSignature {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacStaticDfSignature {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_INSTRUCTION_FLASH_SIGNATURE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacInstructionFlashSignature {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for MacInstructionFlashSignature {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl MacInstructionFlashSignature {
    /// `7:0` - Read the `insn_flsh_sig` field.
    ///
    #[doc(alias = "INSN_FLSH_SIG")]
    #[must_use]
    pub fn insn_flsh_sig(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `insn_flsh_sig` field.
    ///
    #[doc(alias = "INSN_FLSH_SIG")]
    pub fn set_insn_flsh_sig(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacInstructionFlashSignature {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for MacInstructionFlashSignature {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<MacInstructionFlashSignature> for [u8; 1] {
    fn from(val: MacInstructionFlashSignature) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacInstructionFlashSignature {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacInstructionFlashSignature");
        d.field("insn_flsh_sig", &self.insn_flsh_sig());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacInstructionFlashSignature {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacInstructionFlashSignature {{ ");
        defmt::write!(f, "insn_flsh_sig: {=u8}, ", &self.insn_flsh_sig());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacInstructionFlashSignature {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacInstructionFlashSignature {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacInstructionFlashSignature {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacInstructionFlashSignature {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacInstructionFlashSignature {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacInstructionFlashSignature {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacInstructionFlashSignature {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_HARDWARE_VERSION")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacHardwareVersion {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 1],
}
unsafe impl ::device_driver::Fieldset for MacHardwareVersion {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 1] };
}
impl MacHardwareVersion {
    /// `7:0` - Read the `hw_vers` field.
    ///
    #[doc(alias = "HW_VERS")]
    #[must_use]
    pub fn hw_vers(&self) -> u8 {
        let start = 0;
        let end = 7;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `7:0` - Set the `hw_vers` field.
    ///
    #[doc(alias = "HW_VERS")]
    pub fn set_hw_vers(&mut self, value: u8) {
        let start = 0;
        let end = 7;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacHardwareVersion {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 1]> for MacHardwareVersion {
    fn from(bits: [u8; 1]) -> Self {
        Self { bits }
    }
}
impl From<MacHardwareVersion> for [u8; 1] {
    fn from(val: MacHardwareVersion) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacHardwareVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacHardwareVersion");
        d.field("hw_vers", &self.hw_vers());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacHardwareVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacHardwareVersion {{ ");
        defmt::write!(f, "hw_vers: {=u8}, ", &self.hw_vers());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacHardwareVersion {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacHardwareVersion {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacHardwareVersion {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacHardwareVersion {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacHardwareVersion {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacHardwareVersion {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacHardwareVersion {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_FIRMWARE_VERSION")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacFirmwareVersion {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 11],
}
unsafe impl ::device_driver::Fieldset for MacFirmwareVersion {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 11] };
}
impl MacFirmwareVersion {
    /// `15:0` - Read the `device_number` field.
    ///
    #[doc(alias = "DEVICE_NUMBER")]
    #[must_use]
    pub fn device_number(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `31:16` - Read the `version` field.
    ///
    #[doc(alias = "VERSION")]
    #[must_use]
    pub fn version(&self) -> u16 {
        let start = 16;
        let end = 31;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `47:32` - Read the `build_number` field.
    ///
    #[doc(alias = "BUILD_NUMBER")]
    #[must_use]
    pub fn build_number(&self) -> u16 {
        let start = 32;
        let end = 47;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `55:48` - Read the `firmware_type` field.
    ///
    #[doc(alias = "FIRMWARE_TYPE")]
    #[must_use]
    pub fn firmware_type(&self) -> u8 {
        let start = 48;
        let end = 55;
        let raw = unsafe { ::device_driver::ops::load::<u8, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `71:56` - Read the `impedence_track_vers` field.
    ///
    #[doc(alias = "IMPEDENCE_TRACK_VERS")]
    #[must_use]
    pub fn impedence_track_vers(&self) -> u16 {
        let start = 56;
        let end = 71;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `device_number` field.
    ///
    #[doc(alias = "DEVICE_NUMBER")]
    pub fn set_device_number(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `31:16` - Set the `version` field.
    ///
    #[doc(alias = "VERSION")]
    pub fn set_version(&mut self, value: u16) {
        let start = 16;
        let end = 31;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `47:32` - Set the `build_number` field.
    ///
    #[doc(alias = "BUILD_NUMBER")]
    pub fn set_build_number(&mut self, value: u16) {
        let start = 32;
        let end = 47;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `55:48` - Set the `firmware_type` field.
    ///
    #[doc(alias = "FIRMWARE_TYPE")]
    pub fn set_firmware_type(&mut self, value: u8) {
        let start = 48;
        let end = 55;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u8, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
    /// `71:56` - Set the `impedence_track_vers` field.
    ///
    #[doc(alias = "IMPEDENCE_TRACK_VERS")]
    pub fn set_impedence_track_vers(&mut self, value: u16) {
        let start = 56;
        let end = 71;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacFirmwareVersion {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 11]> for MacFirmwareVersion {
    fn from(bits: [u8; 11]) -> Self {
        Self { bits }
    }
}
impl From<MacFirmwareVersion> for [u8; 11] {
    fn from(val: MacFirmwareVersion) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacFirmwareVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacFirmwareVersion");
        d.field("device_number", &self.device_number());
        d.field("version", &self.version());
        d.field("build_number", &self.build_number());
        d.field("firmware_type", &self.firmware_type());
        d.field("impedence_track_vers", &self.impedence_track_vers());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacFirmwareVersion {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacFirmwareVersion {{ ");
        defmt::write!(f, "device_number: {=u16}, ", &self.device_number());
        defmt::write!(f, "version: {=u16}, ", &self.version());
        defmt::write!(f, "build_number: {=u16}, ", &self.build_number());
        defmt::write!(f, "firmware_type: {=u8}, ", &self.firmware_type());
        defmt::write!(f, "impedence_track_vers: {=u16}, ", &self.impedence_track_vers());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacFirmwareVersion {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacFirmwareVersion {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacFirmwareVersion {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacFirmwareVersion {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacFirmwareVersion {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacFirmwareVersion {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacFirmwareVersion {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[doc(alias = "MAC_DEVICE_TYPE")]
#[derive(Copy, Clone, Eq, PartialEq)]
#[repr(transparent)]
pub struct MacDeviceType {
    #[doc(hidden)]
    /// The internal bits
    bits: [u8; 2],
}
unsafe impl ::device_driver::Fieldset for MacDeviceType {
    const METADATA: ::device_driver::FieldsetMetadata =
        ::device_driver::FieldsetMetadata::new().with_byte_order(::device_driver::ByteOrder::LE);
    const ZERO: Self = Self { bits: [0; 2] };
}
impl MacDeviceType {
    /// `15:0` - Read the `device_type` field.
    ///
    #[doc(alias = "DEVICE_TYPE")]
    #[must_use]
    pub fn device_type(&self) -> u16 {
        let start = 0;
        let end = 15;
        let raw = unsafe { ::device_driver::ops::load::<u16, ::device_driver::ops::LE>(&self.bits, start, end) };
        raw
    }
    /// `15:0` - Set the `device_type` field.
    ///
    #[doc(alias = "DEVICE_TYPE")]
    pub fn set_device_type(&mut self, value: u16) {
        let start = 0;
        let end = 15;
        let raw = value;
        unsafe { ::device_driver::ops::store::<u16, ::device_driver::ops::LE>(raw, start, end, &mut self.bits) };
    }
}
impl Default for MacDeviceType {
    fn default() -> Self {
        <Self as ::device_driver::Fieldset>::ZERO
    }
}
impl From<[u8; 2]> for MacDeviceType {
    fn from(bits: [u8; 2]) -> Self {
        Self { bits }
    }
}
impl From<MacDeviceType> for [u8; 2] {
    fn from(val: MacDeviceType) -> Self {
        val.bits
    }
}
impl core::fmt::Debug for MacDeviceType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
        let mut d = f.debug_struct("MacDeviceType");
        d.field("device_type", &self.device_type());
        d.finish()
    }
}
#[cfg(feature = "defmt-03")]
impl defmt::Format for MacDeviceType {
    fn format(&self, f: defmt::Formatter) {
        defmt::write!(f, "MacDeviceType {{ ");
        defmt::write!(f, "device_type: {=u16}, ", &self.device_type());
        defmt::write!(f, "}}");
    }
}
impl core::ops::BitAnd for MacDeviceType {
    type Output = Self;
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self &= rhs;
        self
    }
}
impl core::ops::BitAndAssign for MacDeviceType {
    fn bitand_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l &= *r;
        }
    }
}
impl core::ops::BitOr for MacDeviceType {
    type Output = Self;
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self |= rhs;
        self
    }
}
impl core::ops::BitOrAssign for MacDeviceType {
    fn bitor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l |= *r;
        }
    }
}
impl core::ops::BitXor for MacDeviceType {
    type Output = Self;
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self ^= rhs;
        self
    }
}
impl core::ops::BitXorAssign for MacDeviceType {
    fn bitxor_assign(&mut self, rhs: Self) {
        for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
            *l ^= *r;
        }
    }
}
impl core::ops::Not for MacDeviceType {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        for val in self.bits.iter_mut() {
            *val = !*val;
        }
        self
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum QMaxStatus {
    #[doc(alias = "BattOK")]
    BattOk = 0,
    QMaxUpdated = 1,
    QMaxRTableUpdated = 2,
    Reserved = 3,
}
impl core::convert::TryFrom<u8> for QMaxStatus {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::BattOk),
            1 => Ok(Self::QMaxUpdated),
            2 => Ok(Self::QMaxRTableUpdated),
            3 => Ok(Self::Reserved),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "QMaxStatus",
            }),
        }
    }
}
impl From<QMaxStatus> for u8 {
    fn from(val: QMaxStatus) -> Self {
        match val {
            QMaxStatus::BattOk => 0,
            QMaxStatus::QMaxUpdated => 1,
            QMaxStatus::QMaxRTableUpdated => 2,
            QMaxStatus::Reserved => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for QMaxStatus {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum SecurityMode {
    Reserved = 0,
    FullAccess = 1,
    UnSealed = 2,
    Sealed = 3,
}
impl core::convert::TryFrom<u8> for SecurityMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::FullAccess),
            2 => Ok(Self::UnSealed),
            3 => Ok(Self::Sealed),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "SecurityMode",
            }),
        }
    }
}
impl From<SecurityMode> for u8 {
    fn from(val: SecurityMode) -> Self {
        match val {
            SecurityMode::Reserved => 0,
            SecurityMode::FullAccess => 1,
            SecurityMode::UnSealed => 2,
            SecurityMode::Sealed => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for SecurityMode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "MACQMaxStatus")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum MacqMaxStatus {
    #[doc(alias = "BattOK")]
    BattOk = 0,
    QMaxUpdated = 1,
    QMaxRTableUpdated = 2,
    Reserved = 3,
}
impl core::convert::TryFrom<u8> for MacqMaxStatus {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::BattOk),
            1 => Ok(Self::QMaxUpdated),
            2 => Ok(Self::QMaxRTableUpdated),
            3 => Ok(Self::Reserved),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "MacqMaxStatus",
            }),
        }
    }
}
impl From<MacqMaxStatus> for u8 {
    fn from(val: MacqMaxStatus) -> Self {
        match val {
            MacqMaxStatus::BattOk => 0,
            MacqMaxStatus::QMaxUpdated => 1,
            MacqMaxStatus::QMaxRTableUpdated => 2,
            MacqMaxStatus::Reserved => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for MacqMaxStatus {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
#[doc(alias = "MACSecurityMode")]
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum MacSecurityMode {
    Reserved = 0,
    FullAccess = 1,
    UnSealed = 2,
    Sealed = 3,
}
impl core::convert::TryFrom<u8> for MacSecurityMode {
    type Error = ::device_driver::ConversionError<u8>;
    fn try_from(val: u8) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(Self::Reserved),
            1 => Ok(Self::FullAccess),
            2 => Ok(Self::UnSealed),
            3 => Ok(Self::Sealed),
            val => Err(::device_driver::ConversionError {
                source: val,
                target: "MacSecurityMode",
            }),
        }
    }
}
impl From<MacSecurityMode> for u8 {
    fn from(val: MacSecurityMode) -> Self {
        match val {
            MacSecurityMode::Reserved => 0,
            MacSecurityMode::FullAccess => 1,
            MacSecurityMode::UnSealed => 2,
            MacSecurityMode::Sealed => 3,
        }
    }
}
#[doc(hidden)]
impl ::device_driver::EnumIndex for MacSecurityMode {
    #[track_caller]
    fn index(&self) -> i32 {
        let index = u8::from(*self);
        index.try_into().unwrap()
    }
}
