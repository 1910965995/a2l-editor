use crate::model::*;
use roxmltree::Node;

pub fn to_xml(project: &A2lProject) -> String {
    let mut out = String::new();
    out.push_str("<?xml version=\"1.0\" encoding=\"utf-8\"?>\n");
    out.push_str("<A2lProject xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">\n");

    write_text(&mut out, 1, "ProjectName", &project.project_name);
    write_text(&mut out, 1, "ModuleName", &project.module_name);
    write_text(&mut out, 1, "OutFilePath", &project.out_file_path);
    write_text(&mut out, 1, "TargetPlatform", &project.target_platform);

    write_xcp(&mut out, &project.xcp);
    write_daq(&mut out, &project.daq);

    write_list(&mut out, 1, "EventChannels", "EventChannel", &project.event_channels, write_event);
    write_list(&mut out, 1, "DaqLists", "DaqListConfig", &project.daq_lists, write_daq_list);
    write_list(&mut out, 1, "Measurements", "A2lMeasurement", &project.measurements, write_measurement);
    write_list(
        &mut out,
        1,
        "Characteristics",
        "A2lCharacteristic",
        &project.characteristics,
        write_characteristic,
    );
    write_list(
        &mut out,
        1,
        "MemorySegments",
        "MemorySegment",
        &project.memory_segments,
        write_memory_segment,
    );
    write_list(&mut out, 1, "CompuMethods", "CompuMethod", &project.compu_methods, write_compu_method);

    write_text(&mut out, 1, "LastExportPath", &project.last_export_path);
    out.push_str("</A2lProject>\n");
    out
}

fn write_xcp(out: &mut String, xcp: &XcpOptions) {
    indent(out, 1, "<Xcp>");
    write_text(out, 2, "CroCanId", &xcp.cro_can_id.to_string());
    write_text(out, 2, "DtoCanId", &xcp.dto_can_id.to_string());
    write_text(out, 2, "CanBaudRate", &xcp.can_baud_rate.to_string());
    write_text(out, 2, "ProtocolLayerVersion", &xcp.protocol_layer_version.to_string());
    write_text(out, 2, "T1", &xcp.t1.to_string());
    write_text(out, 2, "T2", &xcp.t2.to_string());
    write_text(out, 2, "T3", &xcp.t3.to_string());
    write_text(out, 2, "T4", &xcp.t4.to_string());
    write_text(out, 2, "T5", &xcp.t5.to_string());
    write_text(out, 2, "T6", &xcp.t6.to_string());
    write_text(out, 2, "T7", &xcp.t7.to_string());
    write_text(out, 2, "MaxCto", &xcp.max_cto.to_string());
    write_text(out, 2, "MaxDto", &xcp.max_dto.to_string());
    write_text(out, 2, "ByteOrder", &xcp.byte_order);
    write_text(out, 2, "AddressGranularity", &xcp.address_granularity.to_string());
    write_text(out, 2, "CanVersion", &xcp.can_version.to_string());
    write_text(out, 2, "AddressMode", &format!("{:?}", xcp.address_mode));
    indent(out, 1, "</Xcp>");
}

fn write_daq(out: &mut String, daq: &DaqConfig) {
    indent(out, 1, "<Daq>");
    write_text(out, 2, "Mode", &daq.mode);
    write_text(out, 2, "MaxDaq", &daq.max_daq.to_string());
    write_text(out, 2, "MaxEventChannel", &daq.max_event_channel.to_string());
    write_text(out, 2, "MinDaq", &daq.min_daq.to_string());
    write_text(out, 2, "OptimisationType", &daq.optimisation_type);
    write_text(out, 2, "AddressExtension", &daq.address_extension);
    write_text(out, 2, "IdentificationFieldType", &daq.identification_field_type);
    write_text(out, 2, "Granularity", &daq.granularity);
    write_text(out, 2, "MaxOdtEntrySizeDaq", &daq.max_odt_entry_size_daq.to_string());
    write_text(out, 2, "OverloadIndication", &daq.overload_indication);
    write_text(out, 2, "DefaultMaxOdt", &daq.default_max_odt.to_string());
    write_text(out, 2, "DefaultMaxOdtEntries", &daq.default_max_odt_entries.to_string());
    write_text(out, 2, "PrescalerSupported", bool_text(daq.prescaler_supported));
    write_text(out, 2, "ResumeSupported", bool_text(daq.resume_supported));
    write_text(out, 2, "PidOffSupported", bool_text(daq.pid_off_supported));
    write_text(out, 2, "TimestampSupported", bool_text(daq.timestamp_supported));
    write_text(out, 2, "TimestampTicks", &daq.timestamp_ticks.to_string());
    write_text(out, 2, "TimestampSize", &daq.timestamp_size);
    write_text(out, 2, "TimestampUnit", &daq.timestamp_unit);
    write_text(out, 2, "TimestampFixed", bool_text(daq.timestamp_fixed));
    write_text(out, 2, "StimGranularity", &daq.stim_granularity);
    write_text(out, 2, "MaxOdtEntrySizeStim", &daq.max_odt_entry_size_stim.to_string());
    indent(out, 1, "</Daq>");
}

fn write_event(out: &mut String, event: &EventChannel) {
    write_text(out, 2, "Name", &event.name);
    write_text(out, 2, "ShortName", &event.short_name);
    write_text(out, 2, "ChannelNumber", &event.channel_number.to_string());
    write_text(out, 2, "Type", &event.event_type);
    write_text(out, 2, "MaxDaqList", &event.max_daq_list.to_string());
    write_text(out, 2, "TimeCycle", &event.time_cycle.to_string());
    write_text(out, 2, "TimeUnit", &event.time_unit.to_string());
    write_text(out, 2, "Priority", &event.priority.to_string());
}

fn write_daq_list(out: &mut String, daq_list: &DaqListConfig) {
    write_text(out, 2, "Number", &daq_list.number.to_string());
    write_text(out, 2, "Type", &daq_list.daq_type);
    write_text(out, 2, "MaxOdt", &daq_list.max_odt.to_string());
    write_text(out, 2, "MaxOdtEntries", &daq_list.max_odt_entries.to_string());
    write_text(out, 2, "FirstPid", &daq_list.first_pid.to_string());
    write_text(out, 2, "EventFixed", &daq_list.event_fixed.to_string());
    write_list(out, 2, "Odts", "OdtConfig", &daq_list.odts, write_odt);
}

fn write_odt(out: &mut String, odt: &OdtConfig) {
    write_text(out, 3, "Number", &odt.number.to_string());
    write_list(out, 3, "Entries", "OdtEntry", &odt.entries, write_odt_entry);
}

fn write_odt_entry(out: &mut String, entry: &OdtEntry) {
    write_text(out, 4, "Number", &entry.number.to_string());
    write_text(out, 4, "Address", &entry.address.to_string());
    write_text(out, 4, "AddressExtension", &entry.address_extension.to_string());
    write_text(out, 4, "Size", &entry.size.to_string());
    write_text(out, 4, "BitOffset", &entry.bit_offset.to_string());
}

fn write_compu_method(out: &mut String, compu: &CompuMethod) {
    write_text(out, 2, "Name", &compu.name);
    write_text(out, 2, "Kind", &compu.kind);
    write_text(out, 2, "Factor", &double_text(compu.factor));
    write_text(out, 2, "Offset", &double_text(compu.offset));
    write_text(out, 2, "Unit", &compu.unit);
}

fn write_measurement(out: &mut String, m: &Measurement) {
    write_text(out, 2, "Name", &m.name);
    write_text(out, 2, "LongIdentifier", &m.long_identifier);
    write_text(out, 2, "EcuAddress", &m.ecu_address.to_string());
    write_text(out, 2, "PhysUnit", &m.phys_unit);
    write_text(out, 2, "Format", &m.format);
    write_text(out, 2, "LowerLimit", &double_text(m.lower_limit));
    write_text(out, 2, "UpperLimit", &double_text(m.upper_limit));
    write_text(out, 2, "Conversion", &m.conversion);
    write_text(out, 2, "IsEnabled", bool_text(m.is_enabled));
    write_text(out, 2, "DataType", m.data_type.as_str());
    write_text(out, 2, "Resolution", &double_text(m.resolution));
    write_text(out, 2, "Accuracy", &double_text(m.accuracy));
}

fn write_characteristic(out: &mut String, c: &Characteristic) {
    write_text(out, 2, "Name", &c.name);
    write_text(out, 2, "LongIdentifier", &c.long_identifier);
    write_text(out, 2, "EcuAddress", &c.ecu_address.to_string());
    write_text(out, 2, "PhysUnit", &c.phys_unit);
    write_text(out, 2, "Format", &c.format);
    write_text(out, 2, "LowerLimit", &double_text(c.lower_limit));
    write_text(out, 2, "UpperLimit", &double_text(c.upper_limit));
    write_text(out, 2, "Conversion", &c.conversion);
    write_text(out, 2, "IsEnabled", bool_text(c.is_enabled));
    write_text(out, 2, "Type", &format!("{:?}", c.characteristic_type));
    write_text(out, 2, "Deposit", &c.deposit);
    write_text(out, 2, "MaxDiff", &double_text(c.max_diff));
    write_text(out, 2, "ExtendedLowerLimit", &double_text(c.extended_lower_limit));
    write_text(out, 2, "ExtendedUpperLimit", &double_text(c.extended_upper_limit));
}

fn write_memory_segment(out: &mut String, s: &MemorySegment) {
    write_text(out, 2, "Name", &s.name);
    write_text(out, 2, "LongIdentifier", &s.long_identifier);
    write_text(out, 2, "PrgType", &format!("{:?}", s.prg_type));
    write_text(out, 2, "MemoryType", &format!("{:?}", s.memory_type));
    write_text(out, 2, "Attribute", &format!("{:?}", s.attribute));
    write_text(out, 2, "Address", &s.address.to_string());
    write_text(out, 2, "Size", &s.size.to_string());
    write_text(out, 2, "Offset0", &s.offset0.to_string());
    write_text(out, 2, "Offset1", &s.offset1.to_string());
    write_text(out, 2, "Offset2", &s.offset2.to_string());
    write_text(out, 2, "Offset3", &s.offset3.to_string());
    write_text(out, 2, "Offset4", &s.offset4.to_string());
}

fn write_list<T>(out: &mut String, depth: usize, wrapper: &str, item: &str, items: &[T], write_item: fn(&mut String, &T)) {
    indent(out, depth, &format!("<{wrapper}>"));
    for entry in items {
        indent(out, depth + 1, &format!("<{item}>"));
        write_item(out, entry);
        indent(out, depth + 1, &format!("</{item}>"));
    }
    indent(out, depth, &format!("</{wrapper}>"));
}

fn write_text(out: &mut String, depth: usize, tag: &str, value: &str) {
    indent(out, depth, &format!("<{tag}>{}</{tag}>", escape(value)));
}

fn indent(out: &mut String, depth: usize, value: &str) {
    for _ in 0..depth {
        out.push_str("  ");
    }
    out.push_str(value);
    out.push('\n');
}

fn escape(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '&' => escaped.push_str("&amp;"),
            '<' => escaped.push_str("&lt;"),
            '>' => escaped.push_str("&gt;"),
            '"' => escaped.push_str("&quot;"),
            '\'' => escaped.push_str("&apos;"),
            _ => escaped.push(ch),
        }
    }
    escaped
}

