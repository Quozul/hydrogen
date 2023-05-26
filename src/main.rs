#![feature(let_chains)]

mod cli;
mod collections;
mod front_matter;

use crate::cli::{Cli, Commands};
use crate::collections::{get_collections, Collections};
use crate::front_matter::get_front_matter;
use clap::Parser;
use handlebars::Handlebars;
use log::{debug, error};
use serde::Serialize;
use serde_json::json;
use std::error::Error;
use std::path::Path;

#[derive(Serialize)]
struct CollectionEntry {
    link: String,
    name: String,
}

fn register_templates(reg: &mut Handlebars, layout_path: &Path) {
    if let Ok(read_dir) = layout_path.read_dir() {
        for result in read_dir {
            if let Ok(entry) = result {
                let path = entry.path();

                if path.is_file() && let Some(extension) = path.extension() && extension == "hbs" {
                    if let Some(os_name) = path.file_stem() && let Some(name) = os_name.to_str() {
                        debug!("Loading template '{:?}'...", name);

                        if let Err(err) = reg.register_template_file(name, path.as_path()) {
                            error!("Error while loading template {}", err);
                        }
                    }
                }
            }
        }
    }
}

fn render_pages(
    reg: &Handlebars,
    root: &Path,
    input: &Path,
    out: &Path,
    collections: &Collections,
) {
    if let Ok(read_dir) = input.read_dir() {
        for result in read_dir {
            if let Ok(entry) = result {
                let path = entry.path();

                if path.is_dir() {
                    render_pages(reg, root, path.as_path(), out, collections);
                } else if path.is_file() {
                    let (front_matter, content) = get_front_matter(&path, root);

                    if let Ok(suffix) = path.strip_prefix(root) {
                        debug!("Rendering page {:?}", path);

                        let rendered = markdown::to_html(&*content);
                        let out_path = out.join(suffix).with_extension("html");
                        let out_data = reg
                            .render(
                                front_matter.layout.as_str(),
                                &json!({
                                    "title": front_matter.title,
                                    "content": rendered,
                                    "collections": collections,
                                }),
                            )
                            .unwrap();

                        if let Some(parent) = out_path.parent() && !parent.exists() {
                            std::fs::create_dir_all(parent)
                                .expect("Was not able to create parent directory.");
                        }

                        std::fs::write(out_path, out_data).unwrap();
                    }
                }
            }
        }
    }
}

fn copy_assets(assets: &Path, root: &Path, output: &Path) {
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
                        std::fs::copy(source, destination).unwrap();
                    }
                } else if source.is_dir() {
                    copy_assets(source.as_path(), root, output)
                }
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let cli = Cli::parse();

    match cli.command {
        None => {}
        Some(Commands::Build { input, output }) => {
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
        _ => {
            println!("This command is not implemented yet.")
        }
    }

    Ok(())
}
