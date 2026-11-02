use std::collections::HashSet;

use crate::model::*;

pub fn validate(project: &A2lProject) -> ValidationResult {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();

    if project.project_name.trim().is_empty() {
        errors.push("项目名不能为空。".to_string());
    }
    if project.module_name.trim().is_empty() {
        errors.push("模块名不能为空。".to_string());
    }
    if project.xcp.cro_can_id == 0 {
        errors.push("CRO / Master CAN ID 不能为空。".to_string());
    }
    if project.xcp.dto_can_id == 0 {
        errors.push("DTO / Slave CAN ID 不能为空。".to_string());
    }

    let enabled_measurements: Vec<&Measurement> = project.measurements.iter().filter(|m| m.is_enabled).collect();
    let enabled_characteristics: Vec<&Characteristic> = project.characteristics.iter().filter(|c| c.is_enabled).collect();
    if enabled_measurements.is_empty() && enabled_characteristics.is_empty() {
        errors.push("至少需要一个 A2L 对象。".to_string());
    }

    let compu_names: HashSet<String> = project
        .compu_methods
        .iter()
        .map(|compu| compu.name.to_lowercase())
        .collect();

    for measurement in &enabled_measurements {
        validate_measurement(measurement, &compu_names, &project.target_platform, &mut errors, &mut warnings);
    }

    for characteristic in &enabled_characteristics {
        validate_characteristic(characteristic, &compu_names, &mut errors, &mut warnings);
    }

    if project.xcp.address_mode == AddressMode::Raw {
        warnings.push("请确认ELF文件地址类型：ELF文件地址是按字节编址还是按字编址。".to_string());
    }

    for daq_list in &project.daq_lists {
        validate_daq_list(daq_list, &mut errors);
    }

    for segment in &project.memory_segments {
        validate_memory_segment(segment, &mut errors, &mut warnings);
    }

    if project.event_channels.len() as u32 > project.daq.max_event_channel {
        warnings.push(format!(
            "EVENT 事件数量 ({}) 超过 MAX_EVENT_CHANNEL ({})，请调整。",
            project.event_channels.len(),
            project.daq.max_event_channel
        ));
    }

    ValidationResult::new(errors, warnings)
}

fn validate_measurement(
    m: &Measurement,
    compu_names: &HashSet<String>,
    target_platform: &str,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    if m.name.trim().is_empty() {
        errors.push("测量量名称不能为空。".to_string());
    }
    if m.conversion.trim().is_empty() || !compu_names.contains(&m.conversion.to_lowercase()) {
        errors.push(format!("测量量 {} 转换公式不存在: {}", m.name, m.conversion));
    }
    if m.phys_unit.trim().is_empty() {
        warnings.push(format!("测量量 {} 单位为空。", m.name));
    }
    if m.long_identifier.trim().is_empty() {
        warnings.push(format!("测量量 {} 描述为空。", m.name));
    }
    if m.ecu_address == 0 {
        warnings.push(format!("测量量 {} 地址为空（可选，但建议填写）。", m.name));
    }
    if target_platform.to_uppercase().contains("C2000")
        && matches!(m.data_type, A2lDataType::UBYTE | A2lDataType::SBYTE)
    {
        warnings.push(format!(
            "测量量 {} 在 C2000 平台使用了 {:?}，请确认地址粒度和数据宽度。",
            m.name, m.data_type
        ));
    }
}

fn validate_characteristic(
    c: &Characteristic,
    compu_names: &HashSet<String>,
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    if c.name.trim().is_empty() {
        errors.push("标定量名称不能为空。".to_string());
    }
    if c.ecu_address == 0 {
        errors.push(format!("标定量 {} 地址不能为空。", c.name));
    }
    if c.deposit.trim().is_empty() {
        errors.push(format!("标定量 {} Deposit (RECORD_LAYOUT 引用) 不能为空。", c.name));
    }
    if c.conversion.trim().is_empty() || !compu_names.contains(&c.conversion.to_lowercase()) {
        errors.push(format!("标定量 {} 转换公式不存在: {}", c.name, c.conversion));
    }
    if c.phys_unit.trim().is_empty() {
        warnings.push(format!("标定量 {} 单位为空。", c.name));
    }
    if c.long_identifier.trim().is_empty() {
        warnings.push(format!("标定量 {} 描述为空。", c.name));
    }
}

fn validate_daq_list(daq_list: &DaqListConfig, errors: &mut Vec<String>) {
    if daq_list.odts.len() as u32 > daq_list.max_odt as u32 {
        errors.push(format!(
            "DAQ_LIST #{} ODT 数量 ({}) 超过 MAX_ODT ({})。",
            daq_list.number,
            daq_list.odts.len(),
            daq_list.max_odt
        ));
    }

    for odt in &daq_list.odts {
        if odt.entries.len() as u32 > daq_list.max_odt_entries as u32 {
            errors.push(format!(
                "DAQ_LIST #{} ODT #{} 条目数 ({}) 超过 MAX_ODT_ENTRIES ({})。",
                daq_list.number,
                odt.number,
                odt.entries.len(),
                daq_list.max_odt_entries
            ));
        }
    }
}

fn validate_memory_segment(segment: &MemorySegment, errors: &mut Vec<String>, warnings: &mut Vec<String>) {
    if segment.name.trim().is_empty() {
        errors.push("内存段名称不能为空。".to_string());
    }
    if segment.address == 0 {
        errors.push(format!("内存段 {} 起始地址不能为空。", segment.name));
    }
    if segment.size == 0 {
        errors.push(format!("内存段 {} 大小不能为空。", segment.name));
    }
    if segment.long_identifier.trim().is_empty() {
        warnings.push(format!("内存段 {} 描述为空。", segment.name));
    }
}
