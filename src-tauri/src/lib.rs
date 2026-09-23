use std::fs;
use std::path::Path;

// 将文本内容写入指定路径（导出 .md / .doc / .html）
#[tauri::command]
fn save_text_file(path: &str, contents: &str) -> Result<(), String> {
    fs::write(Path::new(path), contents).map_err(|e| e.to_string())
}

// 将二进制内容写入指定路径（导出 .png）
#[tauri::command]
fn save_binary_file(path: &str, bytes: Vec<u8>) -> Result<(), String> {
    fs::write(Path::new(path), &bytes).map_err(|e| e.to_string())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            save_text_file,
            save_binary_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
