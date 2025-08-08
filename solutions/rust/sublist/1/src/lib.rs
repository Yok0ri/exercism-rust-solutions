use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }
    if first_list.is_empty() {
        return Comparison::Sublist;
    }
    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    let len_a = first_list.len();
    let len_b = second_list.len();

    match len_a.cmp(&len_b) {
        Ordering::Equal => {
            if first_list == second_list {
                Comparison::Equal
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Less => {
            if second_list.windows(len_a).any(|slice| slice == first_list) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        }
        Ordering::Greater => {
            if first_list.windows(len_b).any(|slice| slice == second_list) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        }
    }
}
