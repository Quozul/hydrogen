use crate::front_matter::{parse_page, FrontMatter};
use std::collections::HashMap;
use std::path::Path;

pub(crate) type Collections = HashMap<String, Vec<FrontMatter>>;

fn get_entries(path: &Path, root: &Path) -> Option<(String, Vec<FrontMatter>)> {
    if let Ok(entries) = path.read_dir() {
        let collection = entries
            .filter_map(|entry_result| {
                if let Ok(entry) = entry_result && entry.path().is_file() {
                    let front_matter = parse_page(None, entry.path().as_path(), root);
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
}

pub(crate) fn get_collections(root: &Path) -> Collections {
    let entries = std::fs::read_dir(root).unwrap();
    let mut collections = HashMap::<String, Vec<FrontMatter>>::new();

    if let Some((name, entries)) = get_entries(root, root) {
        collections.insert(name, entries);
    }

    entries
        .filter_map(|entry_result| {
            if let Ok(entry) = entry_result && entry.path().is_dir() {
                let path = entry.path();
                get_entries(path.as_path(), root)
            } else {
                None
            }
        })
        .for_each(|(name, collection)| {
            collections.insert(name, collection);
        });

    collections
}