fn double_text(value: f64) -> String {
    if value == 0.0 {
        "0".to_string()
    } else if value.is_finite() {
        format!("{:?}", value)
    } else if value.is_nan() {
        "NaN".to_string()
    } else if value > 0.0 {
        "INF".to_string()
    } else {
        "-INF".to_string()
    }
}

fn bool_text(value: bool) -> &'static str {
    if value { "true" } else { "false" }
}

pub fn from_xml(xml: &str) -> Result<A2lProject, String> {
    let xml = xml.trim_start_matches('\u{feff}');
    let doc = roxmltree::Document::parse(xml).map_err(|e| format!("工程文件 XML 解析失败：{e}"))?;
    let root = doc.root_element();
    if root.tag_name().name() != "A2lProject" {
        return Err("不是有效的 A2L Editor 工程文件（缺少 A2lProject 根节点）。".to_string());
    }

    let mut project = A2lProject::default();
    if let Some(v) = text_of(root, "ProjectName") { project.project_name = v; }
    if let Some(v) = text_of(root, "ModuleName") { project.module_name = v; }
    if let Some(v) = text_of(root, "OutFilePath") { project.out_file_path = v; }
    if let Some(v) = text_of(root, "TargetPlatform") { project.target_platform = v; }
    if let Some(x) = child(root, "Xcp") { project.xcp = parse_xcp(x); }
    if let Some(x) = child(root, "Daq") { project.daq = parse_daq(x); }

    project.event_channels = list_of(root, "EventChannels", "EventChannel", parse_event);
    project.daq_lists = list_of(root, "DaqLists", "DaqListConfig", parse_daq_list);
    project.measurements = list_of(root, "Measurements", "A2lMeasurement", parse_measurement);
    project.characteristics = list_of(root, "Characteristics", "A2lCharacteristic", parse_characteristic);
    project.memory_segments = list_of(root, "MemorySegments", "MemorySegment", parse_memory_segment);
    project.compu_methods = list_of(root, "CompuMethods", "CompuMethod", parse_compu_method);
    if let Some(v) = text_of(root, "LastExportPath") { project.last_export_path = v; }

    Ok(project)
}

