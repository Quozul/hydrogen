#![feature(let_chains)]

mod assets;
mod cli;
mod collections;
mod front_matter;
mod minifier;

use crate::cli::{Cli, Commands};
use crate::collections::{get_collections, Collections};
use crate::front_matter::get_front_matter;
use crate::minifier::html::minify_html;
use clap::Parser;
use handlebars::Handlebars;
use log::{debug, error};

use crate::assets::copy_assets;
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
                        debug!("Loading template '{:?}'…", name);

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
                } else if path.is_file() && let Some(os_extension) = path.extension() && let Some(extension) = os_extension.to_str() && let Ok(suffix) = path.strip_prefix(root) {
                    debug!("Rendering page {:?}…", path);

                    let (front_matter, content) = get_front_matter(&path, root);

                    let rendered = match extension {
                        "md" => markdown::to_html(&*content),
                        _ => content,
                    };

                    let out_path = out.join(suffix).with_extension("html");
                    match reg
                        .render(
                            front_matter.layout.as_str(),
                            &json!({
                                "title": front_matter.title,
                                "content": rendered,
                                "collections": collections,
                            }),
                        ) {
                        Ok(out_data) => {
                            if let Some(parent) = out_path.parent() && !parent.exists() {
                                std::fs::create_dir_all(parent)
                                    .expect("Was not able to create parent directory.");
                            }

                            std::fs::write(out_path, minify_html(out_data)).unwrap();
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
