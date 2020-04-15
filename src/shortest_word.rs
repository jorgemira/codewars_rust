// https://www.codewars.com/kata/57cebe1dc6fdc20c57000ac9

pub fn find_short(s: &str) -> u32 {
    s.split_ascii_whitespace()
        .min_by_key(|x| x.len())
        .unwrap()
        .len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(find_short("bitcoin take over the world maybe who knows perhaps"), 3);
        assert_eq!(find_short("turns out random test cases are easier than writing out basic ones"), 3);
        assert_eq!(find_short("lets talk about javascript the best language"), 3);
        assert_eq!(find_short("i want to travel the world writing code one day"), 1);
        assert_eq!(find_short("Lets all go on holiday somewhere very cold"), 2);
    }
}