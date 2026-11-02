pub mod a2l;
pub mod address;
pub mod commands;
pub mod dwarf;
pub mod elf;
pub mod factory;
pub mod model;
pub mod project_xml;
pub mod symbol;
pub mod typemap;
pub mod validate;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::new_project,
            commands::load_project,
            commands::save_project,
            commands::load_symbols,
            commands::preview_a2l,
            commands::validate_project,
            commands::export_a2l,
            commands::detect_address_changes,
            commands::apply_address_changes,
            commands::create_measurement,
            commands::create_characteristic,
            commands::file_exists,
        ])
        .run(tauri::generate_context!())
        .expect("error while building tauri application");
}