fn parse_xcp(node: Node) -> XcpOptions {
    let mut xcp = XcpOptions::default();
    if let Some(v) = text_of(node, "CroCanId") { xcp.cro_can_id = parse_or(&v, xcp.cro_can_id); }
    if let Some(v) = text_of(node, "DtoCanId") { xcp.dto_can_id = parse_or(&v, xcp.dto_can_id); }
    if let Some(v) = text_of(node, "CanBaudRate") { xcp.can_baud_rate = parse_or(&v, xcp.can_baud_rate); }
    if let Some(v) = text_of(node, "ProtocolLayerVersion") { xcp.protocol_layer_version = parse_or(&v, xcp.protocol_layer_version); }
    if let Some(v) = text_of(node, "T1") { xcp.t1 = parse_or(&v, xcp.t1); }
    if let Some(v) = text_of(node, "T2") { xcp.t2 = parse_or(&v, xcp.t2); }
    if let Some(v) = text_of(node, "T3") { xcp.t3 = parse_or(&v, xcp.t3); }
    if let Some(v) = text_of(node, "T4") { xcp.t4 = parse_or(&v, xcp.t4); }
    if let Some(v) = text_of(node, "T5") { xcp.t5 = parse_or(&v, xcp.t5); }
    if let Some(v) = text_of(node, "T6") { xcp.t6 = parse_or(&v, xcp.t6); }
    if let Some(v) = text_of(node, "T7") { xcp.t7 = parse_or(&v, xcp.t7); }
    if let Some(v) = text_of(node, "MaxCto") { xcp.max_cto = parse_or(&v, xcp.max_cto); }
    if let Some(v) = text_of(node, "MaxDto") { xcp.max_dto = parse_or(&v, xcp.max_dto); }
    if let Some(v) = text_of(node, "ByteOrder") { xcp.byte_order = v; }
    if let Some(v) = text_of(node, "AddressGranularity") { xcp.address_granularity = parse_or(&v, xcp.address_granularity); }
    if let Some(v) = text_of(node, "CanVersion") { xcp.can_version = parse_or(&v, xcp.can_version); }
    if let Some(v) = text_of(node, "AddressMode") { xcp.address_mode = parse_enum(&v, xcp.address_mode); }
    xcp
}

