use std::collections::{HashMap, HashSet};

use crate::dwarf::{self, DwarfType};
use crate::elf::{self, ElfObjectSymbol};
use crate::model::{A2lDataType, SymbolNode};
use crate::typemap;

pub fn import(path: &str) -> Result<Vec<SymbolNode>, String> {
    let elf_file = elf::read(path)?;

    if let Some(model) = dwarf::try_read(&elf_file.sections) {
        let tree = build_tree(&model, &elf_file.object_symbols);
        if !tree.is_empty() {
            return Ok(tree);
        }
    }

    let mut symbols: Vec<SymbolNode> = elf_file
        .object_symbols
        .iter()
        .filter(|symbol| symbol.address != 0 && !symbol.name.starts_with('$'))
        .map(to_symbol_node)
        .collect();
    symbols.sort_by(|a, b| a.path.cmp(&b.path));
    Ok(symbols)
}

fn to_symbol_node(symbol: &ElfObjectSymbol) -> SymbolNode {
    let mapped = typemap::map_c28x("", Some(symbol.size));
    SymbolNode {
        name: symbol.name.clone(),
        path: symbol.name.clone(),
        address: symbol.address,
        size: symbol.size,
        ctype: String::new(),
        source_file: String::new(),
        source_line: None,
        is_leaf: true,
        requires_type_confirmation: mapped.requires_confirmation,
        suggested_data_type: mapped.data_type,
        children: Vec::new(),
    }
}

fn build_tree(model: &dwarf::DwarfModel, object_symbols: &[ElfObjectSymbol]) -> Vec<SymbolNode> {
    let object_names: HashSet<&str> = object_symbols
        .iter()
        .filter(|symbol| symbol.address != 0 && !symbol.name.starts_with('$'))
        .map(|symbol| symbol.name.as_str())
        .collect();

    let mut nodes: Vec<SymbolNode> = model
        .variables
        .iter()
        .filter(|variable| object_names.contains(variable.name.as_str()))
        .filter_map(|variable| {
            let type_ = model.types.get(&variable.type_offset)?;
            build_node(&variable.name, &variable.name, variable.address, type_, &model.types)
        })
        .collect();

    nodes.sort_by(|a, b| a.path.cmp(&b.path));
    nodes
}

fn build_node(
    name: &str,
    path: &str,
    address: u32,
    type_: &DwarfType,
    types: &HashMap<u32, DwarfType>,
) -> Option<SymbolNode> {
    let resolved = resolve_alias(type_, types);

    if let DwarfType::Array {
        element_type_offset,
        count,
        byte_size,
    } = resolved
    {
        let Some(element_type) = types.get(element_type_offset) else {
            return Some(leaf_node(name, path, address, resolved));
        };
        let element_type = resolve_alias(element_type, types);
        let element_size = element_type.byte_size();

        let children = (0..*count)
            .map(|index| {
                let element_path = format!("{path}[{index}]");
                let element_address = address.saturating_add(index.saturating_mul(element_size));
                build_node(&element_path, &element_path, element_address, element_type, types)
            })
            .collect::<Option<Vec<_>>>()?;

        return Some(SymbolNode {
            name: name.to_string(),
            path: path.to_string(),
            address,
            size: *byte_size,
            ctype: String::new(),
            source_file: String::new(),
            source_line: None,
            is_leaf: false,
            requires_type_confirmation: false,
            suggested_data_type: A2lDataType::UWORD,
            children,
        });
    }

    if let DwarfType::Struct {
        name: struct_name,
        byte_size,
        members,
    } = resolved
    {
        let children = members
            .iter()
            .filter(|member| types.contains_key(&member.type_offset))
            .map(|member| {
                build_node(
                    &member.name,
                    &format!("{path}.{}", member.name),
                    address.saturating_add(member.offset),
                    &types[&member.type_offset],
                    types,
                )
            })
            .collect::<Option<Vec<_>>>()?;

        return Some(SymbolNode {
            name: name.to_string(),
            path: path.to_string(),
            address,
            size: *byte_size,
            ctype: struct_name.clone(),
            source_file: String::new(),
            source_line: None,
            is_leaf: false,
            requires_type_confirmation: false,
            suggested_data_type: A2lDataType::UWORD,
            children,
        });
    }

    Some(leaf_node(name, path, address, resolved))
}

fn leaf_node(name: &str, path: &str, address: u32, type_: &DwarfType) -> SymbolNode {
    let mapped = typemap::map_c28x(type_.name(), Some(type_.byte_size()));
    SymbolNode {
        name: name.to_string(),
        path: path.to_string(),
        address,
        size: type_.byte_size(),
        ctype: type_.name().to_string(),
        source_file: String::new(),
        source_line: None,
        is_leaf: true,
        requires_type_confirmation: mapped.requires_confirmation,
        suggested_data_type: mapped.data_type,
        children: Vec::new(),
    }
}

fn resolve_alias<'a>(mut type_: &'a DwarfType, types: &'a HashMap<u32, DwarfType>) -> &'a DwarfType {
    let mut depth = 0;
    while let DwarfType::Alias { type_offset, .. } = type_ {
        let Some(resolved) = types.get(type_offset) else {
            break;
        };
        type_ = resolved;
        depth += 1;
        if depth > 32 {
            break;
        }
    }
    type_
}
