// https://www.codewars.com/kata/577b9960df78c19bca00007e

pub fn find_digit(num: i32, nth: i32) -> i32 {
    if nth > 0 {
        num.abs()
            .to_string()
            .chars()
            .rev()
            .nth((nth - 1) as usize)
            .unwrap_or('0')
            .to_digit(10)
            .unwrap() as i32
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(find_digit(5673, 4), 5);
        assert_eq!(find_digit(129, 2), 2);
        assert_eq!(find_digit(-2825, 3), 8);
        assert_eq!(find_digit(-456, 4), 0);
        assert_eq!(find_digit(0, 20), 0);
        assert_eq!(find_digit(65, 0), -1);
        assert_eq!(find_digit(24, -8), -1);
    }
}