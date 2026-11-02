use std::collections::HashMap;

const DW_TAG_ARRAY_TYPE: u32 = 0x01;
const DW_TAG_ENUMERATION_TYPE: u32 = 0x04;
const DW_TAG_STRUCTURE_TYPE: u32 = 0x13;
const DW_TAG_MEMBER: u32 = 0x0D;
const DW_TAG_UNION_TYPE: u32 = 0x17;
const DW_TAG_TYPEDEF: u32 = 0x16;
const DW_TAG_SUBRANGE_TYPE: u32 = 0x21;
const DW_TAG_VARIABLE: u32 = 0x34;
const DW_TAG_BASE_TYPE: u32 = 0x24;
const DW_TAG_CONST_TYPE: u32 = 0x26;
const DW_TAG_VOLATILE_TYPE: u32 = 0x35;

const DW_AT_LOCATION: u32 = 0x02;
const DW_AT_NAME: u32 = 0x03;
const DW_AT_BYTE_SIZE: u32 = 0x0B;
const DW_AT_TYPE: u32 = 0x49;
const DW_AT_COUNT: u32 = 0x37;
const DW_AT_DATA_MEMBER_LOCATION: u32 = 0x38;
const DW_AT_UPPER_BOUND: u32 = 0x2F;

const DW_FORM_ADDR: u32 = 0x01;
const DW_FORM_DATA1: u32 = 0x0B;
const DW_FORM_DATA2: u32 = 0x05;
const DW_FORM_DATA4: u32 = 0x06;
const DW_FORM_DATA8: u32 = 0x07;
const DW_FORM_STRING: u32 = 0x08;
const DW_FORM_BLOCK1: u32 = 0x0A;
const DW_FORM_SDATA: u32 = 0x0D;
const DW_FORM_STRP: u32 = 0x0E;
const DW_FORM_UDATA: u32 = 0x0F;
const DW_FORM_REF_ADDR: u32 = 0x10;
const DW_FORM_REF4: u32 = 0x13;
const DW_FORM_FLAG: u32 = 0x0C;
const DW_FORM_SEC_OFFSET: u32 = 0x17;
const DW_FORM_EXPRLOC: u32 = 0x18;
const DW_FORM_FLAG_PRESENT: u32 = 0x19;
const DW_FORM_REF_SIG8: u32 = 0x20;

const DW_OP_ADDR: u8 = 0x03;
const DW_OP_PLUS_UCONST: u8 = 0x23;

#[derive(Clone, Debug)]
pub struct DwarfVariable {
    pub name: String,
    pub address: u32,
    pub type_offset: u32,
}

#[derive(Clone, Debug)]
pub struct DwarfMember {
    pub name: String,
    pub type_offset: u32,
    pub offset: u32,
}

#[derive(Clone, Debug)]
pub enum DwarfType {
    Base {
        name: String,
        byte_size: u32,
    },
    Alias {
        name: String,
        type_offset: u32,
    },
    Array {
        element_type_offset: u32,
        count: u32,
        byte_size: u32,
    },
    Struct {
        name: String,
        byte_size: u32,
        members: Vec<DwarfMember>,
    },
    Enum {
        name: String,
        byte_size: u32,
        type_offset: u32,
    },
}

impl DwarfType {
    pub fn name(&self) -> &str {
        match self {
            Self::Base { name, .. } | Self::Alias { name, .. } | Self::Struct { name, .. } | Self::Enum { name, .. } => name,
            Self::Array { .. } => "",
        }
    }

