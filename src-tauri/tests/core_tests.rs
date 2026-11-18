use a2l_editor_lib::model::*;
use a2l_editor_lib::{a2l, address, factory, project_xml, typemap, validate};

fn sample_project() -> A2lProject {
    let mut project = A2lProject::default();
    project.project_name = "Demo".to_string();
    project.module_name = "ECU".to_string();

    project.measurements.push(Measurement {
        name: "motor.speed".to_string(),
        long_identifier: "Motor speed".to_string(),
        ecu_address: 0xA000,
        phys_unit: "rpm".to_string(),
        lower_limit: 0.0,
        upper_limit: 8000.0,
        data_type: A2lDataType::UWORD,
        ..Measurement::default()
    });
    project.characteristics.push(Characteristic {
        name: "motor.gain".to_string(),
        ecu_address: 0xB000,
        deposit: "__UWORD".to_string(),
        lower_limit: 0.0,
        upper_limit: 100.0,
        ..Characteristic::default()
    });
    project.memory_segments.push(MemorySegment {
        name: "FLASH0".to_string(),
        address: 0x8000,
        size: 0x1000,
        ..MemorySegment::default()
    });
    project
}

#[test]
fn type_mapper_handles_c28x_types() {
    assert_eq!(typemap::map_c28x("unsigned int", Some(2)).data_type, A2lDataType::UWORD);
    assert!(!typemap::map_c28x("unsigned int", Some(2)).requires_confirmation);
    assert_eq!(typemap::map_c28x("int", Some(2)).data_type, A2lDataType::SWORD);
    assert_eq!(typemap::map_c28x("unsigned  long", Some(4)).data_type, A2lDataType::ULONG);
    assert_eq!(typemap::map_c28x("float", Some(4)).data_type, A2lDataType::FLOAT32_IEEE);
    assert!(typemap::map_c28x("char", Some(2)).requires_confirmation);
    assert_eq!(typemap::map_c28x("unknown", Some(4)).data_type, A2lDataType::ULONG);
    assert!(typemap::map_c28x("unknown", Some(4)).requires_confirmation);
}

#[test]
fn address_conversion_handles_word_addressing() {
    assert_eq!(address::to_ecu_address(0x9000, AddressMode::Raw), 0x9000);
    assert_eq!(address::to_ecu_address(0x9000, AddressMode::ByteAddress), 0x9000);
    assert_eq!(address::to_ecu_address(0x9000, AddressMode::WordAddress), 0x12000);
}

#[test]
fn new_characteristic_extended_limits_match_data_type_limits() {
    let symbol = SymbolNode {
        suggested_data_type: A2lDataType::SBYTE,
        ..SymbolNode::leaf("value".to_string(), "value".to_string(), 0x1000, 1)
    };

    let characteristic = factory::create_characteristic(&symbol, AddressMode::Raw);

    assert_eq!(characteristic.lower_limit, i8::MIN as f64);
    assert_eq!(characteristic.upper_limit, i8::MAX as f64);
    assert_eq!(characteristic.extended_lower_limit, characteristic.lower_limit);
    assert_eq!(characteristic.extended_upper_limit, characteristic.upper_limit);
}

#[test]
fn detects_and_applies_address_changes() {
    let mut project = sample_project();
    project.measurements[0].ecu_address = 0x1;
    project.xcp.address_mode = AddressMode::WordAddress;

    let symbols = vec![SymbolNode {
        name: "root".to_string(),
        path: "root".to_string(),
        is_leaf: false,
        children: vec![SymbolNode {
            name: "motor.speed".to_string(),
            path: "motor.speed".to_string(),
            address: 0x5000,
            is_leaf: true,
            ..SymbolNode::leaf(String::new(), String::new(), 0, 0)
        }],
        ..SymbolNode::leaf(String::new(), String::new(), 0, 0)
    }];

    let changes = address::detect(&project, &symbols);
    assert_eq!(changes.len(), 1);
    assert_eq!(changes[0].new_address, 0xA000);

    address::apply(&mut project, &changes);
    assert_eq!(project.measurements[0].ecu_address, 0xA000);
}

#[test]
fn writes_expected_a2l_text() {
    let project = sample_project();
    let text = a2l::write(&project);

    assert!(text.starts_with("ASAP2_VERSION 1 71\n"));
    assert!(text.contains("/begin PROJECT Demo \"Demo\"\n"));
    assert!(text.contains("  /begin MODULE ECU \"ECU\"\n"));
    assert!(text.contains("    /begin MOD_COMMON \"\"\n"));
    assert!(text.contains("      BYTE_ORDER MSB_LAST\n"));
    assert!(text.contains("        CAN_ID_MASTER 0x720\n"));
    assert!(text.contains("    /begin RECORD_LAYOUT __UWORD\n"));
    assert!(text.contains("    /begin COMPU_METHOD IDENTICAL \"IDENTICAL\" IDENTICAL \"%6.2\" \"\"\n"));
    assert!(text.contains(
        "    /begin MEASUREMENT motor.speed \"Motor speed\"\n      UWORD IDENTICAL 1 0 0 8000\n      ECU_ADDRESS 0xA000\n      FORMAT \"%6.2\"\n      PHYS_UNIT \"rpm\"\n"
    ));
    assert!(text.contains(
        "    /begin CHARACTERISTIC motor.gain \"\"\n      VALUE 0xB000 __UWORD 0 IDENTICAL 0 100\n"
    ));
    assert!(text.contains(
        "    /begin MEMORY_SEGMENT FLASH0 \"\"\n      DATA FLASH INTERN 0x8000 0x1000 -1 -1 -1 -1 -1\n"
    ));
    assert!(text.ends_with("  /end MODULE\n/end PROJECT\n"));
}

