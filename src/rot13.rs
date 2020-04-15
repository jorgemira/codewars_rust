// https://www.codewars.com/kata/530e15517bc88ac656000716

pub fn rot13(message: &str) -> String {
    message.chars()
        .map(|x| match x {
            'a'..='z' => ((x as u8 - 97 + 13) % 26 + 97) as char,
            'A'..='Z' => ((x as u8 - 65 + 13) % 26 + 65) as char,
            _ => x
        })
        .collect()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
//        assert_eq!(rot13("test"), "grfg");
        assert_eq!(rot13("Test"), "Grfg");
    }
}