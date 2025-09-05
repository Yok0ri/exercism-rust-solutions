//I. 3-way up-down counter
//II. collecting closures into a vector, cutting in half and comparing 1st with reversed 2nd
//III. after filtering chars other than closures:
//     each opening closure on position `i` must have matching
//     closing closure of the same type on position `len - i`

// 1) iterate over chars
// 2) filter out chars other than closures
// 3) push next opening closure into vector
// 4) if the next closing closure is not the same type as value in the vector, return false
// 5) return true only if vector is empty after all checks
//
// Closures:
// - Brackets
// - Braces
// - Parentheses

pub fn brackets_are_balanced(string: &str) -> bool {
    let mut closure_stack = Vec::new();
    for c in string
        .chars()
        .filter(|c| matches!(c, '[' | '{' | '(' | ']' | '}' | ')'))
    {
        match c {
            '[' | '{' | '(' => closure_stack.push(c),
            ']' => {
                if closure_stack.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if closure_stack.pop() != Some('{') {
                    return false;
                }
            }
            ')' => {
                if closure_stack.pop() != Some('(') {
                    return false;
                }
            }
            _ => continue,
        }
    }

    closure_stack.is_empty()
}
