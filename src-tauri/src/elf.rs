use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct ElfObjectSymbol {
    pub name: String,
    pub address: u32,
    pub size: u32,
    pub bind: String,
    pub sym_type: String,
    pub section_index: u16,
}

#[derive(Clone, Debug)]
pub struct Elf32File {
    pub object_symbols: Vec<ElfObjectSymbol>,
    pub sections: HashMap<String, Vec<u8>>,
}

#[derive(Clone, Copy, Debug)]
struct SectionHeader {
    name_offset: u32,
    section_type: u32,
    offset: u32,
    size: u32,
    link: u32,
    entry_size: u32,
}

struct ByteReader<'a> {
    data: &'a [u8],
}

impl<'a> ByteReader<'a> {
    fn new(data: &'a [u8]) -> Self {
        Self { data }
    }

    fn u16_at(&self, offset: usize) -> Option<u16> {
        let bytes = self.data.get(offset..offset + 2)?;
        Some(u16::from_le_bytes([bytes[0], bytes[1]]))
    }

    fn u32_at(&self, offset: usize) -> Option<u32> {
        let bytes = self.data.get(offset..offset + 4)?;
        Some(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    fn u8_at(&self, offset: usize) -> Option<u8> {
        self.data.get(offset).copied()
    }

    fn bytes(&self, offset: u32, size: u32) -> Option<&'a [u8]> {
        let start = offset as usize;
        let end = start.checked_add(size as usize)?;
        self.data.get(start..end)
    }
}

pub fn read(path: &str) -> Result<Elf32File, String> {
    let data = std::fs::read(path).map_err(|e| format!("读取文件失败：{e}"))?;
    read_bytes(&data)
}

pub fn read_bytes(data: &[u8]) -> Result<Elf32File, String> {
    if data.len() < 52 {
        return Err("文件不是 ELF 格式。".to_string());
    }
    if data[0..4] != [0x7F, b'E', b'L', b'F'] {
        return Err("文件不是 ELF 格式。".to_string());
    }
    if data[4] != 1 {
        return Err("仅支持 ELF32 文件。".to_string());
    }
    if data[5] != 1 {
        return Err("仅支持 little-endian ELF 文件。".to_string());
    }

    let reader = ByteReader::new(data);
    let section_header_offset = reader.u32_at(0x20).ok_or("ELF 文件头不完整。")?;
    let section_header_size = reader.u16_at(0x2E).ok_or("ELF 文件头不完整。")? as u32;
    let section_header_count = reader.u16_at(0x30).ok_or("ELF 文件头不完整。")? as u32;
    let section_name_table_index = reader.u16_at(0x32).ok_or("ELF 文件头不完整。")? as usize;

    if section_header_size < 40 {
        return Err("ELF 节头大小无效。".to_string());
    }

    let sections = read_sections(&reader, section_header_offset, section_header_size, section_header_count)?;
    let section_name_table = if section_name_table_index < sections.len() {
        let header = sections[section_name_table_index];
        reader
            .bytes(header.offset, header.size)
            .ok_or("ELF 节名字表越界。")?
            .to_vec()
    } else {
        Vec::new()
    };

    let mut section_bytes = HashMap::new();
    for header in &sections {
        let name = read_null_terminated_string(&section_name_table, header.name_offset as usize);
        if !name.is_empty() {
            let bytes = reader.bytes(header.offset, header.size).ok_or("ELF 节内容越界。")?;
            section_bytes.insert(name, bytes.to_vec());
        }
    }

    let mut symbols = Vec::new();
    for header in sections.iter().filter(|s| s.section_type == 2 && s.entry_size > 0) {
        if header.link as usize >= sections.len() {
            continue;
        }
        let string_table_header = sections[header.link as usize];
        let string_table = reader
            .bytes(string_table_header.offset, string_table_header.size)
            .ok_or("ELF 字符串表越界。")?;
        let count = header.size / header.entry_size;

        for index in 0..count {
            let base = header.offset as usize + index as usize * header.entry_size as usize;
            let Some(name_offset) = reader.u32_at(base) else { break };
            let Some(address) = reader.u32_at(base + 4) else { break };
            let Some(size) = reader.u32_at(base + 8) else { break };
            let Some(info) = reader.u8_at(base + 12) else { break };
            let Some(section_index) = reader.u16_at(base + 14) else { break };

            let symbol_type = info & 0x0F;
            if symbol_type != 1 || section_index == 0 {
                continue;
            }

            let name = read_null_terminated_string(string_table, name_offset as usize);
            if name.trim().is_empty() {
                continue;
            }

            symbols.push(ElfObjectSymbol {
                name,
                address,
                size,
                bind: bind_name(info),
                sym_type: "OBJECT".to_string(),
                section_index,
            });
        }
    }

    symbols.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(Elf32File {
        object_symbols: symbols,
        sections: section_bytes,
    })
}

fn read_sections(
    reader: &ByteReader,
    offset: u32,
    size: u32,
    count: u32,
) -> Result<Vec<SectionHeader>, String> {
    let mut sections = Vec::with_capacity(count as usize);
    for index in 0..count {
        let base = offset as usize + index as usize * size as usize;
        let Some(name_offset) = reader.u32_at(base) else { break };
        let Some(section_type) = reader.u32_at(base + 4) else { break };
        let Some(section_offset) = reader.u32_at(base + 16) else { break };
        let Some(section_size) = reader.u32_at(base + 20) else { break };
        let Some(link) = reader.u32_at(base + 24) else { break };
        let Some(entry_size) = reader.u32_at(base + 36) else { break };

        sections.push(SectionHeader {
            name_offset,
            section_type,
            offset: section_offset,
            size: section_size,
            link,
            entry_size,
        });
    }
    Ok(sections)
}

fn read_null_terminated_string(table: &[u8], offset: usize) -> String {
    if offset >= table.len() {
        return String::new();
    }
    let end = table[offset..]
        .iter()
        .position(|byte| *byte == 0)
        .map(|position| offset + position)
        .unwrap_or(table.len());
    String::from_utf8_lossy(&table[offset..end]).to_string()
}

fn bind_name(info: u8) -> String {
    match info >> 4 {
        0 => "LOCAL",
        1 => "GLOBAL",
        2 => "WEAK",
        _ => "UNKNOWN",
    }
    .to_string()
}
