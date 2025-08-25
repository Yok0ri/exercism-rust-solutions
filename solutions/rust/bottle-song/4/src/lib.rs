/// usual verse:
/// "`X` green bottles hanging on the wall,
/// `X` green bottles hanging on the wall,
/// And if one green bottle should accidentally fall,
/// There'll be `X-1` green `bottle(s)` hanging on the wall."
///
/// last verse:
/// "One green bottle hanging on the wall,
/// One green bottle hanging on the wall,
/// And if one green bottle should accidentally fall,
/// There'll be no green bottles hanging on the wall."
///
/// 1) starting at `X = start_bottles` and going `take_down` times
/// 2) handling plural form on the last line of usual verse
pub fn recite(start_bottles: u32, take_down: u32) -> String {
    if start_bottles > 10 {
        panic!("The song only down from 10");
    }
    if take_down > start_bottles {
        panic!("Not enough verses to recite!");
    }

    (start_bottles - take_down + 1..=start_bottles)
        .rev()
        .map(|n| make_verse(n)) // a helper that builds one verse
        .fold(String::new(), |mut acc, verse| {
            if !acc.is_empty() {
                acc.push_str("\n\n");
            }
            acc.push_str(&verse);
            acc
        })
}

fn make_verse(number: u32) -> String {
    if number == 1 {
        "One green bottle hanging on the wall,
One green bottle hanging on the wall,
And if one green bottle should accidentally fall,
There'll be no green bottles hanging on the wall."
            .to_string()
    } else {
        let num_word = num_to_words(number);
        let next_num_word = num_to_words(number - 1);

        format!(
            "{x} green bottles hanging on the wall,
{x} green bottles hanging on the wall,
And if one green bottle should accidentally fall,
There'll be {y} green bottle{s} hanging on the wall.",
            x = capitalize(&num_word),
            y = next_num_word,
            s = if number == 2 { "" } else { "s" }
        )
    }
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn num_to_words(n: u32) -> &'static str {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        _ => panic!("Number out of range"),
    }
}
