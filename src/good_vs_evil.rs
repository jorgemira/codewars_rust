// https://www.codewars.com/kata/52761ee4cffbc69732000738

pub fn good_vs_evil(good: &str, evil: &str) -> String {
    let g = vec![1, 2, 3, 3, 4, 10];
    let e = vec![1, 2, 2, 2, 3, 5, 10];
    let good = good.split_ascii_whitespace()
        .zip(g.iter())
        .fold(0, |sum, (i, &v)| sum + i.parse::<i32>().unwrap() * v);
    let evil = evil.split_ascii_whitespace()
        .zip(e.iter())
        .fold(0, |sum, (i, &v)| sum + i.parse::<i32>().unwrap() * v);

    if good > evil {
        String::from("Battle Result: Good triumphs over Evil")
    } else if good < evil {
        String::from("Battle Result: Evil eradicates all trace of Good")
    } else {
        String::from("Battle Result: No victor on this battle field")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 0"),
                   "Battle Result: Good triumphs over Evil");
        assert_eq!(good_vs_evil("0 0 0 0 0 0", "0 0 0 0 0 0 10"),
                   "Battle Result: Evil eradicates all trace of Good");
        assert_eq!(good_vs_evil("0 0 0 0 0 10", "0 0 0 0 0 0 10"),
                   "Battle Result: No victor on this battle field");
    }
}
