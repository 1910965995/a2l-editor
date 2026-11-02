use crate::model::*;

pub fn write(project: &A2lProject) -> String {
    let mut out = String::new();
    line(&mut out, 0, "ASAP2_VERSION 1 71");
    line(
        &mut out,
        0,
        &format!("/begin PROJECT {} {}", project.project_name, quote(&project.project_name)),
    );
    line(
        &mut out,
        2,
        &format!("/begin MODULE {} {}", project.module_name, quote(&project.module_name)),
    );

    write_mod_common(&mut out, &project.xcp);
    write_xcp(&mut out, project);
    write_record_layouts(&mut out);

    for compu in project.compu_methods.iter().filter(|c| !c.name.trim().is_empty()) {
        write_compu_method(&mut out, compu);
    }
    for measurement in project.measurements.iter().filter(|m| m.is_enabled) {
        write_measurement(&mut out, measurement);
    }
    for characteristic in project.characteristics.iter().filter(|c| c.is_enabled) {
        write_characteristic(&mut out, characteristic);
    }
    for segment in project.memory_segments.iter().filter(|s| !s.name.trim().is_empty()) {
        write_memory_segment(&mut out, segment);
    }

    line(&mut out, 2, "/end MODULE");
    line(&mut out, 0, "/end PROJECT");
    out
}

fn write_mod_common(out: &mut String, xcp: &XcpOptions) {
    line(out, 4, "/begin MOD_COMMON \"\"");
    line(out, 6, &format!("BYTE_ORDER {}", xcp.byte_order));
    line(out, 6, "ALIGNMENT_BYTE 1");
    line(out, 6, "ALIGNMENT_WORD 1");
    line(out, 6, "ALIGNMENT_LONG 1");
    line(out, 6, "ALIGNMENT_FLOAT32_IEEE 1");
    line(out, 4, "/end MOD_COMMON");
}

fn write_xcp(out: &mut String, project: &A2lProject) {
    let xcp = &project.xcp;
    line(out, 4, "/begin IF_DATA XCP");
    write_protocol_layer(out, xcp);
    write_daq(out, project);
    line(out, 6, "/begin XCP_ON_CAN");
    line(out, 8, &hex_u16(xcp.can_version));
    line(out, 8, &format!("CAN_ID_MASTER {}", hex_u32(xcp.cro_can_id)));
    line(out, 8, &format!("CAN_ID_SLAVE {}", hex_u32(xcp.dto_can_id)));
    line(out, 8, &format!("BAUDRATE {}", xcp.can_baud_rate));
    line(out, 6, "/end XCP_ON_CAN");
    line(out, 4, "/end IF_DATA");
}

fn write_protocol_layer(out: &mut String, xcp: &XcpOptions) {
    line(out, 6, "/begin PROTOCOL_LAYER");
    line(out, 8, &hex_u16(xcp.protocol_layer_version));
    line(out, 8, &hex_u32(xcp.t1));
    line(out, 8, &hex_u32(xcp.t2));
    line(out, 8, &hex_u32(xcp.t3));
    line(out, 8, &hex_u32(xcp.t4));
    line(out, 8, &hex_u32(xcp.t5));
    line(out, 8, &hex_u32(xcp.t6));
    line(out, 8, &hex_u32(xcp.t7));
    line(out, 8, &hex_u32(xcp.max_cto as u32));
    line(out, 8, &hex_u32(xcp.max_dto as u32));
    line(out, 8, &format!("BYTE_ORDER_{}", xcp.byte_order));
    let granularity = match xcp.address_granularity {
        2 => "ADDRESS_GRANULARITY_WORD",
        4 => "ADDRESS_GRANULARITY_DWORD",
        _ => "ADDRESS_GRANULARITY_BYTE",
    };
    line(out, 8, granularity);
    line(out, 6, "/end PROTOCOL_LAYER");
}

