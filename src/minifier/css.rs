use crate::minifier::replacer::replacer;
use regex::Regex;

pub(crate) fn minify_css(input: String) -> String {
    if let Ok(re) = Regex::new(r"(\{.*?\})|\s+") {
        re.replace_all(&*input, replacer).to_string()
    } else {
        input
    }
}
