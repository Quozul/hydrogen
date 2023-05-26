use std::path::Path;

pub(crate) fn has_extension(path: &Path, format: &str) -> bool {
    if let Some(os_extension) = path.extension() && os_extension == format {
        true
    } else {
        false
    }
}

pub(crate) fn get_extension(path: &Path) -> Option<String> {
    if let Some(os_extension) = path.extension() && let Some(extension) = os_extension.to_str() {
        Some(String::from(extension))
    } else {
        None
    }
}

pub(crate) fn file_stem(path: &Path) -> Option<String> {
    if let Some(os_file_stem) = path.file_stem() && let Some(file_stem) = os_file_stem.to_str() {
        Some(String::from(file_stem))
    } else {
        None
    }
}
