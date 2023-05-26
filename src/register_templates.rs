use crate::has_extension::has_extension;
use handlebars::Handlebars;
use log::{debug, error};
use std::path::Path;

pub(crate) fn register_templates(reg: &mut Handlebars, layout_path: &Path) {
    if let Ok(read_dir) = layout_path.read_dir() {
        for result in read_dir {
            if let Ok(entry) = result {
                let path = entry.path();

                if path.is_file() && has_extension(&path, "hbs") {
                    if let Some(os_name) = path.file_stem() && let Some(name) = os_name.to_str() {
                        debug!("Loading template {:?}…", name);

                        if let Err(err) = reg.register_template_file(name, path.as_path()) {
                            error!("Error while loading template {}", err);
                        }
                    }
                }
            }
        }
    }
}
