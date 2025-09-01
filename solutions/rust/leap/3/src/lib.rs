//EDIT: if year is divisible by 100, it is also divisible by 4
pub fn is_leap_year(year: u64) -> bool {
    match year % 100 {
        0 => return year % 400 == 0,
        _ => return year % 4 == 0,
    }
}
