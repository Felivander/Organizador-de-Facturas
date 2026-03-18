use std::collections::{HashMap, HashSet};
use once_cell::sync::Lazy;
use regex::Regex;

pub static CATEGORIAS: Lazy<Vec<(&'static str, Regex)>> = Lazy::new(|| {
    vec![
        ("⚫ Transferencias", Regex::new(r"(?i)Transferencia_").unwrap()),
        ("⚫ Depósitos", Regex::new(r"(?i)BoletaDeposito_").unwrap()),
        ("⚫ Comprobantes Varios", Regex::new(r"(?i)comprobante-operacion-").unwrap()),
        ("⚫ Planillas de Carga", Regex::new(r"(?i)planilladecarga").unwrap()),
        ("⚫ Detalles Cargas y Descargas", Regex::new(r"(?i)detalle_de_cargas-descargas").unwrap()),
        ("⚫ Detalles de Liquidaciones", Regex::new(r"(?i)detalle_de_liquidaciones").unwrap()),
        ("⚫ Recibos de Clientes", Regex::new(r"(?i)recibocliente").unwrap()),
        ("⚫ Caja y Bancos", Regex::new(r"(?i)PlanillaCaja|saldouncliente|Ultimos_Movimientos|Mayor administrativo").unwrap()),
        ("⚫ Liquidaciones", Regex::new(r"(?i)liquidacion_").unwrap()),
        ("⚫ Anticipos", Regex::new(r"(?i)anticipo_").unwrap()),
        ("⚫ Remitos y Consignaciones", Regex::new(r"(?i)REMITOS|CONSIGNACIONES|Remito|DVCSG|RMCSG|REMCC").unwrap()),
        ("⚫ Presupuestos y Minutas", Regex::new(r"(?i)PRESU").unwrap()),
        ("⚫ Recursos Humanos", Regex::new(r"(?i)(?:^|[^A-Za-z])CV(?:[^A-Za-z]|$)|Curriculum|Resume|Descripcion de puesto").unwrap()),
        ("⚫ Facturas y Notas", Regex::new(r"(?i)FC__|NC__|Factura|FCVTA|PRVTA|FCCSG|DVVTA|AFIP|Recibo|FACTURA_PRESUPUESTO|Fact\s|Nota de credito").unwrap()),
    ]
});

pub static CATEGORIAS_EXTENSION: Lazy<HashMap<&'static str, HashSet<&'static str>>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("⚫ Imágenes", HashSet::from([".jpg", ".jpeg", ".png", ".gif", ".bmp", ".webp", ".svg", ".tiff", ".tif", ".heic", ".heif"]));
    map.insert("⚫ Audio", HashSet::from([".mp3", ".wav", ".ogg", ".flac", ".m4a", ".aac", ".wma", ".opus", ".aiff"]));
    map.insert("⚫ Videos", HashSet::from([".mp4", ".mkv", ".avi", ".mov", ".wmv", ".flv", ".webm", ".m4v"]));
    map.insert("⚫ Planillas Excel", HashSet::from([".xls", ".xlsx", ".csv", ".ods", ".xlsm"]));
    map.insert("⚫ Presentaciones", HashSet::from([".ppt", ".pptx"]));
    map.insert("⚫ Archivos de Texto", HashSet::from([".doc", ".docx", ".txt", ".rtf"]));
    map.insert("⚫ Ejecutables", HashSet::from([".exe", ".msi", ".bat", ".cmd"]));
    map.insert("⚫ Comprimidos", HashSet::from([".zip", ".rar", ".7z", ".tar", ".gz"]));
    map.insert("⚫ Ebooks", HashSet::from([".epub", ".mobi", ".azw", ".azw3", ".lit", ".lrf", ".djvu"]));
    map.insert("⚫ Documentos Varios", HashSet::from([".pdf"]));
    map
});

pub fn get_managed_folders() -> Vec<&'static str> {
    let mut folders = Vec::new();
    for (name, _) in CATEGORIAS.iter() {
        folders.push(*name);
    }
    for name in CATEGORIAS_EXTENSION.keys() {
        folders.push(*name);
    }
    folders
}

pub fn determinar_categoria_pdf(file_name: &str) -> &'static str {
    for (name, re) in CATEGORIAS.iter() {
        if re.is_match(file_name) {
            return name;
        }
    }
    "⚫ Documentos Varios"
}

pub fn determinar_categoria_extension(suffix: &str) -> Option<&'static str> {
    for (name, exts) in CATEGORIAS_EXTENSION.iter() {
        if *name != "⚫ Documentos Varios" && exts.contains(suffix) {
            return Some(name);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex_compilation_and_cv() {
        assert_eq!(determinar_categoria_pdf("mi_cv_juan.pdf"), "⚫ Recursos Humanos");
        assert_eq!(determinar_categoria_pdf("CV Luciana Pierini.2026.pdf"), "⚫ Recursos Humanos");
        assert_eq!(determinar_categoria_pdf("CV-JuanPerez.pdf"), "⚫ Recursos Humanos");
        assert_eq!(determinar_categoria_pdf("2026_Comprobante-FCVTA-38.pdf"), "⚫ Facturas y Notas");
    }
}
