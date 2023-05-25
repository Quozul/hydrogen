use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize, Debug)]
pub(crate) struct FrontMatter {
    pub(crate) title: String,
    pub(crate) layout: String,
}

impl Default for FrontMatter {
    fn default() -> Self {
        FrontMatter {
            title: String::from("undefined"),
            layout: String::from("default"),
        }
    }
}

pub(crate) fn get_front_matter(path: &Path) -> (FrontMatter, String) {
    let content = std::fs::read_to_string(&path).unwrap();

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
            Some(x) => {
                let n = serde_yaml::from_str(x);
                match n {
                    Ok(x) => (x, data),
                    Err(_) => (FrontMatter::default(), data),
                }
            }
            None => (FrontMatter::default(), data),
        }
    } else {
        (FrontMatter::default(), content)
    }
}
