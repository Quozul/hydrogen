use crate::minifier::css::minify_css;
use log::error;
use std::path::Path;

fn handle_asset(extension: &str, source: &Path, destination: &Path) {
    match extension {
        "css" => match std::fs::read_to_string(source) {
            Ok(input) => {
                if let Err(err) = std::fs::write(destination, minify_css(input)) {
                    error!("Cannot write asset {}", err);
                }
            }
            Err(err) => error!("Cannot open asset {}", err),
        },

        "scss" | "sass" => match grass::from_path(source, &grass::Options::default()) {
            Ok(output) => {
                if let Err(err) =
                    std::fs::write(destination.with_extension("css"), minify_css(output))
                {
                    error!("Cannot write asset {}", err);
                }
            }
            Err(err) => error!("Cannot open asset {}", err),
        },

        _ => {
            std::fs::copy(source, destination).unwrap();
        }
    }
}

pub(crate) fn copy_assets(assets: &Path, root: &Path, output: &Path) {
    if let Ok(read_dir) = assets.read_dir() {
        for entry in read_dir {
            if let Ok(asset) = entry {
                let source = asset.path();

                if source.is_file() {
                    if let Ok(source_no_prefix) = source.strip_prefix(root) {
                        let destination = output.join(source_no_prefix);
                        if let Some(parent) = destination.parent() && !parent.exists() {
                            std::fs::create_dir_all(parent).unwrap();
                        }
                        if let Some(os_extension) = source.extension() && let Some(extension) = os_extension.to_str() {
                            handle_asset(extension, &source, &destination);
                        }
                    }
                } else if source.is_dir() {
                    copy_assets(source.as_path(), root, output)
                }
            }
        }
    }
}