fn parse_daq(node: Node) -> DaqConfig {
    let mut daq = DaqConfig::default();
    if let Some(v) = text_of(node, "Mode") { daq.mode = v; }
    if let Some(v) = text_of(node, "MaxDaq") { daq.max_daq = parse_or(&v, daq.max_daq); }
    if let Some(v) = text_of(node, "MaxEventChannel") { daq.max_event_channel = parse_or(&v, daq.max_event_channel); }
    if let Some(v) = text_of(node, "MinDaq") { daq.min_daq = parse_or(&v, daq.min_daq); }
    if let Some(v) = text_of(node, "OptimisationType") { daq.optimisation_type = v; }
    if let Some(v) = text_of(node, "AddressExtension") { daq.address_extension = v; }
    if let Some(v) = text_of(node, "IdentificationFieldType") { daq.identification_field_type = v; }
    if let Some(v) = text_of(node, "Granularity") { daq.granularity = v; }
    if let Some(v) = text_of(node, "MaxOdtEntrySizeDaq") { daq.max_odt_entry_size_daq = parse_or(&v, daq.max_odt_entry_size_daq); }
    if let Some(v) = text_of(node, "OverloadIndication") { daq.overload_indication = v; }
    if let Some(v) = text_of(node, "DefaultMaxOdt") { daq.default_max_odt = parse_or(&v, daq.default_max_odt); }
    if let Some(v) = text_of(node, "DefaultMaxOdtEntries") { daq.default_max_odt_entries = parse_or(&v, daq.default_max_odt_entries); }
    if let Some(v) = text_of(node, "PrescalerSupported") { daq.prescaler_supported = parse_bool(&v).unwrap_or(daq.prescaler_supported); }
    if let Some(v) = text_of(node, "ResumeSupported") { daq.resume_supported = parse_bool(&v).unwrap_or(daq.resume_supported); }
    if let Some(v) = text_of(node, "PidOffSupported") { daq.pid_off_supported = parse_bool(&v).unwrap_or(daq.pid_off_supported); }
    if let Some(v) = text_of(node, "TimestampSupported") { daq.timestamp_supported = parse_bool(&v).unwrap_or(daq.timestamp_supported); }
    if let Some(v) = text_of(node, "TimestampTicks") { daq.timestamp_ticks = parse_or(&v, daq.timestamp_ticks); }
    if let Some(v) = text_of(node, "TimestampSize") { daq.timestamp_size = v; }
    if let Some(v) = text_of(node, "TimestampUnit") { daq.timestamp_unit = v; }
    if let Some(v) = text_of(node, "TimestampFixed") { daq.timestamp_fixed = parse_bool(&v).unwrap_or(daq.timestamp_fixed); }
    if let Some(v) = text_of(node, "StimGranularity") { daq.stim_granularity = v; }
    if let Some(v) = text_of(node, "MaxOdtEntrySizeStim") { daq.max_odt_entry_size_stim = parse_or(&v, daq.max_odt_entry_size_stim); }
    daq
}

