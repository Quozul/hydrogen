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
