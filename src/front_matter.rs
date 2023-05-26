use crate::collections::Collections;
use crate::path_extension::get_extension;
use log::error;
use markdown::{Constructs, Options, ParseOptions};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
struct FrontMatterDeserializer {
    layout: Option<String>,
    title: Option<String>,
    permalink: Option<String>,
}

impl Default for FrontMatterDeserializer {
    fn default() -> Self {
        FrontMatterDeserializer {
            title: Some(String::from("undefined")),
            layout: Some(String::from("default")),
            permalink: Some(String::from("/404.html")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub(crate) struct FrontMatter {
    pub(crate) layout: String,
    pub(crate) permalink: String,
    title: String,
    content: String,
    collections: Option<Collections>,
}

fn get_rendered(file_path: &Path, data: String) -> String {
    if let Some(extension) = get_extension(file_path) {
        match extension.as_str() {
            "md" => {
                let options = Options {
                    parse: ParseOptions {
                        constructs: Constructs {
                            frontmatter: true,
                            ..Constructs::gfm()
                        },
                        ..ParseOptions::gfm()
                    },
                    ..Options::gfm()
                };
                match markdown::to_html_with_options(data.as_str(), &options) {
                    Ok(result) => result,
                    Err(err) => {
                        error!("An error has occurred while rendering {:?}", err);
                        data
                    }
                }
            }
            "html" => {
                let parts = data.splitn(3, "---");
                if let Some(last) = parts.last() {
                    last.to_string()
                } else {
                    data
                }
            }
            _ => data,
        }
    } else {
        data
    }
}

enum Serializer {
    Yaml,
    Toml,
}

fn get_front_matter_string(data: &String) -> Option<(String, Serializer)> {
    let parts = if data.starts_with("---") {
        Some((data.splitn(3, "---"), Serializer::Yaml))
    } else if data.starts_with("+++") {
        Some((data.splitn(3, "+++"), Serializer::Toml))
    } else {
        None
    };

    if let Some((mut parts, serializer)) = parts {
        let _ = parts.next();

        if let Some(header) = parts.next() {
            let header_trimmed = header.trim();

            if !header_trimmed.is_empty() {
                Some((header_trimmed.to_string(), serializer))
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

fn parse_front_matter_string(data: &String) -> Option<FrontMatterDeserializer> {
    if let Some((front_matter_string, serializer)) = get_front_matter_string(data) {
        match serializer {
            Serializer::Toml => {
                match toml::from_str::<FrontMatterDeserializer>(front_matter_string.as_str()) {
                    Ok(parsed) => Some(parsed),
                    Err(_) => None,
                }
            }
            Serializer::Yaml => {
                match serde_yaml::from_str::<FrontMatterDeserializer>(front_matter_string.as_str())
                {
                    Ok(parsed) => Some(parsed),
                    Err(_) => None,
                }
            }
        }
    } else {
        None
    }
}

pub(crate) fn parse_page(
    collections: Option<Collections>,
    file_path: &Path,
    root: &Path,
) -> FrontMatter {
    let content = std::fs::read_to_string(&file_path).unwrap();

    let output_path = String::from(
        file_path
            .strip_prefix(root)
            .unwrap()
            .with_extension("html")
            .to_str()
            .unwrap(),
    );

    let default_front_matter = FrontMatter {
        permalink: format!("/{}", output_path),
        content: content.clone(),
        title: String::from(file_path.file_stem().unwrap().to_str().unwrap()),
        layout: String::from("default"),
        collections,
    };

    let front_matter_deserialized = parse_front_matter_string(&content);

    match front_matter_deserialized {
        Some(deserialized) => {
            let rendered = get_rendered(file_path, content);

            let front_matter = FrontMatter {
                layout: deserialized.layout.unwrap_or(default_front_matter.layout),
                title: deserialized.title.unwrap_or(default_front_matter.title),
                permalink: deserialized
                    .permalink
                    .unwrap_or(default_front_matter.permalink),
                content: rendered,
                collections: default_front_matter.collections,
            };

            front_matter
        }
        None => default_front_matter,
    }
}