fn parse_event(node: Node) -> EventChannel {
    let mut event = EventChannel::default();
    if let Some(v) = text_of(node, "Name") { event.name = v; }
    if let Some(v) = text_of(node, "ShortName") { event.short_name = v; }
    if let Some(v) = text_of(node, "ChannelNumber") { event.channel_number = parse_or(&v, event.channel_number); }
    if let Some(v) = text_of(node, "Type") { event.event_type = v; }
    if let Some(v) = text_of(node, "MaxDaqList") { event.max_daq_list = parse_or(&v, event.max_daq_list); }
    if let Some(v) = text_of(node, "TimeCycle") { event.time_cycle = parse_or(&v, event.time_cycle); }
    if let Some(v) = text_of(node, "TimeUnit") { event.time_unit = parse_or(&v, event.time_unit); }
    if let Some(v) = text_of(node, "Priority") { event.priority = parse_or(&v, event.priority); }
    event
}

fn parse_daq_list(node: Node) -> DaqListConfig {
    let mut daq_list = DaqListConfig::default();
    if let Some(v) = text_of(node, "Number") { daq_list.number = parse_or(&v, daq_list.number); }
    if let Some(v) = text_of(node, "Type") { daq_list.daq_type = v; }
    if let Some(v) = text_of(node, "MaxOdt") { daq_list.max_odt = parse_or(&v, daq_list.max_odt); }
    if let Some(v) = text_of(node, "MaxOdtEntries") { daq_list.max_odt_entries = parse_or(&v, daq_list.max_odt_entries); }
    if let Some(v) = text_of(node, "FirstPid") { daq_list.first_pid = parse_or(&v, daq_list.first_pid); }
    if let Some(v) = text_of(node, "EventFixed") { daq_list.event_fixed = parse_or(&v, daq_list.event_fixed); }
    daq_list.odts = list_of(node, "Odts", "OdtConfig", parse_odt);
    daq_list
}

