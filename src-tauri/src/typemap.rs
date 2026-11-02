use crate::model::A2lDataType;

#[derive(Clone, Copy, Debug)]
pub struct TypeMapping {
    pub data_type: A2lDataType,
    pub requires_confirmation: bool,
}

pub fn map_c28x(c_type: &str, byte_size: Option<u32>) -> TypeMapping {
    let normalized = normalize(c_type);

    match normalized.as_str() {
        "unsigned int" => mapping(A2lDataType::UWORD, false),
        "int" => mapping(A2lDataType::SWORD, false),
        "unsigned short" => mapping(A2lDataType::UWORD, false),
        "short" => mapping(A2lDataType::SWORD, false),
        "unsigned long" => mapping(A2lDataType::ULONG, false),
        "long" => mapping(A2lDataType::SLONG, false),
        "float" => mapping(A2lDataType::FLOAT32_IEEE, false),
        "char" | "signed char" | "unsigned char" => mapping(A2lDataType::UWORD, true),
        _ => guess_by_size(byte_size),
    }
}

fn guess_by_size(byte_size: Option<u32>) -> TypeMapping {
    match byte_size {
        Some(2) => mapping(A2lDataType::UWORD, true),
        Some(4) => mapping(A2lDataType::ULONG, true),
        _ => mapping(A2lDataType::UWORD, true),
    }
}

fn mapping(data_type: A2lDataType, requires_confirmation: bool) -> TypeMapping {
    TypeMapping {
        data_type,
        requires_confirmation,
    }
}

fn normalize(value: &str) -> String {
    value
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
