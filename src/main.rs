#![feature(let_chains)]

mod cli;
mod front_matter;

use crate::cli::{Cli, Commands};
use crate::front_matter::{get_front_matter, FrontMatter};
use clap::Parser;
use handlebars::Handlebars;
use log::debug;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

#[derive(Serialize)]
struct CollectionEntry {
    link: String,
    name: String,
}

fn load_layouts(reg: &mut Handlebars, layout_path: &Path) {
    std::fs::read_dir(layout_path).unwrap().for_each(|entry| {
        let path = entry.unwrap().path();
        debug!("Loading layout {:?}", path);
        if path.is_file() {
            let name = path.file_stem().unwrap();
            let template = std::fs::read_to_string(&path).unwrap();
            reg.register_template_string(name.to_str().unwrap(), template)
                .unwrap();
        }
    });
}

fn convert_directory(reg: &Handlebars, root: &Path, input: &Path, out: &Path) {
    let entries = std::fs::read_dir(input).unwrap();
    let entries2 = std::fs::read_dir(input).unwrap();
    let mut collections = HashMap::<String, Vec<FrontMatter>>::new();

    entries
        .filter_map(|entry_result| {
            if let Ok(entry) = entry_result && entry.path().is_dir() {
                let path = entry.path();

                if let Ok(entries) = path.read_dir() {
                    let collection = entries.filter_map(|entry_result| {
                        if let Ok(entry) = entry_result && entry.path().is_file() {
                            let (front_matter, _) = get_front_matter(entry.path().as_path(), root);
                            Some(front_matter)
                        } else {
                            None
                        }
                    })
                        .collect::<Vec<_>>();

                    let name = path.file_name().unwrap().to_str().unwrap();
                    Some((String::from(name), collection))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .for_each(|(name, collection)| {
            collections.insert(name, collection);
        });

    entries2.for_each(|entry| {
        let path = entry.unwrap().path();
        if path.is_dir() {
            convert_directory(reg, root, path.as_path(), out);
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

                if let Some(parent) = out_path.parent() {
                    if !parent.exists() {
                        std::fs::create_dir_all(parent)
                            .expect("Was not able to create parent directory.");
                    }
                }

                std::fs::write(out_path, out_data).unwrap();
            }
        }
    })
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
            let layouts_path = input.join("layouts");

            let pages = pages_path.as_path();
            let layouts = layouts_path.as_path();

            load_layouts(&mut reg, layouts);

            if !output_directory.exists() {
                std::fs::create_dir(output_directory).expect("Cannot create output directory.");
            } else if !output_directory.is_dir() {
                panic!("`_site` is not a directory.")
            }

            convert_directory(&reg, pages, pages, output_directory);
        }
    }

    Ok(())
}
