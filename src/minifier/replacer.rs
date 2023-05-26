use regex::Captures;

pub(crate) fn replacer(caps: &Captures) -> String {
    if let Some(m) = caps.get(1) {
        m.as_str().to_owned()
    } else {
        " ".to_owned()
    }
}
