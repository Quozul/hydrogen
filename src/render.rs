use crate::collections::Collections;
use crate::front_matter::get_front_matter;
use crate::minifier::html::minify_html;
use handlebars::Handlebars;
use log::{debug, error};
use std::path::Path;

pub(crate) fn render_pages(
    reg: &Handlebars,
    root: &Path,
    input: &Path,
    out: &Path,
    collections: &Collections,
) {
    if let Ok(read_dir) = input.read_dir() {
        for result in read_dir {
            if let Ok(entry) = result {
                let source = entry.path();

                if source.is_dir() {
                    render_pages(reg, root, source.as_path(), out, collections);
                } else if source.is_file() {
                    debug!("Rendering page {:?}…", source);

                    let front_matter = get_front_matter(Some(collections), &source, root);

                    let destination = if front_matter.permalink.starts_with("/") {
                        out.join(&front_matter.permalink[1..])
                            .with_extension("html")
                    } else {
                        out.join(&front_matter.permalink).with_extension("html")
                    };

                    debug!("Writing to {:?}", destination);

                    match reg.render(front_matter.layout.as_str(), &front_matter) {
                        Ok(out_data) => {
                            if let Some(parent) = destination.parent() && !parent.exists() {
                                std::fs::create_dir_all(parent)
                                    .expect("Was not able to create parent directory.");
                            }

                            std::fs::write(destination, minify_html(out_data)).unwrap();
                        }
                        Err(err) => {
                            error!("Error while rendering file {}", err);
                        }
                    }
                }
            }
        }
    }
}