    pub fn byte_size(&self) -> u32 {
        match self {
            Self::Base { byte_size, .. } | Self::Array { byte_size, .. } | Self::Struct { byte_size, .. } | Self::Enum { byte_size, .. } => *byte_size,
            Self::Alias { .. } => 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DwarfModel {
    pub variables: Vec<DwarfVariable>,
    pub types: HashMap<u32, DwarfType>,
}

#[derive(Clone, Debug)]
enum AttrValue {
    U32(u32),
    U64(u64),
    I64(i64),
    Str(String),
    Block(Vec<u8>),
    Flag(bool),
}

impl AttrValue {
    fn as_u32(&self) -> u32 {
        match self {
            Self::U32(value) => *value,
            Self::U64(value) => *value as u32,
            Self::I64(value) => *value as u32,
            Self::Flag(value) => u32::from(*value),
            Self::Str(_) | Self::Block(_) => 0,
        }
    }

    fn as_string(&self) -> &str {
        match self {
            Self::Str(value) => value,
            _ => "",
        }
    }

    fn as_block(&self) -> Option<&[u8]> {
        match self {
            Self::Block(value) => Some(value),
            _ => None,
        }
    }
}

struct DieEntry {
    offset: u32,
    tag: u32,
    attributes: HashMap<u32, AttrValue>,
    children: Vec<usize>,
}

impl DieEntry {
    fn uint(&self, attribute: u32) -> u32 {
        self.attributes.get(&attribute).map(AttrValue::as_u32).unwrap_or(0)
    }

    fn string(&self, attribute: u32) -> String {
        self.attributes.get(&attribute).map(AttrValue::as_string).unwrap_or("").to_string()
    }

    fn member_offset(&self, attribute: u32) -> u32 {
        let Some(value) = self.attributes.get(&attribute) else {
            return 0;
        };
        match value {
            AttrValue::U32(number) => *number,
            AttrValue::I64(number) => *number as u32,
            AttrValue::Block(block) if !block.is_empty() && block[0] == DW_OP_PLUS_UCONST => parse_uleb_slice(block, 1),
            _ => 0,
        }
    }

    fn address(&self, attribute: u32) -> Option<u32> {
        let block = self.attributes.get(&attribute)?.as_block()?;
        if block.len() != 5 || block[0] != DW_OP_ADDR {
            return None;
        }
        Some(u32::from_le_bytes([block[1], block[2], block[3], block[4]]))
    }
}

#[derive(Clone)]
struct Abbreviation {
    tag: u32,
    has_children: bool,
    attributes: Vec<(u32, u32)>,
}

struct DataReader<'a> {
    data: &'a [u8],
    position: usize,
}

impl<'a> DataReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data, position: 0 }
    }

    fn is_end(&self) -> bool {
        self.position >= self.data.len()
    }

    fn read_u8(&mut self) -> Result<u8, String> {
        let value = *self.data.get(self.position).ok_or("DWARF 数据越界。")?;
        self.position += 1;
        Ok(value)
    }

