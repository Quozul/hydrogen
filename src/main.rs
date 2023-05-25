use handlebars::{Handlebars, Registry};
use serde::{Deserialize, Serialize};
use serde_json::{json, map};
use std::collections::HashMap;
use std::error::Error;
use std::path::Path;

fn get_content(path: &Path) -> Option<String> {
    let content_path = path.with_extension("md");

    match content_path.exists() {
        true => {
            let content = std::fs::read_to_string(&content_path).unwrap();
            Some(markdown::to_html(&*content))
        }
        false => None,
    }
}

#[derive(Serialize)]
struct CollectionEntry {
    link: String,
    name: String,
}

#[derive(Deserialize)]
struct FrontMatter {
    title: String,
    layout: String,
}

impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            title: String::from("undefined"),
            layout: String::from("default"),
        }
    }
}

fn load_layouts(reg: &mut Registry) {
    std::fs::read_dir("../data/layouts")
        .unwrap()
        .for_each(|entry| {
            let path = entry.unwrap().path();
            let name = path.file_stem().unwrap();
            let template = std::fs::read_to_string(&path).unwrap();
            reg.register_template_string(name, template);
        });
}

fn get_front_matter(path: &Path) -> (FrontMatter, String) {
    let content = std::fs::read_to_string(&path).unwrap();
    let mut parts = content.splitn(3, "---");
    let _ = parts.next();

    let header_trimmed = if let Some(header) = parts.next() {
        let header_trimmed = header.trim();

        if !header_trimmed.is_empty() {
            Some(header_trimmed)
        } else {
            None
        }
    } else {
        None
    };

    let data = parts.next().unwrap().to_string();

    match header_trimmed {
        Some(x) => {
            let n = serde_yaml::from_str(x);
            match n {
                Ok(x) => (x, data),
                Err(_) => (FrontMatter::default(), data),
            }
        }
        None => (FrontMatter::default(), data),
    }
}

fn convert_directory(reg: &mut Registry, input: &Path, out: &Path) {
    std::fs::read_dir(input).unwrap().for_each(|entry| {
        let path = entry.unwrap().path();
        if path.is_dir() {
            convert_directory(path.as_path(), out);
        } else if path.is_file() {
            let (front_matter, content) = get_front_matter(&path);
            if let Ok(suffix) = path.strip_prefix(input) {
                // TODO: Créer le fichier de sortie
                let out_path = out.join(suffix);
                let out_data = reg.render(
                    front_matter.layout,
                    &json!({
                        "title": front_matter.title,
                        "content": content,
                    })?,
                );
                std::fs::write(out_data);
            }
        }
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut reg: Registry = Handlebars::new();
    load_layouts(reg);

    let output_directory = Path::new("_site");
    let input_directory = Path::new("data");

    if !output_directory.exists() {
        std::fs::create_dir(output_directory).expect("Cannot create output directory.");
    } else if !output_directory.is_dir() {
        panic!("`_site` is not a directory.")
    }

    convert_directory(reg, input_directory, output_directory);

    /*let mut collections = HashMap::<String, Vec<CollectionEntry>>::new();

    std::fs::read_dir("./collections").unwrap().for_each(
        |collection_result| match collection_result {
            Ok(collection_entry) => {
                let collection_path = collection_entry.path();
                let collection_key = collection_path.file_stem().unwrap().to_str().unwrap();

                let elements = collection_path
                    .read_dir()
                    .unwrap()
                    .map(|element| {
                        let path = element.unwrap().path();
                        let link = path.to_str().unwrap().to_string();
                        let name = path.file_stem().unwrap().to_str().unwrap().to_string();

                        CollectionEntry { link, name }
                    })
                    .collect::<Vec<_>>();

                collections.insert(String::from(collection_key), elements);
            }
            Err(_) => {}
        },
    );

    reg.register_template_string("tpl_1", "Good afternoon, {{name}}")?;

    std::fs::read_dir("../data/pages")
        .unwrap()
        .filter(|path| match path {
            Ok(p) => match p.path().extension() {
                None => false,
                Some(extension) => extension == "hbs",
            },
            Err(_) => false,
        })
        .for_each(|dir_entry| {
            let path = dir_entry.unwrap().path();
            let template = std::fs::read_to_string(&path).unwrap();
            let content = get_content(&path);

            let rendered = reg
                .render_template(
                    &*template,
                    &json!({
                        "page_name": "foo",
                        "collections": collections,
                        "content": content,
                    }),
                )
                .unwrap();

            let output_directory = Path::new("_site");

            if !output_directory.exists() {
                std::fs::create_dir(output_directory).expect("Cannot create output directory.");
            } else if !output_directory.is_dir() {
                panic!("`_site` is not a directory.")
            }

            let output_path = output_directory
                .join(path.file_name().unwrap())
                .with_extension("html");
            std::fs::write(output_path, rendered).unwrap();
        });*/

    Ok(())
}
