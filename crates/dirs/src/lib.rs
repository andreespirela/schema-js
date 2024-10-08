use std::path::PathBuf;

pub const BASE_SCHEME_JS_FOLDER: &str = ".scheme-js";

pub fn get_base_path(base_path: Option<PathBuf>) -> PathBuf {
    base_path.unwrap_or_else(|| dirs::data_dir().unwrap().join(BASE_SCHEME_JS_FOLDER))
}
pub fn create_scheme_js_folder(base_path: Option<PathBuf>) {
    let paths = [
        get_base_path(base_path.clone()),
        get_base_path(base_path).join("dbs"),
    ]
    .into_iter();

    for path in paths {
        if !path.exists() {
            let _ = std::fs::create_dir(&path);
        }
    }
}

pub fn create_scheme_js_db(base_path: Option<PathBuf>, db_name: &str) -> PathBuf {
    let path = get_base_path(base_path).join("dbs").join(db_name);

    if !path.exists() {
        let _ = std::fs::create_dir(path.clone());
    }

    path
}

pub fn create_schema_js_table(
    base_path: Option<PathBuf>,
    db_name: &str,
    table_name: &str,
) -> PathBuf {
    let path = get_base_path(base_path)
        .join("dbs")
        .join(db_name)
        .join(table_name);

    if !path.exists() {
        let _ = std::fs::create_dir(path.clone());
    }

    path
}

pub fn create_indx_folder(base_path: Option<PathBuf>, db_name: &str, table_name: &str) -> PathBuf {
    let path = get_base_path(base_path)
        .join("dbs")
        .join(db_name)
        .join(table_name)
        .join("indxs");

    if !path.exists() {
        let _ = std::fs::create_dir(path.clone());
    }

    path
}
