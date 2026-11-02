use std::path::PathBuf;

use a2l_editor_lib::model::{A2lDataType, SymbolNode};
use a2l_editor_lib::symbol;

struct Section {
    name: &'static str,
    section_type: u32,
    bytes: Vec<u8>,
    link: u32,
    entry_size: u32,
}

fn section(name: &'static str, section_type: u32, bytes: Vec<u8>) -> Section {
    Section {
        name,
        section_type,
        bytes,
        link: 0,
        entry_size: 0,
    }
}

fn write_uleb(out: &mut Vec<u8>, mut value: u32) {
    loop {
        let mut current = (value & 0x7F) as u8;
        value >>= 7;
        if value != 0 {
            current |= 0x80;
        }
        out.push(current);
        if value == 0 {
            break;
        }
    }
}

fn write_cstr(out: &mut Vec<u8>, value: &str) {
    out.extend_from_slice(value.as_bytes());
    out.push(0);
}

fn write_block1(out: &mut Vec<u8>, op: u8, value: u32) {
    let mut payload = vec![op];
    write_uleb(&mut payload, value);
    out.push(payload.len() as u8);
    out.extend_from_slice(&payload);
}

fn write_abbrev(out: &mut Vec<u8>, code: u32, tag: u32, has_children: bool, attributes: &[(u32, u32)]) {
    write_uleb(out, code);
    write_uleb(out, tag);
    out.push(if has_children { 1 } else { 0 });
    for (name, form) in attributes {
        write_uleb(out, *name);
        write_uleb(out, *form);
    }
    write_uleb(out, 0);
    write_uleb(out, 0);
}

fn build_union_fixture() -> (Vec<u8>, Vec<u8>) {
    let mut abbrev = Vec::new();
    write_abbrev(&mut abbrev, 1, 0x11, true, &[(0x03, 0x08)]);
    write_abbrev(&mut abbrev, 2, 0x24, false, &[(0x03, 0x08), (0x0B, 0x0B)]);
    write_abbrev(&mut abbrev, 3, 0x13, true, &[(0x03, 0x08), (0x0B, 0x0B)]);
    write_abbrev(&mut abbrev, 4, 0x17, true, &[(0x03, 0x08), (0x0B, 0x0B)]);
    write_abbrev(&mut abbrev, 5, 0x0D, false, &[(0x03, 0x08), (0x49, 0x13), (0x38, 0x0A)]);
    write_abbrev(&mut abbrev, 6, 0x34, false, &[(0x03, 0x08), (0x49, 0x13), (0x02, 0x0A)]);
    write_uleb(&mut abbrev, 0);

    let mut body = Vec::new();
    write_uleb(&mut body, 1);
    write_cstr(&mut body, "unit");
    let base_type_offset = (body.len() + 11) as u32;
    write_uleb(&mut body, 2);
    write_cstr(&mut body, "unsigned int");
    body.push(2);

    let struct_type_offset = (body.len() + 11) as u32;
    write_uleb(&mut body, 3);
    write_cstr(&mut body, "MotorState");
    body.push(4);
    write_uleb(&mut body, 5);
    write_cstr(&mut body, "value");
    let value_ref_position = body.len();
    body.extend_from_slice(&[0, 0, 0, 0]);
    write_block1(&mut body, 0x23, 0);

    let union_type_offset = (body.len() + 11) as u32;
    write_uleb(&mut body, 4);
    write_cstr(&mut body, "");
    body.push(2);
    write_uleb(&mut body, 5);
    write_cstr(&mut body, "raw");
    body.extend_from_slice(&base_type_offset.to_le_bytes());
    write_block1(&mut body, 0x23, 0);
    write_uleb(&mut body, 5);
    write_cstr(&mut body, "bits");
    let bits_ref_position = body.len();
    body.extend_from_slice(&[0, 0, 0, 0]);
    write_block1(&mut body, 0x23, 0);
    write_uleb(&mut body, 0);

    let bits_type_offset = (body.len() + 11) as u32;
    write_uleb(&mut body, 3);
    write_cstr(&mut body, "");
    body.push(2);
    write_uleb(&mut body, 5);
    write_cstr(&mut body, "bit0");
    body.extend_from_slice(&base_type_offset.to_le_bytes());
    write_block1(&mut body, 0x23, 0);
    write_uleb(&mut body, 5);
    write_cstr(&mut body, "bit1");
    body.extend_from_slice(&base_type_offset.to_le_bytes());
    write_block1(&mut body, 0x23, 1);
    write_uleb(&mut body, 0);
    write_uleb(&mut body, 0);

    write_uleb(&mut body, 6);
    write_cstr(&mut body, "motor");
    body.extend_from_slice(&struct_type_offset.to_le_bytes());
    body.push(5);
    body.push(0x03);
    body.extend_from_slice(&0xA000u32.to_le_bytes());
    write_uleb(&mut body, 0);

    patch_u32(&mut body, value_ref_position, union_type_offset);
    patch_u32(&mut body, bits_ref_position, bits_type_offset);

    let mut info = Vec::new();
    info.extend_from_slice(&((body.len() + 7) as u32).to_le_bytes());
    info.extend_from_slice(&4u16.to_le_bytes());
    info.extend_from_slice(&0u32.to_le_bytes());
    info.push(4);
    info.extend_from_slice(&body);

    (info, abbrev)
}

