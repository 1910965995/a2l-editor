use crate::address::to_ecu_address;
use crate::model::*;

pub fn limits(data_type: A2lDataType) -> (f64, f64) {
    match data_type {
        A2lDataType::UBYTE => (0.0, u8::MAX as f64),
        A2lDataType::SBYTE => (i8::MIN as f64, i8::MAX as f64),
        A2lDataType::UWORD => (0.0, u16::MAX as f64),
        A2lDataType::SWORD => (i16::MIN as f64, i16::MAX as f64),
        A2lDataType::ULONG => (0.0, u32::MAX as f64),
        A2lDataType::SLONG => (i32::MIN as f64, i32::MAX as f64),
        A2lDataType::A_UINT64 => (0.0, u64::MAX as f64),
        A2lDataType::A_INT64 => (i64::MIN as f64, i64::MAX as f64),
        A2lDataType::FLOAT32_IEEE => (f32::MIN as f64, f32::MAX as f64),
        A2lDataType::FLOAT64_IEEE => (f64::MIN, f64::MAX),
    }
}

pub fn create_measurement(symbol: &SymbolNode, address_mode: AddressMode) -> Measurement {
    let (lower, upper) = limits(symbol.suggested_data_type);
    Measurement {
        name: symbol.path.clone(),
        ecu_address: to_ecu_address(symbol.address, address_mode),
        data_type: symbol.suggested_data_type,
        lower_limit: lower,
        upper_limit: upper,
        ..Measurement::default()
    }
}

pub fn create_characteristic(symbol: &SymbolNode, address_mode: AddressMode) -> Characteristic {
    let (lower, upper) = limits(symbol.suggested_data_type);
    Characteristic {
        name: symbol.path.clone(),
        ecu_address: to_ecu_address(symbol.address, address_mode),
        deposit: format!("__{}", symbol.suggested_data_type.as_str()),
        lower_limit: lower,
        upper_limit: upper,
        ..Characteristic::default()
    }
}
