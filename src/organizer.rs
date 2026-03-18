use std::path::{Path, PathBuf};
use std::fs;
use std::env;
use regex::Regex;
use chrono::NaiveDate;
use once_cell::sync::Lazy;

use crate::classifier::{determinar_categoria_pdf, determinar_categoria_extension, get_managed_folders};

static PATRON_FECHA: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(\d{14})_?(.+)$").unwrap());

pub fn get_base_dir() -> PathBuf {
    if let Ok(exe_path) = env::current_exe() {
        if let Some(parent) = exe_path.parent() {
            return parent.to_path_buf();
        }
    }
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn parse_fecha(timestamp_str: &str) -> Option<(i32, u32, u32)> {
    if timestamp_str.len() >= 8 {
        let date_str = &timestamp_str[..8];
        if let Ok(date) = NaiveDate::parse_from_str(date_str, "%Y%m%d") {
            use chrono::Datelike;
            return Some((date.year(), date.month(), date.day()));
        }
    }
    None
}

fn nombre_carpeta_mes(año: i32, mes: u32) -> String {
    let meses = [
        "Enero", "Febrero", "Marzo", "Abril", "Mayo", "Junio",
        "Julio", "Agosto", "Septiembre", "Octubre", "Noviembre", "Diciembre"
    ];
    let idx = (mes as usize).saturating_sub(1).min(11);
    format!("{} {}", meses[idx], año)
}

pub fn resolver_colision(path: PathBuf) -> PathBuf {
    if !path.exists() {
        return path;
    }
    let stem = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
    let ext = path.extension().map(|e| format!(".{}", e.to_string_lossy())).unwrap_or_default();
    let parent = path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
    
    let mut counter = 2;
    loop {
        let new_name = format!("{}_{}{}", stem, counter, ext);
        let new_path = parent.join(new_name);
        if !new_path.exists() {
            return new_path;
        }
        counter += 1;
    }
}

pub fn escanear_archivos(base_dir: &Path) -> Vec<(PathBuf, PathBuf)> {
    let mut trabajos = Vec::new();
    let current_exe = env::current_exe().ok();
    
    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            if let Some(ref exe_path) = current_exe {
                if &path == exe_path {
                    continue;
                }
            }
            
            let filename = match path.file_name() {
                Some(n) => n.to_string_lossy().to_string(),
                None => continue,
            };
            
            let suffix = path.extension().map(|e| format!(".{}", e.to_string_lossy().to_lowercase())).unwrap_or_default();
            let mut destino_carpeta: Option<PathBuf> = None;
            
            if suffix == ".pdf" {
                let categoria = determinar_categoria_pdf(&filename);
                let mut dir = base_dir.join(categoria);
                
                if let Some(caps) = PATRON_FECHA.captures(&filename) {
                    if let Some(timestamp) = caps.get(1) {
                        if let Some((year, month, day)) = parse_fecha(timestamp.as_str()) {
                            dir = dir.join(nombre_carpeta_mes(year, month)).join(format!("{:02}", day));
                        }
                    }
                }
                destino_carpeta = Some(dir);
            } else {
                if let Some(categoria) = determinar_categoria_extension(&suffix) {
                    destino_carpeta = Some(base_dir.join(categoria));
                }
            }
            
            if let Some(dest_dir) = destino_carpeta {
                let final_dest = resolver_colision(dest_dir.join(&filename));
                trabajos.append(&mut vec![(path, final_dest)]);
            }
        }
    }
    trabajos
}

pub fn hay_archivos_para_revertir(base_dir: &Path) -> usize {
    let mut total = 0;
    for cat in get_managed_folders() {
        let cat_dir = base_dir.join(cat);
        if cat_dir.exists() {
            for entry in walkdir::WalkDir::new(cat_dir).into_iter().filter_map(|e| e.ok()) {
                if entry.path().is_file() {
                    total += 1;
                }
            }
        }
    }
    total
}

pub fn recolectar_archivos_para_revertir(base_dir: &Path) -> Vec<PathBuf> {
    let mut archivos = Vec::new();
    for cat in get_managed_folders() {
        let cat_dir = base_dir.join(cat);
        if cat_dir.exists() {
            for entry in walkdir::WalkDir::new(cat_dir).into_iter().filter_map(|e| e.ok()) {
                if entry.path().is_file() {
                    archivos.push(entry.path().to_path_buf());
                }
            }
        }
    }
    archivos
}

pub fn limpiar_todas_las_carpetas_vacias(base_dir: &Path) {
    for cat in get_managed_folders() {
        let cat_dir = base_dir.join(cat);
        if !cat_dir.exists() {
            continue;
        }
        
        let mut dirs: Vec<PathBuf> = walkdir::WalkDir::new(&cat_dir)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
            .map(|e| e.path().to_path_buf())
            .collect();
            
        // Sort in reverse order so we delete deepest children first
        dirs.sort_by(|a, b| b.components().count().cmp(&a.components().count()));
        
        for dir in dirs {
            let _ = fs::remove_dir(dir); // Ignore errors (folders might not be empty)
        }
    }
}
