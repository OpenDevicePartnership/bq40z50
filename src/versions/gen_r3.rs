/// Root block of the Device driver
#[derive(Debug)]
pub struct Device<I> {
    pub(crate) interface: I,
    #[doc(hidden)]
    base_address: u32,
}
impl<I> Device<I> {
    /// Create a new instance of the block based on device interface
    pub const fn new(interface: I) -> Self {
        Self {
            interface,
            base_address: 0,
        }
    }
    /// A reference to the interface used to communicate with the device
    pub(crate) fn interface(&mut self) -> &mut I {
        &mut self.interface
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub fn read_all_registers(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::RegisterInterface<AddressType = u8>,
    {
        let reg = self.remaining_capacity_alarm().read()?;
        callback(1 + 0 * 0, "remaining_capacity_alarm", reg.into());
        let reg = self.remaining_time_alarm().read()?;
        callback(2 + 0 * 0, "remaining_time_alarm", reg.into());
        let reg = self.battery_mode().read()?;
        callback(3 + 0 * 0, "battery_mode", reg.into());
        let reg = self.at_rate().read()?;
        callback(4 + 0 * 0, "at_rate", reg.into());
        let reg = self.at_rate_time_to_full().read()?;
        callback(5 + 0 * 0, "at_rate_time_to_full", reg.into());
        let reg = self.at_rate_time_to_empty().read()?;
        callback(6 + 0 * 0, "at_rate_time_to_empty", reg.into());
        let reg = self.at_rate_ok().read()?;
        callback(7 + 0 * 0, "at_rate_ok", reg.into());
        let reg = self.temperature().read()?;
        callback(8 + 0 * 0, "temperature", reg.into());
        let reg = self.voltage().read()?;
        callback(9 + 0 * 0, "voltage", reg.into());
        let reg = self.current().read()?;
        callback(10 + 0 * 0, "current", reg.into());
        let reg = self.avg_current().read()?;
        callback(11 + 0 * 0, "avg_current", reg.into());
        let reg = self.max_error().read()?;
        callback(12 + 0 * 0, "max_error", reg.into());
        let reg = self.relative_state_of_charge().read()?;
        callback(13 + 0 * 0, "relative_state_of_charge", reg.into());
        let reg = self.absolute_state_of_charge().read()?;
        callback(14 + 0 * 0, "absolute_state_of_charge", reg.into());
        let reg = self.remaining_capacity().read()?;
        callback(15 + 0 * 0, "remaining_capacity", reg.into());
        let reg = self.full_charge_capacity().read()?;
        callback(16 + 0 * 0, "full_charge_capacity", reg.into());
        let reg = self.run_time_to_empty().read()?;
        callback(17 + 0 * 0, "run_time_to_empty", reg.into());
        let reg = self.average_time_to_empty().read()?;
        callback(18 + 0 * 0, "average_time_to_empty", reg.into());
        let reg = self.average_time_to_full().read()?;
        callback(19 + 0 * 0, "average_time_to_full", reg.into());
        let reg = self.charging_current().read()?;
        callback(20 + 0 * 0, "charging_current", reg.into());
        let reg = self.charging_voltage().read()?;
        callback(21 + 0 * 0, "charging_voltage", reg.into());
        let reg = self.battery_status().read()?;
        callback(22 + 0 * 0, "battery_status", reg.into());
        let reg = self.cycle_count().read()?;
        callback(23 + 0 * 0, "cycle_count", reg.into());
        let reg = self.design_capacity().read()?;
        callback(24 + 0 * 0, "design_capacity", reg.into());
        let reg = self.design_voltage().read()?;
        callback(25 + 0 * 0, "design_voltage", reg.into());
        let reg = self.specification_info().read()?;
        callback(26 + 0 * 0, "specification_info", reg.into());
        let reg = self.manufacture_date().read()?;
        callback(27 + 0 * 0, "manufacture_date", reg.into());
        let reg = self.serial_number().read()?;
        callback(28 + 0 * 0, "serial_number", reg.into());
        let reg = self.cell_voltage_4().read()?;
        callback(60 + 0 * 0, "cell_voltage_4", reg.into());
        let reg = self.cell_voltage_3().read()?;
        callback(61 + 0 * 0, "cell_voltage_3", reg.into());
        let reg = self.cell_voltage_2().read()?;
        callback(62 + 0 * 0, "cell_voltage_2", reg.into());
        let reg = self.cell_voltage_1().read()?;
        callback(63 + 0 * 0, "cell_voltage_1", reg.into());
        let reg = self.btp_discharge_set().read()?;
        callback(74 + 0 * 0, "btp_discharge_set", reg.into());
        let reg = self.btp_charge_set().read()?;
        callback(75 + 0 * 0, "btp_charge_set", reg.into());
        let reg = self.state_of_health_soh().read()?;
        callback(79 + 0 * 0, "state_of_health_soh", reg.into());
        let reg = self.safety_alert().read()?;
        callback(80 + 0 * 0, "safety_alert", reg.into());
        let reg = self.safety_status().read()?;
        callback(81 + 0 * 0, "safety_status", reg.into());
        let reg = self.pf_alert().read()?;
        callback(82 + 0 * 0, "pf_alert", reg.into());
        let reg = self.pf_status().read()?;
        callback(83 + 0 * 0, "pf_status", reg.into());
        let reg = self.operation_status().read()?;
        callback(84 + 0 * 0, "operation_status", reg.into());
        let reg = self.charging_status().read()?;
        callback(85 + 0 * 0, "charging_status", reg.into());
        let reg = self.gauging_status().read()?;
        callback(86 + 0 * 0, "gauging_status", reg.into());
        let reg = self.manufacturing_status().read()?;
        callback(87 + 0 * 0, "manufacturing_status", reg.into());
        let reg = self.afe_reg().read()?;
        callback(88 + 0 * 0, "afe_reg", reg.into());
        let reg = self.max_turbo_power().read()?;
        callback(89 + 0 * 0, "max_turbo_power", reg.into());
        let reg = self.sus_turbo_power().read()?;
        callback(90 + 0 * 0, "sus_turbo_power", reg.into());
        let reg = self.turbo_pack_r().read()?;
        callback(91 + 0 * 0, "turbo_pack_r", reg.into());
        let reg = self.turbo_sys_r().read()?;
        callback(92 + 0 * 0, "turbo_sys_r", reg.into());
        let reg = self.turbo_edv().read()?;
        callback(93 + 0 * 0, "turbo_edv", reg.into());
        let reg = self.turbo_current().read()?;
        callback(94 + 0 * 0, "turbo_current", reg.into());
        let reg = self.sus_turbo_current().read()?;
        callback(95 + 0 * 0, "sus_turbo_current", reg.into());
        let reg = self.lifetime_data_block_1().read()?;
        callback(96 + 0 * 0, "lifetime_data_block_1", reg.into());
        let reg = self.lifetime_data_block_2().read()?;
        callback(97 + 0 * 0, "lifetime_data_block_2", reg.into());
        let reg = self.lifetime_data_block_3().read()?;
        callback(98 + 0 * 0, "lifetime_data_block_3", reg.into());
        let reg = self.lifetime_data_block_4().read()?;
        callback(99 + 0 * 0, "lifetime_data_block_4", reg.into());
        let reg = self.lifetime_data_block_5().read()?;
        callback(100 + 0 * 0, "lifetime_data_block_5", reg.into());
        let reg = self.lifetime_data_block_6().read()?;
        callback(101 + 0 * 0, "lifetime_data_block_6", reg.into());
        let reg = self.lifetime_data_block_7().read()?;
        callback(102 + 0 * 0, "lifetime_data_block_7", reg.into());
        let reg = self.lifetime_data_block_8().read()?;
        callback(103 + 0 * 0, "lifetime_data_block_8", reg.into());
        let reg = self.turbo_rhf_effective().read()?;
        callback(104 + 0 * 0, "turbo_rhf_effective", reg.into());
        let reg = self.turbo_vload().read()?;
        callback(105 + 0 * 0, "turbo_vload", reg.into());
        let reg = self.lifetime_data_block_11().read()?;
        callback(106 + 0 * 0, "lifetime_data_block_11", reg.into());
        let reg = self.lifetime_data_block_12().read()?;
        callback(107 + 0 * 0, "lifetime_data_block_12", reg.into());
        let reg = self.da_status_1().read()?;
        callback(113 + 0 * 0, "da_status_1", reg.into());
        let reg = self.da_status_2().read()?;
        callback(114 + 0 * 0, "da_status_2", reg.into());
        let reg = self.gauge_status_1().read()?;
        callback(115 + 0 * 0, "gauge_status_1", reg.into());
        let reg = self.gauge_status_2().read()?;
        callback(116 + 0 * 0, "gauge_status_2", reg.into());
        let reg = self.gauge_status_3().read()?;
        callback(117 + 0 * 0, "gauge_status_3", reg.into());
        let reg = self.cb_status().read()?;
        callback(118 + 0 * 0, "cb_status", reg.into());
        let reg = self.state_of_health().read()?;
        callback(119 + 0 * 0, "state_of_health", reg.into());
        let reg = self.filter_capacity().read()?;
        callback(120 + 0 * 0, "filter_capacity", reg.into());
        Ok(())
    }
    /// Read all readable register values in this block from the device.
    /// The callback is called for each of them.
    /// Any registers in child blocks are not included.
    ///
    /// The callback has three arguments:
    ///
    /// - The address of the register
    /// - The name of the register (with index for repeated registers)
    /// - The read value from the register
    ///
    /// This is useful for e.g. debug printing all values.
    /// The given [field_sets::FieldSetValue] has a Debug and Format implementation that forwards to the concrete type
    /// the lies within so it can be printed without matching on it.
    #[allow(unused_mut)]
    #[allow(unused_variables)]
    pub async fn read_all_registers_async(
        &mut self,
        mut callback: impl FnMut(u8, &'static str, field_sets::FieldSetValue),
    ) -> Result<(), I::Error>
    where
        I: ::device_driver::AsyncRegisterInterface<AddressType = u8>,
    {
        let reg = self.remaining_capacity_alarm().read_async().await?;
        callback(1 + 0 * 0, "remaining_capacity_alarm", reg.into());
        let reg = self.remaining_time_alarm().read_async().await?;
        callback(2 + 0 * 0, "remaining_time_alarm", reg.into());
        let reg = self.battery_mode().read_async().await?;
        callback(3 + 0 * 0, "battery_mode", reg.into());
        let reg = self.at_rate().read_async().await?;
        callback(4 + 0 * 0, "at_rate", reg.into());
        let reg = self.at_rate_time_to_full().read_async().await?;
        callback(5 + 0 * 0, "at_rate_time_to_full", reg.into());
        let reg = self.at_rate_time_to_empty().read_async().await?;
        callback(6 + 0 * 0, "at_rate_time_to_empty", reg.into());
        let reg = self.at_rate_ok().read_async().await?;
        callback(7 + 0 * 0, "at_rate_ok", reg.into());
        let reg = self.temperature().read_async().await?;
        callback(8 + 0 * 0, "temperature", reg.into());
        let reg = self.voltage().read_async().await?;
        callback(9 + 0 * 0, "voltage", reg.into());
        let reg = self.current().read_async().await?;
        callback(10 + 0 * 0, "current", reg.into());
        let reg = self.avg_current().read_async().await?;
        callback(11 + 0 * 0, "avg_current", reg.into());
        let reg = self.max_error().read_async().await?;
        callback(12 + 0 * 0, "max_error", reg.into());
        let reg = self.relative_state_of_charge().read_async().await?;
        callback(13 + 0 * 0, "relative_state_of_charge", reg.into());
        let reg = self.absolute_state_of_charge().read_async().await?;
        callback(14 + 0 * 0, "absolute_state_of_charge", reg.into());
        let reg = self.remaining_capacity().read_async().await?;
        callback(15 + 0 * 0, "remaining_capacity", reg.into());
        let reg = self.full_charge_capacity().read_async().await?;
        callback(16 + 0 * 0, "full_charge_capacity", reg.into());
        let reg = self.run_time_to_empty().read_async().await?;
        callback(17 + 0 * 0, "run_time_to_empty", reg.into());
        let reg = self.average_time_to_empty().read_async().await?;
        callback(18 + 0 * 0, "average_time_to_empty", reg.into());
        let reg = self.average_time_to_full().read_async().await?;
        callback(19 + 0 * 0, "average_time_to_full", reg.into());
        let reg = self.charging_current().read_async().await?;
        callback(20 + 0 * 0, "charging_current", reg.into());
        let reg = self.charging_voltage().read_async().await?;
        callback(21 + 0 * 0, "charging_voltage", reg.into());
        let reg = self.battery_status().read_async().await?;
        callback(22 + 0 * 0, "battery_status", reg.into());
        let reg = self.cycle_count().read_async().await?;
        callback(23 + 0 * 0, "cycle_count", reg.into());
        let reg = self.design_capacity().read_async().await?;
        callback(24 + 0 * 0, "design_capacity", reg.into());
        let reg = self.design_voltage().read_async().await?;
        callback(25 + 0 * 0, "design_voltage", reg.into());
        let reg = self.specification_info().read_async().await?;
        callback(26 + 0 * 0, "specification_info", reg.into());
        let reg = self.manufacture_date().read_async().await?;
        callback(27 + 0 * 0, "manufacture_date", reg.into());
        let reg = self.serial_number().read_async().await?;
        callback(28 + 0 * 0, "serial_number", reg.into());
        let reg = self.cell_voltage_4().read_async().await?;
        callback(60 + 0 * 0, "cell_voltage_4", reg.into());
        let reg = self.cell_voltage_3().read_async().await?;
        callback(61 + 0 * 0, "cell_voltage_3", reg.into());
        let reg = self.cell_voltage_2().read_async().await?;
        callback(62 + 0 * 0, "cell_voltage_2", reg.into());
        let reg = self.cell_voltage_1().read_async().await?;
        callback(63 + 0 * 0, "cell_voltage_1", reg.into());
        let reg = self.btp_discharge_set().read_async().await?;
        callback(74 + 0 * 0, "btp_discharge_set", reg.into());
        let reg = self.btp_charge_set().read_async().await?;
        callback(75 + 0 * 0, "btp_charge_set", reg.into());
        let reg = self.state_of_health_soh().read_async().await?;
        callback(79 + 0 * 0, "state_of_health_soh", reg.into());
        let reg = self.safety_alert().read_async().await?;
        callback(80 + 0 * 0, "safety_alert", reg.into());
        let reg = self.safety_status().read_async().await?;
        callback(81 + 0 * 0, "safety_status", reg.into());
        let reg = self.pf_alert().read_async().await?;
        callback(82 + 0 * 0, "pf_alert", reg.into());
        let reg = self.pf_status().read_async().await?;
        callback(83 + 0 * 0, "pf_status", reg.into());
        let reg = self.operation_status().read_async().await?;
        callback(84 + 0 * 0, "operation_status", reg.into());
        let reg = self.charging_status().read_async().await?;
        callback(85 + 0 * 0, "charging_status", reg.into());
        let reg = self.gauging_status().read_async().await?;
        callback(86 + 0 * 0, "gauging_status", reg.into());
        let reg = self.manufacturing_status().read_async().await?;
        callback(87 + 0 * 0, "manufacturing_status", reg.into());
        let reg = self.afe_reg().read_async().await?;
        callback(88 + 0 * 0, "afe_reg", reg.into());
        let reg = self.max_turbo_power().read_async().await?;
        callback(89 + 0 * 0, "max_turbo_power", reg.into());
        let reg = self.sus_turbo_power().read_async().await?;
        callback(90 + 0 * 0, "sus_turbo_power", reg.into());
        let reg = self.turbo_pack_r().read_async().await?;
        callback(91 + 0 * 0, "turbo_pack_r", reg.into());
        let reg = self.turbo_sys_r().read_async().await?;
        callback(92 + 0 * 0, "turbo_sys_r", reg.into());
        let reg = self.turbo_edv().read_async().await?;
        callback(93 + 0 * 0, "turbo_edv", reg.into());
        let reg = self.turbo_current().read_async().await?;
        callback(94 + 0 * 0, "turbo_current", reg.into());
        let reg = self.sus_turbo_current().read_async().await?;
        callback(95 + 0 * 0, "sus_turbo_current", reg.into());
        let reg = self.lifetime_data_block_1().read_async().await?;
        callback(96 + 0 * 0, "lifetime_data_block_1", reg.into());
        let reg = self.lifetime_data_block_2().read_async().await?;
        callback(97 + 0 * 0, "lifetime_data_block_2", reg.into());
        let reg = self.lifetime_data_block_3().read_async().await?;
        callback(98 + 0 * 0, "lifetime_data_block_3", reg.into());
        let reg = self.lifetime_data_block_4().read_async().await?;
        callback(99 + 0 * 0, "lifetime_data_block_4", reg.into());
        let reg = self.lifetime_data_block_5().read_async().await?;
        callback(100 + 0 * 0, "lifetime_data_block_5", reg.into());
        let reg = self.lifetime_data_block_6().read_async().await?;
        callback(101 + 0 * 0, "lifetime_data_block_6", reg.into());
        let reg = self.lifetime_data_block_7().read_async().await?;
        callback(102 + 0 * 0, "lifetime_data_block_7", reg.into());
        let reg = self.lifetime_data_block_8().read_async().await?;
        callback(103 + 0 * 0, "lifetime_data_block_8", reg.into());
        let reg = self.turbo_rhf_effective().read_async().await?;
        callback(104 + 0 * 0, "turbo_rhf_effective", reg.into());
        let reg = self.turbo_vload().read_async().await?;
        callback(105 + 0 * 0, "turbo_vload", reg.into());
        let reg = self.lifetime_data_block_11().read_async().await?;
        callback(106 + 0 * 0, "lifetime_data_block_11", reg.into());
        let reg = self.lifetime_data_block_12().read_async().await?;
        callback(107 + 0 * 0, "lifetime_data_block_12", reg.into());
        let reg = self.da_status_1().read_async().await?;
        callback(113 + 0 * 0, "da_status_1", reg.into());
        let reg = self.da_status_2().read_async().await?;
        callback(114 + 0 * 0, "da_status_2", reg.into());
        let reg = self.gauge_status_1().read_async().await?;
        callback(115 + 0 * 0, "gauge_status_1", reg.into());
        let reg = self.gauge_status_2().read_async().await?;
        callback(116 + 0 * 0, "gauge_status_2", reg.into());
        let reg = self.gauge_status_3().read_async().await?;
        callback(117 + 0 * 0, "gauge_status_3", reg.into());
        let reg = self.cb_status().read_async().await?;
        callback(118 + 0 * 0, "cb_status", reg.into());
        let reg = self.state_of_health().read_async().await?;
        callback(119 + 0 * 0, "state_of_health", reg.into());
        let reg = self.filter_capacity().read_async().await?;
        callback(120 + 0 * 0, "filter_capacity", reg.into());
        Ok(())
    }
    /// The device can be checked for the IC part number.
    pub fn mac_device_type(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacDeviceTypeFieldsOut> {
        let address = self.base_address + 4456704;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacDeviceTypeFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    /// The device can be checked for the firmware version of the IC.
    pub fn mac_firmware_version(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacFirmwareVersionFieldsOut> {
        let address = self.base_address + 4456960;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacFirmwareVersionFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_hardware_version(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacHardwareVersionFieldsOut> {
        let address = self.base_address + 4457216;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacHardwareVersionFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_instruction_flash_signature(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacInstructionFlashSignatureFieldsOut> {
        let address = self.base_address + 4457472;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacInstructionFlashSignatureFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_static_df_signature(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacStaticDfSignatureFieldsOut> {
        let address = self.base_address + 4457728;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacStaticDfSignatureFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_chem_id(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacChemIdFieldsOut> {
        let address = self.base_address + 4457984;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacChemIdFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_static_chem_df_sig(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacStaticChemDfSigFieldsOut> {
        let address = self.base_address + 4458496;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacStaticChemDfSigFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_all_df_signature(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacAllDfSignatureFieldsOut> {
        let address = self.base_address + 4458752;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacAllDfSignatureFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_shutdown_mode(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4460544;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_sleep_mode(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4460800;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_auto_cc_offset(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4461312;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_fuse_toggle(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4463872;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_pchg_fet_toggle(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4464128;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_chg_fet_toggle(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4464384;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_dsg_fet_toggle(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4464640;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    /// This command enables or disables the gauging function to ease
    /// testing during manufacturing.
    pub fn mac_gauging(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4464896;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_fet_ctrl(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4465152;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_lifetime_data_collection(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4465408;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_permanent_failure(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4465664;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_black_block_recorder(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4465920;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_fuse(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4466176;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_led_disp_en(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4466432;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_lifetime_data_rst(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4466688;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_pf_data_rst(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4466944;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_blk_box_rec_reset(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4467200;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_led_toggle(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4467456;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_led_disp_press(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4467712;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_calibration_mode(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4467968;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_lifetime_data_flush(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4468224;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_lifetime_data_speed_up_mode(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4468480;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_seal(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4468736;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_device_reset(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4473088;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_safety_alert(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacSafetyAlertFieldsOut> {
        let address = self.base_address + 4476928;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacSafetyAlertFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_safety_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacSafetyStatusFieldsOut> {
        let address = self.base_address + 4477184;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacSafetyStatusFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_pf_alert(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacPfAlertFieldsOut> {
        let address = self.base_address + 4477440;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacPfAlertFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_pf_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacPfStatusFieldsOut> {
        let address = self.base_address + 4477696;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacPfStatusFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_operation_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacOperationStatusFieldsOut> {
        let address = self.base_address + 4477952;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacOperationStatusFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_charging_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacChargingStatusFieldsOut> {
        let address = self.base_address + 4478208;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacChargingStatusFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_gauging_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacGaugingStatusFieldsOut> {
        let address = self.base_address + 4478464;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacGaugingStatusFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_manufacturing_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacManufacturingStatusFieldsOut> {
        let address = self.base_address + 4478720;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacManufacturingStatusFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_afe_reg(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacAfeRegFieldsOut> {
        let address = self.base_address + 4478976;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacAfeRegFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn no_load_rem_cap(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::NoLoadRemCapFieldsOut> {
        let address = self.base_address + 4479488;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::NoLoadRemCapFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_1(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock1FieldsOut> {
        let address = self.base_address + 4481024;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock1FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_2(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock2FieldsOut> {
        let address = self.base_address + 4481280;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock2FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_3(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock3FieldsOut> {
        let address = self.base_address + 4481536;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock3FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_4(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock4FieldsOut> {
        let address = self.base_address + 4481792;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock4FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_5(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock5FieldsOut> {
        let address = self.base_address + 4482048;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock5FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_6(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock6FieldsOut> {
        let address = self.base_address + 4482304;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock6FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_7(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock7FieldsOut> {
        let address = self.base_address + 4482560;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock7FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_8(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock8FieldsOut> {
        let address = self.base_address + 4482816;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock8FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_9(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock9FieldsOut> {
        let address = self.base_address + 4483072;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock9FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_10(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock10FieldsOut> {
        let address = self.base_address + 4483328;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock10FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_11(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock11FieldsOut> {
        let address = self.base_address + 4483584;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock11FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_lifetime_data_block_12(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacLifetimeDataBlock12FieldsOut> {
        let address = self.base_address + 4483840;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacLifetimeDataBlock12FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_manufacture_info(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacManufactureInfoFieldsOut> {
        let address = self.base_address + 4485120;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacManufactureInfoFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_da_status_1(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacDaStatus1FieldsOut> {
        let address = self.base_address + 4485376;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacDaStatus1FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_da_status_2(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacDaStatus2FieldsOut> {
        let address = self.base_address + 4485632;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacDaStatus2FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_gauge_status_1(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacGaugeStatus1FieldsOut> {
        let address = self.base_address + 4485888;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacGaugeStatus1FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_gauge_status_2(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacGaugeStatus2FieldsOut> {
        let address = self.base_address + 4486144;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacGaugeStatus2FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_gauge_status_3(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacGaugeStatus3FieldsOut> {
        let address = self.base_address + 4486400;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacGaugeStatus3FieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_cb_status(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacCbStatusFieldsOut> {
        let address = self.base_address + 4486656;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacCbStatusFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_state_of_health(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacStateOfHealthFieldsOut> {
        let address = self.base_address + 4486912;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacStateOfHealthFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_filter_capacity(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacFilterCapacityFieldsOut> {
        let address = self.base_address + 4487168;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacFilterCapacityFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_manufacture_info_b(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacManufactureInfoBFieldsOut> {
        let address = self.base_address + 4487680;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacManufactureInfoBFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_rom_mode(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4456463;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_exit_calibration_output_mode(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4489456;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_stop_output_ccadc_cal(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4489712;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_output_ccadc_cal(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacOutputCcadcCalFieldsOut> {
        let address = self.base_address + 4489712;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacOutputCcadcCalFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    pub fn mac_stop_output_shorted_ccadc_cal(&mut self) -> ::device_driver::CommandOperation<'_, I, u32, (), ()> {
        let address = self.base_address + 4489968;
        ::device_driver::CommandOperation::<'_, I, u32, (), ()>::new(self.interface(), address as u32)
    }
    pub fn mac_output_shorted_ccadc_cal(
        &mut self,
    ) -> ::device_driver::CommandOperation<'_, I, u32, (), field_sets::MacOutputShortedCcadcCalFieldsOut> {
        let address = self.base_address + 4489968;
        ::device_driver::CommandOperation::<'_, I, u32, (), field_sets::MacOutputShortedCcadcCalFieldsOut>::new(
            self.interface(),
            address as u32,
        )
    }
    /// This read/write word function sets a low capacity alarm threshold for the cell stack.
    pub fn remaining_capacity_alarm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::RemainingCapacityAlarm, ::device_driver::RW> {
        let address = self.base_address + 1;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::RemainingCapacityAlarm, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::RemainingCapacityAlarm::new,
        )
    }
    /// This read/write word function sets a low remaining time-to-fully discharge alarm threshold for the cell stack.
    pub fn remaining_time_alarm(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::RemainingTimeAlarm, ::device_driver::RW> {
        let address = self.base_address + 2;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::RemainingTimeAlarm, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::RemainingTimeAlarm::new,
        )
    }
    pub fn battery_mode(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BatteryMode, ::device_driver::RW> {
        let address = self.base_address + 3;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BatteryMode, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::BatteryMode::new,
        )
    }
    pub fn at_rate(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AtRate, ::device_driver::RW> {
        let address = self.base_address + 4;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AtRate, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::AtRate::new,
        )
    }
    pub fn at_rate_time_to_full(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AtRateTimeToFull, ::device_driver::RO> {
        let address = self.base_address + 5;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AtRateTimeToFull, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AtRateTimeToFull::new,
        )
    }
    pub fn at_rate_time_to_empty(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AtRateTimeToEmpty, ::device_driver::RO> {
        let address = self.base_address + 6;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AtRateTimeToEmpty, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AtRateTimeToEmpty::new,
        )
    }
    pub fn at_rate_ok(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AtRateOk, ::device_driver::RO> {
        let address = self.base_address + 7;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AtRateOk, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AtRateOk::new,
        )
    }
    pub fn temperature(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::Temperature, ::device_driver::RO> {
        let address = self.base_address + 8;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::Temperature, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::Temperature::new,
        )
    }
    pub fn voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::Voltage, ::device_driver::RO> {
        let address = self.base_address + 9;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::Voltage, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::Voltage::new,
        )
    }
    pub fn current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::Current, ::device_driver::RO> {
        let address = self.base_address + 10;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::Current, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::Current::new,
        )
    }
    pub fn avg_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AvgCurrent, ::device_driver::RO> {
        let address = self.base_address + 11;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AvgCurrent, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AvgCurrent::new,
        )
    }
    pub fn max_error(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::MaxError, ::device_driver::RO> {
        let address = self.base_address + 12;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::MaxError, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::MaxError::new,
        )
    }
    pub fn relative_state_of_charge(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::RelativeStateOfCharge, ::device_driver::RO> {
        let address = self.base_address + 13;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::RelativeStateOfCharge, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::RelativeStateOfCharge::new,
        )
    }
    pub fn absolute_state_of_charge(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AbsoluteStateOfCharge, ::device_driver::RO> {
        let address = self.base_address + 14;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AbsoluteStateOfCharge, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AbsoluteStateOfCharge::new,
        )
    }
    pub fn remaining_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::RemainingCapacity, ::device_driver::RO> {
        let address = self.base_address + 15;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::RemainingCapacity, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::RemainingCapacity::new,
        )
    }
    pub fn full_charge_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::FullChargeCapacity, ::device_driver::RO> {
        let address = self.base_address + 16;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::FullChargeCapacity, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::FullChargeCapacity::new,
        )
    }
    pub fn run_time_to_empty(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::RunTimeToEmpty, ::device_driver::RO> {
        let address = self.base_address + 17;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::RunTimeToEmpty, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::RunTimeToEmpty::new,
        )
    }
    pub fn average_time_to_empty(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AverageTimeToEmpty, ::device_driver::RO> {
        let address = self.base_address + 18;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AverageTimeToEmpty, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AverageTimeToEmpty::new,
        )
    }
    pub fn average_time_to_full(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AverageTimeToFull, ::device_driver::RO> {
        let address = self.base_address + 19;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AverageTimeToFull, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AverageTimeToFull::new,
        )
    }
    pub fn charging_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargingCurrent, ::device_driver::RO> {
        let address = self.base_address + 20;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargingCurrent, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ChargingCurrent::new,
        )
    }
    pub fn charging_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargingVoltage, ::device_driver::RO> {
        let address = self.base_address + 21;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargingVoltage, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ChargingVoltage::new,
        )
    }
    pub fn battery_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BatteryStatus, ::device_driver::RO> {
        let address = self.base_address + 22;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BatteryStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::BatteryStatus::new,
        )
    }
    pub fn cycle_count(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CycleCount, ::device_driver::RW> {
        let address = self.base_address + 23;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CycleCount, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::CycleCount::new,
        )
    }
    pub fn design_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::DesignCapacity, ::device_driver::RW> {
        let address = self.base_address + 24;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::DesignCapacity, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::DesignCapacity::new,
        )
    }
    pub fn design_voltage(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::DesignVoltage, ::device_driver::RW> {
        let address = self.base_address + 25;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::DesignVoltage, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::DesignVoltage::new,
        )
    }
    pub fn specification_info(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::SpecificationInfo, ::device_driver::RW> {
        let address = self.base_address + 26;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::SpecificationInfo, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::SpecificationInfo::new,
        )
    }
    pub fn manufacture_date(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ManufactureDate, ::device_driver::RW> {
        let address = self.base_address + 27;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ManufactureDate, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::ManufactureDate::new,
        )
    }
    pub fn serial_number(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::SerialNumber, ::device_driver::RW> {
        let address = self.base_address + 28;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::SerialNumber, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::SerialNumber::new,
        )
    }
    pub fn manufacture_name(&mut self) -> ::device_driver::BufferOperation<'_, I, u8, ::device_driver::RO> {
        let address = self.base_address + 32;
        ::device_driver::BufferOperation::<'_, I, u8, ::device_driver::RO>::new(self.interface(), address as u8)
    }
    pub fn device_name(&mut self) -> ::device_driver::BufferOperation<'_, I, u8, ::device_driver::RO> {
        let address = self.base_address + 33;
        ::device_driver::BufferOperation::<'_, I, u8, ::device_driver::RO>::new(self.interface(), address as u8)
    }
    pub fn device_chemistry(&mut self) -> ::device_driver::BufferOperation<'_, I, u8, ::device_driver::RO> {
        let address = self.base_address + 34;
        ::device_driver::BufferOperation::<'_, I, u8, ::device_driver::RO>::new(self.interface(), address as u8)
    }
    pub fn manufacturer_data(&mut self) -> ::device_driver::BufferOperation<'_, I, u8, ::device_driver::RO> {
        let address = self.base_address + 35;
        ::device_driver::BufferOperation::<'_, I, u8, ::device_driver::RO>::new(self.interface(), address as u8)
    }
    pub fn authenticate(&mut self) -> ::device_driver::BufferOperation<'_, I, u8, ::device_driver::RW> {
        let address = self.base_address + 47;
        ::device_driver::BufferOperation::<'_, I, u8, ::device_driver::RW>::new(self.interface(), address as u8)
    }
    pub fn cell_voltage_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CellVoltage4, ::device_driver::RO> {
        let address = self.base_address + 60;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CellVoltage4, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CellVoltage4::new,
        )
    }
    pub fn cell_voltage_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CellVoltage3, ::device_driver::RO> {
        let address = self.base_address + 61;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CellVoltage3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CellVoltage3::new,
        )
    }
    pub fn cell_voltage_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CellVoltage2, ::device_driver::RO> {
        let address = self.base_address + 62;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CellVoltage2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CellVoltage2::new,
        )
    }
    pub fn cell_voltage_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CellVoltage1, ::device_driver::RO> {
        let address = self.base_address + 63;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CellVoltage1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CellVoltage1::new,
        )
    }
    pub fn btp_discharge_set(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BtpDischargeSet, ::device_driver::RW> {
        let address = self.base_address + 74;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BtpDischargeSet, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::BtpDischargeSet::new,
        )
    }
    pub fn btp_charge_set(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::BtpChargeSet, ::device_driver::RW> {
        let address = self.base_address + 75;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::BtpChargeSet, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::BtpChargeSet::new,
        )
    }
    pub fn state_of_health_soh(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::StateOfHealthSoh, ::device_driver::RO> {
        let address = self.base_address + 79;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::StateOfHealthSoh, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::StateOfHealthSoh::new,
        )
    }
    pub fn safety_alert(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::SafetyAlert, ::device_driver::RO> {
        let address = self.base_address + 80;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::SafetyAlert, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::SafetyAlert::new,
        )
    }
    pub fn safety_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::SafetyStatus, ::device_driver::RO> {
        let address = self.base_address + 81;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::SafetyStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::SafetyStatus::new,
        )
    }
    pub fn pf_alert(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::PfAlert, ::device_driver::RO> {
        let address = self.base_address + 82;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::PfAlert, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::PfAlert::new,
        )
    }
    pub fn pf_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::PfStatus, ::device_driver::RO> {
        let address = self.base_address + 83;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::PfStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::PfStatus::new,
        )
    }
    pub fn operation_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::OperationStatus, ::device_driver::RO> {
        let address = self.base_address + 84;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::OperationStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::OperationStatus::new,
        )
    }
    pub fn charging_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ChargingStatus, ::device_driver::RO> {
        let address = self.base_address + 85;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ChargingStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ChargingStatus::new,
        )
    }
    pub fn gauging_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GaugingStatus, ::device_driver::RO> {
        let address = self.base_address + 86;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GaugingStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::GaugingStatus::new,
        )
    }
    pub fn manufacturing_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::ManufacturingStatus, ::device_driver::RO> {
        let address = self.base_address + 87;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::ManufacturingStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::ManufacturingStatus::new,
        )
    }
    pub fn afe_reg(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::AfeReg, ::device_driver::RO> {
        let address = self.base_address + 88;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::AfeReg, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::AfeReg::new,
        )
    }
    pub fn max_turbo_power(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::MaxTurboPower, ::device_driver::RW> {
        let address = self.base_address + 89;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::MaxTurboPower, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::MaxTurboPower::new,
        )
    }
    pub fn sus_turbo_power(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::SusTurboPower, ::device_driver::RW> {
        let address = self.base_address + 90;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::SusTurboPower, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::SusTurboPower::new,
        )
    }
    pub fn turbo_pack_r(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::TurboPackR, ::device_driver::RW> {
        let address = self.base_address + 91;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::TurboPackR, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::TurboPackR::new,
        )
    }
    pub fn turbo_sys_r(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::TurboSysR, ::device_driver::RW> {
        let address = self.base_address + 92;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::TurboSysR, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::TurboSysR::new,
        )
    }
    pub fn turbo_edv(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::TurboEdv, ::device_driver::RW> {
        let address = self.base_address + 93;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::TurboEdv, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::TurboEdv::new,
        )
    }
    pub fn turbo_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::TurboCurrent, ::device_driver::RW> {
        let address = self.base_address + 94;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::TurboCurrent, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::TurboCurrent::new,
        )
    }
    pub fn sus_turbo_current(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::SusTurboCurrent, ::device_driver::RW> {
        let address = self.base_address + 95;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::SusTurboCurrent, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::SusTurboCurrent::new,
        )
    }
    pub fn lifetime_data_block_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock1, ::device_driver::RO> {
        let address = self.base_address + 96;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock1::new,
        )
    }
    pub fn lifetime_data_block_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock2, ::device_driver::RO> {
        let address = self.base_address + 97;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock2::new,
        )
    }
    pub fn lifetime_data_block_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock3, ::device_driver::RO> {
        let address = self.base_address + 98;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock3::new,
        )
    }
    pub fn lifetime_data_block_4(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock4, ::device_driver::RO> {
        let address = self.base_address + 99;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock4, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock4::new,
        )
    }
    pub fn lifetime_data_block_5(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock5, ::device_driver::RO> {
        let address = self.base_address + 100;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock5, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock5::new,
        )
    }
    pub fn lifetime_data_block_6(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock6, ::device_driver::RW> {
        let address = self.base_address + 101;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock6, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock6::new,
        )
    }
    pub fn lifetime_data_block_7(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock7, ::device_driver::RW> {
        let address = self.base_address + 102;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock7, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock7::new,
        )
    }
    pub fn lifetime_data_block_8(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock8, ::device_driver::RW> {
        let address = self.base_address + 103;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock8, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock8::new,
        )
    }
    pub fn turbo_rhf_effective(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::TurboRhfEffective, ::device_driver::RO> {
        let address = self.base_address + 104;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::TurboRhfEffective, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::TurboRhfEffective::new,
        )
    }
    pub fn turbo_vload(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::TurboVload, ::device_driver::RO> {
        let address = self.base_address + 105;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::TurboVload, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::TurboVload::new,
        )
    }
    pub fn lifetime_data_block_11(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock11, ::device_driver::RW> {
        let address = self.base_address + 106;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock11, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock11::new,
        )
    }
    pub fn lifetime_data_block_12(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::LifetimeDataBlock12, ::device_driver::RW> {
        let address = self.base_address + 107;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::LifetimeDataBlock12, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::LifetimeDataBlock12::new,
        )
    }
    pub fn da_status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::DaStatus1, ::device_driver::RO> {
        let address = self.base_address + 113;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::DaStatus1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::DaStatus1::new,
        )
    }
    pub fn da_status_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::DaStatus2, ::device_driver::RO> {
        let address = self.base_address + 114;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::DaStatus2, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::DaStatus2::new,
        )
    }
    pub fn gauge_status_1(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GaugeStatus1, ::device_driver::RO> {
        let address = self.base_address + 115;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GaugeStatus1, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::GaugeStatus1::new,
        )
    }
    pub fn gauge_status_2(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GaugeStatus2, ::device_driver::RW> {
        let address = self.base_address + 116;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GaugeStatus2, ::device_driver::RW>::new(
            self.interface(),
            address as u8,
            field_sets::GaugeStatus2::new,
        )
    }
    pub fn gauge_status_3(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::GaugeStatus3, ::device_driver::RO> {
        let address = self.base_address + 117;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::GaugeStatus3, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::GaugeStatus3::new,
        )
    }
    pub fn cb_status(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::CbStatus, ::device_driver::RO> {
        let address = self.base_address + 118;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::CbStatus, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::CbStatus::new,
        )
    }
    pub fn state_of_health(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::StateOfHealth, ::device_driver::RO> {
        let address = self.base_address + 119;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::StateOfHealth, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::StateOfHealth::new,
        )
    }
    pub fn filter_capacity(
        &mut self,
    ) -> ::device_driver::RegisterOperation<'_, I, u8, field_sets::FilterCapacity, ::device_driver::RO> {
        let address = self.base_address + 120;
        ::device_driver::RegisterOperation::<'_, I, u8, field_sets::FilterCapacity, ::device_driver::RO>::new(
            self.interface(),
            address as u8,
            field_sets::FilterCapacity::new,
        )
    }
}
/// Module containing the generated fieldsets of the registers and commands
pub mod field_sets {
    #[allow(unused_imports)]
    use super::*;
    /// The device can be checked for the IC part number.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacDeviceTypeFieldsOut {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for MacDeviceTypeFieldsOut {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacDeviceTypeFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `device_type` field of the register.
        ///
        pub fn device_type(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `device_type` field of the register.
        ///
        pub fn set_device_type(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for MacDeviceTypeFieldsOut {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<MacDeviceTypeFieldsOut> for [u8; 2] {
        fn from(val: MacDeviceTypeFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacDeviceTypeFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacDeviceTypeFieldsOut");
            d.field("device_type", &self.device_type());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacDeviceTypeFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacDeviceTypeFieldsOut {{ ");
            defmt::write!(f, "device_type: {=u16}, ", &self.device_type());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacDeviceTypeFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacDeviceTypeFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacDeviceTypeFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacDeviceTypeFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacDeviceTypeFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacDeviceTypeFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacDeviceTypeFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// The device can be checked for the firmware version of the IC.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacFirmwareVersionFieldsOut {
        /// The internal bits
        bits: [u8; 11],
    }
    impl ::device_driver::FieldSet for MacFirmwareVersionFieldsOut {
        const SIZE_BITS: u32 = 88;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacFirmwareVersionFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 11] }
        }
        ///Read the `device_number` field of the register.
        ///
        pub fn device_number(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `version` field of the register.
        ///
        pub fn version(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `build_number` field of the register.
        ///
        pub fn build_number(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `firmware_type` field of the register.
        ///
        pub fn firmware_type(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 48, 56) };
            raw
        }
        ///Read the `impedence_track_vers` field of the register.
        ///
        pub fn impedence_track_vers(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 56, 72) };
            raw
        }
        ///Write the `device_number` field of the register.
        ///
        pub fn set_device_number(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `version` field of the register.
        ///
        pub fn set_version(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `build_number` field of the register.
        ///
        pub fn set_build_number(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `firmware_type` field of the register.
        ///
        pub fn set_firmware_type(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 48, 56, &mut self.bits) };
        }
        ///Write the `impedence_track_vers` field of the register.
        ///
        pub fn set_impedence_track_vers(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 56, 72, &mut self.bits) };
        }
    }
    impl From<[u8; 11]> for MacFirmwareVersionFieldsOut {
        fn from(bits: [u8; 11]) -> Self {
            Self { bits }
        }
    }
    impl From<MacFirmwareVersionFieldsOut> for [u8; 11] {
        fn from(val: MacFirmwareVersionFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacFirmwareVersionFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacFirmwareVersionFieldsOut");
            d.field("device_number", &self.device_number());
            d.field("version", &self.version());
            d.field("build_number", &self.build_number());
            d.field("firmware_type", &self.firmware_type());
            d.field("impedence_track_vers", &self.impedence_track_vers());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacFirmwareVersionFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacFirmwareVersionFieldsOut {{ ");
            defmt::write!(f, "device_number: {=u16}, ", &self.device_number());
            defmt::write!(f, "version: {=u16}, ", &self.version());
            defmt::write!(f, "build_number: {=u16}, ", &self.build_number());
            defmt::write!(f, "firmware_type: {=u8}, ", &self.firmware_type());
            defmt::write!(f, "impedence_track_vers: {=u16}, ", &self.impedence_track_vers());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacFirmwareVersionFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacFirmwareVersionFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacFirmwareVersionFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacFirmwareVersionFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacFirmwareVersionFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacFirmwareVersionFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacFirmwareVersionFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacHardwareVersionFieldsOut {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for MacHardwareVersionFieldsOut {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacHardwareVersionFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `hw_vers` field of the register.
        ///
        pub fn hw_vers(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `hw_vers` field of the register.
        ///
        pub fn set_hw_vers(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for MacHardwareVersionFieldsOut {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<MacHardwareVersionFieldsOut> for [u8; 1] {
        fn from(val: MacHardwareVersionFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacHardwareVersionFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacHardwareVersionFieldsOut");
            d.field("hw_vers", &self.hw_vers());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacHardwareVersionFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacHardwareVersionFieldsOut {{ ");
            defmt::write!(f, "hw_vers: {=u8}, ", &self.hw_vers());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacHardwareVersionFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacHardwareVersionFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacHardwareVersionFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacHardwareVersionFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacHardwareVersionFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacHardwareVersionFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacHardwareVersionFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacInstructionFlashSignatureFieldsOut {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for MacInstructionFlashSignatureFieldsOut {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacInstructionFlashSignatureFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `insn_flsh_sig` field of the register.
        ///
        pub fn insn_flsh_sig(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `insn_flsh_sig` field of the register.
        ///
        pub fn set_insn_flsh_sig(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for MacInstructionFlashSignatureFieldsOut {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<MacInstructionFlashSignatureFieldsOut> for [u8; 1] {
        fn from(val: MacInstructionFlashSignatureFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacInstructionFlashSignatureFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacInstructionFlashSignatureFieldsOut");
            d.field("insn_flsh_sig", &self.insn_flsh_sig());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacInstructionFlashSignatureFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacInstructionFlashSignatureFieldsOut {{ ");
            defmt::write!(f, "insn_flsh_sig: {=u8}, ", &self.insn_flsh_sig());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacInstructionFlashSignatureFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacInstructionFlashSignatureFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacInstructionFlashSignatureFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacInstructionFlashSignatureFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacInstructionFlashSignatureFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacInstructionFlashSignatureFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacInstructionFlashSignatureFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacStaticDfSignatureFieldsOut {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for MacStaticDfSignatureFieldsOut {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacStaticDfSignatureFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `static_df_sig` field of the register.
        ///
        pub fn static_df_sig(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 15) };
            raw
        }
        ///Read the `sig_mismatch` field of the register.
        ///
        pub fn sig_mismatch(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `static_df_sig` field of the register.
        ///
        pub fn set_static_df_sig(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 15, &mut self.bits) };
        }
        ///Write the `sig_mismatch` field of the register.
        ///
        pub fn set_sig_mismatch(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for MacStaticDfSignatureFieldsOut {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<MacStaticDfSignatureFieldsOut> for [u8; 2] {
        fn from(val: MacStaticDfSignatureFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacStaticDfSignatureFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacStaticDfSignatureFieldsOut");
            d.field("static_df_sig", &self.static_df_sig());
            d.field("sig_mismatch", &self.sig_mismatch());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacStaticDfSignatureFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacStaticDfSignatureFieldsOut {{ ");
            defmt::write!(f, "static_df_sig: {=u16}, ", &self.static_df_sig());
            defmt::write!(f, "sig_mismatch: {=bool}, ", &self.sig_mismatch());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacStaticDfSignatureFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacStaticDfSignatureFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacStaticDfSignatureFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacStaticDfSignatureFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacStaticDfSignatureFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacStaticDfSignatureFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacStaticDfSignatureFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacChemIdFieldsOut {
        /// The internal bits
        bits: [u8; 1],
    }
    impl ::device_driver::FieldSet for MacChemIdFieldsOut {
        const SIZE_BITS: u32 = 8;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacChemIdFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 1] }
        }
        ///Read the `chem_id` field of the register.
        ///
        pub fn chem_id(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Write the `chem_id` field of the register.
        ///
        pub fn set_chem_id(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
    }
    impl From<[u8; 1]> for MacChemIdFieldsOut {
        fn from(bits: [u8; 1]) -> Self {
            Self { bits }
        }
    }
    impl From<MacChemIdFieldsOut> for [u8; 1] {
        fn from(val: MacChemIdFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacChemIdFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacChemIdFieldsOut");
            d.field("chem_id", &self.chem_id());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacChemIdFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacChemIdFieldsOut {{ ");
            defmt::write!(f, "chem_id: {=u8}, ", &self.chem_id());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacChemIdFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacChemIdFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacChemIdFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacChemIdFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacChemIdFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacChemIdFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacChemIdFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacStaticChemDfSigFieldsOut {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for MacStaticChemDfSigFieldsOut {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacStaticChemDfSigFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `static_chem_df_sig` field of the register.
        ///
        pub fn static_chem_df_sig(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 15) };
            raw
        }
        ///Read the `sig_mismatch` field of the register.
        ///
        pub fn sig_mismatch(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `static_chem_df_sig` field of the register.
        ///
        pub fn set_static_chem_df_sig(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 15, &mut self.bits) };
        }
        ///Write the `sig_mismatch` field of the register.
        ///
        pub fn set_sig_mismatch(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for MacStaticChemDfSigFieldsOut {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<MacStaticChemDfSigFieldsOut> for [u8; 2] {
        fn from(val: MacStaticChemDfSigFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacStaticChemDfSigFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacStaticChemDfSigFieldsOut");
            d.field("static_chem_df_sig", &self.static_chem_df_sig());
            d.field("sig_mismatch", &self.sig_mismatch());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacStaticChemDfSigFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacStaticChemDfSigFieldsOut {{ ");
            defmt::write!(f, "static_chem_df_sig: {=u16}, ", &self.static_chem_df_sig());
            defmt::write!(f, "sig_mismatch: {=bool}, ", &self.sig_mismatch());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacStaticChemDfSigFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacStaticChemDfSigFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacStaticChemDfSigFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacStaticChemDfSigFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacStaticChemDfSigFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacStaticChemDfSigFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacStaticChemDfSigFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacAllDfSignatureFieldsOut {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for MacAllDfSignatureFieldsOut {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacAllDfSignatureFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `static_chem_df_sig` field of the register.
        ///
        pub fn static_chem_df_sig(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 15) };
            raw
        }
        ///Read the `sig_mismatch` field of the register.
        ///
        pub fn sig_mismatch(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `static_chem_df_sig` field of the register.
        ///
        pub fn set_static_chem_df_sig(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 15, &mut self.bits) };
        }
        ///Write the `sig_mismatch` field of the register.
        ///
        pub fn set_sig_mismatch(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for MacAllDfSignatureFieldsOut {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<MacAllDfSignatureFieldsOut> for [u8; 2] {
        fn from(val: MacAllDfSignatureFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacAllDfSignatureFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacAllDfSignatureFieldsOut");
            d.field("static_chem_df_sig", &self.static_chem_df_sig());
            d.field("sig_mismatch", &self.sig_mismatch());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacAllDfSignatureFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacAllDfSignatureFieldsOut {{ ");
            defmt::write!(f, "static_chem_df_sig: {=u16}, ", &self.static_chem_df_sig());
            defmt::write!(f, "sig_mismatch: {=bool}, ", &self.sig_mismatch());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacAllDfSignatureFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacAllDfSignatureFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacAllDfSignatureFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacAllDfSignatureFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacAllDfSignatureFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacAllDfSignatureFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacAllDfSignatureFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacSafetyAlertFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacSafetyAlertFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacSafetyAlertFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `cuv` field of the register.
        ///
        pub fn cuv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `cov` field of the register.
        ///
        pub fn cov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `occ_1` field of the register.
        ///
        pub fn occ_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `occ_2` field of the register.
        ///
        pub fn occ_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `ocd_1` field of the register.
        ///
        pub fn ocd_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `ocd_2` field of the register.
        ///
        pub fn ocd_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `aoldl` field of the register.
        ///
        pub fn aoldl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `asccl` field of the register.
        ///
        pub fn asccl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `ascdl` field of the register.
        ///
        pub fn ascdl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `otc` field of the register.
        ///
        pub fn otc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `otd` field of the register.
        ///
        pub fn otd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `cuvc` field of the register.
        ///
        pub fn cuvc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `otf` field of the register.
        ///
        pub fn otf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `pto` field of the register.
        ///
        pub fn pto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `ptos` field of the register.
        ///
        pub fn ptos(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `cto` field of the register.
        ///
        pub fn cto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `ctos` field of the register.
        ///
        pub fn ctos(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `oc` field of the register.
        ///
        pub fn oc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `chgc` field of the register.
        ///
        pub fn chgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `chgv` field of the register.
        ///
        pub fn chgv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `pchgc` field of the register.
        ///
        pub fn pchgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `utc` field of the register.
        ///
        pub fn utc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `utd` field of the register.
        ///
        pub fn utd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 27, 28) };
            raw > 0
        }
        ///Write the `cuv` field of the register.
        ///
        pub fn set_cuv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `cov` field of the register.
        ///
        pub fn set_cov(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `occ_1` field of the register.
        ///
        pub fn set_occ_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `occ_2` field of the register.
        ///
        pub fn set_occ_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `ocd_1` field of the register.
        ///
        pub fn set_ocd_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `ocd_2` field of the register.
        ///
        pub fn set_ocd_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `aoldl` field of the register.
        ///
        pub fn set_aoldl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `asccl` field of the register.
        ///
        pub fn set_asccl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `ascdl` field of the register.
        ///
        pub fn set_ascdl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `otc` field of the register.
        ///
        pub fn set_otc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `otd` field of the register.
        ///
        pub fn set_otd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `cuvc` field of the register.
        ///
        pub fn set_cuvc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `otf` field of the register.
        ///
        pub fn set_otf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 17, &mut self.bits) };
        }
        ///Write the `pto` field of the register.
        ///
        pub fn set_pto(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 18, 19, &mut self.bits) };
        }
        ///Write the `ptos` field of the register.
        ///
        pub fn set_ptos(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 19, 20, &mut self.bits) };
        }
        ///Write the `cto` field of the register.
        ///
        pub fn set_cto(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 20, 21, &mut self.bits) };
        }
        ///Write the `ctos` field of the register.
        ///
        pub fn set_ctos(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 21, 22, &mut self.bits) };
        }
        ///Write the `oc` field of the register.
        ///
        pub fn set_oc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 22, 23, &mut self.bits) };
        }
        ///Write the `chgc` field of the register.
        ///
        pub fn set_chgc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 23, 24, &mut self.bits) };
        }
        ///Write the `chgv` field of the register.
        ///
        pub fn set_chgv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 24, 25, &mut self.bits) };
        }
        ///Write the `pchgc` field of the register.
        ///
        pub fn set_pchgc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 25, 26, &mut self.bits) };
        }
        ///Write the `utc` field of the register.
        ///
        pub fn set_utc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 26, 27, &mut self.bits) };
        }
        ///Write the `utd` field of the register.
        ///
        pub fn set_utd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 27, 28, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacSafetyAlertFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacSafetyAlertFieldsOut> for [u8; 4] {
        fn from(val: MacSafetyAlertFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacSafetyAlertFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacSafetyAlertFieldsOut");
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
    impl defmt::Format for MacSafetyAlertFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacSafetyAlertFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacSafetyAlertFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacSafetyAlertFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacSafetyAlertFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacSafetyAlertFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacSafetyAlertFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacSafetyAlertFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacSafetyAlertFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacSafetyStatusFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacSafetyStatusFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacSafetyStatusFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `cuv` field of the register.
        ///
        pub fn cuv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `cov` field of the register.
        ///
        pub fn cov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `occ_1` field of the register.
        ///
        pub fn occ_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `occ_2` field of the register.
        ///
        pub fn occ_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `ocd_1` field of the register.
        ///
        pub fn ocd_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `ocd_2` field of the register.
        ///
        pub fn ocd_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `aold` field of the register.
        ///
        pub fn aold(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `aoldl` field of the register.
        ///
        pub fn aoldl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `ascc` field of the register.
        ///
        pub fn ascc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `asccl` field of the register.
        ///
        pub fn asccl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `ascd` field of the register.
        ///
        pub fn ascd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `ascdl` field of the register.
        ///
        pub fn ascdl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `otc` field of the register.
        ///
        pub fn otc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `otd` field of the register.
        ///
        pub fn otd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `cuvc` field of the register.
        ///
        pub fn cuvc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `otf` field of the register.
        ///
        pub fn otf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `pto` field of the register.
        ///
        pub fn pto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `ptos` field of the register.
        ///
        pub fn ptos(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `cto` field of the register.
        ///
        pub fn cto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `oc` field of the register.
        ///
        pub fn oc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `chgc` field of the register.
        ///
        pub fn chgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `chgv` field of the register.
        ///
        pub fn chgv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `pchgc` field of the register.
        ///
        pub fn pchgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `utc` field of the register.
        ///
        pub fn utc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `utd` field of the register.
        ///
        pub fn utd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 27, 28) };
            raw > 0
        }
        ///Write the `cuv` field of the register.
        ///
        pub fn set_cuv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `cov` field of the register.
        ///
        pub fn set_cov(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `occ_1` field of the register.
        ///
        pub fn set_occ_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `occ_2` field of the register.
        ///
        pub fn set_occ_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `ocd_1` field of the register.
        ///
        pub fn set_ocd_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `ocd_2` field of the register.
        ///
        pub fn set_ocd_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `aold` field of the register.
        ///
        pub fn set_aold(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `aoldl` field of the register.
        ///
        pub fn set_aoldl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `ascc` field of the register.
        ///
        pub fn set_ascc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `asccl` field of the register.
        ///
        pub fn set_asccl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `ascd` field of the register.
        ///
        pub fn set_ascd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `ascdl` field of the register.
        ///
        pub fn set_ascdl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `otc` field of the register.
        ///
        pub fn set_otc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `otd` field of the register.
        ///
        pub fn set_otd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `cuvc` field of the register.
        ///
        pub fn set_cuvc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `otf` field of the register.
        ///
        pub fn set_otf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 17, &mut self.bits) };
        }
        ///Write the `pto` field of the register.
        ///
        pub fn set_pto(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 18, 19, &mut self.bits) };
        }
        ///Write the `ptos` field of the register.
        ///
        pub fn set_ptos(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 19, 20, &mut self.bits) };
        }
        ///Write the `cto` field of the register.
        ///
        pub fn set_cto(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 20, 21, &mut self.bits) };
        }
        ///Write the `oc` field of the register.
        ///
        pub fn set_oc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 22, 23, &mut self.bits) };
        }
        ///Write the `chgc` field of the register.
        ///
        pub fn set_chgc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 23, 24, &mut self.bits) };
        }
        ///Write the `chgv` field of the register.
        ///
        pub fn set_chgv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 24, 25, &mut self.bits) };
        }
        ///Write the `pchgc` field of the register.
        ///
        pub fn set_pchgc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 25, 26, &mut self.bits) };
        }
        ///Write the `utc` field of the register.
        ///
        pub fn set_utc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 26, 27, &mut self.bits) };
        }
        ///Write the `utd` field of the register.
        ///
        pub fn set_utd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 27, 28, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacSafetyStatusFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacSafetyStatusFieldsOut> for [u8; 4] {
        fn from(val: MacSafetyStatusFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacSafetyStatusFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacSafetyStatusFieldsOut");
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
    impl defmt::Format for MacSafetyStatusFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacSafetyStatusFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacSafetyStatusFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacSafetyStatusFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacSafetyStatusFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacSafetyStatusFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacSafetyStatusFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacSafetyStatusFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacSafetyStatusFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacPfAlertFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacPfAlertFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacPfAlertFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `suv` field of the register.
        ///
        pub fn suv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `sov` field of the register.
        ///
        pub fn sov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `socc` field of the register.
        ///
        pub fn socc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `socd` field of the register.
        ///
        pub fn socd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `sot` field of the register.
        ///
        pub fn sot(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `sotf` field of the register.
        ///
        pub fn sotf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `qim` field of the register.
        ///
        pub fn qim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `cb` field of the register.
        ///
        pub fn cb(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `imp` field of the register.
        ///
        pub fn imp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `cd` field of the register.
        ///
        pub fn cd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `vimr` field of the register.
        ///
        pub fn vimr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `vima` field of the register.
        ///
        pub fn vima(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `cfetf` field of the register.
        ///
        pub fn cfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `dfetf` field of the register.
        ///
        pub fn dfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `fuse` field of the register.
        ///
        pub fn fuse(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `afer` field of the register.
        ///
        pub fn afer(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `afec` field of the register.
        ///
        pub fn afec(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `second_lvl` field of the register.
        ///
        pub fn second_lvl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `opnc` field of the register.
        ///
        pub fn opnc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `ts_1` field of the register.
        ///
        pub fn ts_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 28, 29) };
            raw > 0
        }
        ///Read the `ts_2` field of the register.
        ///
        pub fn ts_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 29, 30) };
            raw > 0
        }
        ///Read the `ts_3` field of the register.
        ///
        pub fn ts_3(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 30, 31) };
            raw > 0
        }
        ///Read the `ts_4` field of the register.
        ///
        pub fn ts_4(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 31, 32) };
            raw > 0
        }
        ///Write the `suv` field of the register.
        ///
        pub fn set_suv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `sov` field of the register.
        ///
        pub fn set_sov(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `socc` field of the register.
        ///
        pub fn set_socc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `socd` field of the register.
        ///
        pub fn set_socd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `sot` field of the register.
        ///
        pub fn set_sot(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `sotf` field of the register.
        ///
        pub fn set_sotf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `qim` field of the register.
        ///
        pub fn set_qim(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `cb` field of the register.
        ///
        pub fn set_cb(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `imp` field of the register.
        ///
        pub fn set_imp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `cd` field of the register.
        ///
        pub fn set_cd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `vimr` field of the register.
        ///
        pub fn set_vimr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `vima` field of the register.
        ///
        pub fn set_vima(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `cfetf` field of the register.
        ///
        pub fn set_cfetf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 17, &mut self.bits) };
        }
        ///Write the `dfetf` field of the register.
        ///
        pub fn set_dfetf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 17, 18, &mut self.bits) };
        }
        ///Write the `fuse` field of the register.
        ///
        pub fn set_fuse(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 19, 20, &mut self.bits) };
        }
        ///Write the `afer` field of the register.
        ///
        pub fn set_afer(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 20, 21, &mut self.bits) };
        }
        ///Write the `afec` field of the register.
        ///
        pub fn set_afec(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 21, 22, &mut self.bits) };
        }
        ///Write the `second_lvl` field of the register.
        ///
        pub fn set_second_lvl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 22, 23, &mut self.bits) };
        }
        ///Write the `opnc` field of the register.
        ///
        pub fn set_opnc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 25, 26, &mut self.bits) };
        }
        ///Write the `ts_1` field of the register.
        ///
        pub fn set_ts_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 28, 29, &mut self.bits) };
        }
        ///Write the `ts_2` field of the register.
        ///
        pub fn set_ts_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 29, 30, &mut self.bits) };
        }
        ///Write the `ts_3` field of the register.
        ///
        pub fn set_ts_3(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 30, 31, &mut self.bits) };
        }
        ///Write the `ts_4` field of the register.
        ///
        pub fn set_ts_4(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 31, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacPfAlertFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacPfAlertFieldsOut> for [u8; 4] {
        fn from(val: MacPfAlertFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacPfAlertFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacPfAlertFieldsOut");
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
    impl defmt::Format for MacPfAlertFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacPfAlertFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacPfAlertFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacPfAlertFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacPfAlertFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacPfAlertFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacPfAlertFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacPfAlertFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacPfAlertFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacPfStatusFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacPfStatusFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacPfStatusFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `suv` field of the register.
        ///
        pub fn suv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `sov` field of the register.
        ///
        pub fn sov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `socc` field of the register.
        ///
        pub fn socc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `socd` field of the register.
        ///
        pub fn socd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `sot` field of the register.
        ///
        pub fn sot(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `sotf` field of the register.
        ///
        pub fn sotf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `qim` field of the register.
        ///
        pub fn qim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `cb` field of the register.
        ///
        pub fn cb(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `imp` field of the register.
        ///
        pub fn imp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `cd` field of the register.
        ///
        pub fn cd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `vimr` field of the register.
        ///
        pub fn vimr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `vima` field of the register.
        ///
        pub fn vima(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `cfetf` field of the register.
        ///
        pub fn cfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `dfetf` field of the register.
        ///
        pub fn dfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `fuse` field of the register.
        ///
        pub fn fuse(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `afer` field of the register.
        ///
        pub fn afer(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `afec` field of the register.
        ///
        pub fn afec(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `second_lvl` field of the register.
        ///
        pub fn second_lvl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `ptc` field of the register.
        ///
        pub fn ptc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `ifc` field of the register.
        ///
        pub fn ifc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `opncell` field of the register.
        ///
        pub fn opncell(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `dfw` field of the register.
        ///
        pub fn dfw(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `ts_1` field of the register.
        ///
        pub fn ts_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 28, 29) };
            raw > 0
        }
        ///Read the `ts_2` field of the register.
        ///
        pub fn ts_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 29, 30) };
            raw > 0
        }
        ///Read the `ts_3` field of the register.
        ///
        pub fn ts_3(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 30, 31) };
            raw > 0
        }
        ///Read the `ts_4` field of the register.
        ///
        pub fn ts_4(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 31, 32) };
            raw > 0
        }
        ///Write the `suv` field of the register.
        ///
        pub fn set_suv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `sov` field of the register.
        ///
        pub fn set_sov(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `socc` field of the register.
        ///
        pub fn set_socc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `socd` field of the register.
        ///
        pub fn set_socd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `sot` field of the register.
        ///
        pub fn set_sot(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `sotf` field of the register.
        ///
        pub fn set_sotf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `qim` field of the register.
        ///
        pub fn set_qim(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `cb` field of the register.
        ///
        pub fn set_cb(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `imp` field of the register.
        ///
        pub fn set_imp(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `cd` field of the register.
        ///
        pub fn set_cd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `vimr` field of the register.
        ///
        pub fn set_vimr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `vima` field of the register.
        ///
        pub fn set_vima(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `cfetf` field of the register.
        ///
        pub fn set_cfetf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 17, &mut self.bits) };
        }
        ///Write the `dfetf` field of the register.
        ///
        pub fn set_dfetf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 17, 18, &mut self.bits) };
        }
        ///Write the `fuse` field of the register.
        ///
        pub fn set_fuse(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 19, 20, &mut self.bits) };
        }
        ///Write the `afer` field of the register.
        ///
        pub fn set_afer(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 20, 21, &mut self.bits) };
        }
        ///Write the `afec` field of the register.
        ///
        pub fn set_afec(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 21, 22, &mut self.bits) };
        }
        ///Write the `second_lvl` field of the register.
        ///
        pub fn set_second_lvl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 22, 23, &mut self.bits) };
        }
        ///Write the `ptc` field of the register.
        ///
        pub fn set_ptc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 23, 24, &mut self.bits) };
        }
        ///Write the `ifc` field of the register.
        ///
        pub fn set_ifc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 24, 25, &mut self.bits) };
        }
        ///Write the `opncell` field of the register.
        ///
        pub fn set_opncell(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 25, 26, &mut self.bits) };
        }
        ///Write the `dfw` field of the register.
        ///
        pub fn set_dfw(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 26, 27, &mut self.bits) };
        }
        ///Write the `ts_1` field of the register.
        ///
        pub fn set_ts_1(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 28, 29, &mut self.bits) };
        }
        ///Write the `ts_2` field of the register.
        ///
        pub fn set_ts_2(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 29, 30, &mut self.bits) };
        }
        ///Write the `ts_3` field of the register.
        ///
        pub fn set_ts_3(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 30, 31, &mut self.bits) };
        }
        ///Write the `ts_4` field of the register.
        ///
        pub fn set_ts_4(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 31, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacPfStatusFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacPfStatusFieldsOut> for [u8; 4] {
        fn from(val: MacPfStatusFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacPfStatusFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacPfStatusFieldsOut");
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
    impl defmt::Format for MacPfStatusFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacPfStatusFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacPfStatusFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacPfStatusFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacPfStatusFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacPfStatusFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacPfStatusFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacPfStatusFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacPfStatusFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacOperationStatusFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacOperationStatusFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacOperationStatusFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `pres` field of the register.
        ///
        pub fn pres(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `dsg` field of the register.
        ///
        pub fn dsg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `chg` field of the register.
        ///
        pub fn chg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `pchg` field of the register.
        ///
        pub fn pchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fuse` field of the register.
        ///
        pub fn fuse(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `btp_int` field of the register.
        ///
        pub fn btp_int(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `sec` field of the register.
        ///
        pub fn sec(&self) -> super::MacSecurityMode {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `sdv` field of the register.
        ///
        pub fn sdv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `ss` field of the register.
        ///
        pub fn ss(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `pf` field of the register.
        ///
        pub fn pf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `xdsg` field of the register.
        ///
        pub fn xdsg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `xchg` field of the register.
        ///
        pub fn xchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `sleep` field of the register.
        ///
        pub fn sleep(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `sdm` field of the register.
        ///
        pub fn sdm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `led` field of the register.
        ///
        pub fn led(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `auth` field of the register.
        ///
        pub fn auth(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `autocalm` field of the register.
        ///
        pub fn autocalm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `cal` field of the register.
        ///
        pub fn cal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `cal_offset` field of the register.
        ///
        pub fn cal_offset(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `xl` field of the register.
        ///
        pub fn xl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `sleepm` field of the register.
        ///
        pub fn sleepm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `init` field of the register.
        ///
        pub fn init(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `smblcal` field of the register.
        ///
        pub fn smblcal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `slpad` field of the register.
        ///
        pub fn slpad(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `slpcc` field of the register.
        ///
        pub fn slpcc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 27, 28) };
            raw > 0
        }
        ///Read the `cb` field of the register.
        ///
        pub fn cb(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 28, 29) };
            raw > 0
        }
        ///Read the `emshut` field of the register.
        ///
        pub fn emshut(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 29, 30) };
            raw > 0
        }
        ///Write the `pres` field of the register.
        ///
        pub fn set_pres(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `dsg` field of the register.
        ///
        pub fn set_dsg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `chg` field of the register.
        ///
        pub fn set_chg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `pchg` field of the register.
        ///
        pub fn set_pchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `fuse` field of the register.
        ///
        pub fn set_fuse(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `btp_int` field of the register.
        ///
        pub fn set_btp_int(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `sec` field of the register.
        ///
        pub fn set_sec(&mut self, value: super::MacSecurityMode) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 10, &mut self.bits) };
        }
        ///Write the `sdv` field of the register.
        ///
        pub fn set_sdv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `ss` field of the register.
        ///
        pub fn set_ss(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `pf` field of the register.
        ///
        pub fn set_pf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `xdsg` field of the register.
        ///
        pub fn set_xdsg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `xchg` field of the register.
        ///
        pub fn set_xchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `sleep` field of the register.
        ///
        pub fn set_sleep(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
        ///Write the `sdm` field of the register.
        ///
        pub fn set_sdm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 17, &mut self.bits) };
        }
        ///Write the `led` field of the register.
        ///
        pub fn set_led(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 17, 18, &mut self.bits) };
        }
        ///Write the `auth` field of the register.
        ///
        pub fn set_auth(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 18, 19, &mut self.bits) };
        }
        ///Write the `autocalm` field of the register.
        ///
        pub fn set_autocalm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 19, 20, &mut self.bits) };
        }
        ///Write the `cal` field of the register.
        ///
        pub fn set_cal(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 20, 21, &mut self.bits) };
        }
        ///Write the `cal_offset` field of the register.
        ///
        pub fn set_cal_offset(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 21, 22, &mut self.bits) };
        }
        ///Write the `xl` field of the register.
        ///
        pub fn set_xl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 22, 23, &mut self.bits) };
        }
        ///Write the `sleepm` field of the register.
        ///
        pub fn set_sleepm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 23, 24, &mut self.bits) };
        }
        ///Write the `init` field of the register.
        ///
        pub fn set_init(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 24, 25, &mut self.bits) };
        }
        ///Write the `smblcal` field of the register.
        ///
        pub fn set_smblcal(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 25, 26, &mut self.bits) };
        }
        ///Write the `slpad` field of the register.
        ///
        pub fn set_slpad(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 26, 27, &mut self.bits) };
        }
        ///Write the `slpcc` field of the register.
        ///
        pub fn set_slpcc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 27, 28, &mut self.bits) };
        }
        ///Write the `cb` field of the register.
        ///
        pub fn set_cb(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 28, 29, &mut self.bits) };
        }
        ///Write the `emshut` field of the register.
        ///
        pub fn set_emshut(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 29, 30, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacOperationStatusFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacOperationStatusFieldsOut> for [u8; 4] {
        fn from(val: MacOperationStatusFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacOperationStatusFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacOperationStatusFieldsOut");
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
    impl defmt::Format for MacOperationStatusFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacOperationStatusFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacOperationStatusFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacOperationStatusFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacOperationStatusFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacOperationStatusFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacOperationStatusFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacOperationStatusFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacOperationStatusFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacChargingStatusFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacChargingStatusFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacChargingStatusFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `ut` field of the register.
        ///
        pub fn ut(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `lt` field of the register.
        ///
        pub fn lt(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `stl` field of the register.
        ///
        pub fn stl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `rt` field of the register.
        ///
        pub fn rt(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `sth` field of the register.
        ///
        pub fn sth(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `ht` field of the register.
        ///
        pub fn ht(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `ot` field of the register.
        ///
        pub fn ot(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `pv` field of the register.
        ///
        pub fn pv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `lv` field of the register.
        ///
        pub fn lv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `mv` field of the register.
        ///
        pub fn mv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `hv` field of the register.
        ///
        pub fn hv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `chg_in` field of the register.
        ///
        pub fn chg_in(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `chg_su` field of the register.
        ///
        pub fn chg_su(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `mchg` field of the register.
        ///
        pub fn mchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `vct` field of the register.
        ///
        pub fn vct(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `ccr` field of the register.
        ///
        pub fn ccr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `cvr` field of the register.
        ///
        pub fn cvr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `ccc` field of the register.
        ///
        pub fn ccc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `nct` field of the register.
        ///
        pub fn nct(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `erm` field of the register.
        ///
        pub fn erm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `eretm` field of the register.
        ///
        pub fn eretm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Write the `ut` field of the register.
        ///
        pub fn set_ut(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `lt` field of the register.
        ///
        pub fn set_lt(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `stl` field of the register.
        ///
        pub fn set_stl(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `rt` field of the register.
        ///
        pub fn set_rt(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `sth` field of the register.
        ///
        pub fn set_sth(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `ht` field of the register.
        ///
        pub fn set_ht(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `ot` field of the register.
        ///
        pub fn set_ot(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `pv` field of the register.
        ///
        pub fn set_pv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `lv` field of the register.
        ///
        pub fn set_lv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `mv` field of the register.
        ///
        pub fn set_mv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `hv` field of the register.
        ///
        pub fn set_hv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `chg_in` field of the register.
        ///
        pub fn set_chg_in(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `chg_su` field of the register.
        ///
        pub fn set_chg_su(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `mchg` field of the register.
        ///
        pub fn set_mchg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `vct` field of the register.
        ///
        pub fn set_vct(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
        ///Write the `ccr` field of the register.
        ///
        pub fn set_ccr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 17, &mut self.bits) };
        }
        ///Write the `cvr` field of the register.
        ///
        pub fn set_cvr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 17, 18, &mut self.bits) };
        }
        ///Write the `ccc` field of the register.
        ///
        pub fn set_ccc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 18, 19, &mut self.bits) };
        }
        ///Write the `nct` field of the register.
        ///
        pub fn set_nct(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 19, 20, &mut self.bits) };
        }
        ///Write the `erm` field of the register.
        ///
        pub fn set_erm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 20, 21, &mut self.bits) };
        }
        ///Write the `eretm` field of the register.
        ///
        pub fn set_eretm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 21, 22, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacChargingStatusFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacChargingStatusFieldsOut> for [u8; 4] {
        fn from(val: MacChargingStatusFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacChargingStatusFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacChargingStatusFieldsOut");
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
            d.field("chg_su", &self.chg_su());
            d.field("mchg", &self.mchg());
            d.field("vct", &self.vct());
            d.field("ccr", &self.ccr());
            d.field("cvr", &self.cvr());
            d.field("ccc", &self.ccc());
            d.field("nct", &self.nct());
            d.field("erm", &self.erm());
            d.field("eretm", &self.eretm());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacChargingStatusFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacChargingStatusFieldsOut {{ ");
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
            defmt::write!(f, "chg_su: {=bool}, ", &self.chg_su());
            defmt::write!(f, "mchg: {=bool}, ", &self.mchg());
            defmt::write!(f, "vct: {=bool}, ", &self.vct());
            defmt::write!(f, "ccr: {=bool}, ", &self.ccr());
            defmt::write!(f, "cvr: {=bool}, ", &self.cvr());
            defmt::write!(f, "ccc: {=bool}, ", &self.ccc());
            defmt::write!(f, "nct: {=bool}, ", &self.nct());
            defmt::write!(f, "erm: {=bool}, ", &self.erm());
            defmt::write!(f, "eretm: {=bool}, ", &self.eretm());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacChargingStatusFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacChargingStatusFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacChargingStatusFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacChargingStatusFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacChargingStatusFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacChargingStatusFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacChargingStatusFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacGaugingStatusFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacGaugingStatusFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacGaugingStatusFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `fd` field of the register.
        ///
        pub fn fd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `fc` field of the register.
        ///
        pub fn fc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `td` field of the register.
        ///
        pub fn td(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `tc` field of the register.
        ///
        pub fn tc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `bal_en` field of the register.
        ///
        pub fn bal_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `edv` field of the register.
        ///
        pub fn edv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `dsg` field of the register.
        ///
        pub fn dsg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `cf` field of the register.
        ///
        pub fn cf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `rest` field of the register.
        ///
        pub fn rest(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `r_dis` field of the register.
        ///
        pub fn r_dis(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `vok` field of the register.
        ///
        pub fn vok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `qen` field of the register.
        ///
        pub fn qen(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `slpqmax` field of the register.
        ///
        pub fn slpqmax(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `nsfm` field of the register.
        ///
        pub fn nsfm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `vdq` field of the register.
        ///
        pub fn vdq(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `qmax` field of the register.
        ///
        pub fn qmax(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `rx` field of the register.
        ///
        pub fn rx(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `ldmd` field of the register.
        ///
        pub fn ldmd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `ocvfr` field of the register.
        ///
        pub fn ocvfr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Write the `fd` field of the register.
        ///
        pub fn set_fd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `fc` field of the register.
        ///
        pub fn set_fc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `td` field of the register.
        ///
        pub fn set_td(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `tc` field of the register.
        ///
        pub fn set_tc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `bal_en` field of the register.
        ///
        pub fn set_bal_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `edv` field of the register.
        ///
        pub fn set_edv(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `dsg` field of the register.
        ///
        pub fn set_dsg(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `cf` field of the register.
        ///
        pub fn set_cf(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `rest` field of the register.
        ///
        pub fn set_rest(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `r_dis` field of the register.
        ///
        pub fn set_r_dis(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `vok` field of the register.
        ///
        pub fn set_vok(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `qen` field of the register.
        ///
        pub fn set_qen(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 12, 13, &mut self.bits) };
        }
        ///Write the `slpqmax` field of the register.
        ///
        pub fn set_slpqmax(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `nsfm` field of the register.
        ///
        pub fn set_nsfm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
        ///Write the `vdq` field of the register.
        ///
        pub fn set_vdq(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 17, &mut self.bits) };
        }
        ///Write the `qmax` field of the register.
        ///
        pub fn set_qmax(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 17, 18, &mut self.bits) };
        }
        ///Write the `rx` field of the register.
        ///
        pub fn set_rx(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 18, 19, &mut self.bits) };
        }
        ///Write the `ldmd` field of the register.
        ///
        pub fn set_ldmd(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 19, 20, &mut self.bits) };
        }
        ///Write the `ocvfr` field of the register.
        ///
        pub fn set_ocvfr(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 20, 21, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacGaugingStatusFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacGaugingStatusFieldsOut> for [u8; 4] {
        fn from(val: MacGaugingStatusFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacGaugingStatusFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacGaugingStatusFieldsOut");
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
    impl defmt::Format for MacGaugingStatusFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacGaugingStatusFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacGaugingStatusFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacGaugingStatusFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacGaugingStatusFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacGaugingStatusFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacGaugingStatusFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacGaugingStatusFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacGaugingStatusFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacManufacturingStatusFieldsOut {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for MacManufacturingStatusFieldsOut {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacManufacturingStatusFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `pchg_en` field of the register.
        ///
        pub fn pchg_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `chg_en` field of the register.
        ///
        pub fn chg_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `dsg_en` field of the register.
        ///
        pub fn dsg_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `gauge_en` field of the register.
        ///
        pub fn gauge_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fet_en` field of the register.
        ///
        pub fn fet_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `lf_en` field of the register.
        ///
        pub fn lf_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `pf_en` field of the register.
        ///
        pub fn pf_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `bbr_en` field of the register.
        ///
        pub fn bbr_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `fuse_en` field of the register.
        ///
        pub fn fuse_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `led_en` field of the register.
        ///
        pub fn led_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `lt_test` field of the register.
        ///
        pub fn lt_test(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `cal_test` field of the register.
        ///
        pub fn cal_test(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `pchg_en` field of the register.
        ///
        pub fn set_pchg_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 1, &mut self.bits) };
        }
        ///Write the `chg_en` field of the register.
        ///
        pub fn set_chg_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 1, 2, &mut self.bits) };
        }
        ///Write the `dsg_en` field of the register.
        ///
        pub fn set_dsg_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 2, 3, &mut self.bits) };
        }
        ///Write the `gauge_en` field of the register.
        ///
        pub fn set_gauge_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 3, 4, &mut self.bits) };
        }
        ///Write the `fet_en` field of the register.
        ///
        pub fn set_fet_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 5, &mut self.bits) };
        }
        ///Write the `lf_en` field of the register.
        ///
        pub fn set_lf_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 5, 6, &mut self.bits) };
        }
        ///Write the `pf_en` field of the register.
        ///
        pub fn set_pf_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 6, 7, &mut self.bits) };
        }
        ///Write the `bbr_en` field of the register.
        ///
        pub fn set_bbr_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 7, 8, &mut self.bits) };
        }
        ///Write the `fuse_en` field of the register.
        ///
        pub fn set_fuse_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `led_en` field of the register.
        ///
        pub fn set_led_en(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `lt_test` field of the register.
        ///
        pub fn set_lt_test(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `cal_test` field of the register.
        ///
        pub fn set_cal_test(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for MacManufacturingStatusFieldsOut {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<MacManufacturingStatusFieldsOut> for [u8; 2] {
        fn from(val: MacManufacturingStatusFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacManufacturingStatusFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacManufacturingStatusFieldsOut");
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
    impl defmt::Format for MacManufacturingStatusFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacManufacturingStatusFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacManufacturingStatusFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacManufacturingStatusFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacManufacturingStatusFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacManufacturingStatusFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacManufacturingStatusFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacManufacturingStatusFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacManufacturingStatusFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacAfeRegFieldsOut {
        /// The internal bits
        bits: [u8; 21],
    }
    impl ::device_driver::FieldSet for MacAfeRegFieldsOut {
        const SIZE_BITS: u32 = 168;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacAfeRegFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 21] }
        }
        ///Read the `afe_int_status` field of the register.
        ///
        pub fn afe_int_status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `afe_fet_status` field of the register.
        ///
        pub fn afe_fet_status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Read the `afe_rxin` field of the register.
        ///
        pub fn afe_rxin(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 24) };
            raw
        }
        ///Read the `afe_latch_status` field of the register.
        ///
        pub fn afe_latch_status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 32) };
            raw
        }
        ///Read the `afe_int_en` field of the register.
        ///
        pub fn afe_int_en(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 32, 40) };
            raw
        }
        ///Read the `afe_ctrl` field of the register.
        ///
        pub fn afe_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 40, 48) };
            raw
        }
        ///Read the `afe_rxien` field of the register.
        ///
        pub fn afe_rxien(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 48, 56) };
            raw
        }
        ///Read the `afe_rlout` field of the register.
        ///
        pub fn afe_rlout(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 56, 64) };
            raw
        }
        ///Read the `afe_rhout` field of the register.
        ///
        pub fn afe_rhout(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 64, 72) };
            raw
        }
        ///Read the `afe_rhint` field of the register.
        ///
        pub fn afe_rhint(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 72, 80) };
            raw
        }
        ///Read the `afe_cell_balance` field of the register.
        ///
        pub fn afe_cell_balance(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 80, 88) };
            raw
        }
        ///Read the `afe_adc_cc_ctrl` field of the register.
        ///
        pub fn afe_adc_cc_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 88, 96) };
            raw
        }
        ///Read the `afe_adc_mux_ctrl` field of the register.
        ///
        pub fn afe_adc_mux_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 96, 104) };
            raw
        }
        ///Read the `afe_led_ctrl` field of the register.
        ///
        pub fn afe_led_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 104, 112) };
            raw
        }
        ///Read the `afe_hw_ctrl` field of the register.
        ///
        pub fn afe_hw_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 112, 120) };
            raw
        }
        ///Read the `afe_tmr_ctrl` field of the register.
        ///
        pub fn afe_tmr_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 120, 128) };
            raw
        }
        ///Read the `afe_protection` field of the register.
        ///
        pub fn afe_protection(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 128, 136) };
            raw
        }
        ///Read the `afe_ocd` field of the register.
        ///
        pub fn afe_ocd(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 136, 144) };
            raw
        }
        ///Read the `afe_scc` field of the register.
        ///
        pub fn afe_scc(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 144, 152) };
            raw
        }
        ///Read the `afe_scd_1` field of the register.
        ///
        pub fn afe_scd_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 152, 160) };
            raw
        }
        ///Read the `afe_scd_2` field of the register.
        ///
        pub fn afe_scd_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 160, 168) };
            raw
        }
        ///Write the `afe_int_status` field of the register.
        ///
        pub fn set_afe_int_status(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
        ///Write the `afe_fet_status` field of the register.
        ///
        pub fn set_afe_fet_status(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 16, &mut self.bits) };
        }
        ///Write the `afe_rxin` field of the register.
        ///
        pub fn set_afe_rxin(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 24, &mut self.bits) };
        }
        ///Write the `afe_latch_status` field of the register.
        ///
        pub fn set_afe_latch_status(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 24, 32, &mut self.bits) };
        }
        ///Write the `afe_int_en` field of the register.
        ///
        pub fn set_afe_int_en(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 32, 40, &mut self.bits) };
        }
        ///Write the `afe_ctrl` field of the register.
        ///
        pub fn set_afe_ctrl(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 40, 48, &mut self.bits) };
        }
        ///Write the `afe_rxien` field of the register.
        ///
        pub fn set_afe_rxien(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 48, 56, &mut self.bits) };
        }
        ///Write the `afe_rlout` field of the register.
        ///
        pub fn set_afe_rlout(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 56, 64, &mut self.bits) };
        }
        ///Write the `afe_rhout` field of the register.
        ///
        pub fn set_afe_rhout(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 64, 72, &mut self.bits) };
        }
        ///Write the `afe_rhint` field of the register.
        ///
        pub fn set_afe_rhint(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 72, 80, &mut self.bits) };
        }
        ///Write the `afe_cell_balance` field of the register.
        ///
        pub fn set_afe_cell_balance(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 80, 88, &mut self.bits) };
        }
        ///Write the `afe_adc_cc_ctrl` field of the register.
        ///
        pub fn set_afe_adc_cc_ctrl(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 88, 96, &mut self.bits) };
        }
        ///Write the `afe_adc_mux_ctrl` field of the register.
        ///
        pub fn set_afe_adc_mux_ctrl(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 96, 104, &mut self.bits) };
        }
        ///Write the `afe_led_ctrl` field of the register.
        ///
        pub fn set_afe_led_ctrl(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 104, 112, &mut self.bits) };
        }
        ///Write the `afe_hw_ctrl` field of the register.
        ///
        pub fn set_afe_hw_ctrl(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 112, 120, &mut self.bits) };
        }
        ///Write the `afe_tmr_ctrl` field of the register.
        ///
        pub fn set_afe_tmr_ctrl(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 120, 128, &mut self.bits) };
        }
        ///Write the `afe_protection` field of the register.
        ///
        pub fn set_afe_protection(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 128, 136, &mut self.bits) };
        }
        ///Write the `afe_ocd` field of the register.
        ///
        pub fn set_afe_ocd(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 136, 144, &mut self.bits) };
        }
        ///Write the `afe_scc` field of the register.
        ///
        pub fn set_afe_scc(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 144, 152, &mut self.bits) };
        }
        ///Write the `afe_scd_1` field of the register.
        ///
        pub fn set_afe_scd_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 152, 160, &mut self.bits) };
        }
        ///Write the `afe_scd_2` field of the register.
        ///
        pub fn set_afe_scd_2(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 160, 168, &mut self.bits) };
        }
    }
    impl From<[u8; 21]> for MacAfeRegFieldsOut {
        fn from(bits: [u8; 21]) -> Self {
            Self { bits }
        }
    }
    impl From<MacAfeRegFieldsOut> for [u8; 21] {
        fn from(val: MacAfeRegFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacAfeRegFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacAfeRegFieldsOut");
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
    impl defmt::Format for MacAfeRegFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacAfeRegFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacAfeRegFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacAfeRegFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacAfeRegFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacAfeRegFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacAfeRegFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacAfeRegFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacAfeRegFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct NoLoadRemCapFieldsOut {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for NoLoadRemCapFieldsOut {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl NoLoadRemCapFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `remaining_capacity` field of the register.
        ///
        pub fn remaining_capacity(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `remaining_capacity` field of the register.
        ///
        pub fn set_remaining_capacity(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for NoLoadRemCapFieldsOut {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<NoLoadRemCapFieldsOut> for [u8; 2] {
        fn from(val: NoLoadRemCapFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for NoLoadRemCapFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("NoLoadRemCapFieldsOut");
            d.field("remaining_capacity", &self.remaining_capacity());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for NoLoadRemCapFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "NoLoadRemCapFieldsOut {{ ");
            defmt::write!(f, "remaining_capacity: {=u16}, ", &self.remaining_capacity());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for NoLoadRemCapFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for NoLoadRemCapFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for NoLoadRemCapFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for NoLoadRemCapFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for NoLoadRemCapFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for NoLoadRemCapFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for NoLoadRemCapFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock1FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock1FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock1FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `cell_1_max_v` field of the register.
        ///
        pub fn cell_1_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `cell_2_max_v` field of the register.
        ///
        pub fn cell_2_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cell_3_max_v` field of the register.
        ///
        pub fn cell_3_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cell_4_max_v` field of the register.
        ///
        pub fn cell_4_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `cell_1_min_v` field of the register.
        ///
        pub fn cell_1_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_2_min_v` field of the register.
        ///
        pub fn cell_2_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `cell_3_min_v` field of the register.
        ///
        pub fn cell_3_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `cell_4_min_v` field of the register.
        ///
        pub fn cell_4_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `max_delta_cell_v` field of the register.
        ///
        pub fn max_delta_cell_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `max_charge_a` field of the register.
        ///
        pub fn max_charge_a(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `max_discharge_a` field of the register.
        ///
        pub fn max_discharge_a(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `max_avg_discharge_a` field of the register.
        ///
        pub fn max_avg_discharge_a(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `max_avg_discharge_pwr` field of the register.
        ///
        pub fn max_avg_discharge_pwr(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `max_temp_cell` field of the register.
        ///
        pub fn max_temp_cell(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 208, 216) };
            raw
        }
        ///Read the `min_temp_cell` field of the register.
        ///
        pub fn min_temp_cell(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 216, 224) };
            raw
        }
        ///Read the `max_delta_cell_temp` field of the register.
        ///
        pub fn max_delta_cell_temp(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 224, 232) };
            raw
        }
        ///Read the `max_temp_int_sensor` field of the register.
        ///
        pub fn max_temp_int_sensor(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 232, 240) };
            raw
        }
        ///Read the `min_temp_int_sensor` field of the register.
        ///
        pub fn min_temp_int_sensor(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 240, 248) };
            raw
        }
        ///Read the `max_temp_fet` field of the register.
        ///
        pub fn max_temp_fet(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 248, 256) };
            raw
        }
        ///Write the `cell_1_max_v` field of the register.
        ///
        pub fn set_cell_1_max_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `cell_2_max_v` field of the register.
        ///
        pub fn set_cell_2_max_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `cell_3_max_v` field of the register.
        ///
        pub fn set_cell_3_max_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `cell_4_max_v` field of the register.
        ///
        pub fn set_cell_4_max_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `cell_1_min_v` field of the register.
        ///
        pub fn set_cell_1_min_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `cell_2_min_v` field of the register.
        ///
        pub fn set_cell_2_min_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `cell_3_min_v` field of the register.
        ///
        pub fn set_cell_3_min_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `cell_4_min_v` field of the register.
        ///
        pub fn set_cell_4_min_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `max_delta_cell_v` field of the register.
        ///
        pub fn set_max_delta_cell_v(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `max_charge_a` field of the register.
        ///
        pub fn set_max_charge_a(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `max_discharge_a` field of the register.
        ///
        pub fn set_max_discharge_a(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `max_avg_discharge_a` field of the register.
        ///
        pub fn set_max_avg_discharge_a(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
        ///Write the `max_avg_discharge_pwr` field of the register.
        ///
        pub fn set_max_avg_discharge_pwr(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 192, 208, &mut self.bits) };
        }
        ///Write the `max_temp_cell` field of the register.
        ///
        pub fn set_max_temp_cell(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 208, 216, &mut self.bits) };
        }
        ///Write the `min_temp_cell` field of the register.
        ///
        pub fn set_min_temp_cell(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 216, 224, &mut self.bits) };
        }
        ///Write the `max_delta_cell_temp` field of the register.
        ///
        pub fn set_max_delta_cell_temp(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 224, 232, &mut self.bits) };
        }
        ///Write the `max_temp_int_sensor` field of the register.
        ///
        pub fn set_max_temp_int_sensor(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 232, 240, &mut self.bits) };
        }
        ///Write the `min_temp_int_sensor` field of the register.
        ///
        pub fn set_min_temp_int_sensor(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 240, 248, &mut self.bits) };
        }
        ///Write the `max_temp_fet` field of the register.
        ///
        pub fn set_max_temp_fet(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 248, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock1FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock1FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock1FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock1FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock1FieldsOut");
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
    impl defmt::Format for MacLifetimeDataBlock1FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock1FieldsOut {{ ");
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
    impl core::ops::BitAnd for MacLifetimeDataBlock1FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock1FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock1FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock1FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock1FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock1FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock1FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock2FieldsOut {
        /// The internal bits
        bits: [u8; 20],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock2FieldsOut {
        const SIZE_BITS: u32 = 160;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock2FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 20] }
        }
        ///Read the `num_shutdowns` field of the register.
        ///
        pub fn num_shutdowns(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `num_part_resets` field of the register.
        ///
        pub fn num_part_resets(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Read the `num_full_resets` field of the register.
        ///
        pub fn num_full_resets(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 24) };
            raw
        }
        ///Read the `num_wdt_resets` field of the register.
        ///
        pub fn num_wdt_resets(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 32) };
            raw
        }
        ///Read the `cb_time_cell_1` field of the register.
        ///
        pub fn cb_time_cell_1(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `cb_time_cell_2` field of the register.
        ///
        pub fn cb_time_cell_2(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `cb_time_cell_3` field of the register.
        ///
        pub fn cb_time_cell_3(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `cb_time_cell_4` field of the register.
        ///
        pub fn cb_time_cell_4(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Write the `num_shutdowns` field of the register.
        ///
        pub fn set_num_shutdowns(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
        ///Write the `num_part_resets` field of the register.
        ///
        pub fn set_num_part_resets(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 16, &mut self.bits) };
        }
        ///Write the `num_full_resets` field of the register.
        ///
        pub fn set_num_full_resets(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 24, &mut self.bits) };
        }
        ///Write the `num_wdt_resets` field of the register.
        ///
        pub fn set_num_wdt_resets(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 24, 32, &mut self.bits) };
        }
        ///Write the `cb_time_cell_1` field of the register.
        ///
        pub fn set_cb_time_cell_1(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `cb_time_cell_2` field of the register.
        ///
        pub fn set_cb_time_cell_2(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `cb_time_cell_3` field of the register.
        ///
        pub fn set_cb_time_cell_3(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `cb_time_cell_4` field of the register.
        ///
        pub fn set_cb_time_cell_4(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
    }
    impl From<[u8; 20]> for MacLifetimeDataBlock2FieldsOut {
        fn from(bits: [u8; 20]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock2FieldsOut> for [u8; 20] {
        fn from(val: MacLifetimeDataBlock2FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock2FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock2FieldsOut");
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
    impl defmt::Format for MacLifetimeDataBlock2FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock2FieldsOut {{ ");
            defmt::write!(f, "num_shutdowns: {=u8}, ", &self.num_shutdowns());
            defmt::write!(f, "num_part_resets: {=u8}, ", &self.num_part_resets());
            defmt::write!(f, "num_full_resets: {=u8}, ", &self.num_full_resets());
            defmt::write!(f, "num_wdt_resets: {=u8}, ", &self.num_wdt_resets());
            defmt::write!(f, "cb_time_cell_1: {=u32}, ", &self.cb_time_cell_1());
            defmt::write!(f, "cb_time_cell_2: {=u32}, ", &self.cb_time_cell_2());
            defmt::write!(f, "cb_time_cell_3: {=u32}, ", &self.cb_time_cell_3());
            defmt::write!(f, "cb_time_cell_4: {=u32}, ", &self.cb_time_cell_4());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock2FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock2FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock2FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock2FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock2FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock2FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock2FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock3FieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock3FieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock3FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `total_fw_runtime` field of the register.
        ///
        pub fn total_fw_runtime(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Write the `total_fw_runtime` field of the register.
        ///
        pub fn set_total_fw_runtime(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacLifetimeDataBlock3FieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock3FieldsOut> for [u8; 4] {
        fn from(val: MacLifetimeDataBlock3FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock3FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock3FieldsOut");
            d.field("total_fw_runtime", &self.total_fw_runtime());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock3FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock3FieldsOut {{ ");
            defmt::write!(f, "total_fw_runtime: {=u32}, ", &self.total_fw_runtime());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock3FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock3FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock3FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock3FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock3FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock3FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock3FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock4FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock4FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock4FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `num_cov_events` field of the register.
        ///
        pub fn num_cov_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `last_cov_event` field of the register.
        ///
        pub fn last_cov_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `num_cuv_events` field of the register.
        ///
        pub fn num_cuv_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `last_cuv_event` field of the register.
        ///
        pub fn last_cuv_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `num_ocd_1_event` field of the register.
        ///
        pub fn num_ocd_1_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `last_ocd_1_event` field of the register.
        ///
        pub fn last_ocd_1_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `num_ocd_2_events` field of the register.
        ///
        pub fn num_ocd_2_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `last_ocd_2_event` field of the register.
        ///
        pub fn last_ocd_2_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `num_occ_1_events` field of the register.
        ///
        pub fn num_occ_1_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `last_occ_1_event` field of the register.
        ///
        pub fn last_occ_1_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `num_occ_2_events` field of the register.
        ///
        pub fn num_occ_2_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `last_occ_2_event` field of the register.
        ///
        pub fn last_occ_2_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `num_aold_events` field of the register.
        ///
        pub fn num_aold_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `last_aold_event` field of the register.
        ///
        pub fn last_aold_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `num_ascd_events` field of the register.
        ///
        pub fn num_ascd_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `last_ascd_event` field of the register.
        ///
        pub fn last_ascd_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
        }
        ///Write the `num_cov_events` field of the register.
        ///
        pub fn set_num_cov_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `last_cov_event` field of the register.
        ///
        pub fn set_last_cov_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `num_cuv_events` field of the register.
        ///
        pub fn set_num_cuv_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `last_cuv_event` field of the register.
        ///
        pub fn set_last_cuv_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `num_ocd_1_event` field of the register.
        ///
        pub fn set_num_ocd_1_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `last_ocd_1_event` field of the register.
        ///
        pub fn set_last_ocd_1_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `num_ocd_2_events` field of the register.
        ///
        pub fn set_num_ocd_2_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `last_ocd_2_event` field of the register.
        ///
        pub fn set_last_ocd_2_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `num_occ_1_events` field of the register.
        ///
        pub fn set_num_occ_1_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `last_occ_1_event` field of the register.
        ///
        pub fn set_last_occ_1_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `num_occ_2_events` field of the register.
        ///
        pub fn set_num_occ_2_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `last_occ_2_event` field of the register.
        ///
        pub fn set_last_occ_2_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
        ///Write the `num_aold_events` field of the register.
        ///
        pub fn set_num_aold_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 192, 208, &mut self.bits) };
        }
        ///Write the `last_aold_event` field of the register.
        ///
        pub fn set_last_aold_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 208, 224, &mut self.bits) };
        }
        ///Write the `num_ascd_events` field of the register.
        ///
        pub fn set_num_ascd_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 224, 240, &mut self.bits) };
        }
        ///Write the `last_ascd_event` field of the register.
        ///
        pub fn set_last_ascd_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 240, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock4FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock4FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock4FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock4FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock4FieldsOut");
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
    impl defmt::Format for MacLifetimeDataBlock4FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock4FieldsOut {{ ");
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
    impl core::ops::BitAnd for MacLifetimeDataBlock4FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock4FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock4FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock4FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock4FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock4FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock4FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock5FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock5FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock5FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `num_ascc_events` field of the register.
        ///
        pub fn num_ascc_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `last_ascc_event` field of the register.
        ///
        pub fn last_ascc_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `num_otc_events` field of the register.
        ///
        pub fn num_otc_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `last_otc_event` field of the register.
        ///
        pub fn last_otc_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `num_otd_event` field of the register.
        ///
        pub fn num_otd_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `last_otd_event` field of the register.
        ///
        pub fn last_otd_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `num_otf_events` field of the register.
        ///
        pub fn num_otf_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `last_otf_event` field of the register.
        ///
        pub fn last_otf_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `num_valid_chg_term` field of the register.
        ///
        pub fn num_valid_chg_term(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `last_valid_chg_term` field of the register.
        ///
        pub fn last_valid_chg_term(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `num_qmax_updates` field of the register.
        ///
        pub fn num_qmax_updates(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `last_qmax_update` field of the register.
        ///
        pub fn last_qmax_update(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `num_ra_updates` field of the register.
        ///
        pub fn num_ra_updates(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `last_ra_update` field of the register.
        ///
        pub fn last_ra_update(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `num_ra_disable` field of the register.
        ///
        pub fn num_ra_disable(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `last_ra_disable` field of the register.
        ///
        pub fn last_ra_disable(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
        }
        ///Write the `num_ascc_events` field of the register.
        ///
        pub fn set_num_ascc_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `last_ascc_event` field of the register.
        ///
        pub fn set_last_ascc_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `num_otc_events` field of the register.
        ///
        pub fn set_num_otc_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `last_otc_event` field of the register.
        ///
        pub fn set_last_otc_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `num_otd_event` field of the register.
        ///
        pub fn set_num_otd_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `last_otd_event` field of the register.
        ///
        pub fn set_last_otd_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `num_otf_events` field of the register.
        ///
        pub fn set_num_otf_events(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `last_otf_event` field of the register.
        ///
        pub fn set_last_otf_event(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `num_valid_chg_term` field of the register.
        ///
        pub fn set_num_valid_chg_term(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `last_valid_chg_term` field of the register.
        ///
        pub fn set_last_valid_chg_term(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `num_qmax_updates` field of the register.
        ///
        pub fn set_num_qmax_updates(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `last_qmax_update` field of the register.
        ///
        pub fn set_last_qmax_update(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
        ///Write the `num_ra_updates` field of the register.
        ///
        pub fn set_num_ra_updates(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 192, 208, &mut self.bits) };
        }
        ///Write the `last_ra_update` field of the register.
        ///
        pub fn set_last_ra_update(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 208, 224, &mut self.bits) };
        }
        ///Write the `num_ra_disable` field of the register.
        ///
        pub fn set_num_ra_disable(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 224, 240, &mut self.bits) };
        }
        ///Write the `last_ra_disable` field of the register.
        ///
        pub fn set_last_ra_disable(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 240, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock5FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock5FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock5FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock5FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock5FieldsOut");
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
    impl defmt::Format for MacLifetimeDataBlock5FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock5FieldsOut {{ ");
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
    impl core::ops::BitAnd for MacLifetimeDataBlock5FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock5FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock5FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock5FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock5FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock5FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock5FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock6FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock6FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock6FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_ut_rsoc_a` field of the register.
        ///
        pub fn time_spent_ut_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_b` field of the register.
        ///
        pub fn time_spent_ut_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_c` field of the register.
        ///
        pub fn time_spent_ut_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_d` field of the register.
        ///
        pub fn time_spent_ut_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_e` field of the register.
        ///
        pub fn time_spent_ut_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_f` field of the register.
        ///
        pub fn time_spent_ut_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_g` field of the register.
        ///
        pub fn time_spent_ut_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_h` field of the register.
        ///
        pub fn time_spent_ut_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
        ///Write the `time_spent_ut_rsoc_a` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_a(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
        ///Write the `time_spent_ut_rsoc_b` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `time_spent_ut_rsoc_c` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_c(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `time_spent_ut_rsoc_d` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_d(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `time_spent_ut_rsoc_e` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_e(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
        ///Write the `time_spent_ut_rsoc_f` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_f(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 160, 192, &mut self.bits) };
        }
        ///Write the `time_spent_ut_rsoc_g` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_g(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 192, 224, &mut self.bits) };
        }
        ///Write the `time_spent_ut_rsoc_h` field of the register.
        ///
        pub fn set_time_spent_ut_rsoc_h(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 224, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock6FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock6FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock6FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock6FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock6FieldsOut");
            d.field("time_spent_ut_rsoc_a", &self.time_spent_ut_rsoc_a());
            d.field("time_spent_ut_rsoc_b", &self.time_spent_ut_rsoc_b());
            d.field("time_spent_ut_rsoc_c", &self.time_spent_ut_rsoc_c());
            d.field("time_spent_ut_rsoc_d", &self.time_spent_ut_rsoc_d());
            d.field("time_spent_ut_rsoc_e", &self.time_spent_ut_rsoc_e());
            d.field("time_spent_ut_rsoc_f", &self.time_spent_ut_rsoc_f());
            d.field("time_spent_ut_rsoc_g", &self.time_spent_ut_rsoc_g());
            d.field("time_spent_ut_rsoc_h", &self.time_spent_ut_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock6FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock6FieldsOut {{ ");
            defmt::write!(f, "time_spent_ut_rsoc_a: {=u32}, ", &self.time_spent_ut_rsoc_a());
            defmt::write!(f, "time_spent_ut_rsoc_b: {=u32}, ", &self.time_spent_ut_rsoc_b());
            defmt::write!(f, "time_spent_ut_rsoc_c: {=u32}, ", &self.time_spent_ut_rsoc_c());
            defmt::write!(f, "time_spent_ut_rsoc_d: {=u32}, ", &self.time_spent_ut_rsoc_d());
            defmt::write!(f, "time_spent_ut_rsoc_e: {=u32}, ", &self.time_spent_ut_rsoc_e());
            defmt::write!(f, "time_spent_ut_rsoc_f: {=u32}, ", &self.time_spent_ut_rsoc_f());
            defmt::write!(f, "time_spent_ut_rsoc_g: {=u32}, ", &self.time_spent_ut_rsoc_g());
            defmt::write!(f, "time_spent_ut_rsoc_h: {=u32}, ", &self.time_spent_ut_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock6FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock6FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock6FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock6FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock6FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock6FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock6FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock7FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock7FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock7FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_lt_rsoc_a` field of the register.
        ///
        pub fn time_spent_lt_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_b` field of the register.
        ///
        pub fn time_spent_lt_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_c` field of the register.
        ///
        pub fn time_spent_lt_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_d` field of the register.
        ///
        pub fn time_spent_lt_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_e` field of the register.
        ///
        pub fn time_spent_lt_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_f` field of the register.
        ///
        pub fn time_spent_lt_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_g` field of the register.
        ///
        pub fn time_spent_lt_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_h` field of the register.
        ///
        pub fn time_spent_lt_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
        ///Write the `time_spent_lt_rsoc_a` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_a(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
        ///Write the `time_spent_lt_rsoc_b` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `time_spent_lt_rsoc_c` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_c(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `time_spent_lt_rsoc_d` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_d(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `time_spent_lt_rsoc_e` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_e(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
        ///Write the `time_spent_lt_rsoc_f` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_f(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 160, 192, &mut self.bits) };
        }
        ///Write the `time_spent_lt_rsoc_g` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_g(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 192, 224, &mut self.bits) };
        }
        ///Write the `time_spent_lt_rsoc_h` field of the register.
        ///
        pub fn set_time_spent_lt_rsoc_h(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 224, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock7FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock7FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock7FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock7FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock7FieldsOut");
            d.field("time_spent_lt_rsoc_a", &self.time_spent_lt_rsoc_a());
            d.field("time_spent_lt_rsoc_b", &self.time_spent_lt_rsoc_b());
            d.field("time_spent_lt_rsoc_c", &self.time_spent_lt_rsoc_c());
            d.field("time_spent_lt_rsoc_d", &self.time_spent_lt_rsoc_d());
            d.field("time_spent_lt_rsoc_e", &self.time_spent_lt_rsoc_e());
            d.field("time_spent_lt_rsoc_f", &self.time_spent_lt_rsoc_f());
            d.field("time_spent_lt_rsoc_g", &self.time_spent_lt_rsoc_g());
            d.field("time_spent_lt_rsoc_h", &self.time_spent_lt_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock7FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock7FieldsOut {{ ");
            defmt::write!(f, "time_spent_lt_rsoc_a: {=u32}, ", &self.time_spent_lt_rsoc_a());
            defmt::write!(f, "time_spent_lt_rsoc_b: {=u32}, ", &self.time_spent_lt_rsoc_b());
            defmt::write!(f, "time_spent_lt_rsoc_c: {=u32}, ", &self.time_spent_lt_rsoc_c());
            defmt::write!(f, "time_spent_lt_rsoc_d: {=u32}, ", &self.time_spent_lt_rsoc_d());
            defmt::write!(f, "time_spent_lt_rsoc_e: {=u32}, ", &self.time_spent_lt_rsoc_e());
            defmt::write!(f, "time_spent_lt_rsoc_f: {=u32}, ", &self.time_spent_lt_rsoc_f());
            defmt::write!(f, "time_spent_lt_rsoc_g: {=u32}, ", &self.time_spent_lt_rsoc_g());
            defmt::write!(f, "time_spent_lt_rsoc_h: {=u32}, ", &self.time_spent_lt_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock7FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock7FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock7FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock7FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock7FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock7FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock7FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock8FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock8FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock8FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_stl_rsoc_a` field of the register.
        ///
        pub fn time_spent_stl_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_b` field of the register.
        ///
        pub fn time_spent_stl_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_c` field of the register.
        ///
        pub fn time_spent_stl_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_d` field of the register.
        ///
        pub fn time_spent_stl_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_e` field of the register.
        ///
        pub fn time_spent_stl_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_f` field of the register.
        ///
        pub fn time_spent_stl_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_g` field of the register.
        ///
        pub fn time_spent_stl_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_h` field of the register.
        ///
        pub fn time_spent_stl_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
        ///Write the `time_spent_stl_rsoc_a` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_a(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
        ///Write the `time_spent_stl_rsoc_b` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `time_spent_stl_rsoc_c` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_c(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `time_spent_stl_rsoc_d` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_d(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `time_spent_stl_rsoc_e` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_e(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
        ///Write the `time_spent_stl_rsoc_f` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_f(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 160, 192, &mut self.bits) };
        }
        ///Write the `time_spent_stl_rsoc_g` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_g(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 192, 224, &mut self.bits) };
        }
        ///Write the `time_spent_stl_rsoc_h` field of the register.
        ///
        pub fn set_time_spent_stl_rsoc_h(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 224, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock8FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock8FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock8FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock8FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock8FieldsOut");
            d.field("time_spent_stl_rsoc_a", &self.time_spent_stl_rsoc_a());
            d.field("time_spent_stl_rsoc_b", &self.time_spent_stl_rsoc_b());
            d.field("time_spent_stl_rsoc_c", &self.time_spent_stl_rsoc_c());
            d.field("time_spent_stl_rsoc_d", &self.time_spent_stl_rsoc_d());
            d.field("time_spent_stl_rsoc_e", &self.time_spent_stl_rsoc_e());
            d.field("time_spent_stl_rsoc_f", &self.time_spent_stl_rsoc_f());
            d.field("time_spent_stl_rsoc_g", &self.time_spent_stl_rsoc_g());
            d.field("time_spent_stl_rsoc_h", &self.time_spent_stl_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock8FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock8FieldsOut {{ ");
            defmt::write!(f, "time_spent_stl_rsoc_a: {=u32}, ", &self.time_spent_stl_rsoc_a());
            defmt::write!(f, "time_spent_stl_rsoc_b: {=u32}, ", &self.time_spent_stl_rsoc_b());
            defmt::write!(f, "time_spent_stl_rsoc_c: {=u32}, ", &self.time_spent_stl_rsoc_c());
            defmt::write!(f, "time_spent_stl_rsoc_d: {=u32}, ", &self.time_spent_stl_rsoc_d());
            defmt::write!(f, "time_spent_stl_rsoc_e: {=u32}, ", &self.time_spent_stl_rsoc_e());
            defmt::write!(f, "time_spent_stl_rsoc_f: {=u32}, ", &self.time_spent_stl_rsoc_f());
            defmt::write!(f, "time_spent_stl_rsoc_g: {=u32}, ", &self.time_spent_stl_rsoc_g());
            defmt::write!(f, "time_spent_stl_rsoc_h: {=u32}, ", &self.time_spent_stl_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock8FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock8FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock8FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock8FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock8FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock8FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock8FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock9FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock9FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock9FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_rt_rsoc_a` field of the register.
        ///
        pub fn time_spent_rt_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_rt_rsoc_b` field of the register.
        ///
        pub fn time_spent_rt_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_rt_rsoc_c` field of the register.
        ///
        pub fn time_spent_rt_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_rt_rsoc_d` field of the register.
        ///
        pub fn time_spent_rt_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_rt_rsoc_e` field of the register.
        ///
        pub fn time_spent_rt_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_rt_rsoc_f` field of the register.
        ///
        pub fn time_spent_rt_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_rt_rsoc_g` field of the register.
        ///
        pub fn time_spent_rt_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_rt_rsoc_h` field of the register.
        ///
        pub fn time_spent_rt_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
        ///Write the `time_spent_rt_rsoc_a` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_a(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
        ///Write the `time_spent_rt_rsoc_b` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `time_spent_rt_rsoc_c` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_c(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `time_spent_rt_rsoc_d` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_d(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `time_spent_rt_rsoc_e` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_e(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
        ///Write the `time_spent_rt_rsoc_f` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_f(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 160, 192, &mut self.bits) };
        }
        ///Write the `time_spent_rt_rsoc_g` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_g(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 192, 224, &mut self.bits) };
        }
        ///Write the `time_spent_rt_rsoc_h` field of the register.
        ///
        pub fn set_time_spent_rt_rsoc_h(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 224, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock9FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock9FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock9FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock9FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock9FieldsOut");
            d.field("time_spent_rt_rsoc_a", &self.time_spent_rt_rsoc_a());
            d.field("time_spent_rt_rsoc_b", &self.time_spent_rt_rsoc_b());
            d.field("time_spent_rt_rsoc_c", &self.time_spent_rt_rsoc_c());
            d.field("time_spent_rt_rsoc_d", &self.time_spent_rt_rsoc_d());
            d.field("time_spent_rt_rsoc_e", &self.time_spent_rt_rsoc_e());
            d.field("time_spent_rt_rsoc_f", &self.time_spent_rt_rsoc_f());
            d.field("time_spent_rt_rsoc_g", &self.time_spent_rt_rsoc_g());
            d.field("time_spent_rt_rsoc_h", &self.time_spent_rt_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock9FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock9FieldsOut {{ ");
            defmt::write!(f, "time_spent_rt_rsoc_a: {=u32}, ", &self.time_spent_rt_rsoc_a());
            defmt::write!(f, "time_spent_rt_rsoc_b: {=u32}, ", &self.time_spent_rt_rsoc_b());
            defmt::write!(f, "time_spent_rt_rsoc_c: {=u32}, ", &self.time_spent_rt_rsoc_c());
            defmt::write!(f, "time_spent_rt_rsoc_d: {=u32}, ", &self.time_spent_rt_rsoc_d());
            defmt::write!(f, "time_spent_rt_rsoc_e: {=u32}, ", &self.time_spent_rt_rsoc_e());
            defmt::write!(f, "time_spent_rt_rsoc_f: {=u32}, ", &self.time_spent_rt_rsoc_f());
            defmt::write!(f, "time_spent_rt_rsoc_g: {=u32}, ", &self.time_spent_rt_rsoc_g());
            defmt::write!(f, "time_spent_rt_rsoc_h: {=u32}, ", &self.time_spent_rt_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock9FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock9FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock9FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock9FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock9FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock9FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock9FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock10FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock10FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock10FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_sth_rsoc_a` field of the register.
        ///
        pub fn time_spent_sth_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_sth_rsoc_b` field of the register.
        ///
        pub fn time_spent_sth_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_sth_rsoc_c` field of the register.
        ///
        pub fn time_spent_sth_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_sth_rsoc_d` field of the register.
        ///
        pub fn time_spent_sth_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_sth_rsoc_e` field of the register.
        ///
        pub fn time_spent_sth_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_sth_rsoc_f` field of the register.
        ///
        pub fn time_spent_sth_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_sth_rsoc_g` field of the register.
        ///
        pub fn time_spent_sth_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_sth_rsoc_h` field of the register.
        ///
        pub fn time_spent_sth_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
        ///Write the `time_spent_sth_rsoc_a` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_a(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
        ///Write the `time_spent_sth_rsoc_b` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `time_spent_sth_rsoc_c` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_c(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `time_spent_sth_rsoc_d` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_d(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `time_spent_sth_rsoc_e` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_e(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
        ///Write the `time_spent_sth_rsoc_f` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_f(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 160, 192, &mut self.bits) };
        }
        ///Write the `time_spent_sth_rsoc_g` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_g(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 192, 224, &mut self.bits) };
        }
        ///Write the `time_spent_sth_rsoc_h` field of the register.
        ///
        pub fn set_time_spent_sth_rsoc_h(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 224, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock10FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock10FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock10FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock10FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock10FieldsOut");
            d.field("time_spent_sth_rsoc_a", &self.time_spent_sth_rsoc_a());
            d.field("time_spent_sth_rsoc_b", &self.time_spent_sth_rsoc_b());
            d.field("time_spent_sth_rsoc_c", &self.time_spent_sth_rsoc_c());
            d.field("time_spent_sth_rsoc_d", &self.time_spent_sth_rsoc_d());
            d.field("time_spent_sth_rsoc_e", &self.time_spent_sth_rsoc_e());
            d.field("time_spent_sth_rsoc_f", &self.time_spent_sth_rsoc_f());
            d.field("time_spent_sth_rsoc_g", &self.time_spent_sth_rsoc_g());
            d.field("time_spent_sth_rsoc_h", &self.time_spent_sth_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock10FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock10FieldsOut {{ ");
            defmt::write!(f, "time_spent_sth_rsoc_a: {=u32}, ", &self.time_spent_sth_rsoc_a());
            defmt::write!(f, "time_spent_sth_rsoc_b: {=u32}, ", &self.time_spent_sth_rsoc_b());
            defmt::write!(f, "time_spent_sth_rsoc_c: {=u32}, ", &self.time_spent_sth_rsoc_c());
            defmt::write!(f, "time_spent_sth_rsoc_d: {=u32}, ", &self.time_spent_sth_rsoc_d());
            defmt::write!(f, "time_spent_sth_rsoc_e: {=u32}, ", &self.time_spent_sth_rsoc_e());
            defmt::write!(f, "time_spent_sth_rsoc_f: {=u32}, ", &self.time_spent_sth_rsoc_f());
            defmt::write!(f, "time_spent_sth_rsoc_g: {=u32}, ", &self.time_spent_sth_rsoc_g());
            defmt::write!(f, "time_spent_sth_rsoc_h: {=u32}, ", &self.time_spent_sth_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock10FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock10FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock10FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock10FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock10FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock10FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock10FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock11FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock11FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock11FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_ht_rsoc_a` field of the register.
        ///
        pub fn time_spent_ht_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_b` field of the register.
        ///
        pub fn time_spent_ht_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_c` field of the register.
        ///
        pub fn time_spent_ht_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_d` field of the register.
        ///
        pub fn time_spent_ht_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_e` field of the register.
        ///
        pub fn time_spent_ht_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_f` field of the register.
        ///
        pub fn time_spent_ht_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_g` field of the register.
        ///
        pub fn time_spent_ht_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_h` field of the register.
        ///
        pub fn time_spent_ht_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
        ///Write the `time_spent_ht_rsoc_a` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_a(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
        ///Write the `time_spent_ht_rsoc_b` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `time_spent_ht_rsoc_c` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_c(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `time_spent_ht_rsoc_d` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_d(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `time_spent_ht_rsoc_e` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_e(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
        ///Write the `time_spent_ht_rsoc_f` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_f(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 160, 192, &mut self.bits) };
        }
        ///Write the `time_spent_ht_rsoc_g` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_g(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 192, 224, &mut self.bits) };
        }
        ///Write the `time_spent_ht_rsoc_h` field of the register.
        ///
        pub fn set_time_spent_ht_rsoc_h(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 224, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock11FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock11FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock11FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock11FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock11FieldsOut");
            d.field("time_spent_ht_rsoc_a", &self.time_spent_ht_rsoc_a());
            d.field("time_spent_ht_rsoc_b", &self.time_spent_ht_rsoc_b());
            d.field("time_spent_ht_rsoc_c", &self.time_spent_ht_rsoc_c());
            d.field("time_spent_ht_rsoc_d", &self.time_spent_ht_rsoc_d());
            d.field("time_spent_ht_rsoc_e", &self.time_spent_ht_rsoc_e());
            d.field("time_spent_ht_rsoc_f", &self.time_spent_ht_rsoc_f());
            d.field("time_spent_ht_rsoc_g", &self.time_spent_ht_rsoc_g());
            d.field("time_spent_ht_rsoc_h", &self.time_spent_ht_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock11FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock11FieldsOut {{ ");
            defmt::write!(f, "time_spent_ht_rsoc_a: {=u32}, ", &self.time_spent_ht_rsoc_a());
            defmt::write!(f, "time_spent_ht_rsoc_b: {=u32}, ", &self.time_spent_ht_rsoc_b());
            defmt::write!(f, "time_spent_ht_rsoc_c: {=u32}, ", &self.time_spent_ht_rsoc_c());
            defmt::write!(f, "time_spent_ht_rsoc_d: {=u32}, ", &self.time_spent_ht_rsoc_d());
            defmt::write!(f, "time_spent_ht_rsoc_e: {=u32}, ", &self.time_spent_ht_rsoc_e());
            defmt::write!(f, "time_spent_ht_rsoc_f: {=u32}, ", &self.time_spent_ht_rsoc_f());
            defmt::write!(f, "time_spent_ht_rsoc_g: {=u32}, ", &self.time_spent_ht_rsoc_g());
            defmt::write!(f, "time_spent_ht_rsoc_h: {=u32}, ", &self.time_spent_ht_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock11FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock11FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock11FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock11FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock11FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock11FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock11FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacLifetimeDataBlock12FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacLifetimeDataBlock12FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacLifetimeDataBlock12FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_ot_rsoc_a` field of the register.
        ///
        pub fn time_spent_ot_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_b` field of the register.
        ///
        pub fn time_spent_ot_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_c` field of the register.
        ///
        pub fn time_spent_ot_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_d` field of the register.
        ///
        pub fn time_spent_ot_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_e` field of the register.
        ///
        pub fn time_spent_ot_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_f` field of the register.
        ///
        pub fn time_spent_ot_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_g` field of the register.
        ///
        pub fn time_spent_ot_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_h` field of the register.
        ///
        pub fn time_spent_ot_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
        ///Write the `time_spent_ot_rsoc_a` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_a(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
        ///Write the `time_spent_ot_rsoc_b` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 32, 64, &mut self.bits) };
        }
        ///Write the `time_spent_ot_rsoc_c` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_c(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 64, 96, &mut self.bits) };
        }
        ///Write the `time_spent_ot_rsoc_d` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_d(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 96, 128, &mut self.bits) };
        }
        ///Write the `time_spent_ot_rsoc_e` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_e(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 128, 160, &mut self.bits) };
        }
        ///Write the `time_spent_ot_rsoc_f` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_f(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 160, 192, &mut self.bits) };
        }
        ///Write the `time_spent_ot_rsoc_g` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_g(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 192, 224, &mut self.bits) };
        }
        ///Write the `time_spent_ot_rsoc_h` field of the register.
        ///
        pub fn set_time_spent_ot_rsoc_h(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 224, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacLifetimeDataBlock12FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacLifetimeDataBlock12FieldsOut> for [u8; 32] {
        fn from(val: MacLifetimeDataBlock12FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacLifetimeDataBlock12FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacLifetimeDataBlock12FieldsOut");
            d.field("time_spent_ot_rsoc_a", &self.time_spent_ot_rsoc_a());
            d.field("time_spent_ot_rsoc_b", &self.time_spent_ot_rsoc_b());
            d.field("time_spent_ot_rsoc_c", &self.time_spent_ot_rsoc_c());
            d.field("time_spent_ot_rsoc_d", &self.time_spent_ot_rsoc_d());
            d.field("time_spent_ot_rsoc_e", &self.time_spent_ot_rsoc_e());
            d.field("time_spent_ot_rsoc_f", &self.time_spent_ot_rsoc_f());
            d.field("time_spent_ot_rsoc_g", &self.time_spent_ot_rsoc_g());
            d.field("time_spent_ot_rsoc_h", &self.time_spent_ot_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacLifetimeDataBlock12FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacLifetimeDataBlock12FieldsOut {{ ");
            defmt::write!(f, "time_spent_ot_rsoc_a: {=u32}, ", &self.time_spent_ot_rsoc_a());
            defmt::write!(f, "time_spent_ot_rsoc_b: {=u32}, ", &self.time_spent_ot_rsoc_b());
            defmt::write!(f, "time_spent_ot_rsoc_c: {=u32}, ", &self.time_spent_ot_rsoc_c());
            defmt::write!(f, "time_spent_ot_rsoc_d: {=u32}, ", &self.time_spent_ot_rsoc_d());
            defmt::write!(f, "time_spent_ot_rsoc_e: {=u32}, ", &self.time_spent_ot_rsoc_e());
            defmt::write!(f, "time_spent_ot_rsoc_f: {=u32}, ", &self.time_spent_ot_rsoc_f());
            defmt::write!(f, "time_spent_ot_rsoc_g: {=u32}, ", &self.time_spent_ot_rsoc_g());
            defmt::write!(f, "time_spent_ot_rsoc_h: {=u32}, ", &self.time_spent_ot_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacLifetimeDataBlock12FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacLifetimeDataBlock12FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacLifetimeDataBlock12FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacLifetimeDataBlock12FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacLifetimeDataBlock12FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacLifetimeDataBlock12FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacLifetimeDataBlock12FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacManufactureInfoFieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacManufactureInfoFieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacManufactureInfoFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `manufacture_info_0` field of the register.
        ///
        pub fn manufacture_info_0(&self) -> u64 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u64, ::device_driver::ops::LE>(&self.bits, 0, 64) };
            raw
        }
        ///Read the `manufacture_info_1` field of the register.
        ///
        pub fn manufacture_info_1(&self) -> u64 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u64, ::device_driver::ops::LE>(&self.bits, 64, 128) };
            raw
        }
        ///Read the `manufacture_info_2` field of the register.
        ///
        pub fn manufacture_info_2(&self) -> u64 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u64, ::device_driver::ops::LE>(&self.bits, 128, 192) };
            raw
        }
        ///Read the `manufacture_info_3` field of the register.
        ///
        pub fn manufacture_info_3(&self) -> u64 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u64, ::device_driver::ops::LE>(&self.bits, 192, 256) };
            raw
        }
        ///Write the `manufacture_info_0` field of the register.
        ///
        pub fn set_manufacture_info_0(&mut self, value: u64) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u64, ::device_driver::ops::LE>(raw, 0, 64, &mut self.bits) };
        }
        ///Write the `manufacture_info_1` field of the register.
        ///
        pub fn set_manufacture_info_1(&mut self, value: u64) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u64, ::device_driver::ops::LE>(raw, 64, 128, &mut self.bits) };
        }
        ///Write the `manufacture_info_2` field of the register.
        ///
        pub fn set_manufacture_info_2(&mut self, value: u64) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u64, ::device_driver::ops::LE>(raw, 128, 192, &mut self.bits) };
        }
        ///Write the `manufacture_info_3` field of the register.
        ///
        pub fn set_manufacture_info_3(&mut self, value: u64) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u64, ::device_driver::ops::LE>(raw, 192, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacManufactureInfoFieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacManufactureInfoFieldsOut> for [u8; 32] {
        fn from(val: MacManufactureInfoFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacManufactureInfoFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacManufactureInfoFieldsOut");
            d.field("manufacture_info_0", &self.manufacture_info_0());
            d.field("manufacture_info_1", &self.manufacture_info_1());
            d.field("manufacture_info_2", &self.manufacture_info_2());
            d.field("manufacture_info_3", &self.manufacture_info_3());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacManufactureInfoFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacManufactureInfoFieldsOut {{ ");
            defmt::write!(f, "manufacture_info_0: {=u64}, ", &self.manufacture_info_0());
            defmt::write!(f, "manufacture_info_1: {=u64}, ", &self.manufacture_info_1());
            defmt::write!(f, "manufacture_info_2: {=u64}, ", &self.manufacture_info_2());
            defmt::write!(f, "manufacture_info_3: {=u64}, ", &self.manufacture_info_3());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacManufactureInfoFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacManufactureInfoFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacManufactureInfoFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacManufactureInfoFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacManufactureInfoFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacManufactureInfoFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacManufactureInfoFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacDaStatus1FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacDaStatus1FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacDaStatus1FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `cell_voltage_1` field of the register.
        ///
        pub fn cell_voltage_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `cell_voltage_2` field of the register.
        ///
        pub fn cell_voltage_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cell_voltage_3` field of the register.
        ///
        pub fn cell_voltage_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cell_voltage_4` field of the register.
        ///
        pub fn cell_voltage_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `bat_voltage` field of the register.
        ///
        pub fn bat_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `pack_voltage` field of the register.
        ///
        pub fn pack_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `cell_current_1` field of the register.
        ///
        pub fn cell_current_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `cell_current_2` field of the register.
        ///
        pub fn cell_current_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `cell_current_3` field of the register.
        ///
        pub fn cell_current_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `cell_current_4` field of the register.
        ///
        pub fn cell_current_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `cell_pwr_1` field of the register.
        ///
        pub fn cell_pwr_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `cell_pwr_2` field of the register.
        ///
        pub fn cell_pwr_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `cell_pwr_3` field of the register.
        ///
        pub fn cell_pwr_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `cell_pwr_4` field of the register.
        ///
        pub fn cell_pwr_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `total_pwr` field of the register.
        ///
        pub fn total_pwr(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `avg_pwr` field of the register.
        ///
        pub fn avg_pwr(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
        }
        ///Write the `cell_voltage_1` field of the register.
        ///
        pub fn set_cell_voltage_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `cell_voltage_2` field of the register.
        ///
        pub fn set_cell_voltage_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `cell_voltage_3` field of the register.
        ///
        pub fn set_cell_voltage_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `cell_voltage_4` field of the register.
        ///
        pub fn set_cell_voltage_4(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `bat_voltage` field of the register.
        ///
        pub fn set_bat_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `pack_voltage` field of the register.
        ///
        pub fn set_pack_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `cell_current_1` field of the register.
        ///
        pub fn set_cell_current_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `cell_current_2` field of the register.
        ///
        pub fn set_cell_current_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `cell_current_3` field of the register.
        ///
        pub fn set_cell_current_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `cell_current_4` field of the register.
        ///
        pub fn set_cell_current_4(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `cell_pwr_1` field of the register.
        ///
        pub fn set_cell_pwr_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `cell_pwr_2` field of the register.
        ///
        pub fn set_cell_pwr_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
        ///Write the `cell_pwr_3` field of the register.
        ///
        pub fn set_cell_pwr_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 192, 208, &mut self.bits) };
        }
        ///Write the `cell_pwr_4` field of the register.
        ///
        pub fn set_cell_pwr_4(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 208, 224, &mut self.bits) };
        }
        ///Write the `total_pwr` field of the register.
        ///
        pub fn set_total_pwr(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 224, 240, &mut self.bits) };
        }
        ///Write the `avg_pwr` field of the register.
        ///
        pub fn set_avg_pwr(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 240, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacDaStatus1FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacDaStatus1FieldsOut> for [u8; 32] {
        fn from(val: MacDaStatus1FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacDaStatus1FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacDaStatus1FieldsOut");
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
    impl defmt::Format for MacDaStatus1FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacDaStatus1FieldsOut {{ ");
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
    impl core::ops::BitAnd for MacDaStatus1FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacDaStatus1FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacDaStatus1FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacDaStatus1FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacDaStatus1FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacDaStatus1FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacDaStatus1FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacDaStatus2FieldsOut {
        /// The internal bits
        bits: [u8; 26],
    }
    impl ::device_driver::FieldSet for MacDaStatus2FieldsOut {
        const SIZE_BITS: u32 = 208;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacDaStatus2FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 26] }
        }
        ///Read the `int_temp` field of the register.
        ///
        pub fn int_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `ts_1_temp` field of the register.
        ///
        pub fn ts_1_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `ts_2_temp` field of the register.
        ///
        pub fn ts_2_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `ts_3_temp` field of the register.
        ///
        pub fn ts_3_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `ts_4_temp` field of the register.
        ///
        pub fn ts_4_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_temp` field of the register.
        ///
        pub fn cell_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `fet_temp` field of the register.
        ///
        pub fn fet_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `gauging_temp` field of the register.
        ///
        pub fn gauging_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Write the `int_temp` field of the register.
        ///
        pub fn set_int_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `ts_1_temp` field of the register.
        ///
        pub fn set_ts_1_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `ts_2_temp` field of the register.
        ///
        pub fn set_ts_2_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `ts_3_temp` field of the register.
        ///
        pub fn set_ts_3_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `ts_4_temp` field of the register.
        ///
        pub fn set_ts_4_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `cell_temp` field of the register.
        ///
        pub fn set_cell_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `fet_temp` field of the register.
        ///
        pub fn set_fet_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `gauging_temp` field of the register.
        ///
        pub fn set_gauging_temp(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
    }
    impl From<[u8; 26]> for MacDaStatus2FieldsOut {
        fn from(bits: [u8; 26]) -> Self {
            Self { bits }
        }
    }
    impl From<MacDaStatus2FieldsOut> for [u8; 26] {
        fn from(val: MacDaStatus2FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacDaStatus2FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacDaStatus2FieldsOut");
            d.field("int_temp", &self.int_temp());
            d.field("ts_1_temp", &self.ts_1_temp());
            d.field("ts_2_temp", &self.ts_2_temp());
            d.field("ts_3_temp", &self.ts_3_temp());
            d.field("ts_4_temp", &self.ts_4_temp());
            d.field("cell_temp", &self.cell_temp());
            d.field("fet_temp", &self.fet_temp());
            d.field("gauging_temp", &self.gauging_temp());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacDaStatus2FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacDaStatus2FieldsOut {{ ");
            defmt::write!(f, "int_temp: {=u16}, ", &self.int_temp());
            defmt::write!(f, "ts_1_temp: {=u16}, ", &self.ts_1_temp());
            defmt::write!(f, "ts_2_temp: {=u16}, ", &self.ts_2_temp());
            defmt::write!(f, "ts_3_temp: {=u16}, ", &self.ts_3_temp());
            defmt::write!(f, "ts_4_temp: {=u16}, ", &self.ts_4_temp());
            defmt::write!(f, "cell_temp: {=u16}, ", &self.cell_temp());
            defmt::write!(f, "fet_temp: {=u16}, ", &self.fet_temp());
            defmt::write!(f, "gauging_temp: {=u16}, ", &self.gauging_temp());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacDaStatus2FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacDaStatus2FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacDaStatus2FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacDaStatus2FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacDaStatus2FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacDaStatus2FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacDaStatus2FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacGaugeStatus1FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacGaugeStatus1FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacGaugeStatus1FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `true_rem_q` field of the register.
        ///
        pub fn true_rem_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `true_rem_e` field of the register.
        ///
        pub fn true_rem_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `initial_q` field of the register.
        ///
        pub fn initial_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `initial_e` field of the register.
        ///
        pub fn initial_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `true_fcc_q` field of the register.
        ///
        pub fn true_fcc_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `true_fcc_e` field of the register.
        ///
        pub fn true_fcc_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `t_sim` field of the register.
        ///
        pub fn t_sim(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `t_ambient` field of the register.
        ///
        pub fn t_ambient(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `ra_scale_0` field of the register.
        ///
        pub fn ra_scale_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `ra_scale_1` field of the register.
        ///
        pub fn ra_scale_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `ra_scale_2` field of the register.
        ///
        pub fn ra_scale_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `ra_scale_3` field of the register.
        ///
        pub fn ra_scale_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `comp_res_0` field of the register.
        ///
        pub fn comp_res_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `comp_res_1` field of the register.
        ///
        pub fn comp_res_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `comp_res_2` field of the register.
        ///
        pub fn comp_res_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `comp_res_3` field of the register.
        ///
        pub fn comp_res_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
        }
        ///Write the `true_rem_q` field of the register.
        ///
        pub fn set_true_rem_q(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `true_rem_e` field of the register.
        ///
        pub fn set_true_rem_e(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `initial_q` field of the register.
        ///
        pub fn set_initial_q(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `initial_e` field of the register.
        ///
        pub fn set_initial_e(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `true_fcc_q` field of the register.
        ///
        pub fn set_true_fcc_q(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `true_fcc_e` field of the register.
        ///
        pub fn set_true_fcc_e(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `t_sim` field of the register.
        ///
        pub fn set_t_sim(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `t_ambient` field of the register.
        ///
        pub fn set_t_ambient(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `ra_scale_0` field of the register.
        ///
        pub fn set_ra_scale_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `ra_scale_1` field of the register.
        ///
        pub fn set_ra_scale_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `ra_scale_2` field of the register.
        ///
        pub fn set_ra_scale_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `ra_scale_3` field of the register.
        ///
        pub fn set_ra_scale_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
        ///Write the `comp_res_0` field of the register.
        ///
        pub fn set_comp_res_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 192, 208, &mut self.bits) };
        }
        ///Write the `comp_res_1` field of the register.
        ///
        pub fn set_comp_res_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 208, 224, &mut self.bits) };
        }
        ///Write the `comp_res_2` field of the register.
        ///
        pub fn set_comp_res_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 224, 240, &mut self.bits) };
        }
        ///Write the `comp_res_3` field of the register.
        ///
        pub fn set_comp_res_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 240, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacGaugeStatus1FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacGaugeStatus1FieldsOut> for [u8; 32] {
        fn from(val: MacGaugeStatus1FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacGaugeStatus1FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacGaugeStatus1FieldsOut");
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
    impl defmt::Format for MacGaugeStatus1FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacGaugeStatus1FieldsOut {{ ");
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
    impl core::ops::BitAnd for MacGaugeStatus1FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacGaugeStatus1FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacGaugeStatus1FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacGaugeStatus1FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacGaugeStatus1FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacGaugeStatus1FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacGaugeStatus1FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacGaugeStatus2FieldsOut {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for MacGaugeStatus2FieldsOut {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacGaugeStatus2FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `pack_grid` field of the register.
        ///
        pub fn pack_grid(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `q_max_status` field of the register.
        ///
        pub fn q_max_status(&self) -> super::MacqMaxStatus {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `iten` field of the register.
        ///
        pub fn iten(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `qmax_field_updated` field of the register.
        ///
        pub fn qmax_field_updated(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `cell_grid_0` field of the register.
        ///
        pub fn cell_grid_0(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 24) };
            raw
        }
        ///Read the `cell_grid_1` field of the register.
        ///
        pub fn cell_grid_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 32) };
            raw
        }
        ///Read the `cell_grid_2` field of the register.
        ///
        pub fn cell_grid_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 32, 40) };
            raw
        }
        ///Read the `cell_grid_3` field of the register.
        ///
        pub fn cell_grid_3(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 40, 48) };
            raw
        }
        ///Read the `state_time` field of the register.
        ///
        pub fn state_time(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 48, 80) };
            raw
        }
        ///Read the `dod_0_0` field of the register.
        ///
        pub fn dod_0_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `dod_0_1` field of the register.
        ///
        pub fn dod_0_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `dod_0_2` field of the register.
        ///
        pub fn dod_0_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `dod_0_3` field of the register.
        ///
        pub fn dod_0_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `dod_0_passed_q` field of the register.
        ///
        pub fn dod_0_passed_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `dod_0_passed_e` field of the register.
        ///
        pub fn dod_0_passed_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `dod_0_time` field of the register.
        ///
        pub fn dod_0_time(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `dodeoc_0` field of the register.
        ///
        pub fn dodeoc_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `dodeoc_1` field of the register.
        ///
        pub fn dodeoc_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `dodeoc_2` field of the register.
        ///
        pub fn dodeoc_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `dodeoc_3` field of the register.
        ///
        pub fn dodeoc_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
        }
        ///Write the `pack_grid` field of the register.
        ///
        pub fn set_pack_grid(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
        ///Write the `q_max_status` field of the register.
        ///
        pub fn set_q_max_status(&mut self, value: super::MacqMaxStatus) {
            let raw = value.into();
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 10, &mut self.bits) };
        }
        ///Write the `iten` field of the register.
        ///
        pub fn set_iten(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 10, 11, &mut self.bits) };
        }
        ///Write the `qmax_field_updated` field of the register.
        ///
        pub fn set_qmax_field_updated(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 11, 12, &mut self.bits) };
        }
        ///Write the `cell_grid_0` field of the register.
        ///
        pub fn set_cell_grid_0(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 16, 24, &mut self.bits) };
        }
        ///Write the `cell_grid_1` field of the register.
        ///
        pub fn set_cell_grid_1(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 24, 32, &mut self.bits) };
        }
        ///Write the `cell_grid_2` field of the register.
        ///
        pub fn set_cell_grid_2(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 32, 40, &mut self.bits) };
        }
        ///Write the `cell_grid_3` field of the register.
        ///
        pub fn set_cell_grid_3(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 40, 48, &mut self.bits) };
        }
        ///Write the `state_time` field of the register.
        ///
        pub fn set_state_time(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 48, 80, &mut self.bits) };
        }
        ///Write the `dod_0_0` field of the register.
        ///
        pub fn set_dod_0_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `dod_0_1` field of the register.
        ///
        pub fn set_dod_0_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `dod_0_2` field of the register.
        ///
        pub fn set_dod_0_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `dod_0_3` field of the register.
        ///
        pub fn set_dod_0_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `dod_0_passed_q` field of the register.
        ///
        pub fn set_dod_0_passed_q(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `dod_0_passed_e` field of the register.
        ///
        pub fn set_dod_0_passed_e(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `dod_0_time` field of the register.
        ///
        pub fn set_dod_0_time(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
        ///Write the `dodeoc_0` field of the register.
        ///
        pub fn set_dodeoc_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 192, 208, &mut self.bits) };
        }
        ///Write the `dodeoc_1` field of the register.
        ///
        pub fn set_dodeoc_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 208, 224, &mut self.bits) };
        }
        ///Write the `dodeoc_2` field of the register.
        ///
        pub fn set_dodeoc_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 224, 240, &mut self.bits) };
        }
        ///Write the `dodeoc_3` field of the register.
        ///
        pub fn set_dodeoc_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 240, 256, &mut self.bits) };
        }
    }
    impl From<[u8; 32]> for MacGaugeStatus2FieldsOut {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<MacGaugeStatus2FieldsOut> for [u8; 32] {
        fn from(val: MacGaugeStatus2FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacGaugeStatus2FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacGaugeStatus2FieldsOut");
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
    impl defmt::Format for MacGaugeStatus2FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacGaugeStatus2FieldsOut {{ ");
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
    impl core::ops::BitAnd for MacGaugeStatus2FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacGaugeStatus2FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacGaugeStatus2FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacGaugeStatus2FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacGaugeStatus2FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacGaugeStatus2FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacGaugeStatus2FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacGaugeStatus3FieldsOut {
        /// The internal bits
        bits: [u8; 24],
    }
    impl ::device_driver::FieldSet for MacGaugeStatus3FieldsOut {
        const SIZE_BITS: u32 = 192;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacGaugeStatus3FieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 24] }
        }
        ///Read the `qmax_0` field of the register.
        ///
        pub fn qmax_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `qmax_1` field of the register.
        ///
        pub fn qmax_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `qmax_2` field of the register.
        ///
        pub fn qmax_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `qmax_3` field of the register.
        ///
        pub fn qmax_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `qmax_dod_0_0` field of the register.
        ///
        pub fn qmax_dod_0_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `qmax_dod_0_1` field of the register.
        ///
        pub fn qmax_dod_0_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `qmax_dod_0_2` field of the register.
        ///
        pub fn qmax_dod_0_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `qmax_dod_0_3` field of the register.
        ///
        pub fn qmax_dod_0_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `qmax_passed_q` field of the register.
        ///
        pub fn qmax_passed_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `qmax_time` field of the register.
        ///
        pub fn qmax_time(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `temp_k_factor` field of the register.
        ///
        pub fn temp_k_factor(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `temp_a_factor` field of the register.
        ///
        pub fn temp_a_factor(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Write the `qmax_0` field of the register.
        ///
        pub fn set_qmax_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `qmax_1` field of the register.
        ///
        pub fn set_qmax_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `qmax_2` field of the register.
        ///
        pub fn set_qmax_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `qmax_3` field of the register.
        ///
        pub fn set_qmax_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `qmax_dod_0_0` field of the register.
        ///
        pub fn set_qmax_dod_0_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `qmax_dod_0_1` field of the register.
        ///
        pub fn set_qmax_dod_0_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `qmax_dod_0_2` field of the register.
        ///
        pub fn set_qmax_dod_0_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `qmax_dod_0_3` field of the register.
        ///
        pub fn set_qmax_dod_0_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `qmax_passed_q` field of the register.
        ///
        pub fn set_qmax_passed_q(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `qmax_time` field of the register.
        ///
        pub fn set_qmax_time(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `temp_k_factor` field of the register.
        ///
        pub fn set_temp_k_factor(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `temp_a_factor` field of the register.
        ///
        pub fn set_temp_a_factor(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
    }
    impl From<[u8; 24]> for MacGaugeStatus3FieldsOut {
        fn from(bits: [u8; 24]) -> Self {
            Self { bits }
        }
    }
    impl From<MacGaugeStatus3FieldsOut> for [u8; 24] {
        fn from(val: MacGaugeStatus3FieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacGaugeStatus3FieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacGaugeStatus3FieldsOut");
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
    impl defmt::Format for MacGaugeStatus3FieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacGaugeStatus3FieldsOut {{ ");
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
    impl core::ops::BitAnd for MacGaugeStatus3FieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacGaugeStatus3FieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacGaugeStatus3FieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacGaugeStatus3FieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacGaugeStatus3FieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacGaugeStatus3FieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacGaugeStatus3FieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacCbStatusFieldsOut {
        /// The internal bits
        bits: [u8; 19],
    }
    impl ::device_driver::FieldSet for MacCbStatusFieldsOut {
        const SIZE_BITS: u32 = 152;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacCbStatusFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 19] }
        }
        ///Read the `cb_time_0` field of the register.
        ///
        pub fn cb_time_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `cb_time_1` field of the register.
        ///
        pub fn cb_time_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cb_time_2` field of the register.
        ///
        pub fn cb_time_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cb_time_3` field of the register.
        ///
        pub fn cb_time_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `cell_1_balance_dod` field of the register.
        ///
        pub fn cell_1_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_2_balance_dod` field of the register.
        ///
        pub fn cell_2_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `cell_3_balance_dod` field of the register.
        ///
        pub fn cell_3_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `cell_4_balance_dod` field of the register.
        ///
        pub fn cell_4_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `total_dod_chrg` field of the register.
        ///
        pub fn total_dod_chrg(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `cell_balance_stat` field of the register.
        ///
        pub fn cell_balance_stat(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 144, 152) };
            raw
        }
        ///Write the `cb_time_0` field of the register.
        ///
        pub fn set_cb_time_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `cb_time_1` field of the register.
        ///
        pub fn set_cb_time_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `cb_time_2` field of the register.
        ///
        pub fn set_cb_time_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `cb_time_3` field of the register.
        ///
        pub fn set_cb_time_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `cell_1_balance_dod` field of the register.
        ///
        pub fn set_cell_1_balance_dod(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `cell_2_balance_dod` field of the register.
        ///
        pub fn set_cell_2_balance_dod(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `cell_3_balance_dod` field of the register.
        ///
        pub fn set_cell_3_balance_dod(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `cell_4_balance_dod` field of the register.
        ///
        pub fn set_cell_4_balance_dod(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `total_dod_chrg` field of the register.
        ///
        pub fn set_total_dod_chrg(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `cell_balance_stat` field of the register.
        ///
        pub fn set_cell_balance_stat(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 144, 152, &mut self.bits) };
        }
    }
    impl From<[u8; 19]> for MacCbStatusFieldsOut {
        fn from(bits: [u8; 19]) -> Self {
            Self { bits }
        }
    }
    impl From<MacCbStatusFieldsOut> for [u8; 19] {
        fn from(val: MacCbStatusFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacCbStatusFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacCbStatusFieldsOut");
            d.field("cb_time_0", &self.cb_time_0());
            d.field("cb_time_1", &self.cb_time_1());
            d.field("cb_time_2", &self.cb_time_2());
            d.field("cb_time_3", &self.cb_time_3());
            d.field("cell_1_balance_dod", &self.cell_1_balance_dod());
            d.field("cell_2_balance_dod", &self.cell_2_balance_dod());
            d.field("cell_3_balance_dod", &self.cell_3_balance_dod());
            d.field("cell_4_balance_dod", &self.cell_4_balance_dod());
            d.field("total_dod_chrg", &self.total_dod_chrg());
            d.field("cell_balance_stat", &self.cell_balance_stat());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacCbStatusFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacCbStatusFieldsOut {{ ");
            defmt::write!(f, "cb_time_0: {=u16}, ", &self.cb_time_0());
            defmt::write!(f, "cb_time_1: {=u16}, ", &self.cb_time_1());
            defmt::write!(f, "cb_time_2: {=u16}, ", &self.cb_time_2());
            defmt::write!(f, "cb_time_3: {=u16}, ", &self.cb_time_3());
            defmt::write!(f, "cell_1_balance_dod: {=u16}, ", &self.cell_1_balance_dod());
            defmt::write!(f, "cell_2_balance_dod: {=u16}, ", &self.cell_2_balance_dod());
            defmt::write!(f, "cell_3_balance_dod: {=u16}, ", &self.cell_3_balance_dod());
            defmt::write!(f, "cell_4_balance_dod: {=u16}, ", &self.cell_4_balance_dod());
            defmt::write!(f, "total_dod_chrg: {=u16}, ", &self.total_dod_chrg());
            defmt::write!(f, "cell_balance_stat: {=u8}, ", &self.cell_balance_stat());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacCbStatusFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacCbStatusFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacCbStatusFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacCbStatusFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacCbStatusFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacCbStatusFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacCbStatusFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacStateOfHealthFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacStateOfHealthFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacStateOfHealthFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `soh_fcc` field of the register.
        ///
        pub fn soh_fcc(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `soh_energy` field of the register.
        ///
        pub fn soh_energy(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Write the `soh_fcc` field of the register.
        ///
        pub fn set_soh_fcc(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `soh_energy` field of the register.
        ///
        pub fn set_soh_energy(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacStateOfHealthFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacStateOfHealthFieldsOut> for [u8; 4] {
        fn from(val: MacStateOfHealthFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacStateOfHealthFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacStateOfHealthFieldsOut");
            d.field("soh_fcc", &self.soh_fcc());
            d.field("soh_energy", &self.soh_energy());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacStateOfHealthFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacStateOfHealthFieldsOut {{ ");
            defmt::write!(f, "soh_fcc: {=u16}, ", &self.soh_fcc());
            defmt::write!(f, "soh_energy: {=u16}, ", &self.soh_energy());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacStateOfHealthFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacStateOfHealthFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacStateOfHealthFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacStateOfHealthFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacStateOfHealthFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacStateOfHealthFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacStateOfHealthFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacFilterCapacityFieldsOut {
        /// The internal bits
        bits: [u8; 8],
    }
    impl ::device_driver::FieldSet for MacFilterCapacityFieldsOut {
        const SIZE_BITS: u32 = 64;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacFilterCapacityFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 8] }
        }
        ///Read the `filt_rem_cap` field of the register.
        ///
        pub fn filt_rem_cap(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `filt_rem_energy` field of the register.
        ///
        pub fn filt_rem_energy(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `filt_full_chg_cap` field of the register.
        ///
        pub fn filt_full_chg_cap(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `filt_full_chg_energy` field of the register.
        ///
        pub fn filt_full_chg_energy(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Write the `filt_rem_cap` field of the register.
        ///
        pub fn set_filt_rem_cap(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
        ///Write the `filt_rem_energy` field of the register.
        ///
        pub fn set_filt_rem_energy(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `filt_full_chg_cap` field of the register.
        ///
        pub fn set_filt_full_chg_cap(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `filt_full_chg_energy` field of the register.
        ///
        pub fn set_filt_full_chg_energy(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
    }
    impl From<[u8; 8]> for MacFilterCapacityFieldsOut {
        fn from(bits: [u8; 8]) -> Self {
            Self { bits }
        }
    }
    impl From<MacFilterCapacityFieldsOut> for [u8; 8] {
        fn from(val: MacFilterCapacityFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacFilterCapacityFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacFilterCapacityFieldsOut");
            d.field("filt_rem_cap", &self.filt_rem_cap());
            d.field("filt_rem_energy", &self.filt_rem_energy());
            d.field("filt_full_chg_cap", &self.filt_full_chg_cap());
            d.field("filt_full_chg_energy", &self.filt_full_chg_energy());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacFilterCapacityFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacFilterCapacityFieldsOut {{ ");
            defmt::write!(f, "filt_rem_cap: {=u16}, ", &self.filt_rem_cap());
            defmt::write!(f, "filt_rem_energy: {=u16}, ", &self.filt_rem_energy());
            defmt::write!(f, "filt_full_chg_cap: {=u16}, ", &self.filt_full_chg_cap());
            defmt::write!(f, "filt_full_chg_energy: {=u16}, ", &self.filt_full_chg_energy());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacFilterCapacityFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacFilterCapacityFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacFilterCapacityFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacFilterCapacityFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacFilterCapacityFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacFilterCapacityFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacFilterCapacityFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacManufactureInfoBFieldsOut {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for MacManufactureInfoBFieldsOut {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacManufactureInfoBFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `manufacture_info_b` field of the register.
        ///
        pub fn manufacture_info_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Write the `manufacture_info_b` field of the register.
        ///
        pub fn set_manufacture_info_b(&mut self, value: u32) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u32, ::device_driver::ops::LE>(raw, 0, 32, &mut self.bits) };
        }
    }
    impl From<[u8; 4]> for MacManufactureInfoBFieldsOut {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<MacManufactureInfoBFieldsOut> for [u8; 4] {
        fn from(val: MacManufactureInfoBFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacManufactureInfoBFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacManufactureInfoBFieldsOut");
            d.field("manufacture_info_b", &self.manufacture_info_b());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MacManufactureInfoBFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacManufactureInfoBFieldsOut {{ ");
            defmt::write!(f, "manufacture_info_b: {=u32}, ", &self.manufacture_info_b());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MacManufactureInfoBFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacManufactureInfoBFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacManufactureInfoBFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacManufactureInfoBFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacManufactureInfoBFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacManufactureInfoBFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacManufactureInfoBFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacOutputCcadcCalFieldsOut {
        /// The internal bits
        bits: [u8; 24],
    }
    impl ::device_driver::FieldSet for MacOutputCcadcCalFieldsOut {
        const SIZE_BITS: u32 = 192;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacOutputCcadcCalFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 24] }
        }
        ///Read the `refresh_ctr` field of the register.
        ///
        pub fn refresh_ctr(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `status` field of the register.
        ///
        pub fn status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Read the `current` field of the register.
        ///
        pub fn current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cell_voltage_1` field of the register.
        ///
        pub fn cell_voltage_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cell_voltage_2` field of the register.
        ///
        pub fn cell_voltage_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `cell_voltage_3` field of the register.
        ///
        pub fn cell_voltage_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_voltage_4` field of the register.
        ///
        pub fn cell_voltage_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `pack_voltage` field of the register.
        ///
        pub fn pack_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `bat_voltage` field of the register.
        ///
        pub fn bat_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `cell_current_1` field of the register.
        ///
        pub fn cell_current_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `cell_current_2` field of the register.
        ///
        pub fn cell_current_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `cell_current_3` field of the register.
        ///
        pub fn cell_current_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `cell_current_4` field of the register.
        ///
        pub fn cell_current_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Write the `refresh_ctr` field of the register.
        ///
        pub fn set_refresh_ctr(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
        ///Write the `status` field of the register.
        ///
        pub fn set_status(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 16, &mut self.bits) };
        }
        ///Write the `current` field of the register.
        ///
        pub fn set_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `cell_voltage_1` field of the register.
        ///
        pub fn set_cell_voltage_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `cell_voltage_2` field of the register.
        ///
        pub fn set_cell_voltage_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `cell_voltage_3` field of the register.
        ///
        pub fn set_cell_voltage_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `cell_voltage_4` field of the register.
        ///
        pub fn set_cell_voltage_4(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `pack_voltage` field of the register.
        ///
        pub fn set_pack_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `bat_voltage` field of the register.
        ///
        pub fn set_bat_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `cell_current_1` field of the register.
        ///
        pub fn set_cell_current_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `cell_current_2` field of the register.
        ///
        pub fn set_cell_current_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `cell_current_3` field of the register.
        ///
        pub fn set_cell_current_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `cell_current_4` field of the register.
        ///
        pub fn set_cell_current_4(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
    }
    impl From<[u8; 24]> for MacOutputCcadcCalFieldsOut {
        fn from(bits: [u8; 24]) -> Self {
            Self { bits }
        }
    }
    impl From<MacOutputCcadcCalFieldsOut> for [u8; 24] {
        fn from(val: MacOutputCcadcCalFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacOutputCcadcCalFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacOutputCcadcCalFieldsOut");
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
    impl defmt::Format for MacOutputCcadcCalFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacOutputCcadcCalFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacOutputCcadcCalFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacOutputCcadcCalFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacOutputCcadcCalFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacOutputCcadcCalFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacOutputCcadcCalFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacOutputCcadcCalFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacOutputCcadcCalFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MacOutputShortedCcadcCalFieldsOut {
        /// The internal bits
        bits: [u8; 24],
    }
    impl ::device_driver::FieldSet for MacOutputShortedCcadcCalFieldsOut {
        const SIZE_BITS: u32 = 192;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MacOutputShortedCcadcCalFieldsOut {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 24] }
        }
        ///Read the `refresh_ctr` field of the register.
        ///
        pub fn refresh_ctr(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `status` field of the register.
        ///
        pub fn status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Read the `current` field of the register.
        ///
        pub fn current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cell_voltage_1` field of the register.
        ///
        pub fn cell_voltage_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cell_voltage_2` field of the register.
        ///
        pub fn cell_voltage_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `cell_voltage_3` field of the register.
        ///
        pub fn cell_voltage_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_voltage_4` field of the register.
        ///
        pub fn cell_voltage_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `pack_voltage` field of the register.
        ///
        pub fn pack_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `bat_voltage` field of the register.
        ///
        pub fn bat_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `cell_current_1` field of the register.
        ///
        pub fn cell_current_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `cell_current_2` field of the register.
        ///
        pub fn cell_current_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `cell_current_3` field of the register.
        ///
        pub fn cell_current_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `cell_current_4` field of the register.
        ///
        pub fn cell_current_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Write the `refresh_ctr` field of the register.
        ///
        pub fn set_refresh_ctr(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 8, &mut self.bits) };
        }
        ///Write the `status` field of the register.
        ///
        pub fn set_status(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 16, &mut self.bits) };
        }
        ///Write the `current` field of the register.
        ///
        pub fn set_current(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 16, 32, &mut self.bits) };
        }
        ///Write the `cell_voltage_1` field of the register.
        ///
        pub fn set_cell_voltage_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 32, 48, &mut self.bits) };
        }
        ///Write the `cell_voltage_2` field of the register.
        ///
        pub fn set_cell_voltage_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 48, 64, &mut self.bits) };
        }
        ///Write the `cell_voltage_3` field of the register.
        ///
        pub fn set_cell_voltage_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 64, 80, &mut self.bits) };
        }
        ///Write the `cell_voltage_4` field of the register.
        ///
        pub fn set_cell_voltage_4(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
        }
        ///Write the `pack_voltage` field of the register.
        ///
        pub fn set_pack_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 96, 112, &mut self.bits) };
        }
        ///Write the `bat_voltage` field of the register.
        ///
        pub fn set_bat_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 112, 128, &mut self.bits) };
        }
        ///Write the `cell_current_1` field of the register.
        ///
        pub fn set_cell_current_1(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 128, 144, &mut self.bits) };
        }
        ///Write the `cell_current_2` field of the register.
        ///
        pub fn set_cell_current_2(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 144, 160, &mut self.bits) };
        }
        ///Write the `cell_current_3` field of the register.
        ///
        pub fn set_cell_current_3(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 160, 176, &mut self.bits) };
        }
        ///Write the `cell_current_4` field of the register.
        ///
        pub fn set_cell_current_4(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 176, 192, &mut self.bits) };
        }
    }
    impl From<[u8; 24]> for MacOutputShortedCcadcCalFieldsOut {
        fn from(bits: [u8; 24]) -> Self {
            Self { bits }
        }
    }
    impl From<MacOutputShortedCcadcCalFieldsOut> for [u8; 24] {
        fn from(val: MacOutputShortedCcadcCalFieldsOut) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MacOutputShortedCcadcCalFieldsOut {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MacOutputShortedCcadcCalFieldsOut");
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
    impl defmt::Format for MacOutputShortedCcadcCalFieldsOut {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MacOutputShortedCcadcCalFieldsOut {{ ");
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
    impl core::ops::BitAnd for MacOutputShortedCcadcCalFieldsOut {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MacOutputShortedCcadcCalFieldsOut {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MacOutputShortedCcadcCalFieldsOut {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MacOutputShortedCcadcCalFieldsOut {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MacOutputShortedCcadcCalFieldsOut {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MacOutputShortedCcadcCalFieldsOut {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MacOutputShortedCcadcCalFieldsOut {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    /// This read/write word function sets a low capacity alarm threshold for the cell stack.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RemainingCapacityAlarm {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for RemainingCapacityAlarm {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl RemainingCapacityAlarm {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [44, 1] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `remaining_capacity_alarm` field of the register.
        ///
        pub fn remaining_capacity_alarm(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `remaining_capacity_alarm` field of the register.
        ///
        pub fn set_remaining_capacity_alarm(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    /// This read/write word function sets a low remaining time-to-fully discharge alarm threshold for the cell stack.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RemainingTimeAlarm {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for RemainingTimeAlarm {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl RemainingTimeAlarm {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [10, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `remaining_time_alarm` field of the register.
        ///
        pub fn remaining_time_alarm(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `remaining_time_alarm` field of the register.
        ///
        pub fn set_remaining_time_alarm(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryMode {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BatteryMode {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BatteryMode {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 64] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `icc` field of the register.
        ///
        pub fn icc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `pbs` field of the register.
        ///
        pub fn pbs(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `cf` field of the register.
        ///
        pub fn cf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `cc` field of the register.
        ///
        pub fn cc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `pb` field of the register.
        ///
        pub fn pb(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `am` field of the register.
        ///
        pub fn am(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `chgm` field of the register.
        ///
        pub fn chgm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `capm` field of the register.
        ///
        pub fn capm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Write the `cc` field of the register.
        ///
        pub fn set_cc(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 8, 9, &mut self.bits) };
        }
        ///Write the `pb` field of the register.
        ///
        pub fn set_pb(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 9, 10, &mut self.bits) };
        }
        ///Write the `am` field of the register.
        ///
        pub fn set_am(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 13, 14, &mut self.bits) };
        }
        ///Write the `chgm` field of the register.
        ///
        pub fn set_chgm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 14, 15, &mut self.bits) };
        }
        ///Write the `capm` field of the register.
        ///
        pub fn set_capm(&mut self, value: bool) {
            let raw = value as _;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 15, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AtRate {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AtRate {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AtRate {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `at_rate` field of the register.
        ///
        pub fn at_rate(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `at_rate` field of the register.
        ///
        pub fn set_at_rate(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AtRateTimeToFull {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AtRateTimeToFull {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AtRateTimeToFull {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `at_rate_time_to_full` field of the register.
        ///
        pub fn at_rate_time_to_full(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AtRateTimeToEmpty {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AtRateTimeToEmpty {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AtRateTimeToEmpty {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `at_rate_time_to_empty` field of the register.
        ///
        pub fn at_rate_time_to_empty(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AtRateOk {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AtRateOk {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AtRateOk {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `at_rate_ok` field of the register.
        ///
        pub fn at_rate_ok(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Temperature {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for Temperature {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Temperature {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `temperature` field of the register.
        ///
        pub fn temperature(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Voltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for Voltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Voltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `voltage` field of the register.
        ///
        pub fn voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Current {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for Current {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl Current {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `current` field of the register.
        ///
        pub fn current(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AvgCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AvgCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AvgCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `avg_current` field of the register.
        ///
        pub fn avg_current(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxError {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for MaxError {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MaxError {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `max_error` field of the register.
        ///
        pub fn max_error(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RelativeStateOfCharge {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for RelativeStateOfCharge {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl RelativeStateOfCharge {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `relative_state_of_charge` field of the register.
        ///
        pub fn relative_state_of_charge(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AbsoluteStateOfCharge {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AbsoluteStateOfCharge {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AbsoluteStateOfCharge {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `absolute_state_of_charge` field of the register.
        ///
        pub fn absolute_state_of_charge(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RemainingCapacity {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for RemainingCapacity {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl RemainingCapacity {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `remaining_capacity` field of the register.
        ///
        pub fn remaining_capacity(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FullChargeCapacity {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for FullChargeCapacity {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl FullChargeCapacity {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `full_charge_capacity` field of the register.
        ///
        pub fn full_charge_capacity(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct RunTimeToEmpty {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for RunTimeToEmpty {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl RunTimeToEmpty {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `run_time_to_empty` field of the register.
        ///
        pub fn run_time_to_empty(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AverageTimeToEmpty {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AverageTimeToEmpty {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AverageTimeToEmpty {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `average_time_to_empty` field of the register.
        ///
        pub fn average_time_to_empty(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AverageTimeToFull {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for AverageTimeToFull {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AverageTimeToFull {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `average_time_to_full` field of the register.
        ///
        pub fn average_time_to_full(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargingCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargingCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargingCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `charging_current` field of the register.
        ///
        pub fn charging_current(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargingVoltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ChargingVoltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargingVoltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `charging_voltage` field of the register.
        ///
        pub fn charging_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BatteryStatus {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BatteryStatus {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BatteryStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `ec` field of the register.
        ///
        pub fn ec(&self) -> ::embedded_batteries_async::smart_battery::ErrorCode {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 3) };
            raw.into()
        }
        ///Read the `fd` field of the register.
        ///
        pub fn fd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `fc` field of the register.
        ///
        pub fn fc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `dsg` field of the register.
        ///
        pub fn dsg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `init` field of the register.
        ///
        pub fn init(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `rta` field of the register.
        ///
        pub fn rta(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `rca` field of the register.
        ///
        pub fn rca(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `tda` field of the register.
        ///
        pub fn tda(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `ota` field of the register.
        ///
        pub fn ota(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `tca` field of the register.
        ///
        pub fn tca(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `oca` field of the register.
        ///
        pub fn oca(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CycleCount {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CycleCount {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CycleCount {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `cycle_count` field of the register.
        ///
        pub fn cycle_count(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `cycle_count` field of the register.
        ///
        pub fn set_cycle_count(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DesignCapacity {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for DesignCapacity {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DesignCapacity {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `design_capacity` field of the register.
        ///
        pub fn design_capacity(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `design_capacity` field of the register.
        ///
        pub fn set_design_capacity(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DesignVoltage {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for DesignVoltage {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DesignVoltage {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `design_voltage` field of the register.
        ///
        pub fn design_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `design_voltage` field of the register.
        ///
        pub fn set_design_voltage(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SpecificationInfo {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for SpecificationInfo {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SpecificationInfo {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `revision` field of the register.
        ///
        pub fn revision(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 4) };
            raw
        }
        ///Read the `version` field of the register.
        ///
        pub fn version(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 8) };
            raw
        }
        ///Read the `vscale` field of the register.
        ///
        pub fn vscale(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 12) };
            raw
        }
        ///Read the `ipscale` field of the register.
        ///
        pub fn ipscale(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 16) };
            raw
        }
        ///Write the `revision` field of the register.
        ///
        pub fn set_revision(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 0, 4, &mut self.bits) };
        }
        ///Write the `version` field of the register.
        ///
        pub fn set_version(&mut self, value: u8) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u8, ::device_driver::ops::LE>(raw, 4, 8, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ManufactureDate {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ManufactureDate {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ManufactureDate {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `manufacture_date` field of the register.
        ///
        pub fn manufacture_date(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `manufacture_date` field of the register.
        ///
        pub fn set_manufacture_date(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SerialNumber {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for SerialNumber {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SerialNumber {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [1, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `serial_number` field of the register.
        ///
        pub fn serial_number(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `serial_number` field of the register.
        ///
        pub fn set_serial_number(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CellVoltage4 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CellVoltage4 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CellVoltage4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `cell_voltage_4` field of the register.
        ///
        pub fn cell_voltage_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CellVoltage3 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CellVoltage3 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CellVoltage3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `cell_voltage_3` field of the register.
        ///
        pub fn cell_voltage_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CellVoltage2 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CellVoltage2 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CellVoltage2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `cell_voltage_2` field of the register.
        ///
        pub fn cell_voltage_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CellVoltage1 {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for CellVoltage1 {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CellVoltage1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `cell_voltage_1` field of the register.
        ///
        pub fn cell_voltage_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BtpDischargeSet {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BtpDischargeSet {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BtpDischargeSet {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [150, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `btp_discharge_set` field of the register.
        ///
        pub fn btp_discharge_set(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `btp_discharge_set` field of the register.
        ///
        pub fn set_btp_discharge_set(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct BtpChargeSet {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for BtpChargeSet {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl BtpChargeSet {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [175, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `btp_charge_set` field of the register.
        ///
        pub fn btp_charge_set(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `btp_charge_set` field of the register.
        ///
        pub fn set_btp_charge_set(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StateOfHealthSoh {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for StateOfHealthSoh {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl StateOfHealthSoh {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `state_of_health_soh` field of the register.
        ///
        pub fn state_of_health_soh(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SafetyAlert {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for SafetyAlert {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SafetyAlert {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `cuv` field of the register.
        ///
        pub fn cuv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `cov` field of the register.
        ///
        pub fn cov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `occ_1` field of the register.
        ///
        pub fn occ_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `occ_2` field of the register.
        ///
        pub fn occ_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `ocd_1` field of the register.
        ///
        pub fn ocd_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `ocd_2` field of the register.
        ///
        pub fn ocd_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `aoldl` field of the register.
        ///
        pub fn aoldl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `asccl` field of the register.
        ///
        pub fn asccl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `ascdl` field of the register.
        ///
        pub fn ascdl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `otc` field of the register.
        ///
        pub fn otc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `otd` field of the register.
        ///
        pub fn otd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `cuvc` field of the register.
        ///
        pub fn cuvc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `otf` field of the register.
        ///
        pub fn otf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `pto` field of the register.
        ///
        pub fn pto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `ptos` field of the register.
        ///
        pub fn ptos(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `cto` field of the register.
        ///
        pub fn cto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `ctos` field of the register.
        ///
        pub fn ctos(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `oc` field of the register.
        ///
        pub fn oc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `chgc` field of the register.
        ///
        pub fn chgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `chgv` field of the register.
        ///
        pub fn chgv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `pchgc` field of the register.
        ///
        pub fn pchgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `utc` field of the register.
        ///
        pub fn utc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `utd` field of the register.
        ///
        pub fn utd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 27, 28) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SafetyStatus {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for SafetyStatus {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SafetyStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `cuv` field of the register.
        ///
        pub fn cuv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `cov` field of the register.
        ///
        pub fn cov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `occ_1` field of the register.
        ///
        pub fn occ_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `occ_2` field of the register.
        ///
        pub fn occ_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `ocd_1` field of the register.
        ///
        pub fn ocd_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `ocd_2` field of the register.
        ///
        pub fn ocd_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `aold` field of the register.
        ///
        pub fn aold(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `aoldl` field of the register.
        ///
        pub fn aoldl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `ascc` field of the register.
        ///
        pub fn ascc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `asccl` field of the register.
        ///
        pub fn asccl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `ascd` field of the register.
        ///
        pub fn ascd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `ascdl` field of the register.
        ///
        pub fn ascdl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `otc` field of the register.
        ///
        pub fn otc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `otd` field of the register.
        ///
        pub fn otd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `cuvc` field of the register.
        ///
        pub fn cuvc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `otf` field of the register.
        ///
        pub fn otf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `pto` field of the register.
        ///
        pub fn pto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `ptos` field of the register.
        ///
        pub fn ptos(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `cto` field of the register.
        ///
        pub fn cto(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `oc` field of the register.
        ///
        pub fn oc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `chgc` field of the register.
        ///
        pub fn chgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `chgv` field of the register.
        ///
        pub fn chgv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `pchgc` field of the register.
        ///
        pub fn pchgc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `utc` field of the register.
        ///
        pub fn utc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `utd` field of the register.
        ///
        pub fn utd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 27, 28) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PfAlert {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for PfAlert {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PfAlert {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `suv` field of the register.
        ///
        pub fn suv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `sov` field of the register.
        ///
        pub fn sov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `socc` field of the register.
        ///
        pub fn socc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `socd` field of the register.
        ///
        pub fn socd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `sot` field of the register.
        ///
        pub fn sot(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `sotf` field of the register.
        ///
        pub fn sotf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `qim` field of the register.
        ///
        pub fn qim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `cb` field of the register.
        ///
        pub fn cb(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `imp` field of the register.
        ///
        pub fn imp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `cd` field of the register.
        ///
        pub fn cd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `vimr` field of the register.
        ///
        pub fn vimr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `vima` field of the register.
        ///
        pub fn vima(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `cfetf` field of the register.
        ///
        pub fn cfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `dfetf` field of the register.
        ///
        pub fn dfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `fuse` field of the register.
        ///
        pub fn fuse(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `afer` field of the register.
        ///
        pub fn afer(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `afec` field of the register.
        ///
        pub fn afec(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `second_lvl` field of the register.
        ///
        pub fn second_lvl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `opnc` field of the register.
        ///
        pub fn opnc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `ts_1` field of the register.
        ///
        pub fn ts_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 28, 29) };
            raw > 0
        }
        ///Read the `ts_2` field of the register.
        ///
        pub fn ts_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 29, 30) };
            raw > 0
        }
        ///Read the `ts_3` field of the register.
        ///
        pub fn ts_3(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 30, 31) };
            raw > 0
        }
        ///Read the `ts_4` field of the register.
        ///
        pub fn ts_4(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 31, 32) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct PfStatus {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for PfStatus {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl PfStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `suv` field of the register.
        ///
        pub fn suv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `sov` field of the register.
        ///
        pub fn sov(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `socc` field of the register.
        ///
        pub fn socc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `socd` field of the register.
        ///
        pub fn socd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `sot` field of the register.
        ///
        pub fn sot(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `sotf` field of the register.
        ///
        pub fn sotf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `qim` field of the register.
        ///
        pub fn qim(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `cb` field of the register.
        ///
        pub fn cb(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `imp` field of the register.
        ///
        pub fn imp(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `cd` field of the register.
        ///
        pub fn cd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `vimr` field of the register.
        ///
        pub fn vimr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `vima` field of the register.
        ///
        pub fn vima(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `cfetf` field of the register.
        ///
        pub fn cfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `dfetf` field of the register.
        ///
        pub fn dfetf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `fuse` field of the register.
        ///
        pub fn fuse(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `afer` field of the register.
        ///
        pub fn afer(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `afec` field of the register.
        ///
        pub fn afec(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `second_lvl` field of the register.
        ///
        pub fn second_lvl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `ptc` field of the register.
        ///
        pub fn ptc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `ifc` field of the register.
        ///
        pub fn ifc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `opncell` field of the register.
        ///
        pub fn opncell(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `dfw` field of the register.
        ///
        pub fn dfw(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `ts_1` field of the register.
        ///
        pub fn ts_1(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 28, 29) };
            raw > 0
        }
        ///Read the `ts_2` field of the register.
        ///
        pub fn ts_2(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 29, 30) };
            raw > 0
        }
        ///Read the `ts_3` field of the register.
        ///
        pub fn ts_3(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 30, 31) };
            raw > 0
        }
        ///Read the `ts_4` field of the register.
        ///
        pub fn ts_4(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 31, 32) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct OperationStatus {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for OperationStatus {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl OperationStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `pres` field of the register.
        ///
        pub fn pres(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `dsg` field of the register.
        ///
        pub fn dsg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `chg` field of the register.
        ///
        pub fn chg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `pchg` field of the register.
        ///
        pub fn pchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fuse` field of the register.
        ///
        pub fn fuse(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `btp_int` field of the register.
        ///
        pub fn btp_int(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `sec` field of the register.
        ///
        pub fn sec(&self) -> super::SecurityMode {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `sdv` field of the register.
        ///
        pub fn sdv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `ss` field of the register.
        ///
        pub fn ss(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `pf` field of the register.
        ///
        pub fn pf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `xdsg` field of the register.
        ///
        pub fn xdsg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `xchg` field of the register.
        ///
        pub fn xchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `sleep` field of the register.
        ///
        pub fn sleep(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `sdm` field of the register.
        ///
        pub fn sdm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `led` field of the register.
        ///
        pub fn led(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `auth` field of the register.
        ///
        pub fn auth(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `autocalm` field of the register.
        ///
        pub fn autocalm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `cal` field of the register.
        ///
        pub fn cal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `cal_offset` field of the register.
        ///
        pub fn cal_offset(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
        }
        ///Read the `xl` field of the register.
        ///
        pub fn xl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 22, 23) };
            raw > 0
        }
        ///Read the `sleepm` field of the register.
        ///
        pub fn sleepm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 23, 24) };
            raw > 0
        }
        ///Read the `init` field of the register.
        ///
        pub fn init(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 25) };
            raw > 0
        }
        ///Read the `smblcal` field of the register.
        ///
        pub fn smblcal(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 25, 26) };
            raw > 0
        }
        ///Read the `slpad` field of the register.
        ///
        pub fn slpad(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 26, 27) };
            raw > 0
        }
        ///Read the `slpcc` field of the register.
        ///
        pub fn slpcc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 27, 28) };
            raw > 0
        }
        ///Read the `cb` field of the register.
        ///
        pub fn cb(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 28, 29) };
            raw > 0
        }
        ///Read the `emshut` field of the register.
        ///
        pub fn emshut(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 29, 30) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ChargingStatus {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for ChargingStatus {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ChargingStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `ut` field of the register.
        ///
        pub fn ut(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `lt` field of the register.
        ///
        pub fn lt(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `stl` field of the register.
        ///
        pub fn stl(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `rt` field of the register.
        ///
        pub fn rt(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `sth` field of the register.
        ///
        pub fn sth(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `ht` field of the register.
        ///
        pub fn ht(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `ot` field of the register.
        ///
        pub fn ot(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `pv` field of the register.
        ///
        pub fn pv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `lv` field of the register.
        ///
        pub fn lv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `mv` field of the register.
        ///
        pub fn mv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `hv` field of the register.
        ///
        pub fn hv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `chg_in` field of the register.
        ///
        pub fn chg_in(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `chg_su` field of the register.
        ///
        pub fn chg_su(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `mchg` field of the register.
        ///
        pub fn mchg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `vct` field of the register.
        ///
        pub fn vct(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `ccr` field of the register.
        ///
        pub fn ccr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `cvr` field of the register.
        ///
        pub fn cvr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `ccc` field of the register.
        ///
        pub fn ccc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `nct` field of the register.
        ///
        pub fn nct(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `erm` field of the register.
        ///
        pub fn erm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
        }
        ///Read the `eretm` field of the register.
        ///
        pub fn eretm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 21, 22) };
            raw > 0
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
            d.field("chg_su", &self.chg_su());
            d.field("mchg", &self.mchg());
            d.field("vct", &self.vct());
            d.field("ccr", &self.ccr());
            d.field("cvr", &self.cvr());
            d.field("ccc", &self.ccc());
            d.field("nct", &self.nct());
            d.field("erm", &self.erm());
            d.field("eretm", &self.eretm());
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
            defmt::write!(f, "chg_su: {=bool}, ", &self.chg_su());
            defmt::write!(f, "mchg: {=bool}, ", &self.mchg());
            defmt::write!(f, "vct: {=bool}, ", &self.vct());
            defmt::write!(f, "ccr: {=bool}, ", &self.ccr());
            defmt::write!(f, "cvr: {=bool}, ", &self.cvr());
            defmt::write!(f, "ccc: {=bool}, ", &self.ccc());
            defmt::write!(f, "nct: {=bool}, ", &self.nct());
            defmt::write!(f, "erm: {=bool}, ", &self.erm());
            defmt::write!(f, "eretm: {=bool}, ", &self.eretm());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaugingStatus {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for GaugingStatus {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GaugingStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `fd` field of the register.
        ///
        pub fn fd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `fc` field of the register.
        ///
        pub fn fc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `td` field of the register.
        ///
        pub fn td(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `tc` field of the register.
        ///
        pub fn tc(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `bal_en` field of the register.
        ///
        pub fn bal_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `edv` field of the register.
        ///
        pub fn edv(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `dsg` field of the register.
        ///
        pub fn dsg(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `cf` field of the register.
        ///
        pub fn cf(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `rest` field of the register.
        ///
        pub fn rest(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `r_dis` field of the register.
        ///
        pub fn r_dis(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `vok` field of the register.
        ///
        pub fn vok(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `qen` field of the register.
        ///
        pub fn qen(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 12, 13) };
            raw > 0
        }
        ///Read the `slpqmax` field of the register.
        ///
        pub fn slpqmax(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 13, 14) };
            raw > 0
        }
        ///Read the `nsfm` field of the register.
        ///
        pub fn nsfm(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
        }
        ///Read the `vdq` field of the register.
        ///
        pub fn vdq(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 17) };
            raw > 0
        }
        ///Read the `qmax` field of the register.
        ///
        pub fn qmax(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 17, 18) };
            raw > 0
        }
        ///Read the `rx` field of the register.
        ///
        pub fn rx(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 18, 19) };
            raw > 0
        }
        ///Read the `ldmd` field of the register.
        ///
        pub fn ldmd(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 19, 20) };
            raw > 0
        }
        ///Read the `ocvfr` field of the register.
        ///
        pub fn ocvfr(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 20, 21) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct ManufacturingStatus {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for ManufacturingStatus {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl ManufacturingStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `pchg_en` field of the register.
        ///
        pub fn pchg_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 1) };
            raw > 0
        }
        ///Read the `chg_en` field of the register.
        ///
        pub fn chg_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 1, 2) };
            raw > 0
        }
        ///Read the `dsg_en` field of the register.
        ///
        pub fn dsg_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 2, 3) };
            raw > 0
        }
        ///Read the `gauge_en` field of the register.
        ///
        pub fn gauge_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 3, 4) };
            raw > 0
        }
        ///Read the `fet_en` field of the register.
        ///
        pub fn fet_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 4, 5) };
            raw > 0
        }
        ///Read the `lf_en` field of the register.
        ///
        pub fn lf_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 5, 6) };
            raw > 0
        }
        ///Read the `pf_en` field of the register.
        ///
        pub fn pf_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 6, 7) };
            raw > 0
        }
        ///Read the `bbr_en` field of the register.
        ///
        pub fn bbr_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 7, 8) };
            raw > 0
        }
        ///Read the `fuse_en` field of the register.
        ///
        pub fn fuse_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 9) };
            raw > 0
        }
        ///Read the `led_en` field of the register.
        ///
        pub fn led_en(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 9, 10) };
            raw > 0
        }
        ///Read the `lt_test` field of the register.
        ///
        pub fn lt_test(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 14, 15) };
            raw > 0
        }
        ///Read the `cal_test` field of the register.
        ///
        pub fn cal_test(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 15, 16) };
            raw > 0
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct AfeReg {
        /// The internal bits
        bits: [u8; 21],
    }
    impl ::device_driver::FieldSet for AfeReg {
        const SIZE_BITS: u32 = 168;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl AfeReg {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 21] }
        }
        ///Read the `afe_int_status` field of the register.
        ///
        pub fn afe_int_status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `afe_fet_status` field of the register.
        ///
        pub fn afe_fet_status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Read the `afe_rxin` field of the register.
        ///
        pub fn afe_rxin(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 24) };
            raw
        }
        ///Read the `afe_latch_status` field of the register.
        ///
        pub fn afe_latch_status(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 32) };
            raw
        }
        ///Read the `afe_int_en` field of the register.
        ///
        pub fn afe_int_en(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 32, 40) };
            raw
        }
        ///Read the `afe_ctrl` field of the register.
        ///
        pub fn afe_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 40, 48) };
            raw
        }
        ///Read the `afe_rxien` field of the register.
        ///
        pub fn afe_rxien(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 48, 56) };
            raw
        }
        ///Read the `afe_rlout` field of the register.
        ///
        pub fn afe_rlout(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 56, 64) };
            raw
        }
        ///Read the `afe_rhout` field of the register.
        ///
        pub fn afe_rhout(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 64, 72) };
            raw
        }
        ///Read the `afe_rhint` field of the register.
        ///
        pub fn afe_rhint(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 72, 80) };
            raw
        }
        ///Read the `afe_cell_balance` field of the register.
        ///
        pub fn afe_cell_balance(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 80, 88) };
            raw
        }
        ///Read the `afe_adc_cc_ctrl` field of the register.
        ///
        pub fn afe_adc_cc_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 88, 96) };
            raw
        }
        ///Read the `afe_adc_mux_ctrl` field of the register.
        ///
        pub fn afe_adc_mux_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 96, 104) };
            raw
        }
        ///Read the `afe_led_ctrl` field of the register.
        ///
        pub fn afe_led_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 104, 112) };
            raw
        }
        ///Read the `afe_hw_ctrl` field of the register.
        ///
        pub fn afe_hw_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 112, 120) };
            raw
        }
        ///Read the `afe_tmr_ctrl` field of the register.
        ///
        pub fn afe_tmr_ctrl(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 120, 128) };
            raw
        }
        ///Read the `afe_protection` field of the register.
        ///
        pub fn afe_protection(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 128, 136) };
            raw
        }
        ///Read the `afe_ocd` field of the register.
        ///
        pub fn afe_ocd(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 136, 144) };
            raw
        }
        ///Read the `afe_scc` field of the register.
        ///
        pub fn afe_scc(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 144, 152) };
            raw
        }
        ///Read the `afe_scd_1` field of the register.
        ///
        pub fn afe_scd_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 152, 160) };
            raw
        }
        ///Read the `afe_scd_2` field of the register.
        ///
        pub fn afe_scd_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 160, 168) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct MaxTurboPower {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for MaxTurboPower {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl MaxTurboPower {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `max_turbo_power` field of the register.
        ///
        pub fn max_turbo_power(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `max_turbo_power` field of the register.
        ///
        pub fn set_max_turbo_power(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for MaxTurboPower {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<MaxTurboPower> for [u8; 2] {
        fn from(val: MaxTurboPower) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for MaxTurboPower {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("MaxTurboPower");
            d.field("max_turbo_power", &self.max_turbo_power());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for MaxTurboPower {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "MaxTurboPower {{ ");
            defmt::write!(f, "max_turbo_power: {=i16}, ", &self.max_turbo_power());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for MaxTurboPower {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for MaxTurboPower {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for MaxTurboPower {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for MaxTurboPower {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for MaxTurboPower {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for MaxTurboPower {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for MaxTurboPower {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SusTurboPower {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for SusTurboPower {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SusTurboPower {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `sus_turbo_power` field of the register.
        ///
        pub fn sus_turbo_power(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `sus_turbo_power` field of the register.
        ///
        pub fn set_sus_turbo_power(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for SusTurboPower {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<SusTurboPower> for [u8; 2] {
        fn from(val: SusTurboPower) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for SusTurboPower {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("SusTurboPower");
            d.field("sus_turbo_power", &self.sus_turbo_power());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for SusTurboPower {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SusTurboPower {{ ");
            defmt::write!(f, "sus_turbo_power: {=i16}, ", &self.sus_turbo_power());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for SusTurboPower {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for SusTurboPower {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for SusTurboPower {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for SusTurboPower {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for SusTurboPower {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for SusTurboPower {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for SusTurboPower {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TurboPackR {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for TurboPackR {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl TurboPackR {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `turbo_pack_r` field of the register.
        ///
        pub fn turbo_pack_r(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `turbo_pack_r` field of the register.
        ///
        pub fn set_turbo_pack_r(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
            defmt::write!(f, "turbo_pack_r: {=i16}, ", &self.turbo_pack_r());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TurboSysR {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for TurboSysR {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl TurboSysR {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `turbo_sys_r` field of the register.
        ///
        pub fn turbo_sys_r(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `turbo_sys_r` field of the register.
        ///
        pub fn set_turbo_sys_r(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
            defmt::write!(f, "turbo_sys_r: {=i16}, ", &self.turbo_sys_r());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TurboEdv {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for TurboEdv {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl TurboEdv {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `turbo_edv` field of the register.
        ///
        pub fn turbo_edv(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `turbo_edv` field of the register.
        ///
        pub fn set_turbo_edv(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
            defmt::write!(f, "turbo_edv: {=i16}, ", &self.turbo_edv());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TurboCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for TurboCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl TurboCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `turbo_current` field of the register.
        ///
        pub fn turbo_current(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `turbo_current` field of the register.
        ///
        pub fn set_turbo_current(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
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
            defmt::write!(f, "turbo_current: {=i16}, ", &self.turbo_current());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct SusTurboCurrent {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for SusTurboCurrent {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl SusTurboCurrent {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `sus_turbo_current` field of the register.
        ///
        pub fn sus_turbo_current(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Write the `sus_turbo_current` field of the register.
        ///
        pub fn set_sus_turbo_current(&mut self, value: i16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<i16, ::device_driver::ops::LE>(raw, 0, 16, &mut self.bits) };
        }
    }
    impl From<[u8; 2]> for SusTurboCurrent {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<SusTurboCurrent> for [u8; 2] {
        fn from(val: SusTurboCurrent) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for SusTurboCurrent {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("SusTurboCurrent");
            d.field("sus_turbo_current", &self.sus_turbo_current());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for SusTurboCurrent {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "SusTurboCurrent {{ ");
            defmt::write!(f, "sus_turbo_current: {=i16}, ", &self.sus_turbo_current());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for SusTurboCurrent {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for SusTurboCurrent {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for SusTurboCurrent {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for SusTurboCurrent {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for SusTurboCurrent {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for SusTurboCurrent {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for SusTurboCurrent {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock1 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock1 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `cell_1_max_v` field of the register.
        ///
        pub fn cell_1_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `cell_2_max_v` field of the register.
        ///
        pub fn cell_2_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cell_3_max_v` field of the register.
        ///
        pub fn cell_3_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cell_4_max_v` field of the register.
        ///
        pub fn cell_4_max_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `cell_1_min_v` field of the register.
        ///
        pub fn cell_1_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_2_min_v` field of the register.
        ///
        pub fn cell_2_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `cell_3_min_v` field of the register.
        ///
        pub fn cell_3_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `cell_4_min_v` field of the register.
        ///
        pub fn cell_4_min_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `max_delta_cell_v` field of the register.
        ///
        pub fn max_delta_cell_v(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `max_charge_a` field of the register.
        ///
        pub fn max_charge_a(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `max_discharge_a` field of the register.
        ///
        pub fn max_discharge_a(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `max_avg_discharge_a` field of the register.
        ///
        pub fn max_avg_discharge_a(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `max_avg_discharge_pwr` field of the register.
        ///
        pub fn max_avg_discharge_pwr(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `max_temp_cell` field of the register.
        ///
        pub fn max_temp_cell(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 208, 216) };
            raw
        }
        ///Read the `min_temp_cell` field of the register.
        ///
        pub fn min_temp_cell(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 216, 224) };
            raw
        }
        ///Read the `max_delta_cell_temp` field of the register.
        ///
        pub fn max_delta_cell_temp(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 224, 232) };
            raw
        }
        ///Read the `max_temp_int_sensor` field of the register.
        ///
        pub fn max_temp_int_sensor(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 232, 240) };
            raw
        }
        ///Read the `min_temp_int_sensor` field of the register.
        ///
        pub fn min_temp_int_sensor(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 240, 248) };
            raw
        }
        ///Read the `max_temp_fet` field of the register.
        ///
        pub fn max_temp_fet(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 248, 256) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock2 {
        /// The internal bits
        bits: [u8; 20],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock2 {
        const SIZE_BITS: u32 = 160;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 20] }
        }
        ///Read the `num_shutdowns` field of the register.
        ///
        pub fn num_shutdowns(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `num_part_resets` field of the register.
        ///
        pub fn num_part_resets(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 16) };
            raw
        }
        ///Read the `num_full_resets` field of the register.
        ///
        pub fn num_full_resets(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 24) };
            raw
        }
        ///Read the `num_wdt_resets` field of the register.
        ///
        pub fn num_wdt_resets(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 32) };
            raw
        }
        ///Read the `cb_time_cell_1` field of the register.
        ///
        pub fn cb_time_cell_1(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `cb_time_cell_2` field of the register.
        ///
        pub fn cb_time_cell_2(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `cb_time_cell_3` field of the register.
        ///
        pub fn cb_time_cell_3(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `cb_time_cell_4` field of the register.
        ///
        pub fn cb_time_cell_4(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
    }
    impl From<[u8; 20]> for LifetimeDataBlock2 {
        fn from(bits: [u8; 20]) -> Self {
            Self { bits }
        }
    }
    impl From<LifetimeDataBlock2> for [u8; 20] {
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
            defmt::write!(f, "cb_time_cell_1: {=u32}, ", &self.cb_time_cell_1());
            defmt::write!(f, "cb_time_cell_2: {=u32}, ", &self.cb_time_cell_2());
            defmt::write!(f, "cb_time_cell_3: {=u32}, ", &self.cb_time_cell_3());
            defmt::write!(f, "cb_time_cell_4: {=u32}, ", &self.cb_time_cell_4());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock3 {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock3 {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `total_fw_runtime` field of the register.
        ///
        pub fn total_fw_runtime(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
    }
    impl From<[u8; 4]> for LifetimeDataBlock3 {
        fn from(bits: [u8; 4]) -> Self {
            Self { bits }
        }
    }
    impl From<LifetimeDataBlock3> for [u8; 4] {
        fn from(val: LifetimeDataBlock3) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for LifetimeDataBlock3 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("LifetimeDataBlock3");
            d.field("total_fw_runtime", &self.total_fw_runtime());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for LifetimeDataBlock3 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LifetimeDataBlock3 {{ ");
            defmt::write!(f, "total_fw_runtime: {=u32}, ", &self.total_fw_runtime());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock4 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock4 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock4 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `num_cov_events` field of the register.
        ///
        pub fn num_cov_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `last_cov_event` field of the register.
        ///
        pub fn last_cov_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `num_cuv_events` field of the register.
        ///
        pub fn num_cuv_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `last_cuv_event` field of the register.
        ///
        pub fn last_cuv_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `num_ocd_1_event` field of the register.
        ///
        pub fn num_ocd_1_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `last_ocd_1_event` field of the register.
        ///
        pub fn last_ocd_1_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `num_ocd_2_events` field of the register.
        ///
        pub fn num_ocd_2_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `last_ocd_2_event` field of the register.
        ///
        pub fn last_ocd_2_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `num_occ_1_events` field of the register.
        ///
        pub fn num_occ_1_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `last_occ_1_event` field of the register.
        ///
        pub fn last_occ_1_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `num_occ_2_events` field of the register.
        ///
        pub fn num_occ_2_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `last_occ_2_event` field of the register.
        ///
        pub fn last_occ_2_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `num_aold_events` field of the register.
        ///
        pub fn num_aold_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `last_aold_event` field of the register.
        ///
        pub fn last_aold_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `num_ascd_events` field of the register.
        ///
        pub fn num_ascd_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `last_ascd_event` field of the register.
        ///
        pub fn last_ascd_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock5 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock5 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock5 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `num_ascc_events` field of the register.
        ///
        pub fn num_ascc_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `last_ascc_event` field of the register.
        ///
        pub fn last_ascc_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `num_otc_events` field of the register.
        ///
        pub fn num_otc_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `last_otc_event` field of the register.
        ///
        pub fn last_otc_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `num_otd_event` field of the register.
        ///
        pub fn num_otd_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `last_otd_event` field of the register.
        ///
        pub fn last_otd_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `num_otf_events` field of the register.
        ///
        pub fn num_otf_events(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `last_otf_event` field of the register.
        ///
        pub fn last_otf_event(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `num_valid_chg_term` field of the register.
        ///
        pub fn num_valid_chg_term(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `last_valid_chg_term` field of the register.
        ///
        pub fn last_valid_chg_term(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `num_qmax_updates` field of the register.
        ///
        pub fn num_qmax_updates(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `last_qmax_update` field of the register.
        ///
        pub fn last_qmax_update(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `num_ra_updates` field of the register.
        ///
        pub fn num_ra_updates(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `last_ra_update` field of the register.
        ///
        pub fn last_ra_update(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `num_ra_disable` field of the register.
        ///
        pub fn num_ra_disable(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `last_ra_disable` field of the register.
        ///
        pub fn last_ra_disable(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock6 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock6 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock6 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_ut_rsoc_a` field of the register.
        ///
        pub fn time_spent_ut_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_b` field of the register.
        ///
        pub fn time_spent_ut_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_c` field of the register.
        ///
        pub fn time_spent_ut_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_d` field of the register.
        ///
        pub fn time_spent_ut_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_e` field of the register.
        ///
        pub fn time_spent_ut_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_f` field of the register.
        ///
        pub fn time_spent_ut_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_g` field of the register.
        ///
        pub fn time_spent_ut_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_ut_rsoc_h` field of the register.
        ///
        pub fn time_spent_ut_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
    }
    impl From<[u8; 32]> for LifetimeDataBlock6 {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<LifetimeDataBlock6> for [u8; 32] {
        fn from(val: LifetimeDataBlock6) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for LifetimeDataBlock6 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("LifetimeDataBlock6");
            d.field("time_spent_ut_rsoc_a", &self.time_spent_ut_rsoc_a());
            d.field("time_spent_ut_rsoc_b", &self.time_spent_ut_rsoc_b());
            d.field("time_spent_ut_rsoc_c", &self.time_spent_ut_rsoc_c());
            d.field("time_spent_ut_rsoc_d", &self.time_spent_ut_rsoc_d());
            d.field("time_spent_ut_rsoc_e", &self.time_spent_ut_rsoc_e());
            d.field("time_spent_ut_rsoc_f", &self.time_spent_ut_rsoc_f());
            d.field("time_spent_ut_rsoc_g", &self.time_spent_ut_rsoc_g());
            d.field("time_spent_ut_rsoc_h", &self.time_spent_ut_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for LifetimeDataBlock6 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LifetimeDataBlock6 {{ ");
            defmt::write!(f, "time_spent_ut_rsoc_a: {=u32}, ", &self.time_spent_ut_rsoc_a());
            defmt::write!(f, "time_spent_ut_rsoc_b: {=u32}, ", &self.time_spent_ut_rsoc_b());
            defmt::write!(f, "time_spent_ut_rsoc_c: {=u32}, ", &self.time_spent_ut_rsoc_c());
            defmt::write!(f, "time_spent_ut_rsoc_d: {=u32}, ", &self.time_spent_ut_rsoc_d());
            defmt::write!(f, "time_spent_ut_rsoc_e: {=u32}, ", &self.time_spent_ut_rsoc_e());
            defmt::write!(f, "time_spent_ut_rsoc_f: {=u32}, ", &self.time_spent_ut_rsoc_f());
            defmt::write!(f, "time_spent_ut_rsoc_g: {=u32}, ", &self.time_spent_ut_rsoc_g());
            defmt::write!(f, "time_spent_ut_rsoc_h: {=u32}, ", &self.time_spent_ut_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for LifetimeDataBlock6 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for LifetimeDataBlock6 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for LifetimeDataBlock6 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for LifetimeDataBlock6 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for LifetimeDataBlock6 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for LifetimeDataBlock6 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for LifetimeDataBlock6 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock7 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock7 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock7 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_lt_rsoc_a` field of the register.
        ///
        pub fn time_spent_lt_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_b` field of the register.
        ///
        pub fn time_spent_lt_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_c` field of the register.
        ///
        pub fn time_spent_lt_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_d` field of the register.
        ///
        pub fn time_spent_lt_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_e` field of the register.
        ///
        pub fn time_spent_lt_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_f` field of the register.
        ///
        pub fn time_spent_lt_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_g` field of the register.
        ///
        pub fn time_spent_lt_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_lt_rsoc_h` field of the register.
        ///
        pub fn time_spent_lt_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
    }
    impl From<[u8; 32]> for LifetimeDataBlock7 {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<LifetimeDataBlock7> for [u8; 32] {
        fn from(val: LifetimeDataBlock7) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for LifetimeDataBlock7 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("LifetimeDataBlock7");
            d.field("time_spent_lt_rsoc_a", &self.time_spent_lt_rsoc_a());
            d.field("time_spent_lt_rsoc_b", &self.time_spent_lt_rsoc_b());
            d.field("time_spent_lt_rsoc_c", &self.time_spent_lt_rsoc_c());
            d.field("time_spent_lt_rsoc_d", &self.time_spent_lt_rsoc_d());
            d.field("time_spent_lt_rsoc_e", &self.time_spent_lt_rsoc_e());
            d.field("time_spent_lt_rsoc_f", &self.time_spent_lt_rsoc_f());
            d.field("time_spent_lt_rsoc_g", &self.time_spent_lt_rsoc_g());
            d.field("time_spent_lt_rsoc_h", &self.time_spent_lt_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for LifetimeDataBlock7 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LifetimeDataBlock7 {{ ");
            defmt::write!(f, "time_spent_lt_rsoc_a: {=u32}, ", &self.time_spent_lt_rsoc_a());
            defmt::write!(f, "time_spent_lt_rsoc_b: {=u32}, ", &self.time_spent_lt_rsoc_b());
            defmt::write!(f, "time_spent_lt_rsoc_c: {=u32}, ", &self.time_spent_lt_rsoc_c());
            defmt::write!(f, "time_spent_lt_rsoc_d: {=u32}, ", &self.time_spent_lt_rsoc_d());
            defmt::write!(f, "time_spent_lt_rsoc_e: {=u32}, ", &self.time_spent_lt_rsoc_e());
            defmt::write!(f, "time_spent_lt_rsoc_f: {=u32}, ", &self.time_spent_lt_rsoc_f());
            defmt::write!(f, "time_spent_lt_rsoc_g: {=u32}, ", &self.time_spent_lt_rsoc_g());
            defmt::write!(f, "time_spent_lt_rsoc_h: {=u32}, ", &self.time_spent_lt_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for LifetimeDataBlock7 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for LifetimeDataBlock7 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for LifetimeDataBlock7 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for LifetimeDataBlock7 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for LifetimeDataBlock7 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for LifetimeDataBlock7 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for LifetimeDataBlock7 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock8 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock8 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock8 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_stl_rsoc_a` field of the register.
        ///
        pub fn time_spent_stl_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_b` field of the register.
        ///
        pub fn time_spent_stl_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_c` field of the register.
        ///
        pub fn time_spent_stl_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_d` field of the register.
        ///
        pub fn time_spent_stl_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_e` field of the register.
        ///
        pub fn time_spent_stl_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_f` field of the register.
        ///
        pub fn time_spent_stl_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_g` field of the register.
        ///
        pub fn time_spent_stl_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_stl_rsoc_h` field of the register.
        ///
        pub fn time_spent_stl_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
    }
    impl From<[u8; 32]> for LifetimeDataBlock8 {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<LifetimeDataBlock8> for [u8; 32] {
        fn from(val: LifetimeDataBlock8) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for LifetimeDataBlock8 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("LifetimeDataBlock8");
            d.field("time_spent_stl_rsoc_a", &self.time_spent_stl_rsoc_a());
            d.field("time_spent_stl_rsoc_b", &self.time_spent_stl_rsoc_b());
            d.field("time_spent_stl_rsoc_c", &self.time_spent_stl_rsoc_c());
            d.field("time_spent_stl_rsoc_d", &self.time_spent_stl_rsoc_d());
            d.field("time_spent_stl_rsoc_e", &self.time_spent_stl_rsoc_e());
            d.field("time_spent_stl_rsoc_f", &self.time_spent_stl_rsoc_f());
            d.field("time_spent_stl_rsoc_g", &self.time_spent_stl_rsoc_g());
            d.field("time_spent_stl_rsoc_h", &self.time_spent_stl_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for LifetimeDataBlock8 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LifetimeDataBlock8 {{ ");
            defmt::write!(f, "time_spent_stl_rsoc_a: {=u32}, ", &self.time_spent_stl_rsoc_a());
            defmt::write!(f, "time_spent_stl_rsoc_b: {=u32}, ", &self.time_spent_stl_rsoc_b());
            defmt::write!(f, "time_spent_stl_rsoc_c: {=u32}, ", &self.time_spent_stl_rsoc_c());
            defmt::write!(f, "time_spent_stl_rsoc_d: {=u32}, ", &self.time_spent_stl_rsoc_d());
            defmt::write!(f, "time_spent_stl_rsoc_e: {=u32}, ", &self.time_spent_stl_rsoc_e());
            defmt::write!(f, "time_spent_stl_rsoc_f: {=u32}, ", &self.time_spent_stl_rsoc_f());
            defmt::write!(f, "time_spent_stl_rsoc_g: {=u32}, ", &self.time_spent_stl_rsoc_g());
            defmt::write!(f, "time_spent_stl_rsoc_h: {=u32}, ", &self.time_spent_stl_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for LifetimeDataBlock8 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for LifetimeDataBlock8 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for LifetimeDataBlock8 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for LifetimeDataBlock8 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for LifetimeDataBlock8 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for LifetimeDataBlock8 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for LifetimeDataBlock8 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TurboRhfEffective {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for TurboRhfEffective {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl TurboRhfEffective {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `turbo_rhf_effective` field of the register.
        ///
        pub fn turbo_rhf_effective(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
    }
    impl From<[u8; 2]> for TurboRhfEffective {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<TurboRhfEffective> for [u8; 2] {
        fn from(val: TurboRhfEffective) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for TurboRhfEffective {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("TurboRhfEffective");
            d.field("turbo_rhf_effective", &self.turbo_rhf_effective());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for TurboRhfEffective {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TurboRhfEffective {{ ");
            defmt::write!(f, "turbo_rhf_effective: {=i16}, ", &self.turbo_rhf_effective());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for TurboRhfEffective {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for TurboRhfEffective {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for TurboRhfEffective {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for TurboRhfEffective {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for TurboRhfEffective {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for TurboRhfEffective {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for TurboRhfEffective {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct TurboVload {
        /// The internal bits
        bits: [u8; 2],
    }
    impl ::device_driver::FieldSet for TurboVload {
        const SIZE_BITS: u32 = 16;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl TurboVload {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 2] }
        }
        ///Read the `turbo_vload` field of the register.
        ///
        pub fn turbo_vload(&self) -> i16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<i16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
    }
    impl From<[u8; 2]> for TurboVload {
        fn from(bits: [u8; 2]) -> Self {
            Self { bits }
        }
    }
    impl From<TurboVload> for [u8; 2] {
        fn from(val: TurboVload) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for TurboVload {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("TurboVload");
            d.field("turbo_vload", &self.turbo_vload());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for TurboVload {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "TurboVload {{ ");
            defmt::write!(f, "turbo_vload: {=i16}, ", &self.turbo_vload());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for TurboVload {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for TurboVload {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for TurboVload {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for TurboVload {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for TurboVload {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for TurboVload {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for TurboVload {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock11 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock11 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock11 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_ht_rsoc_a` field of the register.
        ///
        pub fn time_spent_ht_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_b` field of the register.
        ///
        pub fn time_spent_ht_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_c` field of the register.
        ///
        pub fn time_spent_ht_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_d` field of the register.
        ///
        pub fn time_spent_ht_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_e` field of the register.
        ///
        pub fn time_spent_ht_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_f` field of the register.
        ///
        pub fn time_spent_ht_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_g` field of the register.
        ///
        pub fn time_spent_ht_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_ht_rsoc_h` field of the register.
        ///
        pub fn time_spent_ht_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
    }
    impl From<[u8; 32]> for LifetimeDataBlock11 {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<LifetimeDataBlock11> for [u8; 32] {
        fn from(val: LifetimeDataBlock11) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for LifetimeDataBlock11 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("LifetimeDataBlock11");
            d.field("time_spent_ht_rsoc_a", &self.time_spent_ht_rsoc_a());
            d.field("time_spent_ht_rsoc_b", &self.time_spent_ht_rsoc_b());
            d.field("time_spent_ht_rsoc_c", &self.time_spent_ht_rsoc_c());
            d.field("time_spent_ht_rsoc_d", &self.time_spent_ht_rsoc_d());
            d.field("time_spent_ht_rsoc_e", &self.time_spent_ht_rsoc_e());
            d.field("time_spent_ht_rsoc_f", &self.time_spent_ht_rsoc_f());
            d.field("time_spent_ht_rsoc_g", &self.time_spent_ht_rsoc_g());
            d.field("time_spent_ht_rsoc_h", &self.time_spent_ht_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for LifetimeDataBlock11 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LifetimeDataBlock11 {{ ");
            defmt::write!(f, "time_spent_ht_rsoc_a: {=u32}, ", &self.time_spent_ht_rsoc_a());
            defmt::write!(f, "time_spent_ht_rsoc_b: {=u32}, ", &self.time_spent_ht_rsoc_b());
            defmt::write!(f, "time_spent_ht_rsoc_c: {=u32}, ", &self.time_spent_ht_rsoc_c());
            defmt::write!(f, "time_spent_ht_rsoc_d: {=u32}, ", &self.time_spent_ht_rsoc_d());
            defmt::write!(f, "time_spent_ht_rsoc_e: {=u32}, ", &self.time_spent_ht_rsoc_e());
            defmt::write!(f, "time_spent_ht_rsoc_f: {=u32}, ", &self.time_spent_ht_rsoc_f());
            defmt::write!(f, "time_spent_ht_rsoc_g: {=u32}, ", &self.time_spent_ht_rsoc_g());
            defmt::write!(f, "time_spent_ht_rsoc_h: {=u32}, ", &self.time_spent_ht_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for LifetimeDataBlock11 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for LifetimeDataBlock11 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for LifetimeDataBlock11 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for LifetimeDataBlock11 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for LifetimeDataBlock11 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for LifetimeDataBlock11 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for LifetimeDataBlock11 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct LifetimeDataBlock12 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for LifetimeDataBlock12 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl LifetimeDataBlock12 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `time_spent_ot_rsoc_a` field of the register.
        ///
        pub fn time_spent_ot_rsoc_a(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 0, 32) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_b` field of the register.
        ///
        pub fn time_spent_ot_rsoc_b(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 32, 64) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_c` field of the register.
        ///
        pub fn time_spent_ot_rsoc_c(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 64, 96) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_d` field of the register.
        ///
        pub fn time_spent_ot_rsoc_d(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 96, 128) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_e` field of the register.
        ///
        pub fn time_spent_ot_rsoc_e(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 128, 160) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_f` field of the register.
        ///
        pub fn time_spent_ot_rsoc_f(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 160, 192) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_g` field of the register.
        ///
        pub fn time_spent_ot_rsoc_g(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 192, 224) };
            raw
        }
        ///Read the `time_spent_ot_rsoc_h` field of the register.
        ///
        pub fn time_spent_ot_rsoc_h(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 224, 256) };
            raw
        }
    }
    impl From<[u8; 32]> for LifetimeDataBlock12 {
        fn from(bits: [u8; 32]) -> Self {
            Self { bits }
        }
    }
    impl From<LifetimeDataBlock12> for [u8; 32] {
        fn from(val: LifetimeDataBlock12) -> Self {
            val.bits
        }
    }
    impl core::fmt::Debug for LifetimeDataBlock12 {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> {
            let mut d = f.debug_struct("LifetimeDataBlock12");
            d.field("time_spent_ot_rsoc_a", &self.time_spent_ot_rsoc_a());
            d.field("time_spent_ot_rsoc_b", &self.time_spent_ot_rsoc_b());
            d.field("time_spent_ot_rsoc_c", &self.time_spent_ot_rsoc_c());
            d.field("time_spent_ot_rsoc_d", &self.time_spent_ot_rsoc_d());
            d.field("time_spent_ot_rsoc_e", &self.time_spent_ot_rsoc_e());
            d.field("time_spent_ot_rsoc_f", &self.time_spent_ot_rsoc_f());
            d.field("time_spent_ot_rsoc_g", &self.time_spent_ot_rsoc_g());
            d.field("time_spent_ot_rsoc_h", &self.time_spent_ot_rsoc_h());
            d.finish()
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for LifetimeDataBlock12 {
        fn format(&self, f: defmt::Formatter) {
            defmt::write!(f, "LifetimeDataBlock12 {{ ");
            defmt::write!(f, "time_spent_ot_rsoc_a: {=u32}, ", &self.time_spent_ot_rsoc_a());
            defmt::write!(f, "time_spent_ot_rsoc_b: {=u32}, ", &self.time_spent_ot_rsoc_b());
            defmt::write!(f, "time_spent_ot_rsoc_c: {=u32}, ", &self.time_spent_ot_rsoc_c());
            defmt::write!(f, "time_spent_ot_rsoc_d: {=u32}, ", &self.time_spent_ot_rsoc_d());
            defmt::write!(f, "time_spent_ot_rsoc_e: {=u32}, ", &self.time_spent_ot_rsoc_e());
            defmt::write!(f, "time_spent_ot_rsoc_f: {=u32}, ", &self.time_spent_ot_rsoc_f());
            defmt::write!(f, "time_spent_ot_rsoc_g: {=u32}, ", &self.time_spent_ot_rsoc_g());
            defmt::write!(f, "time_spent_ot_rsoc_h: {=u32}, ", &self.time_spent_ot_rsoc_h());
            defmt::write!(f, "}}");
        }
    }
    impl core::ops::BitAnd for LifetimeDataBlock12 {
        type Output = Self;
        fn bitand(mut self, rhs: Self) -> Self::Output {
            self &= rhs;
            self
        }
    }
    impl core::ops::BitAndAssign for LifetimeDataBlock12 {
        fn bitand_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l &= *r;
            }
        }
    }
    impl core::ops::BitOr for LifetimeDataBlock12 {
        type Output = Self;
        fn bitor(mut self, rhs: Self) -> Self::Output {
            self |= rhs;
            self
        }
    }
    impl core::ops::BitOrAssign for LifetimeDataBlock12 {
        fn bitor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l |= *r;
            }
        }
    }
    impl core::ops::BitXor for LifetimeDataBlock12 {
        type Output = Self;
        fn bitxor(mut self, rhs: Self) -> Self::Output {
            self ^= rhs;
            self
        }
    }
    impl core::ops::BitXorAssign for LifetimeDataBlock12 {
        fn bitxor_assign(&mut self, rhs: Self) {
            for (l, r) in self.bits.iter_mut().zip(&rhs.bits) {
                *l ^= *r;
            }
        }
    }
    impl core::ops::Not for LifetimeDataBlock12 {
        type Output = Self;
        fn not(mut self) -> Self::Output {
            for val in self.bits.iter_mut() {
                *val = !*val;
            }
            self
        }
    }
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DaStatus1 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for DaStatus1 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DaStatus1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `cell_voltage_1` field of the register.
        ///
        pub fn cell_voltage_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `cell_voltage_2` field of the register.
        ///
        pub fn cell_voltage_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cell_voltage_3` field of the register.
        ///
        pub fn cell_voltage_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cell_voltage_4` field of the register.
        ///
        pub fn cell_voltage_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `bat_voltage` field of the register.
        ///
        pub fn bat_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `pack_voltage` field of the register.
        ///
        pub fn pack_voltage(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `cell_current_1` field of the register.
        ///
        pub fn cell_current_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `cell_current_2` field of the register.
        ///
        pub fn cell_current_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `cell_current_3` field of the register.
        ///
        pub fn cell_current_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `cell_current_4` field of the register.
        ///
        pub fn cell_current_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `cell_pwr_1` field of the register.
        ///
        pub fn cell_pwr_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `cell_pwr_2` field of the register.
        ///
        pub fn cell_pwr_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `cell_pwr_3` field of the register.
        ///
        pub fn cell_pwr_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `cell_pwr_4` field of the register.
        ///
        pub fn cell_pwr_4(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `total_pwr` field of the register.
        ///
        pub fn total_pwr(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `avg_pwr` field of the register.
        ///
        pub fn avg_pwr(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct DaStatus2 {
        /// The internal bits
        bits: [u8; 16],
    }
    impl ::device_driver::FieldSet for DaStatus2 {
        const SIZE_BITS: u32 = 128;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl DaStatus2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 16] }
        }
        ///Read the `int_temp` field of the register.
        ///
        pub fn int_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `ts_1_temp` field of the register.
        ///
        pub fn ts_1_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `ts_2_temp` field of the register.
        ///
        pub fn ts_2_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `ts_3_temp` field of the register.
        ///
        pub fn ts_3_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `ts_4_temp` field of the register.
        ///
        pub fn ts_4_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_temp` field of the register.
        ///
        pub fn cell_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `fet_temp` field of the register.
        ///
        pub fn fet_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `gauging_temp` field of the register.
        ///
        pub fn gauging_temp(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
    }
    impl From<[u8; 16]> for DaStatus2 {
        fn from(bits: [u8; 16]) -> Self {
            Self { bits }
        }
    }
    impl From<DaStatus2> for [u8; 16] {
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
            d.field("gauging_temp", &self.gauging_temp());
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
            defmt::write!(f, "gauging_temp: {=u16}, ", &self.gauging_temp());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaugeStatus1 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for GaugeStatus1 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GaugeStatus1 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `true_rem_q` field of the register.
        ///
        pub fn true_rem_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `true_rem_e` field of the register.
        ///
        pub fn true_rem_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `initial_q` field of the register.
        ///
        pub fn initial_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `initial_e` field of the register.
        ///
        pub fn initial_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `true_fcc_q` field of the register.
        ///
        pub fn true_fcc_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `true_fcc_e` field of the register.
        ///
        pub fn true_fcc_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `t_sim` field of the register.
        ///
        pub fn t_sim(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `t_ambient` field of the register.
        ///
        pub fn t_ambient(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `ra_scale_0` field of the register.
        ///
        pub fn ra_scale_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `ra_scale_1` field of the register.
        ///
        pub fn ra_scale_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `ra_scale_2` field of the register.
        ///
        pub fn ra_scale_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `ra_scale_3` field of the register.
        ///
        pub fn ra_scale_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `comp_res_0` field of the register.
        ///
        pub fn comp_res_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `comp_res_1` field of the register.
        ///
        pub fn comp_res_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `comp_res_2` field of the register.
        ///
        pub fn comp_res_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `comp_res_3` field of the register.
        ///
        pub fn comp_res_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaugeStatus2 {
        /// The internal bits
        bits: [u8; 32],
    }
    impl ::device_driver::FieldSet for GaugeStatus2 {
        const SIZE_BITS: u32 = 256;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GaugeStatus2 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [
                    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                ],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 32] }
        }
        ///Read the `pack_grid` field of the register.
        ///
        pub fn pack_grid(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 0, 8) };
            raw
        }
        ///Read the `q_max_status` field of the register.
        ///
        pub fn q_max_status(&self) -> super::QMaxStatus {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 8, 10) };
            unsafe { raw.try_into().unwrap_unchecked() }
        }
        ///Read the `iten` field of the register.
        ///
        pub fn iten(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 10, 11) };
            raw > 0
        }
        ///Read the `qmax_field_updated` field of the register.
        ///
        pub fn qmax_field_updated(&self) -> bool {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 11, 12) };
            raw > 0
        }
        ///Read the `cell_grid_0` field of the register.
        ///
        pub fn cell_grid_0(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 16, 24) };
            raw
        }
        ///Read the `cell_grid_1` field of the register.
        ///
        pub fn cell_grid_1(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 24, 32) };
            raw
        }
        ///Read the `cell_grid_2` field of the register.
        ///
        pub fn cell_grid_2(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 32, 40) };
            raw
        }
        ///Read the `cell_grid_3` field of the register.
        ///
        pub fn cell_grid_3(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 40, 48) };
            raw
        }
        ///Read the `state_time` field of the register.
        ///
        pub fn state_time(&self) -> u32 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u32, ::device_driver::ops::LE>(&self.bits, 48, 80) };
            raw
        }
        ///Read the `dod_0_0` field of the register.
        ///
        pub fn dod_0_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `dod_0_1` field of the register.
        ///
        pub fn dod_0_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `dod_0_2` field of the register.
        ///
        pub fn dod_0_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `dod_0_3` field of the register.
        ///
        pub fn dod_0_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `dod_0_passed_q` field of the register.
        ///
        pub fn dod_0_passed_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `dod_0_passed_e` field of the register.
        ///
        pub fn dod_0_passed_e(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `dod_0_time` field of the register.
        ///
        pub fn dod_0_time(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
        }
        ///Read the `dodeoc_0` field of the register.
        ///
        pub fn dodeoc_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 192, 208) };
            raw
        }
        ///Read the `dodeoc_1` field of the register.
        ///
        pub fn dodeoc_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 208, 224) };
            raw
        }
        ///Read the `dodeoc_2` field of the register.
        ///
        pub fn dodeoc_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 224, 240) };
            raw
        }
        ///Read the `dodeoc_3` field of the register.
        ///
        pub fn dodeoc_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 240, 256) };
            raw
        }
        ///Write the `dod_0_0` field of the register.
        ///
        pub fn set_dod_0_0(&mut self, value: u16) {
            let raw = value;
            unsafe { ::device_driver::ops::store_lsb0::<u16, ::device_driver::ops::LE>(raw, 80, 96, &mut self.bits) };
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct GaugeStatus3 {
        /// The internal bits
        bits: [u8; 24],
    }
    impl ::device_driver::FieldSet for GaugeStatus3 {
        const SIZE_BITS: u32 = 192;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl GaugeStatus3 {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 24] }
        }
        ///Read the `qmax_0` field of the register.
        ///
        pub fn qmax_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `qmax_1` field of the register.
        ///
        pub fn qmax_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `qmax_2` field of the register.
        ///
        pub fn qmax_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `qmax_3` field of the register.
        ///
        pub fn qmax_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `qmax_dod_0_0` field of the register.
        ///
        pub fn qmax_dod_0_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `qmax_dod_0_1` field of the register.
        ///
        pub fn qmax_dod_0_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `qmax_dod_0_2` field of the register.
        ///
        pub fn qmax_dod_0_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `qmax_dod_0_3` field of the register.
        ///
        pub fn qmax_dod_0_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `qmax_passed_q` field of the register.
        ///
        pub fn qmax_passed_q(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `qmax_time` field of the register.
        ///
        pub fn qmax_time(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 144, 160) };
            raw
        }
        ///Read the `temp_k_factor` field of the register.
        ///
        pub fn temp_k_factor(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 160, 176) };
            raw
        }
        ///Read the `temp_a_factor` field of the register.
        ///
        pub fn temp_a_factor(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 176, 192) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct CbStatus {
        /// The internal bits
        bits: [u8; 19],
    }
    impl ::device_driver::FieldSet for CbStatus {
        const SIZE_BITS: u32 = 152;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl CbStatus {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 19] }
        }
        ///Read the `cb_time_0` field of the register.
        ///
        pub fn cb_time_0(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `cb_time_1` field of the register.
        ///
        pub fn cb_time_1(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `cb_time_2` field of the register.
        ///
        pub fn cb_time_2(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `cb_time_3` field of the register.
        ///
        pub fn cb_time_3(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
        }
        ///Read the `cell_1_balance_dod` field of the register.
        ///
        pub fn cell_1_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 64, 80) };
            raw
        }
        ///Read the `cell_2_balance_dod` field of the register.
        ///
        pub fn cell_2_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 80, 96) };
            raw
        }
        ///Read the `cell_3_balance_dod` field of the register.
        ///
        pub fn cell_3_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 96, 112) };
            raw
        }
        ///Read the `cell_4_balance_dod` field of the register.
        ///
        pub fn cell_4_balance_dod(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 112, 128) };
            raw
        }
        ///Read the `total_dod_chrg` field of the register.
        ///
        pub fn total_dod_chrg(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 128, 144) };
            raw
        }
        ///Read the `cell_balance_stat` field of the register.
        ///
        pub fn cell_balance_stat(&self) -> u8 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u8, ::device_driver::ops::LE>(&self.bits, 144, 152) };
            raw
        }
    }
    impl From<[u8; 19]> for CbStatus {
        fn from(bits: [u8; 19]) -> Self {
            Self { bits }
        }
    }
    impl From<CbStatus> for [u8; 19] {
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
            d.field("cell_1_balance_dod", &self.cell_1_balance_dod());
            d.field("cell_2_balance_dod", &self.cell_2_balance_dod());
            d.field("cell_3_balance_dod", &self.cell_3_balance_dod());
            d.field("cell_4_balance_dod", &self.cell_4_balance_dod());
            d.field("total_dod_chrg", &self.total_dod_chrg());
            d.field("cell_balance_stat", &self.cell_balance_stat());
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
            defmt::write!(f, "cell_1_balance_dod: {=u16}, ", &self.cell_1_balance_dod());
            defmt::write!(f, "cell_2_balance_dod: {=u16}, ", &self.cell_2_balance_dod());
            defmt::write!(f, "cell_3_balance_dod: {=u16}, ", &self.cell_3_balance_dod());
            defmt::write!(f, "cell_4_balance_dod: {=u16}, ", &self.cell_4_balance_dod());
            defmt::write!(f, "total_dod_chrg: {=u16}, ", &self.total_dod_chrg());
            defmt::write!(f, "cell_balance_stat: {=u8}, ", &self.cell_balance_stat());
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct StateOfHealth {
        /// The internal bits
        bits: [u8; 4],
    }
    impl ::device_driver::FieldSet for StateOfHealth {
        const SIZE_BITS: u32 = 32;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl StateOfHealth {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self { bits: [0, 0, 0, 0] }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 4] }
        }
        ///Read the `soh_fcc` field of the register.
        ///
        pub fn soh_fcc(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `soh_energy` field of the register.
        ///
        pub fn soh_energy(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
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
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct FilterCapacity {
        /// The internal bits
        bits: [u8; 8],
    }
    impl ::device_driver::FieldSet for FilterCapacity {
        const SIZE_BITS: u32 = 64;
        fn new_with_zero() -> Self {
            Self::new_zero()
        }
        fn get_inner_buffer(&self) -> &[u8] {
            &self.bits
        }
        fn get_inner_buffer_mut(&mut self) -> &mut [u8] {
            &mut self.bits
        }
    }
    impl FilterCapacity {
        /// Create a new instance, loaded with the reset value (if any)
        pub const fn new() -> Self {
            Self {
                bits: [0, 0, 0, 0, 0, 0, 0, 0],
            }
        }
        /// Create a new instance, loaded with all zeroes
        pub const fn new_zero() -> Self {
            Self { bits: [0; 8] }
        }
        ///Read the `filt_rem_cap` field of the register.
        ///
        pub fn filt_rem_cap(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 0, 16) };
            raw
        }
        ///Read the `filt_rem_energy` field of the register.
        ///
        pub fn filt_rem_energy(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 16, 32) };
            raw
        }
        ///Read the `filt_full_chg_cap` field of the register.
        ///
        pub fn filt_full_chg_cap(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 32, 48) };
            raw
        }
        ///Read the `filt_full_chg_energy` field of the register.
        ///
        pub fn filt_full_chg_energy(&self) -> u16 {
            let raw = unsafe { ::device_driver::ops::load_lsb0::<u16, ::device_driver::ops::LE>(&self.bits, 48, 64) };
            raw
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
    /// Enum containing all possible field set types
    pub enum FieldSetValue {
        /// The device can be checked for the IC part number.
        MacDeviceTypeFieldsOut(MacDeviceTypeFieldsOut),
        /// The device can be checked for the firmware version of the IC.
        MacFirmwareVersionFieldsOut(MacFirmwareVersionFieldsOut),
        MacHardwareVersionFieldsOut(MacHardwareVersionFieldsOut),
        MacInstructionFlashSignatureFieldsOut(MacInstructionFlashSignatureFieldsOut),
        MacStaticDfSignatureFieldsOut(MacStaticDfSignatureFieldsOut),
        MacChemIdFieldsOut(MacChemIdFieldsOut),
        MacStaticChemDfSigFieldsOut(MacStaticChemDfSigFieldsOut),
        MacAllDfSignatureFieldsOut(MacAllDfSignatureFieldsOut),
        MacSafetyAlertFieldsOut(MacSafetyAlertFieldsOut),
        MacSafetyStatusFieldsOut(MacSafetyStatusFieldsOut),
        MacPfAlertFieldsOut(MacPfAlertFieldsOut),
        MacPfStatusFieldsOut(MacPfStatusFieldsOut),
        MacOperationStatusFieldsOut(MacOperationStatusFieldsOut),
        MacChargingStatusFieldsOut(MacChargingStatusFieldsOut),
        MacGaugingStatusFieldsOut(MacGaugingStatusFieldsOut),
        MacManufacturingStatusFieldsOut(MacManufacturingStatusFieldsOut),
        MacAfeRegFieldsOut(MacAfeRegFieldsOut),
        NoLoadRemCapFieldsOut(NoLoadRemCapFieldsOut),
        MacLifetimeDataBlock1FieldsOut(MacLifetimeDataBlock1FieldsOut),
        MacLifetimeDataBlock2FieldsOut(MacLifetimeDataBlock2FieldsOut),
        MacLifetimeDataBlock3FieldsOut(MacLifetimeDataBlock3FieldsOut),
        MacLifetimeDataBlock4FieldsOut(MacLifetimeDataBlock4FieldsOut),
        MacLifetimeDataBlock5FieldsOut(MacLifetimeDataBlock5FieldsOut),
        MacLifetimeDataBlock6FieldsOut(MacLifetimeDataBlock6FieldsOut),
        MacLifetimeDataBlock7FieldsOut(MacLifetimeDataBlock7FieldsOut),
        MacLifetimeDataBlock8FieldsOut(MacLifetimeDataBlock8FieldsOut),
        MacLifetimeDataBlock9FieldsOut(MacLifetimeDataBlock9FieldsOut),
        MacLifetimeDataBlock10FieldsOut(MacLifetimeDataBlock10FieldsOut),
        MacLifetimeDataBlock11FieldsOut(MacLifetimeDataBlock11FieldsOut),
        MacLifetimeDataBlock12FieldsOut(MacLifetimeDataBlock12FieldsOut),
        MacManufactureInfoFieldsOut(MacManufactureInfoFieldsOut),
        MacDaStatus1FieldsOut(MacDaStatus1FieldsOut),
        MacDaStatus2FieldsOut(MacDaStatus2FieldsOut),
        MacGaugeStatus1FieldsOut(MacGaugeStatus1FieldsOut),
        MacGaugeStatus2FieldsOut(MacGaugeStatus2FieldsOut),
        MacGaugeStatus3FieldsOut(MacGaugeStatus3FieldsOut),
        MacCbStatusFieldsOut(MacCbStatusFieldsOut),
        MacStateOfHealthFieldsOut(MacStateOfHealthFieldsOut),
        MacFilterCapacityFieldsOut(MacFilterCapacityFieldsOut),
        MacManufactureInfoBFieldsOut(MacManufactureInfoBFieldsOut),
        MacOutputCcadcCalFieldsOut(MacOutputCcadcCalFieldsOut),
        MacOutputShortedCcadcCalFieldsOut(MacOutputShortedCcadcCalFieldsOut),
        /// This read/write word function sets a low capacity alarm threshold for the cell stack.
        RemainingCapacityAlarm(RemainingCapacityAlarm),
        /// This read/write word function sets a low remaining time-to-fully discharge alarm threshold for the cell stack.
        RemainingTimeAlarm(RemainingTimeAlarm),
        BatteryMode(BatteryMode),
        AtRate(AtRate),
        AtRateTimeToFull(AtRateTimeToFull),
        AtRateTimeToEmpty(AtRateTimeToEmpty),
        AtRateOk(AtRateOk),
        Temperature(Temperature),
        Voltage(Voltage),
        Current(Current),
        AvgCurrent(AvgCurrent),
        MaxError(MaxError),
        RelativeStateOfCharge(RelativeStateOfCharge),
        AbsoluteStateOfCharge(AbsoluteStateOfCharge),
        RemainingCapacity(RemainingCapacity),
        FullChargeCapacity(FullChargeCapacity),
        RunTimeToEmpty(RunTimeToEmpty),
        AverageTimeToEmpty(AverageTimeToEmpty),
        AverageTimeToFull(AverageTimeToFull),
        ChargingCurrent(ChargingCurrent),
        ChargingVoltage(ChargingVoltage),
        BatteryStatus(BatteryStatus),
        CycleCount(CycleCount),
        DesignCapacity(DesignCapacity),
        DesignVoltage(DesignVoltage),
        SpecificationInfo(SpecificationInfo),
        ManufactureDate(ManufactureDate),
        SerialNumber(SerialNumber),
        CellVoltage4(CellVoltage4),
        CellVoltage3(CellVoltage3),
        CellVoltage2(CellVoltage2),
        CellVoltage1(CellVoltage1),
        BtpDischargeSet(BtpDischargeSet),
        BtpChargeSet(BtpChargeSet),
        StateOfHealthSoh(StateOfHealthSoh),
        SafetyAlert(SafetyAlert),
        SafetyStatus(SafetyStatus),
        PfAlert(PfAlert),
        PfStatus(PfStatus),
        OperationStatus(OperationStatus),
        ChargingStatus(ChargingStatus),
        GaugingStatus(GaugingStatus),
        ManufacturingStatus(ManufacturingStatus),
        AfeReg(AfeReg),
        MaxTurboPower(MaxTurboPower),
        SusTurboPower(SusTurboPower),
        TurboPackR(TurboPackR),
        TurboSysR(TurboSysR),
        TurboEdv(TurboEdv),
        TurboCurrent(TurboCurrent),
        SusTurboCurrent(SusTurboCurrent),
        LifetimeDataBlock1(LifetimeDataBlock1),
        LifetimeDataBlock2(LifetimeDataBlock2),
        LifetimeDataBlock3(LifetimeDataBlock3),
        LifetimeDataBlock4(LifetimeDataBlock4),
        LifetimeDataBlock5(LifetimeDataBlock5),
        LifetimeDataBlock6(LifetimeDataBlock6),
        LifetimeDataBlock7(LifetimeDataBlock7),
        LifetimeDataBlock8(LifetimeDataBlock8),
        TurboRhfEffective(TurboRhfEffective),
        TurboVload(TurboVload),
        LifetimeDataBlock11(LifetimeDataBlock11),
        LifetimeDataBlock12(LifetimeDataBlock12),
        DaStatus1(DaStatus1),
        DaStatus2(DaStatus2),
        GaugeStatus1(GaugeStatus1),
        GaugeStatus2(GaugeStatus2),
        GaugeStatus3(GaugeStatus3),
        CbStatus(CbStatus),
        StateOfHealth(StateOfHealth),
        FilterCapacity(FilterCapacity),
    }
    impl core::fmt::Debug for FieldSetValue {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match self {
                Self::MacDeviceTypeFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacFirmwareVersionFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacHardwareVersionFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacInstructionFlashSignatureFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacStaticDfSignatureFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacChemIdFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacStaticChemDfSigFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacAllDfSignatureFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacSafetyAlertFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacSafetyStatusFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacPfAlertFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacPfStatusFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacOperationStatusFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacChargingStatusFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacGaugingStatusFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacManufacturingStatusFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacAfeRegFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::NoLoadRemCapFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock1FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock2FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock3FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock4FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock5FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock6FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock7FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock8FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock9FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock10FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock11FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacLifetimeDataBlock12FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacManufactureInfoFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacDaStatus1FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacDaStatus2FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacGaugeStatus1FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacGaugeStatus2FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacGaugeStatus3FieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacCbStatusFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacStateOfHealthFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacFilterCapacityFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacManufactureInfoBFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacOutputCcadcCalFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::MacOutputShortedCcadcCalFieldsOut(val) => core::fmt::Debug::fmt(val, f),
                Self::RemainingCapacityAlarm(val) => core::fmt::Debug::fmt(val, f),
                Self::RemainingTimeAlarm(val) => core::fmt::Debug::fmt(val, f),
                Self::BatteryMode(val) => core::fmt::Debug::fmt(val, f),
                Self::AtRate(val) => core::fmt::Debug::fmt(val, f),
                Self::AtRateTimeToFull(val) => core::fmt::Debug::fmt(val, f),
                Self::AtRateTimeToEmpty(val) => core::fmt::Debug::fmt(val, f),
                Self::AtRateOk(val) => core::fmt::Debug::fmt(val, f),
                Self::Temperature(val) => core::fmt::Debug::fmt(val, f),
                Self::Voltage(val) => core::fmt::Debug::fmt(val, f),
                Self::Current(val) => core::fmt::Debug::fmt(val, f),
                Self::AvgCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::MaxError(val) => core::fmt::Debug::fmt(val, f),
                Self::RelativeStateOfCharge(val) => core::fmt::Debug::fmt(val, f),
                Self::AbsoluteStateOfCharge(val) => core::fmt::Debug::fmt(val, f),
                Self::RemainingCapacity(val) => core::fmt::Debug::fmt(val, f),
                Self::FullChargeCapacity(val) => core::fmt::Debug::fmt(val, f),
                Self::RunTimeToEmpty(val) => core::fmt::Debug::fmt(val, f),
                Self::AverageTimeToEmpty(val) => core::fmt::Debug::fmt(val, f),
                Self::AverageTimeToFull(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargingCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargingVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::BatteryStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::CycleCount(val) => core::fmt::Debug::fmt(val, f),
                Self::DesignCapacity(val) => core::fmt::Debug::fmt(val, f),
                Self::DesignVoltage(val) => core::fmt::Debug::fmt(val, f),
                Self::SpecificationInfo(val) => core::fmt::Debug::fmt(val, f),
                Self::ManufactureDate(val) => core::fmt::Debug::fmt(val, f),
                Self::SerialNumber(val) => core::fmt::Debug::fmt(val, f),
                Self::CellVoltage4(val) => core::fmt::Debug::fmt(val, f),
                Self::CellVoltage3(val) => core::fmt::Debug::fmt(val, f),
                Self::CellVoltage2(val) => core::fmt::Debug::fmt(val, f),
                Self::CellVoltage1(val) => core::fmt::Debug::fmt(val, f),
                Self::BtpDischargeSet(val) => core::fmt::Debug::fmt(val, f),
                Self::BtpChargeSet(val) => core::fmt::Debug::fmt(val, f),
                Self::StateOfHealthSoh(val) => core::fmt::Debug::fmt(val, f),
                Self::SafetyAlert(val) => core::fmt::Debug::fmt(val, f),
                Self::SafetyStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::PfAlert(val) => core::fmt::Debug::fmt(val, f),
                Self::PfStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::OperationStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::ChargingStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::GaugingStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::ManufacturingStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::AfeReg(val) => core::fmt::Debug::fmt(val, f),
                Self::MaxTurboPower(val) => core::fmt::Debug::fmt(val, f),
                Self::SusTurboPower(val) => core::fmt::Debug::fmt(val, f),
                Self::TurboPackR(val) => core::fmt::Debug::fmt(val, f),
                Self::TurboSysR(val) => core::fmt::Debug::fmt(val, f),
                Self::TurboEdv(val) => core::fmt::Debug::fmt(val, f),
                Self::TurboCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::SusTurboCurrent(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock1(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock2(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock3(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock4(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock5(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock6(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock7(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock8(val) => core::fmt::Debug::fmt(val, f),
                Self::TurboRhfEffective(val) => core::fmt::Debug::fmt(val, f),
                Self::TurboVload(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock11(val) => core::fmt::Debug::fmt(val, f),
                Self::LifetimeDataBlock12(val) => core::fmt::Debug::fmt(val, f),
                Self::DaStatus1(val) => core::fmt::Debug::fmt(val, f),
                Self::DaStatus2(val) => core::fmt::Debug::fmt(val, f),
                Self::GaugeStatus1(val) => core::fmt::Debug::fmt(val, f),
                Self::GaugeStatus2(val) => core::fmt::Debug::fmt(val, f),
                Self::GaugeStatus3(val) => core::fmt::Debug::fmt(val, f),
                Self::CbStatus(val) => core::fmt::Debug::fmt(val, f),
                Self::StateOfHealth(val) => core::fmt::Debug::fmt(val, f),
                Self::FilterCapacity(val) => core::fmt::Debug::fmt(val, f),
                #[allow(unreachable_patterns)]
                _ => unreachable!(),
            }
        }
    }
    #[cfg(feature = "defmt-03")]
    impl defmt::Format for FieldSetValue {
        fn format(&self, f: defmt::Formatter) {
            match self {
                Self::MacDeviceTypeFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacFirmwareVersionFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacHardwareVersionFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacInstructionFlashSignatureFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacStaticDfSignatureFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacChemIdFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacStaticChemDfSigFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacAllDfSignatureFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacSafetyAlertFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacSafetyStatusFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacPfAlertFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacPfStatusFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacOperationStatusFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacChargingStatusFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacGaugingStatusFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacManufacturingStatusFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacAfeRegFieldsOut(val) => defmt::Format::format(val, f),
                Self::NoLoadRemCapFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock1FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock2FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock3FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock4FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock5FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock6FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock7FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock8FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock9FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock10FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock11FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacLifetimeDataBlock12FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacManufactureInfoFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacDaStatus1FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacDaStatus2FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacGaugeStatus1FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacGaugeStatus2FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacGaugeStatus3FieldsOut(val) => defmt::Format::format(val, f),
                Self::MacCbStatusFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacStateOfHealthFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacFilterCapacityFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacManufactureInfoBFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacOutputCcadcCalFieldsOut(val) => defmt::Format::format(val, f),
                Self::MacOutputShortedCcadcCalFieldsOut(val) => defmt::Format::format(val, f),
                Self::RemainingCapacityAlarm(val) => defmt::Format::format(val, f),
                Self::RemainingTimeAlarm(val) => defmt::Format::format(val, f),
                Self::BatteryMode(val) => defmt::Format::format(val, f),
                Self::AtRate(val) => defmt::Format::format(val, f),
                Self::AtRateTimeToFull(val) => defmt::Format::format(val, f),
                Self::AtRateTimeToEmpty(val) => defmt::Format::format(val, f),
                Self::AtRateOk(val) => defmt::Format::format(val, f),
                Self::Temperature(val) => defmt::Format::format(val, f),
                Self::Voltage(val) => defmt::Format::format(val, f),
                Self::Current(val) => defmt::Format::format(val, f),
                Self::AvgCurrent(val) => defmt::Format::format(val, f),
                Self::MaxError(val) => defmt::Format::format(val, f),
                Self::RelativeStateOfCharge(val) => defmt::Format::format(val, f),
                Self::AbsoluteStateOfCharge(val) => defmt::Format::format(val, f),
                Self::RemainingCapacity(val) => defmt::Format::format(val, f),
                Self::FullChargeCapacity(val) => defmt::Format::format(val, f),
                Self::RunTimeToEmpty(val) => defmt::Format::format(val, f),
                Self::AverageTimeToEmpty(val) => defmt::Format::format(val, f),
                Self::AverageTimeToFull(val) => defmt::Format::format(val, f),
                Self::ChargingCurrent(val) => defmt::Format::format(val, f),
                Self::ChargingVoltage(val) => defmt::Format::format(val, f),
                Self::BatteryStatus(val) => defmt::Format::format(val, f),
                Self::CycleCount(val) => defmt::Format::format(val, f),
                Self::DesignCapacity(val) => defmt::Format::format(val, f),
                Self::DesignVoltage(val) => defmt::Format::format(val, f),
                Self::SpecificationInfo(val) => defmt::Format::format(val, f),
                Self::ManufactureDate(val) => defmt::Format::format(val, f),
                Self::SerialNumber(val) => defmt::Format::format(val, f),
                Self::CellVoltage4(val) => defmt::Format::format(val, f),
                Self::CellVoltage3(val) => defmt::Format::format(val, f),
                Self::CellVoltage2(val) => defmt::Format::format(val, f),
                Self::CellVoltage1(val) => defmt::Format::format(val, f),
                Self::BtpDischargeSet(val) => defmt::Format::format(val, f),
                Self::BtpChargeSet(val) => defmt::Format::format(val, f),
                Self::StateOfHealthSoh(val) => defmt::Format::format(val, f),
                Self::SafetyAlert(val) => defmt::Format::format(val, f),
                Self::SafetyStatus(val) => defmt::Format::format(val, f),
                Self::PfAlert(val) => defmt::Format::format(val, f),
                Self::PfStatus(val) => defmt::Format::format(val, f),
                Self::OperationStatus(val) => defmt::Format::format(val, f),
                Self::ChargingStatus(val) => defmt::Format::format(val, f),
                Self::GaugingStatus(val) => defmt::Format::format(val, f),
                Self::ManufacturingStatus(val) => defmt::Format::format(val, f),
                Self::AfeReg(val) => defmt::Format::format(val, f),
                Self::MaxTurboPower(val) => defmt::Format::format(val, f),
                Self::SusTurboPower(val) => defmt::Format::format(val, f),
                Self::TurboPackR(val) => defmt::Format::format(val, f),
                Self::TurboSysR(val) => defmt::Format::format(val, f),
                Self::TurboEdv(val) => defmt::Format::format(val, f),
                Self::TurboCurrent(val) => defmt::Format::format(val, f),
                Self::SusTurboCurrent(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock1(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock2(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock3(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock4(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock5(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock6(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock7(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock8(val) => defmt::Format::format(val, f),
                Self::TurboRhfEffective(val) => defmt::Format::format(val, f),
                Self::TurboVload(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock11(val) => defmt::Format::format(val, f),
                Self::LifetimeDataBlock12(val) => defmt::Format::format(val, f),
                Self::DaStatus1(val) => defmt::Format::format(val, f),
                Self::DaStatus2(val) => defmt::Format::format(val, f),
                Self::GaugeStatus1(val) => defmt::Format::format(val, f),
                Self::GaugeStatus2(val) => defmt::Format::format(val, f),
                Self::GaugeStatus3(val) => defmt::Format::format(val, f),
                Self::CbStatus(val) => defmt::Format::format(val, f),
                Self::StateOfHealth(val) => defmt::Format::format(val, f),
                Self::FilterCapacity(val) => defmt::Format::format(val, f),
            }
        }
    }
    impl From<MacDeviceTypeFieldsOut> for FieldSetValue {
        fn from(val: MacDeviceTypeFieldsOut) -> Self {
            Self::MacDeviceTypeFieldsOut(val)
        }
    }
    impl From<MacFirmwareVersionFieldsOut> for FieldSetValue {
        fn from(val: MacFirmwareVersionFieldsOut) -> Self {
            Self::MacFirmwareVersionFieldsOut(val)
        }
    }
    impl From<MacHardwareVersionFieldsOut> for FieldSetValue {
        fn from(val: MacHardwareVersionFieldsOut) -> Self {
            Self::MacHardwareVersionFieldsOut(val)
        }
    }
    impl From<MacInstructionFlashSignatureFieldsOut> for FieldSetValue {
        fn from(val: MacInstructionFlashSignatureFieldsOut) -> Self {
            Self::MacInstructionFlashSignatureFieldsOut(val)
        }
    }
    impl From<MacStaticDfSignatureFieldsOut> for FieldSetValue {
        fn from(val: MacStaticDfSignatureFieldsOut) -> Self {
            Self::MacStaticDfSignatureFieldsOut(val)
        }
    }
    impl From<MacChemIdFieldsOut> for FieldSetValue {
        fn from(val: MacChemIdFieldsOut) -> Self {
            Self::MacChemIdFieldsOut(val)
        }
    }
    impl From<MacStaticChemDfSigFieldsOut> for FieldSetValue {
        fn from(val: MacStaticChemDfSigFieldsOut) -> Self {
            Self::MacStaticChemDfSigFieldsOut(val)
        }
    }
    impl From<MacAllDfSignatureFieldsOut> for FieldSetValue {
        fn from(val: MacAllDfSignatureFieldsOut) -> Self {
            Self::MacAllDfSignatureFieldsOut(val)
        }
    }
    impl From<MacSafetyAlertFieldsOut> for FieldSetValue {
        fn from(val: MacSafetyAlertFieldsOut) -> Self {
            Self::MacSafetyAlertFieldsOut(val)
        }
    }
    impl From<MacSafetyStatusFieldsOut> for FieldSetValue {
        fn from(val: MacSafetyStatusFieldsOut) -> Self {
            Self::MacSafetyStatusFieldsOut(val)
        }
    }
    impl From<MacPfAlertFieldsOut> for FieldSetValue {
        fn from(val: MacPfAlertFieldsOut) -> Self {
            Self::MacPfAlertFieldsOut(val)
        }
    }
    impl From<MacPfStatusFieldsOut> for FieldSetValue {
        fn from(val: MacPfStatusFieldsOut) -> Self {
            Self::MacPfStatusFieldsOut(val)
        }
    }
    impl From<MacOperationStatusFieldsOut> for FieldSetValue {
        fn from(val: MacOperationStatusFieldsOut) -> Self {
            Self::MacOperationStatusFieldsOut(val)
        }
    }
    impl From<MacChargingStatusFieldsOut> for FieldSetValue {
        fn from(val: MacChargingStatusFieldsOut) -> Self {
            Self::MacChargingStatusFieldsOut(val)
        }
    }
    impl From<MacGaugingStatusFieldsOut> for FieldSetValue {
        fn from(val: MacGaugingStatusFieldsOut) -> Self {
            Self::MacGaugingStatusFieldsOut(val)
        }
    }
    impl From<MacManufacturingStatusFieldsOut> for FieldSetValue {
        fn from(val: MacManufacturingStatusFieldsOut) -> Self {
            Self::MacManufacturingStatusFieldsOut(val)
        }
    }
    impl From<MacAfeRegFieldsOut> for FieldSetValue {
        fn from(val: MacAfeRegFieldsOut) -> Self {
            Self::MacAfeRegFieldsOut(val)
        }
    }
    impl From<NoLoadRemCapFieldsOut> for FieldSetValue {
        fn from(val: NoLoadRemCapFieldsOut) -> Self {
            Self::NoLoadRemCapFieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock1FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock1FieldsOut) -> Self {
            Self::MacLifetimeDataBlock1FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock2FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock2FieldsOut) -> Self {
            Self::MacLifetimeDataBlock2FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock3FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock3FieldsOut) -> Self {
            Self::MacLifetimeDataBlock3FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock4FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock4FieldsOut) -> Self {
            Self::MacLifetimeDataBlock4FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock5FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock5FieldsOut) -> Self {
            Self::MacLifetimeDataBlock5FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock6FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock6FieldsOut) -> Self {
            Self::MacLifetimeDataBlock6FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock7FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock7FieldsOut) -> Self {
            Self::MacLifetimeDataBlock7FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock8FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock8FieldsOut) -> Self {
            Self::MacLifetimeDataBlock8FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock9FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock9FieldsOut) -> Self {
            Self::MacLifetimeDataBlock9FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock10FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock10FieldsOut) -> Self {
            Self::MacLifetimeDataBlock10FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock11FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock11FieldsOut) -> Self {
            Self::MacLifetimeDataBlock11FieldsOut(val)
        }
    }
    impl From<MacLifetimeDataBlock12FieldsOut> for FieldSetValue {
        fn from(val: MacLifetimeDataBlock12FieldsOut) -> Self {
            Self::MacLifetimeDataBlock12FieldsOut(val)
        }
    }
    impl From<MacManufactureInfoFieldsOut> for FieldSetValue {
        fn from(val: MacManufactureInfoFieldsOut) -> Self {
            Self::MacManufactureInfoFieldsOut(val)
        }
    }
    impl From<MacDaStatus1FieldsOut> for FieldSetValue {
        fn from(val: MacDaStatus1FieldsOut) -> Self {
            Self::MacDaStatus1FieldsOut(val)
        }
    }
    impl From<MacDaStatus2FieldsOut> for FieldSetValue {
        fn from(val: MacDaStatus2FieldsOut) -> Self {
            Self::MacDaStatus2FieldsOut(val)
        }
    }
    impl From<MacGaugeStatus1FieldsOut> for FieldSetValue {
        fn from(val: MacGaugeStatus1FieldsOut) -> Self {
            Self::MacGaugeStatus1FieldsOut(val)
        }
    }
    impl From<MacGaugeStatus2FieldsOut> for FieldSetValue {
        fn from(val: MacGaugeStatus2FieldsOut) -> Self {
            Self::MacGaugeStatus2FieldsOut(val)
        }
    }
    impl From<MacGaugeStatus3FieldsOut> for FieldSetValue {
        fn from(val: MacGaugeStatus3FieldsOut) -> Self {
            Self::MacGaugeStatus3FieldsOut(val)
        }
    }
    impl From<MacCbStatusFieldsOut> for FieldSetValue {
        fn from(val: MacCbStatusFieldsOut) -> Self {
            Self::MacCbStatusFieldsOut(val)
        }
    }
    impl From<MacStateOfHealthFieldsOut> for FieldSetValue {
        fn from(val: MacStateOfHealthFieldsOut) -> Self {
            Self::MacStateOfHealthFieldsOut(val)
        }
    }
    impl From<MacFilterCapacityFieldsOut> for FieldSetValue {
        fn from(val: MacFilterCapacityFieldsOut) -> Self {
            Self::MacFilterCapacityFieldsOut(val)
        }
    }
    impl From<MacManufactureInfoBFieldsOut> for FieldSetValue {
        fn from(val: MacManufactureInfoBFieldsOut) -> Self {
            Self::MacManufactureInfoBFieldsOut(val)
        }
    }
    impl From<MacOutputCcadcCalFieldsOut> for FieldSetValue {
        fn from(val: MacOutputCcadcCalFieldsOut) -> Self {
            Self::MacOutputCcadcCalFieldsOut(val)
        }
    }
    impl From<MacOutputShortedCcadcCalFieldsOut> for FieldSetValue {
        fn from(val: MacOutputShortedCcadcCalFieldsOut) -> Self {
            Self::MacOutputShortedCcadcCalFieldsOut(val)
        }
    }
    impl From<RemainingCapacityAlarm> for FieldSetValue {
        fn from(val: RemainingCapacityAlarm) -> Self {
            Self::RemainingCapacityAlarm(val)
        }
    }
    impl From<RemainingTimeAlarm> for FieldSetValue {
        fn from(val: RemainingTimeAlarm) -> Self {
            Self::RemainingTimeAlarm(val)
        }
    }
    impl From<BatteryMode> for FieldSetValue {
        fn from(val: BatteryMode) -> Self {
            Self::BatteryMode(val)
        }
    }
    impl From<AtRate> for FieldSetValue {
        fn from(val: AtRate) -> Self {
            Self::AtRate(val)
        }
    }
    impl From<AtRateTimeToFull> for FieldSetValue {
        fn from(val: AtRateTimeToFull) -> Self {
            Self::AtRateTimeToFull(val)
        }
    }
    impl From<AtRateTimeToEmpty> for FieldSetValue {
        fn from(val: AtRateTimeToEmpty) -> Self {
            Self::AtRateTimeToEmpty(val)
        }
    }
    impl From<AtRateOk> for FieldSetValue {
        fn from(val: AtRateOk) -> Self {
            Self::AtRateOk(val)
        }
    }
    impl From<Temperature> for FieldSetValue {
        fn from(val: Temperature) -> Self {
            Self::Temperature(val)
        }
    }
    impl From<Voltage> for FieldSetValue {
        fn from(val: Voltage) -> Self {
            Self::Voltage(val)
        }
    }
    impl From<Current> for FieldSetValue {
        fn from(val: Current) -> Self {
            Self::Current(val)
        }
    }
    impl From<AvgCurrent> for FieldSetValue {
        fn from(val: AvgCurrent) -> Self {
            Self::AvgCurrent(val)
        }
    }
    impl From<MaxError> for FieldSetValue {
        fn from(val: MaxError) -> Self {
            Self::MaxError(val)
        }
    }
    impl From<RelativeStateOfCharge> for FieldSetValue {
        fn from(val: RelativeStateOfCharge) -> Self {
            Self::RelativeStateOfCharge(val)
        }
    }
    impl From<AbsoluteStateOfCharge> for FieldSetValue {
        fn from(val: AbsoluteStateOfCharge) -> Self {
            Self::AbsoluteStateOfCharge(val)
        }
    }
    impl From<RemainingCapacity> for FieldSetValue {
        fn from(val: RemainingCapacity) -> Self {
            Self::RemainingCapacity(val)
        }
    }
    impl From<FullChargeCapacity> for FieldSetValue {
        fn from(val: FullChargeCapacity) -> Self {
            Self::FullChargeCapacity(val)
        }
    }
    impl From<RunTimeToEmpty> for FieldSetValue {
        fn from(val: RunTimeToEmpty) -> Self {
            Self::RunTimeToEmpty(val)
        }
    }
    impl From<AverageTimeToEmpty> for FieldSetValue {
        fn from(val: AverageTimeToEmpty) -> Self {
            Self::AverageTimeToEmpty(val)
        }
    }
    impl From<AverageTimeToFull> for FieldSetValue {
        fn from(val: AverageTimeToFull) -> Self {
            Self::AverageTimeToFull(val)
        }
    }
    impl From<ChargingCurrent> for FieldSetValue {
        fn from(val: ChargingCurrent) -> Self {
            Self::ChargingCurrent(val)
        }
    }
    impl From<ChargingVoltage> for FieldSetValue {
        fn from(val: ChargingVoltage) -> Self {
            Self::ChargingVoltage(val)
        }
    }
    impl From<BatteryStatus> for FieldSetValue {
        fn from(val: BatteryStatus) -> Self {
            Self::BatteryStatus(val)
        }
    }
    impl From<CycleCount> for FieldSetValue {
        fn from(val: CycleCount) -> Self {
            Self::CycleCount(val)
        }
    }
    impl From<DesignCapacity> for FieldSetValue {
        fn from(val: DesignCapacity) -> Self {
            Self::DesignCapacity(val)
        }
    }
    impl From<DesignVoltage> for FieldSetValue {
        fn from(val: DesignVoltage) -> Self {
            Self::DesignVoltage(val)
        }
    }
    impl From<SpecificationInfo> for FieldSetValue {
        fn from(val: SpecificationInfo) -> Self {
            Self::SpecificationInfo(val)
        }
    }
    impl From<ManufactureDate> for FieldSetValue {
        fn from(val: ManufactureDate) -> Self {
            Self::ManufactureDate(val)
        }
    }
    impl From<SerialNumber> for FieldSetValue {
        fn from(val: SerialNumber) -> Self {
            Self::SerialNumber(val)
        }
    }
    impl From<CellVoltage4> for FieldSetValue {
        fn from(val: CellVoltage4) -> Self {
            Self::CellVoltage4(val)
        }
    }
    impl From<CellVoltage3> for FieldSetValue {
        fn from(val: CellVoltage3) -> Self {
            Self::CellVoltage3(val)
        }
    }
    impl From<CellVoltage2> for FieldSetValue {
        fn from(val: CellVoltage2) -> Self {
            Self::CellVoltage2(val)
        }
    }
    impl From<CellVoltage1> for FieldSetValue {
        fn from(val: CellVoltage1) -> Self {
            Self::CellVoltage1(val)
        }
    }
    impl From<BtpDischargeSet> for FieldSetValue {
        fn from(val: BtpDischargeSet) -> Self {
            Self::BtpDischargeSet(val)
        }
    }
    impl From<BtpChargeSet> for FieldSetValue {
        fn from(val: BtpChargeSet) -> Self {
            Self::BtpChargeSet(val)
        }
    }
    impl From<StateOfHealthSoh> for FieldSetValue {
        fn from(val: StateOfHealthSoh) -> Self {
            Self::StateOfHealthSoh(val)
        }
    }
    impl From<SafetyAlert> for FieldSetValue {
        fn from(val: SafetyAlert) -> Self {
            Self::SafetyAlert(val)
        }
    }
    impl From<SafetyStatus> for FieldSetValue {
        fn from(val: SafetyStatus) -> Self {
            Self::SafetyStatus(val)
        }
    }
    impl From<PfAlert> for FieldSetValue {
        fn from(val: PfAlert) -> Self {
            Self::PfAlert(val)
        }
    }
    impl From<PfStatus> for FieldSetValue {
        fn from(val: PfStatus) -> Self {
            Self::PfStatus(val)
        }
    }
    impl From<OperationStatus> for FieldSetValue {
        fn from(val: OperationStatus) -> Self {
            Self::OperationStatus(val)
        }
    }
    impl From<ChargingStatus> for FieldSetValue {
        fn from(val: ChargingStatus) -> Self {
            Self::ChargingStatus(val)
        }
    }
    impl From<GaugingStatus> for FieldSetValue {
        fn from(val: GaugingStatus) -> Self {
            Self::GaugingStatus(val)
        }
    }
    impl From<ManufacturingStatus> for FieldSetValue {
        fn from(val: ManufacturingStatus) -> Self {
            Self::ManufacturingStatus(val)
        }
    }
    impl From<AfeReg> for FieldSetValue {
        fn from(val: AfeReg) -> Self {
            Self::AfeReg(val)
        }
    }
    impl From<MaxTurboPower> for FieldSetValue {
        fn from(val: MaxTurboPower) -> Self {
            Self::MaxTurboPower(val)
        }
    }
    impl From<SusTurboPower> for FieldSetValue {
        fn from(val: SusTurboPower) -> Self {
            Self::SusTurboPower(val)
        }
    }
    impl From<TurboPackR> for FieldSetValue {
        fn from(val: TurboPackR) -> Self {
            Self::TurboPackR(val)
        }
    }
    impl From<TurboSysR> for FieldSetValue {
        fn from(val: TurboSysR) -> Self {
            Self::TurboSysR(val)
        }
    }
    impl From<TurboEdv> for FieldSetValue {
        fn from(val: TurboEdv) -> Self {
            Self::TurboEdv(val)
        }
    }
    impl From<TurboCurrent> for FieldSetValue {
        fn from(val: TurboCurrent) -> Self {
            Self::TurboCurrent(val)
        }
    }
    impl From<SusTurboCurrent> for FieldSetValue {
        fn from(val: SusTurboCurrent) -> Self {
            Self::SusTurboCurrent(val)
        }
    }
    impl From<LifetimeDataBlock1> for FieldSetValue {
        fn from(val: LifetimeDataBlock1) -> Self {
            Self::LifetimeDataBlock1(val)
        }
    }
    impl From<LifetimeDataBlock2> for FieldSetValue {
        fn from(val: LifetimeDataBlock2) -> Self {
            Self::LifetimeDataBlock2(val)
        }
    }
    impl From<LifetimeDataBlock3> for FieldSetValue {
        fn from(val: LifetimeDataBlock3) -> Self {
            Self::LifetimeDataBlock3(val)
        }
    }
    impl From<LifetimeDataBlock4> for FieldSetValue {
        fn from(val: LifetimeDataBlock4) -> Self {
            Self::LifetimeDataBlock4(val)
        }
    }
    impl From<LifetimeDataBlock5> for FieldSetValue {
        fn from(val: LifetimeDataBlock5) -> Self {
            Self::LifetimeDataBlock5(val)
        }
    }
    impl From<LifetimeDataBlock6> for FieldSetValue {
        fn from(val: LifetimeDataBlock6) -> Self {
            Self::LifetimeDataBlock6(val)
        }
    }
    impl From<LifetimeDataBlock7> for FieldSetValue {
        fn from(val: LifetimeDataBlock7) -> Self {
            Self::LifetimeDataBlock7(val)
        }
    }
    impl From<LifetimeDataBlock8> for FieldSetValue {
        fn from(val: LifetimeDataBlock8) -> Self {
            Self::LifetimeDataBlock8(val)
        }
    }
    impl From<TurboRhfEffective> for FieldSetValue {
        fn from(val: TurboRhfEffective) -> Self {
            Self::TurboRhfEffective(val)
        }
    }
    impl From<TurboVload> for FieldSetValue {
        fn from(val: TurboVload) -> Self {
            Self::TurboVload(val)
        }
    }
    impl From<LifetimeDataBlock11> for FieldSetValue {
        fn from(val: LifetimeDataBlock11) -> Self {
            Self::LifetimeDataBlock11(val)
        }
    }
    impl From<LifetimeDataBlock12> for FieldSetValue {
        fn from(val: LifetimeDataBlock12) -> Self {
            Self::LifetimeDataBlock12(val)
        }
    }
    impl From<DaStatus1> for FieldSetValue {
        fn from(val: DaStatus1) -> Self {
            Self::DaStatus1(val)
        }
    }
    impl From<DaStatus2> for FieldSetValue {
        fn from(val: DaStatus2) -> Self {
            Self::DaStatus2(val)
        }
    }
    impl From<GaugeStatus1> for FieldSetValue {
        fn from(val: GaugeStatus1) -> Self {
            Self::GaugeStatus1(val)
        }
    }
    impl From<GaugeStatus2> for FieldSetValue {
        fn from(val: GaugeStatus2) -> Self {
            Self::GaugeStatus2(val)
        }
    }
    impl From<GaugeStatus3> for FieldSetValue {
        fn from(val: GaugeStatus3) -> Self {
            Self::GaugeStatus3(val)
        }
    }
    impl From<CbStatus> for FieldSetValue {
        fn from(val: CbStatus) -> Self {
            Self::CbStatus(val)
        }
    }
    impl From<StateOfHealth> for FieldSetValue {
        fn from(val: StateOfHealth) -> Self {
            Self::StateOfHealth(val)
        }
    }
    impl From<FilterCapacity> for FieldSetValue {
        fn from(val: FilterCapacity) -> Self {
            Self::FilterCapacity(val)
        }
    }
}
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum MacqMaxStatus {
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
#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[cfg_attr(feature = "defmt-03", derive(defmt::Format))]
pub enum QMaxStatus {
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