    fn read_u16(&mut self) -> Result<u16, String> {
        let bytes = self.data.get(self.position..self.position + 2).ok_or("DWARF 数据越界。")?;
        self.position += 2;
        Ok(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn read_u32(&mut self) -> Result<u32, String> {
        let bytes = self.data.get(self.position..self.position + 4).ok_or("DWARF 数据越界。")?;
        self.position += 4;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn read_u64(&mut self) -> Result<u64, String> {
        let bytes = self.data.get(self.position..self.position + 8).ok_or("DWARF 数据越界。")?;
        self.position += 8;
        let mut array = [0u8; 8];
        array.copy_from_slice(bytes);
        Ok(u64::from_le_bytes(array))
    }

    fn read_uleb128(&mut self) -> Result<u32, String> {
        let mut result = 0u32;
        let mut shift = 0u32;
        loop {
            let current = self.read_u8()?;
            result |= ((current & 0x7F) as u32).checked_shl(shift).unwrap_or(0);
            if current & 0x80 == 0 {
                return Ok(result);
            }
            shift += 7;
            if shift >= 35 {
                return Err("DWARF ULEB128 数据无效。".to_string());
            }
        }
    }

    fn read_sleb128(&mut self) -> Result<i64, String> {
        let mut result = 0i64;
        let mut shift = 0u32;
        loop {
            let current = self.read_u8()?;
            result |= ((current & 0x7F) as i64).checked_shl(shift).unwrap_or(0);
            shift += 7;
            if current & 0x80 == 0 {
                if shift < 64 && current & 0x40 != 0 {
                    result |= -1i64 << shift;
                }
                return Ok(result);
            }
            if shift >= 64 {
                return Err("DWARF SLEB128 数据无效。".to_string());
            }
        }
    }

    fn read_string(&mut self) -> Result<String, String> {
        let start = self.position;
        while self.position < self.data.len() && self.data[self.position] != 0 {
            self.position += 1;
        }
        let value = String::from_utf8_lossy(&self.data[start..self.position]).to_string();
        if self.position < self.data.len() {
            self.position += 1;
        }
        Ok(value)
    }

    fn read_block1(&mut self) -> Result<Vec<u8>, String> {
        let length = self.read_u8()? as usize;
        self.read_block(length)
    }

    fn read_block(&mut self, length: usize) -> Result<Vec<u8>, String> {
        let bytes = self.data.get(self.position..self.position + length).ok_or("DWARF 数据越界。")?;
        self.position += length;
        Ok(bytes.to_vec())
    }
}

pub fn try_read(sections: &HashMap<String, Vec<u8>>) -> Option<DwarfModel> {
    let debug_info = sections.get(".debug_info")?;
    let debug_abbrev = sections.get(".debug_abbrev")?;
    let debug_str = sections.get(".debug_str").map(Vec::as_slice).unwrap_or(&[]);

    let model = read_debug_info(debug_info, debug_abbrev, debug_str).ok()?;
    if model.variables.is_empty() || model.types.is_empty() {
        None
    } else {
        Some(model)
    }
}

fn read_debug_info(debug_info: &[u8], debug_abbrev: &[u8], debug_str: &[u8]) -> Result<DwarfModel, String> {
    let mut reader = DataReader::new(debug_info);
    let mut abbreviation_sets: HashMap<u32, HashMap<u32, Abbreviation>> = HashMap::new();
    let mut entries: Vec<DieEntry> = Vec::new();

    while !reader.is_end() {
        let unit_offset = reader.position as u32;
        if debug_info.len() - reader.position < 11 {
            break;
        }

        let unit_length = reader.read_u32()?;
        if unit_length == 0 {
            break;
        }
        let unit_end = reader.position + unit_length as usize;
        if unit_end > debug_info.len() {
            break;
        }

        reader.read_u16()?;
        let abbreviation_offset = reader.read_u32()?;
        reader.read_u8()?;

        if !abbreviation_sets.contains_key(&abbreviation_offset) {
            let abbreviations = read_abbreviations(debug_abbrev, abbreviation_offset)?;
            abbreviation_sets.insert(abbreviation_offset, abbreviations);
        }
        let abbreviations = &abbreviation_sets[&abbreviation_offset];

        read_die_sequence(&mut reader, unit_end, abbreviations, unit_offset, debug_str, &mut entries)?;
        reader.position = unit_end;
    }

    let mut variables = Vec::new();
    for entry in &entries {
        if entry.tag != DW_TAG_VARIABLE {
            continue;
        }
        if let Some(address) = entry.address(DW_AT_LOCATION) {
            variables.push(DwarfVariable {
                name: entry.string(DW_AT_NAME),
                address,
                type_offset: entry.uint(DW_AT_TYPE),
            });
        }
    }

    // 两阶段解析：先注册聚合/别名类型的壳，再填充成员，保证任意顺序的前向引用都能解析。
    let mut types: HashMap<u32, DwarfType> = HashMap::new();
    let mut pending: Vec<usize> = Vec::new();

    for (index, entry) in entries.iter().enumerate() {
        match entry.tag {
            DW_TAG_BASE_TYPE => {
                types.insert(
                    entry.offset,
                    DwarfType::Base {
                        name: entry.string(DW_AT_NAME),
                        byte_size: entry.uint(DW_AT_BYTE_SIZE),
                    },
                );
            }
            DW_TAG_TYPEDEF | DW_TAG_CONST_TYPE | DW_TAG_VOLATILE_TYPE => {
                types.insert(
                    entry.offset,
                    DwarfType::Alias {
                        name: entry.string(DW_AT_NAME),
                        type_offset: entry.uint(DW_AT_TYPE),
                    },
                );
            }
            DW_TAG_ARRAY_TYPE | DW_TAG_STRUCTURE_TYPE | DW_TAG_UNION_TYPE | DW_TAG_ENUMERATION_TYPE => {
                pending.push(index);
            }
            _ => {}
        }
    }

    for index in &pending {
        let entry = &entries[*index];
        match entry.tag {
            DW_TAG_ARRAY_TYPE => {
                let subrange = entry
                    .children
                    .iter()
                    .map(|child| &entries[*child])
                    .find(|child| child.tag == DW_TAG_SUBRANGE_TYPE);
                let mut count = subrange.map(|child| child.uint(DW_AT_COUNT)).unwrap_or(0);
                if count == 0 {
                    if let Some(upper_bound) = subrange.and_then(|child| child.attributes.get(&DW_AT_UPPER_BOUND)) {
                        count = upper_bound.as_u32() + 1;
                    }
                }
                types.insert(
                    entry.offset,
                    DwarfType::Array {
                        element_type_offset: entry.uint(DW_AT_TYPE),
                        count,
                        byte_size: 0,
                    },
                );
            }
            DW_TAG_STRUCTURE_TYPE | DW_TAG_UNION_TYPE => {
                let members = entry
                    .children
                    .iter()
                    .map(|child| &entries[*child])
                    .filter(|child| child.tag == DW_TAG_MEMBER)
                    .map(|child| DwarfMember {
                        name: child.string(DW_AT_NAME),
                        type_offset: child.uint(DW_AT_TYPE),
                        offset: child.member_offset(DW_AT_DATA_MEMBER_LOCATION),
                    })
                    .collect();
                types.insert(
                    entry.offset,
                    DwarfType::Struct {
                        name: entry.string(DW_AT_NAME),
                        byte_size: entry.uint(DW_AT_BYTE_SIZE),
                        members,
                    },
                );
            }
            DW_TAG_ENUMERATION_TYPE => {
                types.insert(
                    entry.offset,
                    DwarfType::Enum {
                        name: entry.string(DW_AT_NAME),
                        byte_size: entry.uint(DW_AT_BYTE_SIZE),
                        type_offset: entry.uint(DW_AT_TYPE),
                    },
                );
            }
            _ => {}
        }
    }

    // 数组元素大小依赖元素类型（可能出现在同一编译单元的任意位置），最后统一补齐。
    for index in &pending {
        let entry = &entries[*index];
        if entry.tag != DW_TAG_ARRAY_TYPE {
            continue;
        }
        let (element_type_offset, count) = match types.get(&entry.offset) {
            Some(DwarfType::Array { element_type_offset, count, .. }) => (*element_type_offset, *count),
            _ => continue,
        };
        let element_size = resolve_alias(element_type_offset, &types)
            .map(DwarfType::byte_size)
            .unwrap_or(0);
        types.insert(
            entry.offset,
            DwarfType::Array {
                element_type_offset,
                count,
                byte_size: count.saturating_mul(element_size),
            },
        );
    }

    Ok(DwarfModel { variables, types })
}

fn resolve_alias<'a>(type_offset: u32, types: &'a HashMap<u32, DwarfType>) -> Option<&'a DwarfType> {
    let mut current = types.get(&type_offset)?;
    let mut depth = 0;
    while let DwarfType::Alias { type_offset: next, .. } = current {
        let Some(next_type) = types.get(next) else { return Some(current) };
        current = next_type;
        depth += 1;
        if depth > 32 {
            return Some(current);
        }
    }
    Some(current)
}

fn read_abbreviations(data: &[u8], offset: u32) -> Result<HashMap<u32, Abbreviation>, String> {
    let mut reader = DataReader::new(data);
    if offset as usize >= data.len() {
        return Err("DWARF 缩写表偏移越界。".to_string());
    }
    reader.position = offset as usize;
    let mut abbreviations = HashMap::new();

    loop {
        let code = reader.read_uleb128()?;
        if code == 0 {
            break;
        }
        let tag = reader.read_uleb128()?;
        let has_children = reader.read_u8()? != 0;
        let mut attributes = Vec::new();
        loop {
            let name = reader.read_uleb128()?;
            let form = reader.read_uleb128()?;
            if name == 0 && form == 0 {
                break;
            }
            attributes.push((name, form));
        }
        abbreviations.insert(code, Abbreviation { tag, has_children, attributes });
    }

    Ok(abbreviations)
}

fn read_die_sequence(
    reader: &mut DataReader,
    unit_end: usize,
    abbreviations: &HashMap<u32, Abbreviation>,
    unit_offset: u32,
    debug_str: &[u8],
    entries: &mut Vec<DieEntry>,
) -> Result<Vec<usize>, String> {
    let mut indices = Vec::new();

    while reader.position < unit_end {
        let offset = reader.position as u32;
        let code = reader.read_uleb128()?;
        if code == 0 {
            break;
        }
        let abbreviation = abbreviations
            .get(&code)
            .ok_or_else(|| "未知 DWARF abbrev code。".to_string())?;

        let mut attributes = HashMap::new();
        for (name, form) in &abbreviation.attributes {
            attributes.insert(*name, read_attribute(reader, *form, debug_str, unit_offset)?);
        }

        let index = entries.len();
        entries.push(DieEntry {
            offset,
            tag: abbreviation.tag,
            attributes,
            children: Vec::new(),
        });
        indices.push(index);

        if abbreviation.has_children {
            let children = read_die_sequence(reader, unit_end, abbreviations, unit_offset, debug_str, entries)?;
            entries[index].children = children;
        }
    }

    Ok(indices)
}

fn read_attribute(
    reader: &mut DataReader,
    form: u32,
    debug_str: &[u8],
    unit_offset: u32,
) -> Result<AttrValue, String> {
    match form {
        DW_FORM_ADDR => Ok(AttrValue::U32(reader.read_u32()?)),
        DW_FORM_DATA1 => Ok(AttrValue::U32(reader.read_u8()? as u32)),
        DW_FORM_DATA2 => Ok(AttrValue::U32(reader.read_u16()? as u32)),
        DW_FORM_DATA4 => Ok(AttrValue::U32(reader.read_u32()?)),
        DW_FORM_DATA8 => Ok(AttrValue::U64(reader.read_u64()?)),
        DW_FORM_REF4 => Ok(AttrValue::U32(unit_offset.wrapping_add(reader.read_u32()?))),
        DW_FORM_STRING => Ok(AttrValue::Str(reader.read_string()?)),
        DW_FORM_STRP => {
            let offset = reader.read_u32()? as usize;
            Ok(AttrValue::Str(read_string_at(debug_str, offset)))
        }
        DW_FORM_BLOCK1 => Ok(AttrValue::Block(reader.read_block1()?)),
        DW_FORM_FLAG => Ok(AttrValue::Flag(reader.read_u8()? != 0)),
        DW_FORM_SDATA => Ok(AttrValue::I64(reader.read_sleb128()?)),
        DW_FORM_UDATA => Ok(AttrValue::U32(reader.read_uleb128()?)),
        DW_FORM_REF_ADDR => Ok(AttrValue::U32(reader.read_u32()?)),
        DW_FORM_SEC_OFFSET => Ok(AttrValue::U32(reader.read_u32()?)),
        DW_FORM_EXPRLOC => {
            let length = reader.read_uleb128()? as usize;
            Ok(AttrValue::Block(reader.read_block(length)?))
        }
        DW_FORM_FLAG_PRESENT => Ok(AttrValue::Flag(true)),
        DW_FORM_REF_SIG8 => Ok(AttrValue::U64(reader.read_u64()?)),
        _ => Err(format!("不支持的 DWARF form: {form}")),
    }
}

fn read_string_at(data: &[u8], offset: usize) -> String {
    if offset >= data.len() {
        return String::new();
    }
    let end = data[offset..]
        .iter()
        .position(|byte| *byte == 0)
        .map(|position| offset + position)
        .unwrap_or(data.len());
    String::from_utf8_lossy(&data[offset..end]).to_string()
}

fn parse_uleb_slice(data: &[u8], start: usize) -> u32 {
    let mut result = 0u32;
    let mut shift = 0u32;
    for byte in data.iter().skip(start) {
        result |= ((byte & 0x7F) as u32).checked_shl(shift).unwrap_or(0);
        if byte & 0x80 == 0 {
            return result;
        }
        shift += 7;
        if shift >= 35 {
            return result;
        }
    }
    result
}
