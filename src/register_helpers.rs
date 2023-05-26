use crate::path_extension::{file_stem, has_extension};
use handlebars::Handlebars;
use log::{error, warn};
use std::path::PathBuf;

pub(crate) fn register_helpers(reg: &mut Handlebars, layout_path: PathBuf) {
    if let Ok(read_dir) = layout_path.read_dir() {
        for result in read_dir {
            if let Ok(entry) = result {
                let path = entry.path();

                if path.is_file() && has_extension(&path, "rhai") && let Some(name) = file_stem(&path) {
                    if let Err(err) = reg.register_script_helper_file(name.as_str(), path) {
                        error!("An error has occurred while loading script helper {}", err);
                    }
                } else {
                    warn!(
                        "File {:?} is not a Handlebar template and will be ignored.",
                        path
                    );
                }
            }
        }
    }
}
