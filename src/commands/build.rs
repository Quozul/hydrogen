use crate::assets::copy_assets;
use crate::collections::get_collections;
use crate::register_helpers::register_helpers;
use crate::register_templates::register_templates;
use crate::render::render_pages;
use handlebars::Handlebars;
use log::error;
use std::path::PathBuf;
use std::process::exit;

pub(crate) fn build(input: PathBuf, output: PathBuf) {
    if !input.exists() || !input.is_dir() {
        error!("{:?} does not exists or is not a directory.", input);
        exit(1);
    }

    let pages_path = input.join("pages");
    let pages = pages_path.as_path();

    if !pages.exists() {
        error!("No pages directory");
        exit(1);
    }

    let output_directory = output.as_path();

    let static_path = input.join("assets");
    let templates_path = input.join("templates");
    let scripts_path = input.join("scripts");

    let assets = static_path.as_path();

    let mut reg: Handlebars = Handlebars::new();

    if templates_path.exists() {
        register_templates(&mut reg, templates_path);
    }

    if scripts_path.exists() {
        register_helpers(&mut reg, scripts_path);
    }

    // Create output directory
    if !output_directory.exists() {
        std::fs::create_dir(output_directory).expect("Cannot create output directory.");
    } else if !output_directory.is_dir() {
        error!("{:?} is not a directory.", output);
        exit(1);
    }

    let collections = get_collections(pages);

    // Clear output directory
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

    if assets.exists() {
        copy_assets(assets, assets, output_directory);
    }

    render_pages(&reg, pages, pages, output_directory, collections);
}
