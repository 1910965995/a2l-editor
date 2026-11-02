#![allow(non_camel_case_types)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum A2lDataType {
    UBYTE,
    SBYTE,
    UWORD,
    SWORD,
    ULONG,
    SLONG,
    A_UINT64,
    A_INT64,
    FLOAT32_IEEE,
    FLOAT64_IEEE,
}

impl Default for A2lDataType {
    fn default() -> Self {
        Self::UWORD
    }
}

pub const A2L_DATA_TYPES: [A2lDataType; 10] = [
    A2lDataType::UBYTE,
    A2lDataType::SBYTE,
    A2lDataType::UWORD,
    A2lDataType::SWORD,
    A2lDataType::ULONG,
    A2lDataType::SLONG,
    A2lDataType::A_UINT64,
    A2lDataType::A_INT64,
    A2lDataType::FLOAT32_IEEE,
    A2lDataType::FLOAT64_IEEE,
];

impl A2lDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UBYTE => "UBYTE",
            Self::SBYTE => "SBYTE",
            Self::UWORD => "UWORD",
            Self::SWORD => "SWORD",
            Self::ULONG => "ULONG",
            Self::SLONG => "SLONG",
            Self::A_UINT64 => "A_UINT64",
            Self::A_INT64 => "A_INT64",
            Self::FLOAT32_IEEE => "FLOAT32_IEEE",
            Self::FLOAT64_IEEE => "FLOAT64_IEEE",
        }
    }

    pub fn byte_size(self) -> u8 {
        match self {
            Self::UBYTE | Self::SBYTE => 1,
            Self::UWORD | Self::SWORD => 2,
            Self::ULONG | Self::SLONG | Self::FLOAT32_IEEE => 4,
            Self::A_UINT64 | Self::A_INT64 | Self::FLOAT64_IEEE => 8,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum AddressMode {
    #[default]
    Raw,
    ByteAddress,
    WordAddress,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum CharacteristicType {
    #[default]
    VALUE,
    CURVE,
    MAP,
    CUBOID,
    CUBE_4,
    CUBE_5,
    VAL_BLK,
    ASCII,
}

pub const CHARACTERISTIC_TYPES: [CharacteristicType; 8] = [
    CharacteristicType::VALUE,
    CharacteristicType::CURVE,
    CharacteristicType::MAP,
    CharacteristicType::CUBOID,
    CharacteristicType::CUBE_4,
    CharacteristicType::CUBE_5,
    CharacteristicType::VAL_BLK,
    CharacteristicType::ASCII,
];

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum PrgType {
    CALIBRATION_VARIABLES,
    CODE,
    #[default]
    DATA,
    EXCLUDE_FROM_FLASH,
    OFFLINE_DATA,
    RESERVED,
    SERAM,
    VARIABLES,
}

pub const PRG_TYPES: [PrgType; 8] = [
    PrgType::CALIBRATION_VARIABLES,
    PrgType::CODE,
    PrgType::DATA,
    PrgType::EXCLUDE_FROM_FLASH,
    PrgType::OFFLINE_DATA,
    PrgType::RESERVED,
    PrgType::SERAM,
    PrgType::VARIABLES,
];

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MemMemoryType {
    EEPROM,
    EPROM,
    #[default]
    FLASH,
    RAM,
    ROM,
    REGISTER,
    NOT_IN_ECU,
}

pub const MEM_MEMORY_TYPES: [MemMemoryType; 7] = [
    MemMemoryType::EEPROM,
    MemMemoryType::EPROM,
    MemMemoryType::FLASH,
    MemMemoryType::RAM,
    MemMemoryType::ROM,
    MemMemoryType::REGISTER,
    MemMemoryType::NOT_IN_ECU,
];

#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum MemAttribute {
    #[default]
    INTERN,
    EXTERN,
}

pub const MEM_ATTRIBUTES: [MemAttribute; 2] = [MemAttribute::INTERN, MemAttribute::EXTERN];

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CompuMethod {
    pub name: String,
    pub kind: String,
    pub factor: f64,
    pub offset: f64,
    pub unit: String,
}

impl Default for CompuMethod {
    fn default() -> Self {
        Self {
            name: "IDENTICAL".to_string(),
            kind: "IDENTICAL".to_string(),
            factor: 1.0,
            offset: 0.0,
            unit: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Measurement {
    pub name: String,
    pub long_identifier: String,
    pub ecu_address: u32,
    pub phys_unit: String,
    pub format: String,
    pub lower_limit: f64,
    pub upper_limit: f64,
    pub conversion: String,
    pub is_enabled: bool,
    pub data_type: A2lDataType,
    pub resolution: f64,
    pub accuracy: f64,
}

impl Default for Measurement {
    fn default() -> Self {
        Self {
            name: String::new(),
            long_identifier: String::new(),
            ecu_address: 0,
            phys_unit: String::new(),
            format: "%6.2".to_string(),
            lower_limit: 0.0,
            upper_limit: 65535.0,
            conversion: "IDENTICAL".to_string(),
            is_enabled: true,
            data_type: A2lDataType::UWORD,
            resolution: 1.0,
            accuracy: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Characteristic {
    pub name: String,
    pub long_identifier: String,
    pub ecu_address: u32,
    pub phys_unit: String,
    pub format: String,
    pub lower_limit: f64,
    pub upper_limit: f64,
    pub conversion: String,
    pub is_enabled: bool,
    pub characteristic_type: CharacteristicType,
    pub deposit: String,
    pub max_diff: f64,
    pub extended_lower_limit: f64,
    pub extended_upper_limit: f64,
}

impl Default for Characteristic {
    fn default() -> Self {
        Self {
            name: String::new(),
            long_identifier: String::new(),
            ecu_address: 0,
            phys_unit: String::new(),
            format: "%6.2".to_string(),
            lower_limit: 0.0,
            upper_limit: 65535.0,
            conversion: "IDENTICAL".to_string(),
            is_enabled: true,
            characteristic_type: CharacteristicType::VALUE,
            deposit: String::new(),
            max_diff: 0.0,
            extended_lower_limit: f64::MIN,
            extended_upper_limit: f64::MAX,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MemorySegment {
    pub name: String,
    pub long_identifier: String,
    pub prg_type: PrgType,
    pub memory_type: MemMemoryType,
    pub attribute: MemAttribute,
    pub address: u64,
    pub size: u64,
    pub offset0: i64,
    pub offset1: i64,
    pub offset2: i64,
    pub offset3: i64,
    pub offset4: i64,
}

impl Default for MemorySegment {
    fn default() -> Self {
        Self {
            name: String::new(),
            long_identifier: String::new(),
            prg_type: PrgType::DATA,
            memory_type: MemMemoryType::FLASH,
            attribute: MemAttribute::INTERN,
            address: 0,
            size: 0,
            offset0: -1,
            offset1: -1,
            offset2: -1,
            offset3: -1,
            offset4: -1,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct XcpOptions {
    pub cro_can_id: u32,
    pub dto_can_id: u32,
    pub can_baud_rate: i32,
    pub protocol_layer_version: u16,
    pub t1: u32,
    pub t2: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    pub t7: u32,
    pub max_cto: i32,
    pub max_dto: i32,
    pub byte_order: String,
    pub address_granularity: i32,
    pub can_version: u16,
    pub address_mode: AddressMode,
}

impl Default for XcpOptions {
    fn default() -> Self {
        Self {
            cro_can_id: 0x720,
            dto_can_id: 0x700,
            can_baud_rate: 500_000,
            protocol_layer_version: 0x0100,
            t1: 0,
            t2: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            t7: 0,
            max_cto: 8,
            max_dto: 8,
            byte_order: "MSB_LAST".to_string(),
            address_granularity: 1,
            can_version: 0x0100,
            address_mode: AddressMode::Raw,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DaqConfig {
    pub mode: String,
    pub max_daq: u32,
    pub max_event_channel: u32,
    pub min_daq: u8,
    pub optimisation_type: String,
    pub address_extension: String,
    pub identification_field_type: String,
    pub granularity: String,
    pub max_odt_entry_size_daq: u8,
    pub overload_indication: String,
    pub default_max_odt: u8,
    pub default_max_odt_entries: u8,
    pub prescaler_supported: bool,
    pub resume_supported: bool,
    pub pid_off_supported: bool,
    pub timestamp_supported: bool,
    pub timestamp_ticks: u32,
    pub timestamp_size: String,
    pub timestamp_unit: String,
    pub timestamp_fixed: bool,
    pub stim_granularity: String,
    pub max_odt_entry_size_stim: u8,
}

impl Default for DaqConfig {
    fn default() -> Self {
        Self {
            mode: "DYNAMIC".to_string(),
            max_daq: 0,
            max_event_channel: 5,
            min_daq: 0,
            optimisation_type: "OPTIMISATION_TYPE_DEFAULT".to_string(),
            address_extension: "ADDRESS_EXTENSION_FREE".to_string(),
            identification_field_type: "IDENTIFICATION_FIELD_TYPE_RELATIVE_BYTE".to_string(),
            granularity: "GRANULARITY_ODT_ENTRY_SIZE_DAQ_BYTE".to_string(),
            max_odt_entry_size_daq: 5,
            overload_indication: "NO_OVERLOAD_INDICATION".to_string(),
            default_max_odt: 1,
            default_max_odt_entries: 1,
            prescaler_supported: false,
            resume_supported: false,
            pid_off_supported: false,
            timestamp_supported: true,
            timestamp_ticks: 0,
            timestamp_size: "SIZE_DWORD".to_string(),
            timestamp_unit: "UNIT_1MS".to_string(),
            timestamp_fixed: true,
            stim_granularity: "GRANULARITY_ODT_ENTRY_SIZE_STIM_BYTE".to_string(),
            max_odt_entry_size_stim: 6,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EventChannel {
    pub name: String,
    pub short_name: String,
    pub channel_number: u32,
    #[serde(rename = "type")]
    pub event_type: String,
    pub max_daq_list: u8,
    pub time_cycle: u8,
    pub time_unit: u8,
    pub priority: u8,
}

impl Default for EventChannel {
    fn default() -> Self {
        Self {
            name: String::new(),
            short_name: String::new(),
            channel_number: 0,
            event_type: "DAQ".to_string(),
            max_daq_list: 1,
            time_cycle: 0,
            time_unit: 6,
            priority: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DaqListConfig {
    pub number: u32,
    #[serde(rename = "type")]
    pub daq_type: String,
    pub max_odt: u8,
    pub max_odt_entries: u8,
    pub first_pid: u8,
    pub event_fixed: u32,
    pub odts: Vec<OdtConfig>,
}

impl Default for DaqListConfig {
    fn default() -> Self {
        Self {
            number: 0,
            daq_type: "DAQ".to_string(),
            max_odt: 1,
            max_odt_entries: 1,
            first_pid: 0,
            event_fixed: 0,
            odts: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct OdtConfig {
    pub number: u8,
    pub entries: Vec<OdtEntry>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OdtEntry {
    pub number: u8,
    pub address: u64,
    pub address_extension: u8,
    pub size: u8,
    pub bit_offset: u8,
}

impl Default for OdtEntry {
    fn default() -> Self {
        Self {
            number: 0,
            address: 0,
            address_extension: 0,
            size: 2,
            bit_offset: 0xFF,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct A2lProject {
    pub project_name: String,
    pub module_name: String,
    pub out_file_path: String,
    pub target_platform: String,
    pub xcp: XcpOptions,
    pub daq: DaqConfig,
    pub event_channels: Vec<EventChannel>,
    pub daq_lists: Vec<DaqListConfig>,
    pub measurements: Vec<Measurement>,
    pub characteristics: Vec<Characteristic>,
    pub memory_segments: Vec<MemorySegment>,
    pub compu_methods: Vec<CompuMethod>,
    pub last_export_path: String,
}

impl Default for A2lProject {
    fn default() -> Self {
        Self {
            project_name: "Project".to_string(),
            module_name: "Module".to_string(),
            out_file_path: String::new(),
            target_platform: "TI C2000 / C28x".to_string(),
            xcp: XcpOptions::default(),
            daq: DaqConfig::default(),
            event_channels: Vec::new(),
            daq_lists: Vec::new(),
            measurements: Vec::new(),
            characteristics: Vec::new(),
            memory_segments: Vec::new(),
            compu_methods: vec![CompuMethod::default()],
            last_export_path: String::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SymbolNode {
    pub name: String,
    pub path: String,
    pub address: u32,
    pub size: u32,
    pub ctype: String,
    pub source_file: String,
    pub source_line: Option<i32>,
    pub is_leaf: bool,
    pub requires_type_confirmation: bool,
    pub suggested_data_type: A2lDataType,
    pub children: Vec<SymbolNode>,
}

impl SymbolNode {
    pub fn leaf(name: String, path: String, address: u32, size: u32) -> Self {
        Self {
            name,
            path,
            address,
            size,
            ctype: String::new(),
            source_file: String::new(),
            source_line: None,
            is_leaf: true,
            requires_type_confirmation: false,
            suggested_data_type: A2lDataType::UWORD,
            children: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddressChange {
    pub name: String,
    pub old_address: u32,
    pub new_address: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub can_export: bool,
}

impl ValidationResult {
    pub fn new(errors: Vec<String>, warnings: Vec<String>) -> Self {
        Self {
            can_export: errors.is_empty(),
            errors,
            warnings,
        }
    }
}

macro_rules! impl_from_str {
    ($ty:ty, [$($name:literal => $variant:path),* $(,)?]) => {
        impl std::str::FromStr for $ty {
            type Err = ();

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let normalized = value.trim();
                $(
                    if normalized.eq_ignore_ascii_case($name) {
                        return Ok($variant);
                    }
                )*
                Err(())
            }
        }
    };
}

impl_from_str!(A2lDataType, [
    "UBYTE" => A2lDataType::UBYTE,
    "SBYTE" => A2lDataType::SBYTE,
    "UWORD" => A2lDataType::UWORD,
    "SWORD" => A2lDataType::SWORD,
    "ULONG" => A2lDataType::ULONG,
    "SLONG" => A2lDataType::SLONG,
    "A_UINT64" => A2lDataType::A_UINT64,
    "A_INT64" => A2lDataType::A_INT64,
    "FLOAT32_IEEE" => A2lDataType::FLOAT32_IEEE,
    "FLOAT64_IEEE" => A2lDataType::FLOAT64_IEEE,
]);

impl_from_str!(AddressMode, [
    "Raw" => AddressMode::Raw,
    "ByteAddress" => AddressMode::ByteAddress,
    "WordAddress" => AddressMode::WordAddress,
]);

impl_from_str!(CharacteristicType, [
    "VALUE" => CharacteristicType::VALUE,
    "CURVE" => CharacteristicType::CURVE,
    "MAP" => CharacteristicType::MAP,
    "CUBOID" => CharacteristicType::CUBOID,
    "CUBE_4" => CharacteristicType::CUBE_4,
    "CUBE_5" => CharacteristicType::CUBE_5,
    "VAL_BLK" => CharacteristicType::VAL_BLK,
    "ASCII" => CharacteristicType::ASCII,
]);

impl_from_str!(PrgType, [
    "CALIBRATION_VARIABLES" => PrgType::CALIBRATION_VARIABLES,
    "CODE" => PrgType::CODE,
    "DATA" => PrgType::DATA,
    "EXCLUDE_FROM_FLASH" => PrgType::EXCLUDE_FROM_FLASH,
    "OFFLINE_DATA" => PrgType::OFFLINE_DATA,
    "RESERVED" => PrgType::RESERVED,
    "SERAM" => PrgType::SERAM,
    "VARIABLES" => PrgType::VARIABLES,
]);

impl_from_str!(MemMemoryType, [
    "EEPROM" => MemMemoryType::EEPROM,
    "EPROM" => MemMemoryType::EPROM,
    "FLASH" => MemMemoryType::FLASH,
    "RAM" => MemMemoryType::RAM,
    "ROM" => MemMemoryType::ROM,
    "REGISTER" => MemMemoryType::REGISTER,
    "NOT_IN_ECU" => MemMemoryType::NOT_IN_ECU,
]);

impl_from_str!(MemAttribute, [
    "INTERN" => MemAttribute::INTERN,
    "EXTERN" => MemAttribute::EXTERN,
]);
