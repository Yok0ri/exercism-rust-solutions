use unicode_segmentation::UnicodeSegmentation;

/// Version that supports grapheme clusters (optimized)
pub fn reverse(input: &str) -> String {
    input.graphemes(true).rev().collect()
}
