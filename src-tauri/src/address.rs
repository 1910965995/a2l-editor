use std::collections::HashMap;

use crate::model::{A2lProject, AddressChange, AddressMode, SymbolNode};

pub fn to_ecu_address(elf_address: u32, mode: AddressMode) -> u32 {
    match mode {
        AddressMode::WordAddress => elf_address.saturating_mul(2),
        AddressMode::Raw | AddressMode::ByteAddress => elf_address,
    }
}

pub fn detect(project: &A2lProject, symbols: &[SymbolNode]) -> Vec<AddressChange> {
    let symbol_map = flatten_symbols(symbols);
    let mode = project.xcp.address_mode;
    let mut changes = Vec::new();

    for measurement in &project.measurements {
        if let Some(address) = symbol_map.get(&measurement.name) {
            let new_address = to_ecu_address(*address, mode);
            if new_address != measurement.ecu_address {
                changes.push(AddressChange {
                    name: measurement.name.clone(),
                    old_address: measurement.ecu_address,
                    new_address,
                });
            }
        }
    }

    for characteristic in &project.characteristics {
        if let Some(address) = symbol_map.get(&characteristic.name) {
            let new_address = to_ecu_address(*address, mode);
            if new_address != characteristic.ecu_address {
                changes.push(AddressChange {
                    name: characteristic.name.clone(),
                    old_address: characteristic.ecu_address,
                    new_address,
                });
            }
        }
    }

    changes
}

pub fn apply(project: &mut A2lProject, changes: &[AddressChange]) {
    let change_map: HashMap<&str, u32> = changes
        .iter()
        .map(|change| (change.name.as_str(), change.new_address))
        .collect();

    for measurement in &mut project.measurements {
        if let Some(address) = change_map.get(measurement.name.as_str()) {
            measurement.ecu_address = *address;
        }
    }

    for characteristic in &mut project.characteristics {
        if let Some(address) = change_map.get(characteristic.name.as_str()) {
            characteristic.ecu_address = *address;
        }
    }
}

pub fn flatten_symbols(symbols: &[SymbolNode]) -> HashMap<String, u32> {
    let mut result = HashMap::new();
    for node in symbols {
        collect_leaves(node, &mut result);
    }
    result
}

fn collect_leaves(node: &SymbolNode, result: &mut HashMap<String, u32>) {
    if node.is_leaf {
        result.insert(node.path.clone(), node.address);
        return;
    }

    for child in &node.children {
        collect_leaves(child, result);
    }
}
