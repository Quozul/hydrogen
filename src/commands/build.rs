use crate::assets::copy_assets;
use crate::collections::get_collections;
use crate::register_templates::register_templates;
use crate::render::render_pages;
use handlebars::Handlebars;
use std::path::PathBuf;

pub(crate) fn build(input: PathBuf, output: PathBuf) {
    let mut reg: Handlebars = Handlebars::new();
    let output_directory = output.as_path();

    let pages_path = input.join("pages");
    let layouts_path = input.join("templates");
    let static_path = input.join("assets");

    let pages = pages_path.as_path();
    let layouts = layouts_path.as_path();
    let assets = static_path.as_path();

    register_templates(&mut reg, layouts);

    // Create output directory
    if !output_directory.exists() {
        std::fs::create_dir(output_directory).expect("Cannot create output directory.");
    } else if !output_directory.is_dir() {
        panic!("`_site` is not a directory.")
    }

    let collections = get_collections(pages);

    if let Ok(read_dir) = output_directory.read_dir() {
        for result in read_dir {
            if let Ok(entry) = result {
                let path = entry.path();
                if path.is_dir() {
                    std::fs::remove_dir_all(path).unwrap();
                } else if path.is_file() {
                    std::fs::remove_file(path).unwrap();
                }
            }
        }
    }

    copy_assets(assets, assets, output_directory);
    render_pages(&reg, pages, pages, output_directory, &collections);
}
