use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Serialize, Deserialize)]
pub struct DirEntry {
    pub name: String,
    pub is_directory: bool,
}

#[derive(Serialize)]
pub struct ReadDirResult {
    pub ok: bool,
    pub entries: Vec<DirEntry>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct JsonFileResult {
    pub path: String,
    pub data: Option<serde_json::Value>,
    pub error: Option<String>,
}

#[tauri::command]
fn get_default_quests_root() -> Option<String> {
    // Try to find quests/ relative to the executable or working directory
    let exe_path = std::env::current_exe().ok()?;
    let exe_dir = exe_path.parent()?;
    let cwd = std::env::current_dir().ok();

    let mut candidates: Vec<PathBuf> = vec![
        // Production: exe is in config-editor-tauri/, quests is at ../quests
        exe_dir.join("../quests"),
        // Dev: exe is in src-tauri/target/debug/, quests is at ../../../quests
        exe_dir.join("../../../quests"),
        // Dev alternative: exe in target/debug, project root is ../../
        exe_dir.join("../../../../quests"),
    ];

    // Also try relative to cwd (when launched from config-editor-tauri/)
    if let Some(ref cwd) = cwd {
        candidates.push(cwd.join("../quests"));
        candidates.push(cwd.join("quests"));
    }

    for candidate in candidates {
        if candidate.is_dir() {
            return candidate.canonicalize().ok()
                .map(|p| p.to_string_lossy().to_string());
        }
    }
    None
}


#[tauri::command]
fn read_dir(dir_path: String) -> ReadDirResult {
    match fs::read_dir(&dir_path) {
        Ok(entries) => {
            let mut result = Vec::new();
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                let is_directory = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                result.push(DirEntry { name, is_directory });
            }
            ReadDirResult { ok: true, entries: result, error: None }
        }
        Err(e) => ReadDirResult { ok: false, entries: vec![], error: Some(e.to_string()) },
    }
}

#[tauri::command]
fn path_exists(file_path: String) -> bool {
    Path::new(&file_path).exists()
}

#[tauri::command]
fn read_text_file(file_path: String) -> Result<String, String> {
    fs::read_to_string(&file_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn write_text_file(file_path: String, content: String) -> Result<(), String> {
    fs::write(&file_path, &content).map_err(|e| e.to_string())
}

#[tauri::command]
fn copy_file(src: String, dest: String) -> Result<(), String> {
    fs::copy(&src, &dest).map(|_| ()).map_err(|e| e.to_string())
}

#[tauri::command]
fn join_path(parts: Vec<String>) -> String {
    let mut path = PathBuf::new();
    for part in parts {
        path.push(part);
    }
    path.to_string_lossy().to_string()
}


#[tauri::command]
fn find_json_files(dir_path: String) -> Vec<String> {
    let mut results = Vec::new();
    for entry in WalkDir::new(&dir_path).into_iter().flatten() {
        if entry.file_type().is_file() {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    results.push(entry.path().to_string_lossy().to_string());
                }
            }
        }
    }
    results
}

#[tauri::command]
fn read_json_files(file_paths: Vec<String>) -> Vec<JsonFileResult> {
    file_paths.iter().map(|path| {
        match fs::read_to_string(path) {
            Ok(mut content) => {
                // Strip UTF-8 BOM
                if content.starts_with('\u{feff}') {
                    content = content[3..].to_string();
                }
                match serde_json::from_str::<serde_json::Value>(&content) {
                    Ok(data) => JsonFileResult { path: path.clone(), data: Some(data), error: None },
                    Err(e) => JsonFileResult { path: path.clone(), data: None, error: Some(e.to_string()) },
                }
            }
            Err(e) => JsonFileResult { path: path.clone(), data: None, error: Some(e.to_string()) },
        }
    }).collect()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            get_default_quests_root,
            read_dir,
            path_exists,
            read_text_file,
            write_text_file,
            copy_file,
            join_path,
            find_json_files,
            read_json_files,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