fn parse_odt(node: Node) -> OdtConfig {
    let mut odt = OdtConfig::default();
    if let Some(v) = text_of(node, "Number") { odt.number = parse_or(&v, odt.number); }
    odt.entries = list_of(node, "Entries", "OdtEntry", parse_odt_entry);
    odt
}

fn parse_odt_entry(node: Node) -> OdtEntry {
    let mut entry = OdtEntry::default();
    if let Some(v) = text_of(node, "Number") { entry.number = parse_or(&v, entry.number); }
    if let Some(v) = text_of(node, "Address") { entry.address = parse_or(&v, entry.address); }
    if let Some(v) = text_of(node, "AddressExtension") { entry.address_extension = parse_or(&v, entry.address_extension); }
    if let Some(v) = text_of(node, "Size") { entry.size = parse_or(&v, entry.size); }
    if let Some(v) = text_of(node, "BitOffset") { entry.bit_offset = parse_or(&v, entry.bit_offset); }
    entry
}

fn parse_compu_method(node: Node) -> CompuMethod {
    let mut compu = CompuMethod::default();
    if let Some(v) = text_of(node, "Name") { compu.name = v; }
    if let Some(v) = text_of(node, "Kind") { compu.kind = v; }
    if let Some(v) = text_of(node, "Factor") { compu.factor = parse_or(&v, compu.factor); }
    if let Some(v) = text_of(node, "Offset") { compu.offset = parse_or(&v, compu.offset); }
    if let Some(v) = text_of(node, "Unit") { compu.unit = v; }
    compu
}

fn parse_measurement(node: Node) -> Measurement {
    let mut m = Measurement::default();
    if let Some(v) = text_of(node, "Name") { m.name = v; }
    if let Some(v) = text_of(node, "LongIdentifier") { m.long_identifier = v; }
    if let Some(v) = text_of(node, "EcuAddress") { m.ecu_address = parse_or(&v, m.ecu_address); }
    if let Some(v) = text_of(node, "PhysUnit") { m.phys_unit = v; }
    if let Some(v) = text_of(node, "Format") { m.format = v; }
    if let Some(v) = text_of(node, "LowerLimit") { m.lower_limit = parse_or(&v, m.lower_limit); }
    if let Some(v) = text_of(node, "UpperLimit") { m.upper_limit = parse_or(&v, m.upper_limit); }
    if let Some(v) = text_of(node, "Conversion") { m.conversion = v; }
    if let Some(v) = text_of(node, "IsEnabled") { m.is_enabled = parse_bool(&v).unwrap_or(m.is_enabled); }
    if let Some(v) = text_of(node, "DataType") { m.data_type = parse_enum(&v, m.data_type); }
    if let Some(v) = text_of(node, "Resolution") { m.resolution = parse_or(&v, m.resolution); }
    if let Some(v) = text_of(node, "Accuracy") { m.accuracy = parse_or(&v, m.accuracy); }
    m
}

