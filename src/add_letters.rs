// https://www.codewars.com/kata/5d50e3914861a500121e1958

pub fn add_letters(letters: Vec<char>) -> char {
    let a = (letters.iter().map(|x| x.to_digit(36).unwrap() - 9).sum::<u32>() % 26) as u8;
    if a == 0 { 'z' } else { (a + 96) as char }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(add_letters(vec!['a', 'b', 'c']), 'f');
        assert_eq!(add_letters(vec!['z']), 'z');
        assert_eq!(add_letters(vec!['a', 'b']), 'c');
        assert_eq!(add_letters(vec!['c']), 'c');
        assert_eq!(add_letters(vec!['z', 'a']), 'a');
        assert_eq!(add_letters(vec!['y', 'c', 'b']), 'd');
        assert_eq!(add_letters(vec![]), 'z');
    }
}