fn write_daq(out: &mut String, project: &A2lProject) {
    let daq = &project.daq;
    line(out, 6, "/begin DAQ");
    line(out, 8, &daq.mode);
    line(out, 8, &hex_u32(daq.max_daq));
    line(out, 8, &hex_u32(daq.max_event_channel));
    line(out, 8, &hex_u32(daq.min_daq as u32));
    line(out, 8, &daq.optimisation_type);
    line(out, 8, &daq.address_extension);
    line(out, 8, &daq.identification_field_type);
    line(out, 8, &daq.granularity);
    line(out, 8, &hex_u32(daq.max_odt_entry_size_daq as u32));
    line(out, 8, &daq.overload_indication);
    if daq.prescaler_supported {
        line(out, 8, "PRESCALER_SUPPORTED");
    }
    if daq.resume_supported {
        line(out, 8, "RESUME_SUPPORTED");
    }
    if daq.pid_off_supported {
        line(out, 8, "PID_OFF_SUPPORTED");
    }

    line(out, 8, "/begin STIM");
    line(out, 10, &daq.stim_granularity);
    line(out, 10, &hex_u32(daq.max_odt_entry_size_stim as u32));
    line(out, 8, "/end STIM");

    let timestamp_size = if daq.timestamp_supported {
        daq.timestamp_size.as_str()
    } else {
        "NO_TIME_STAMP"
    };
    line(out, 8, "/begin TIMESTAMP_SUPPORTED");
    line(out, 10, &hex_u32(daq.timestamp_ticks));
    line(out, 10, timestamp_size);
    line(out, 10, &daq.timestamp_unit);
    if daq.timestamp_fixed {
        line(out, 10, "TIMESTAMP_FIXED");
    }
    line(out, 8, "/end TIMESTAMP_SUPPORTED");

    for evt in &project.event_channels {
        line(out, 8, "/begin EVENT");
        line(out, 10, &quote(&evt.name));
        line(out, 10, &quote(&evt.short_name));
        line(out, 10, &hex_u32(evt.channel_number));
        line(out, 10, &evt.event_type);
        line(out, 10, &hex_u32(evt.max_daq_list as u32));
        line(out, 10, &hex_u32(evt.time_cycle as u32));
        line(out, 10, &hex_u32(evt.time_unit as u32));
        line(out, 10, &hex_u32(evt.priority as u32));
        line(out, 8, "/end EVENT");
    }

    if daq.mode == "STATIC" {
        for daq_list in &project.daq_lists {
            line(out, 8, "/begin DAQ_LIST");
            line(out, 10, &hex_u32(daq_list.number));
            line(out, 10, &format!("DAQ_LIST_TYPE {}", daq_list.daq_type));
            line(out, 10, &format!("MAX_ODT {}", hex_u32(daq_list.max_odt as u32)));
            line(
                out,
                10,
                &format!("MAX_ODT_ENTRIES {}", hex_u32(daq_list.max_odt_entries as u32)),
            );
            line(out, 10, &format!("FIRST_PID {}", hex_u32(daq_list.first_pid as u32)));
            if daq_list.event_fixed > 0 {
                line(out, 10, &format!("EVENT_FIXED {}", hex_u32(daq_list.event_fixed)));
            }
            if !daq_list.odts.is_empty() {
                line(out, 10, "/begin PREDEFINED");
                for odt in &daq_list.odts {
                    line(out, 12, "/begin ODT");
                    line(out, 14, &odt.number.to_string());
                    for entry in &odt.entries {
                        line(
                            out,
                            14,
                            &format!(
                                "ODT_ENTRY {} {} {} {} {}",
                                entry.number,
                                hex_u64(entry.address),
                                entry.address_extension,
                                entry.size,
                                entry.bit_offset
                            ),
                        );
                    }
                    line(out, 12, "/end ODT");
                }
                line(out, 10, "/end PREDEFINED");
            }
            line(out, 8, "/end DAQ_LIST");
        }
    }

    line(out, 6, "/end DAQ");
}

fn write_record_layouts(out: &mut String) {
    for data_type in A2L_DATA_TYPES {
        line(out, 4, &format!("/begin RECORD_LAYOUT __{}", data_type.as_str()));
        line(
            out,
            6,
            &format!("FNC_VALUES 1 {} COLUMN_DIR DIRECT", data_type.as_str()),
        );
        line(out, 4, "/end RECORD_LAYOUT");
    }
}

