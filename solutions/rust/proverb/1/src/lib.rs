/// *Template for `n` items:*
///
/// "For want of a {1} the {2} was lost.
///
/// For want of a {2} the {3} was lost.
///
/// ...
///
/// For want of a {n-1} the {n} was lost.
///
/// And all for the want of a {1}."
///
/// **`n-1` "for" sentences and 1 last sentence**
pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }

    let mut lines: Vec<String> = list
        .windows(2)
        .map(|w| format!("For want of a {} the {} was lost.", w[0], w[1]))
        .collect();

    lines.push(format!("And all for the want of a {}.", list[0]));
    lines.join("\n")
}
