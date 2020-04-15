// https://www.codewars.com/kata/587731fda577b3d1b0001196/


fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn camel_case(txt: &str) -> String {
    txt.to_string()
        .split_ascii_whitespace()
        .map(capitalize)
        .collect::<Vec<String>>()
        .join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_test() {
        assert_eq!(camel_case("test case"), "TestCase");
        assert_eq!(camel_case("camel case method"), "CamelCaseMethod");
        assert_eq!(camel_case("say hello "), "SayHello");
        assert_eq!(camel_case(" camel case word"), "CamelCaseWord");
        assert_eq!(camel_case(""), "");
    }
}