#[test]
fn formats_numbers_like_dotnet_custom_format() {
    assert_eq!(a2l::number(1.0), "1");
    assert_eq!(a2l::number(0.5), "0.5");
    assert_eq!(a2l::number(65535.0), "65535");
    assert_eq!(a2l::number(-0.0), "0");
    assert_eq!(a2l::number(1.2345678901234), "1.234567890123");
}

#[test]
fn xml_round_trip_preserves_project() {
    let project = sample_project();
    let xml = project_xml::to_xml(&project);
    let loaded = project_xml::from_xml(&xml).expect("xml should parse");

    assert_eq!(loaded.project_name, "Demo");
    assert_eq!(loaded.module_name, "ECU");
    assert_eq!(loaded.measurements.len(), 1);
    assert_eq!(loaded.measurements[0].name, "motor.speed");
    assert_eq!(loaded.measurements[0].ecu_address, 0xA000);
    assert_eq!(loaded.measurements[0].data_type, A2lDataType::UWORD);
    assert_eq!(loaded.characteristics[0].deposit, "__UWORD");
    assert_eq!(loaded.characteristics[0].extended_lower_limit, f64::MIN);
    assert_eq!(loaded.memory_segments[0].size, 0x1000);
    assert_eq!(loaded.compu_methods.len(), 1);
}

#[test]
fn reads_legacy_xml_with_bom_and_hex_helpers() {
    let xml = "\u{feff}<?xml version=\"1.0\" encoding=\"utf-8\"?>
<A2lProject xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xmlns:xsd=\"http://www.w3.org/2001/XMLSchema\">
  <ProjectName>Old</ProjectName>
  <ModuleName>Mod</ModuleName>
  <Xcp>
    <CroCanId>1824</CroCanId>
    <DtoCanId>1792</DtoCanId>
    <AddressGranularity>2</AddressGranularity>
    <AddressMode>WordAddress</AddressMode>
  </Xcp>
  <Measurements>
    <A2lMeasurement>
      <Name>speed</Name>
      <EcuAddress>4096</EcuAddress>
      <EcuAddressHex>0x1000</EcuAddressHex>
      <DataType>UWORD</DataType>
      <IsEnabled>true</IsEnabled>
    </A2lMeasurement>
  </Measurements>
  <CompuMethods>
    <CompuMethod><Name>IDENTICAL</Name><Kind>IDENTICAL</Kind><Factor>1</Factor><Offset>0</Offset><Unit /></CompuMethod>
  </CompuMethods>
</A2lProject>";

    let project = project_xml::from_xml(xml).expect("legacy xml should parse");
    assert_eq!(project.project_name, "Old");
    assert_eq!(project.xcp.address_mode, AddressMode::WordAddress);
    assert_eq!(project.xcp.address_granularity, 2);
    assert_eq!(project.measurements[0].ecu_address, 4096);
    assert_eq!(project.measurements[0].data_type, A2lDataType::UWORD);
}

#[test]
fn validator_reports_required_fields() {
    let project = A2lProject::default();
    let result = validate::validate(&project);
    assert!(!result.can_export);
    assert!(result.errors.contains(&"至少需要一个 A2L 对象。".to_string()));

    let mut project = sample_project();
    project.characteristics[0].ecu_address = 0;
    let result = validate::validate(&project);
    assert!(!result.can_export);
    assert!(result.errors.iter().any(|error| error.contains("地址不能为空")));
    assert!(result.warnings.iter().any(|warning| warning.contains("单位为空")));
    assert!(result.warnings.iter().any(|warning| warning.contains("地址类型")));
}

#[test]
fn validator_flags_daq_capacity() {
    let mut project = sample_project();
    project.daq_lists.push(DaqListConfig {
        number: 1,
        max_odt: 1,
        max_odt_entries: 1,
        odts: vec![
            OdtConfig::default(),
            OdtConfig {
                number: 1,
                entries: vec![OdtEntry::default(), OdtEntry::default()],
            },
        ],
        ..DaqListConfig::default()
    });

    let result = validate::validate(&project);
    assert!(result.errors.iter().any(|error| error.contains("ODT 数量")));
    assert!(result.errors.iter().any(|error| error.contains("条目数")));
}
