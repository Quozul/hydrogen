use crate::collections::Collections;
use crate::has_extension::has_extension;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

#[derive(Serialize, Debug)]
pub(crate) struct FrontMatter<'a> {
    pub(crate) layout: String,
    pub(crate) title: String,
    pub(crate) permalink: String,
    pub(crate) content: String,
    pub(crate) collections: Option<&'a Collections<'a>>,
}

pub(crate) fn get_front_matter<'a, 'b>(
    collections: Option<&'b HashMap<String, Vec<FrontMatter<'b>>>>,
    file_path: &'a Path,
    root: &'b Path,
) -> FrontMatter<'b> {
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
        collections: None,
    };

    if content.starts_with("---") {
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
            Some(x) => match serde_yaml::from_str::<FrontMatterDeserializer>(x) {
                Ok(deserialized) => {
                    let rendered = if has_extension(file_path, "md") {
                        markdown::to_html(&*data)
                    } else {
                        data
                    };

                    let front_matter = FrontMatter::<'b> {
                        layout: deserialized.layout.unwrap_or(default_front_matter.layout),
                        title: deserialized.title.unwrap_or(default_front_matter.title),
                        permalink: deserialized
                            .permalink
                            .unwrap_or(default_front_matter.permalink),
                        content: rendered,
                        collections,
                    };

                    front_matter
                }
                Err(_) => default_front_matter,
            },
            None => default_front_matter,
        }
    } else {
        default_front_matter
    }
}
