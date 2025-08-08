pub fn annotate(garden: &[&str]) -> Vec<String> {
    // Handle empty list (fix for Test 9 - no_rows)
    if garden.is_empty() {
        return Vec::new();
    }

    let mut output = Vec::new();
    let rows = garden.len();
    let cols = garden[0].len();

    // Iterate through each element in 2x2 array (rows & columns)
    for i in 0..rows {
        let row = garden[i].as_bytes();
        let mut current_row = Vec::with_capacity(cols);

        for j in 0..cols {
            // Either flower (skip) or empty space(assign number)
            if row[j] == b'*' {
                current_row.push(row[j]);
            } else {
                let mut count = 0;

                // Checking all 8 directions (if available)
                let directions: [(isize, isize); 8] = [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ];
                for (di, dj) in directions {
                    let pos_i = i as isize + di;
                    let pos_j = j as isize + dj;

                    if pos_i >= 0 && pos_i < rows as isize && pos_j >= 0 && pos_j < cols as isize {
                        if garden[pos_i as usize].as_bytes()[pos_j as usize] == b'*' {
                            count += 1;
                        }
                    }
                }

                if count == 0 {
                    current_row.push(b' ');
                } else {
                    current_row.push(b'0' + count);
                }
            }
        }
        // Input in ASCII, safe to unwrap
        output.push(String::from_utf8(current_row).unwrap());
    }
    output
}
