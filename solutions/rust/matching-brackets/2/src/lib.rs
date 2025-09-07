//FAILED:
//I. 3-type up-down counter: + for opening, - for closing, must be (0,0,0) at the end
//II. after filtering chars other than closures:
//     each opening closure on position `i` must have matching
//     closing closure of the same type on position `len - i`,
//     so after cutting vector in half, 1st half
//     should be equal to reversed 2nd
//
// Method 1:
// 1) iterate over chars
// 2) filter out chars other than closures
// 3) push next opening closure into vector
// 4) if the next closing closure is not the same type as value in the vector, return false
// 5) return true only if vector is empty after all checks
//
// Method 2:
// 1) create a mutable String with only closure chars
// 2) In a loop, remove matching closures next to each other
// 3) Do this uintil there are no pairs to remove
// 4) return true if the starting String becomes empty
//
// Closures:
// - Brackets
// - Braces
// - Parentheses

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut closures: String = string.chars().filter(|c| "[]{}()".contains(*c)).collect();
    loop {
        let reduced = closures
            .replace("[]", "")
            .replace("{}", "")
            .replace("()", "");

        if reduced == closures {
            break;
        }

        closures = reduced;
    }

    closures.is_empty()
}