fn build_enum_fixture() -> (Vec<u8>, Vec<u8>) {
    let mut abbrev = Vec::new();
    write_abbrev(&mut abbrev, 1, 0x11, true, &[(0x03, 0x08)]);
    write_abbrev(&mut abbrev, 2, 0x24, false, &[(0x03, 0x08), (0x0B, 0x0B)]);
    write_abbrev(&mut abbrev, 3, 0x13, true, &[(0x03, 0x08), (0x0B, 0x0B)]);
    write_abbrev(&mut abbrev, 4, 0x0D, false, &[(0x03, 0x08), (0x49, 0x13), (0x38, 0x0A)]);
    write_abbrev(&mut abbrev, 5, 0x34, false, &[(0x03, 0x08), (0x49, 0x13), (0x02, 0x0A)]);
    write_abbrev(&mut abbrev, 6, 0x04, false, &[(0x03, 0x08), (0x0B, 0x0B), (0x49, 0x13)]);
    write_abbrev(&mut abbrev, 7, 0x28, false, &[(0x03, 0x08)]);
    write_uleb(&mut abbrev, 0);

    let mut body = Vec::new();
    write_uleb(&mut body, 1);
    write_cstr(&mut body, "unit");
    let base_type_offset = (body.len() + 11) as u32;
    write_uleb(&mut body, 2);
    write_cstr(&mut body, "unsigned int");
    body.push(2);

    let struct_type_offset = (body.len() + 11) as u32;
    write_uleb(&mut body, 3);
    write_cstr(&mut body, "MotorState");
    body.push(4);
    write_uleb(&mut body, 4);
    write_cstr(&mut body, "speed");
    body.extend_from_slice(&base_type_offset.to_le_bytes());
    write_block1(&mut body, 0x23, 0);
    write_uleb(&mut body, 4);
    write_cstr(&mut body, "state");
    let enum_ref_position = body.len();
    body.extend_from_slice(&[0, 0, 0, 0]);
    write_block1(&mut body, 0x23, 2);
    write_uleb(&mut body, 0);

    write_uleb(&mut body, 5);
    write_cstr(&mut body, "motor");
    body.extend_from_slice(&struct_type_offset.to_le_bytes());
    body.push(5);
    body.push(0x03);
    body.extend_from_slice(&0xA000u32.to_le_bytes());
    write_uleb(&mut body, 0);

    let enum_type_offset = (body.len() + 11) as u32;
    write_uleb(&mut body, 6);
    write_cstr(&mut body, "MotorStateEnum");
    body.push(2);
    body.extend_from_slice(&base_type_offset.to_le_bytes());
    write_uleb(&mut body, 7);
    write_cstr(&mut body, "STATE_IDLE");
    write_uleb(&mut body, 7);
    write_cstr(&mut body, "STATE_RUNNING");
    write_uleb(&mut body, 0);

    patch_u32(&mut body, enum_ref_position, enum_type_offset);

    let mut info = Vec::new();
    info.extend_from_slice(&((body.len() + 7) as u32).to_le_bytes());
    info.extend_from_slice(&4u16.to_le_bytes());
    info.extend_from_slice(&0u32.to_le_bytes());
    info.push(4);
    info.extend_from_slice(&body);

    (info, abbrev)
}

