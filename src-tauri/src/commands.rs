use crate::address;
use crate::model::*;
use crate::project_xml;
use crate::symbol;
use crate::{a2l, factory, validate};

#[tauri::command]
pub fn new_project() -> A2lProject {
    A2lProject::default()
}

#[tauri::command]
pub fn load_project(path: String) -> Result<A2lProject, String> {
    let xml = std::fs::read_to_string(&path).map_err(|e| format!("读取工程文件失败：{e}"))?;
    project_xml::from_xml(&xml)
}

#[tauri::command]
pub fn save_project(path: String, mut project: A2lProject) -> Result<A2lProject, String> {
    if !project.out_file_path.trim().is_empty() {
        if let Ok(absolute) = std::path::absolute(&project.out_file_path) {
            project.out_file_path = absolute.to_string_lossy().to_string();
        }
    }
    std::fs::write(&path, project_xml::to_xml(&project)).map_err(|e| format!("保存工程失败：{e}"))?;
    Ok(project)
}

#[tauri::command]
pub fn load_symbols(path: String) -> Result<Vec<SymbolNode>, String> {
    symbol::import(&path)
}

#[tauri::command]
pub fn preview_a2l(project: A2lProject) -> String {
    a2l::write(&project)
}

#[tauri::command]
pub fn validate_project(project: A2lProject) -> ValidationResult {
    validate::validate(&project)
}

#[tauri::command]
pub fn export_a2l(path: String, project: A2lProject) -> Result<(), String> {
    std::fs::write(&path, a2l::write(&project)).map_err(|e| format!("导出 A2L 失败：{e}"))
}

#[tauri::command]
pub fn detect_address_changes(project: A2lProject, symbols: Vec<SymbolNode>) -> Vec<AddressChange> {
    address::detect(&project, &symbols)
}

#[tauri::command]
pub fn apply_address_changes(mut project: A2lProject, changes: Vec<AddressChange>) -> A2lProject {
    address::apply(&mut project, &changes);
    project
}

#[tauri::command]
pub fn create_measurement(symbol: SymbolNode, address_mode: AddressMode) -> Measurement {
    factory::create_measurement(&symbol, address_mode)
}

#[tauri::command]
pub fn create_characteristic(symbol: SymbolNode, address_mode: AddressMode) -> Characteristic {
    factory::create_characteristic(&symbol, address_mode)
}

#[tauri::command]
pub fn file_exists(path: String) -> bool {
    std::path::Path::new(&path).is_file()
}
