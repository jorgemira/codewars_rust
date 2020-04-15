pub fn duplicate_encode(word: &str) -> String {
    let word = word.to_uppercase();
    word.chars()
        .map(|x| if word
            .chars()
            .filter(|&y| y == x).count() > 1 { ")" } else { "(" })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_tests() {
        assert_eq!(duplicate_encode("din"), "(((");
        assert_eq!(duplicate_encode("recede"), "()()()");
        assert_eq!(duplicate_encode("Success"), ")())())", "should ignore case");
        assert_eq!(duplicate_encode("(( @"), "))((");
    }
}