fn patch_u32(bytes: &mut [u8], position: usize, value: u32) {
    bytes[position..position + 4].copy_from_slice(&value.to_le_bytes());
}

fn put_u16(bytes: &mut [u8], position: usize, value: u16) {
    bytes[position..position + 2].copy_from_slice(&value.to_le_bytes());
}

fn put_u32(bytes: &mut [u8], position: usize, value: u32) {
    bytes[position..position + 4].copy_from_slice(&value.to_le_bytes());
}

fn build_elf(mut sections: Vec<Section>) -> Vec<u8> {
    sections.insert(
        0,
        Section {
            name: "",
            section_type: 0,
            bytes: Vec::new(),
            link: 0,
            entry_size: 0,
        },
    );

    let mut name_offsets = std::collections::HashMap::new();
    let mut shstrtab = vec![0u8];
    for entry in &sections {
        if entry.name.is_empty() {
            name_offsets.insert("", 0u32);
            continue;
        }
        name_offsets.insert(entry.name, shstrtab.len() as u32);
        shstrtab.extend_from_slice(entry.name.as_bytes());
        shstrtab.push(0);
    }
    name_offsets.insert(".shstrtab", shstrtab.len() as u32);
    shstrtab.extend_from_slice(b".shstrtab\0");
    let shstrtab_index = sections.len();
    sections.push(Section {
        name: ".shstrtab",
        section_type: 3,
        bytes: shstrtab,
        link: 0,
        entry_size: 0,
    });

    let mut offsets = vec![0u32; sections.len()];
    let mut cursor = 0x100u32;
    for index in 1..sections.len() {
        offsets[index] = cursor;
        cursor += sections[index].bytes.len() as u32;
        cursor = (cursor + 3) & !3;
    }
    let section_header_offset = cursor;
    let total = section_header_offset as usize + sections.len() * 40;
    let mut elf = vec![0u8; total];

    elf[0..4].copy_from_slice(&[0x7F, b'E', b'L', b'F']);
    elf[4] = 1;
    elf[5] = 1;
    elf[6] = 1;
    put_u32(&mut elf, 0x20, section_header_offset);
    put_u16(&mut elf, 0x2E, 40);
    put_u16(&mut elf, 0x30, sections.len() as u16);
    put_u16(&mut elf, 0x32, shstrtab_index as u16);

    for index in 1..sections.len() {
        let start = offsets[index] as usize;
        let end = start + sections[index].bytes.len();
        elf[start..end].copy_from_slice(&sections[index].bytes);
    }

    for (index, entry) in sections.iter().enumerate() {
        let base = section_header_offset as usize + index * 40;
        put_u32(&mut elf, base, name_offsets[entry.name]);
        put_u32(&mut elf, base + 4, entry.section_type);
        put_u32(&mut elf, base + 16, offsets[index]);
        put_u32(&mut elf, base + 20, entry.bytes.len() as u32);
        put_u32(&mut elf, base + 24, entry.link);
        put_u32(&mut elf, base + 36, entry.entry_size);
    }

    elf
}

fn build_symbol_table(name: &str, address: u32, size: u32) -> (Vec<u8>, Vec<u8>) {
    let mut string_table = vec![0u8];
    string_table.extend_from_slice(name.as_bytes());
    string_table.push(0);

    let mut symbols = vec![0u8; 16];
    symbols.extend_from_slice(&1u32.to_le_bytes());
    symbols.extend_from_slice(&address.to_le_bytes());
    symbols.extend_from_slice(&size.to_le_bytes());
    symbols.push(0x11);
    symbols.push(0);
    symbols.extend_from_slice(&1u16.to_le_bytes());

    (symbols, string_table)
}

fn write_temp_elf(elf: Vec<u8>, prefix: &str) -> PathBuf {
    let unique = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!("a2l-editor-{prefix}-{}-{unique}.out", std::process::id()));
    std::fs::write(&path, elf).unwrap();
    path
}

fn debug_elf(info: Vec<u8>, abbrev: Vec<u8>, symbol_name: &str, address: u32, size: u32) -> Vec<u8> {
    let (symbol_table, string_table) = build_symbol_table(symbol_name, address, size);
    let mut symtab = section(".symtab", 2, symbol_table);
    symtab.link = 3;
    symtab.entry_size = 16;

    build_elf(vec![
        section(".text", 1, Vec::new()),
        symtab,
        section(".strtab", 3, string_table),
        section(".debug_info", 1, info),
        section(".debug_abbrev", 1, abbrev),
        section(".debug_str", 1, Vec::new()),
    ])
}