fn write_compu_method(out: &mut String, compu: &CompuMethod) {
    let conversion_type = if compu.kind.eq_ignore_ascii_case("LINEAR") {
        "LINEAR"
    } else {
        "IDENTICAL"
    };
    line(
        out,
        4,
        &format!(
            "/begin COMPU_METHOD {} {} {} \"%6.2\" {}",
            compu.name,
            quote(&compu.name),
            conversion_type,
            quote(&compu.unit)
        ),
    );
    if conversion_type == "LINEAR" {
        line(
            out,
            6,
            &format!(
                "COEFFS_LINEAR {} {}",
                number(compu.offset),
                number(compu.factor)
            ),
        );
    }
    line(out, 4, "/end COMPU_METHOD");
}

fn write_measurement(out: &mut String, m: &Measurement) {
    line(
        out,
        4,
        &format!("/begin MEASUREMENT {} {}", m.name, quote(&m.long_identifier)),
    );
    line(
        out,
        6,
        &format!(
            "{} {} {} {} {} {}",
            m.data_type.as_str(),
            m.conversion,
            number(m.resolution),
            number(m.accuracy),
            number(m.lower_limit),
            number(m.upper_limit)
        ),
    );
    if m.ecu_address != 0 {
        line(out, 6, &format!("ECU_ADDRESS {}", hex_u32(m.ecu_address)));
    }
    if !m.format.trim().is_empty() {
        line(out, 6, &format!("FORMAT {}", quote(&m.format)));
    }
    if !m.phys_unit.trim().is_empty() {
        line(out, 6, &format!("PHYS_UNIT {}", quote(&m.phys_unit)));
    }
    line(out, 4, "/end MEASUREMENT");
}

fn write_characteristic(out: &mut String, c: &Characteristic) {
    line(
        out,
        4,
        &format!("/begin CHARACTERISTIC {} {}", c.name, quote(&c.long_identifier)),
    );
    line(
        out,
        6,
        &format!(
            "{:?} {} {} {} {} {} {}",
            c.characteristic_type,
            hex_u32(c.ecu_address),
            c.deposit,
            number(c.max_diff),
            c.conversion,
            number(c.lower_limit),
            number(c.upper_limit)
        ),
    );
    if c.extended_lower_limit != f64::MIN || c.extended_upper_limit != f64::MAX {
        line(
            out,
            6,
            &format!(
                "EXTENDED_LIMITS {} {}",
                number(c.extended_lower_limit),
                number(c.extended_upper_limit)
            ),
        );
    }
    if !c.format.trim().is_empty() {
        line(out, 6, &format!("FORMAT {}", quote(&c.format)));
    }
    if !c.phys_unit.trim().is_empty() {
        line(out, 6, &format!("PHYS_UNIT {}", quote(&c.phys_unit)));
    }
    line(out, 4, "/end CHARACTERISTIC");
}

fn write_memory_segment(out: &mut String, s: &MemorySegment) {
    line(
        out,
        4,
        &format!("/begin MEMORY_SEGMENT {} {}", s.name, quote(&s.long_identifier)),
    );
    line(
        out,
        6,
        &format!(
            "{:?} {:?} {:?} {} {} {} {} {} {} {}",
            s.prg_type,
            s.memory_type,
            s.attribute,
            hex_u64(s.address),
            hex_u64(s.size),
            s.offset0,
            s.offset1,
            s.offset2,
            s.offset3,
            s.offset4
        ),
    );
    line(out, 4, "/end MEMORY_SEGMENT");
}

fn line(out: &mut String, indent: usize, value: &str) {
    for _ in 0..indent {
        out.push(' ');
    }
    out.push_str(value);
    out.push('\n');
}

pub fn quote(value: &str) -> String {
    format!("\"{}\"", value.replace('\\', "\\\\").replace('"', "\\\""))
}

pub fn hex_u32(value: u32) -> String {
    format!("0x{value:X}")
}

pub fn hex_u64(value: u64) -> String {
    format!("0x{value:X}")
}

fn hex_u16(value: u16) -> String {
    format!("0x{value:X}")
}

pub fn number(value: f64) -> String {
    if value.is_nan() {
        return "NaN".to_string();
    }
    if value.is_infinite() {
        return if value > 0.0 { "Infinity" } else { "-Infinity" }.to_string();
    }

    let mut text = format!("{value:.12}");
    if text.contains('.') {
        while text.ends_with('0') {
            text.pop();
        }
        if text.ends_with('.') {
            text.pop();
        }
    }
    if text == "-0" {
        text = "0".to_string();
    }
    text
}
