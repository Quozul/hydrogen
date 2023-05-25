use log::debug;
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

#[derive(Serialize, Debug)]
pub(crate) struct FrontMatter {
    pub(crate) layout: String,
    pub(crate) title: String,
    pub(crate) permalink: String,
}

pub(crate) fn get_front_matter(file_path: &Path, input_path: &Path) -> (FrontMatter, String) {
    let content = std::fs::read_to_string(&file_path).unwrap();

    let output_path = String::from(
        file_path
            .strip_prefix(input_path)
            .unwrap()
            .with_extension("html")
            .to_str()
            .unwrap(),
    );

    let default_front_matter = FrontMatter {
        permalink: format!("/{}", output_path),
        title: String::from(file_path.file_stem().unwrap().to_str().unwrap()),
        layout: String::from("default"),
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
                    let front_matter = FrontMatter {
                        layout: deserialized.layout.unwrap_or(default_front_matter.layout),
                        title: deserialized.title.unwrap_or(default_front_matter.title),
                        permalink: deserialized
                            .permalink
                            .unwrap_or(default_front_matter.permalink),
                    };

                    (front_matter, data)
                }
                Err(_) => (default_front_matter, data),
            },
            None => (default_front_matter, data),
        }
    } else {
        (default_front_matter, content)
    }
}