#[test]
fn imports_union_and_nested_struct_members() {
    let (info, abbrev) = build_union_fixture();
    let path = write_temp_elf(debug_elf(info, abbrev, "motor", 0xA000, 4), "union");

    let symbols = symbol::import(path.to_str().unwrap()).unwrap();
    let motor = symbols.iter().find(|node| node.name == "motor").expect("motor node");
    assert!(!motor.is_leaf);

    let value = motor.children.iter().find(|node| node.name == "value").expect("value member");
    assert!(!value.is_leaf);
    assert_eq!(value.path, "motor.value");
    assert_eq!(value.address, 0xA000);

    let raw = value.children.iter().find(|node| node.name == "raw").expect("raw member");
    assert!(raw.is_leaf);
    assert_eq!(raw.path, "motor.value.raw");
    assert_eq!(raw.address, 0xA000);
    assert_eq!(raw.suggested_data_type, A2lDataType::UWORD);
    assert!(!raw.requires_type_confirmation);

    let bits = value.children.iter().find(|node| node.name == "bits").expect("bits member");
    assert!(!bits.is_leaf);
    let bit0 = bits.children.iter().find(|node| node.name == "bit0").expect("bit0");
    let bit1 = bits.children.iter().find(|node| node.name == "bit1").expect("bit1");
    assert_eq!(bit0.path, "motor.value.bits.bit0");
    assert_eq!(bit0.address, 0xA000);
    assert_eq!(bit1.address, 0xA001);

    std::fs::remove_file(path).ok();
}

#[test]
fn imports_enum_defined_after_variable() {
    let (info, abbrev) = build_enum_fixture();
    let path = write_temp_elf(debug_elf(info, abbrev, "motor", 0xA000, 4), "enum");

    let symbols = symbol::import(path.to_str().unwrap()).unwrap();
    let motor = symbols.iter().find(|node| node.name == "motor").expect("motor node");
    let speed = motor.children.iter().find(|node| node.name == "speed").expect("speed");
    let state = motor.children.iter().find(|node| node.name == "state").expect("state");

    assert_eq!(speed.suggested_data_type, A2lDataType::UWORD);
    assert!(state.is_leaf);
    assert_eq!(state.path, "motor.state");
    assert_eq!(state.address, 0xA002);

    std::fs::remove_file(path).ok();
}

#[test]
fn imports_real_ti_c2000_out_file() {
    let path = format!(
        "{}/tests/fixtures/TI2800157DemoProject.out",
        env!("CARGO_MANIFEST_DIR")
    );
    let symbols = symbol::import(&path).expect("real TI .out should import");
    assert!(!symbols.is_empty(), "expected at least one symbol");

    fn count_leaves(nodes: &[SymbolNode]) -> usize {
        nodes
            .iter()
            .map(|node| {
                if node.is_leaf {
                    1
                } else {
                    count_leaves(&node.children)
                }
            })
            .sum()
    }

    let leaves = count_leaves(&symbols);
    let expandable = symbols.iter().any(|node| !node.children.is_empty());
    eprintln!(
        "real out: top-level={} leaves={} expandable={}",
        symbols.len(),
        leaves,
        expandable
    );

    assert!(leaves > 0, "expected leaf symbols");
    assert!(expandable, "expected DWARF-derived expandable symbols");
}

#[test]
fn falls_back_to_flat_elf_symbols() {
    let (symbol_table, string_table) = build_symbol_table("speed", 0xC000, 2);
    let mut symtab = section(".symtab", 2, symbol_table);
    symtab.link = 3;
    symtab.entry_size = 16;

    let elf = build_elf(vec![
        section(".text", 1, Vec::new()),
        symtab,
        section(".strtab", 3, string_table),
    ]);
    let path = write_temp_elf(elf, "flat");

    let symbols = symbol::import(path.to_str().unwrap()).unwrap();
    let speed: &SymbolNode = symbols.iter().find(|node| node.name == "speed").expect("speed node");
    assert!(speed.is_leaf);
    assert!(speed.children.is_empty());
    assert_eq!(speed.path, "speed");
    assert_eq!(speed.address, 0xC000);

    std::fs::remove_file(path).ok();
}
