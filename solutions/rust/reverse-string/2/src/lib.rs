use unicode_segmentation::UnicodeSegmentation;

/// Version that supports grapheme clusters
pub fn reverse(input: &str) -> String {
    let graphemes = UnicodeSegmentation::graphemes(input, true).collect::<Vec<&str>>();
    graphemes.into_iter().rev().collect::<String>()
}