fn parse_characteristic(node: Node) -> Characteristic {
    let mut c = Characteristic::default();
    if let Some(v) = text_of(node, "Name") { c.name = v; }
    if let Some(v) = text_of(node, "LongIdentifier") { c.long_identifier = v; }
    if let Some(v) = text_of(node, "EcuAddress") { c.ecu_address = parse_or(&v, c.ecu_address); }
    if let Some(v) = text_of(node, "PhysUnit") { c.phys_unit = v; }
    if let Some(v) = text_of(node, "Format") { c.format = v; }
    if let Some(v) = text_of(node, "LowerLimit") { c.lower_limit = parse_or(&v, c.lower_limit); }
    if let Some(v) = text_of(node, "UpperLimit") { c.upper_limit = parse_or(&v, c.upper_limit); }
    if let Some(v) = text_of(node, "Conversion") { c.conversion = v; }
    if let Some(v) = text_of(node, "IsEnabled") { c.is_enabled = parse_bool(&v).unwrap_or(c.is_enabled); }
    if let Some(v) = text_of(node, "Type") { c.characteristic_type = parse_enum(&v, c.characteristic_type); }
    if let Some(v) = text_of(node, "Deposit") { c.deposit = v; }
    if let Some(v) = text_of(node, "MaxDiff") { c.max_diff = parse_or(&v, c.max_diff); }
    if let Some(v) = text_of(node, "ExtendedLowerLimit") { c.extended_lower_limit = parse_or(&v, c.extended_lower_limit); }
    if let Some(v) = text_of(node, "ExtendedUpperLimit") { c.extended_upper_limit = parse_or(&v, c.extended_upper_limit); }
    c
}

fn parse_memory_segment(node: Node) -> MemorySegment {
    let mut s = MemorySegment::default();
    if let Some(v) = text_of(node, "Name") { s.name = v; }
    if let Some(v) = text_of(node, "LongIdentifier") { s.long_identifier = v; }
    if let Some(v) = text_of(node, "PrgType") { s.prg_type = parse_enum(&v, s.prg_type); }
    if let Some(v) = text_of(node, "MemoryType") { s.memory_type = parse_enum(&v, s.memory_type); }
    if let Some(v) = text_of(node, "Attribute") { s.attribute = parse_enum(&v, s.attribute); }
    if let Some(v) = text_of(node, "Address") { s.address = parse_or(&v, s.address); }
    if let Some(v) = text_of(node, "Size") { s.size = parse_or(&v, s.size); }
    if let Some(v) = text_of(node, "Offset0") { s.offset0 = parse_or(&v, s.offset0); }
    if let Some(v) = text_of(node, "Offset1") { s.offset1 = parse_or(&v, s.offset1); }
    if let Some(v) = text_of(node, "Offset2") { s.offset2 = parse_or(&v, s.offset2); }
    if let Some(v) = text_of(node, "Offset3") { s.offset3 = parse_or(&v, s.offset3); }
    if let Some(v) = text_of(node, "Offset4") { s.offset4 = parse_or(&v, s.offset4); }
    s
}

fn child<'a, 'input>(node: Node<'a, 'input>, tag: &str) -> Option<Node<'a, 'input>> {
    node.children().find(|child| child.is_element() && child.tag_name().name() == tag)
}

fn text_of<'a, 'input>(node: Node<'a, 'input>, tag: &str) -> Option<String> {
    child(node, tag).and_then(|child| child.text()).map(|text| text.to_string())
}

fn list_of<'a, 'input, T>(
    node: Node<'a, 'input>,
    wrapper: &str,
    item: &str,
    parse: fn(Node<'a, 'input>) -> T,
) -> Vec<T> {
    let Some(wrapper_node) = child(node, wrapper) else {
        return Vec::new();
    };
    wrapper_node
        .children()
        .filter(|child| child.is_element() && child.tag_name().name() == item)
        .map(parse)
        .collect()
}

fn parse_or<T: std::str::FromStr>(text: &str, default: T) -> T {
    text.trim().parse().unwrap_or(default)
}

fn parse_bool(text: &str) -> Option<bool> {
    match text.trim().to_ascii_lowercase().as_str() {
        "true" | "1" => Some(true),
        "false" | "0" => Some(false),
        _ => None,
    }
}

fn parse_enum<T: Default>(text: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    text.trim().parse().unwrap_or(default)
}
