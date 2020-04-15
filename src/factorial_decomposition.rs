// https://www.codewars.com/kata/5a045fee46d843effa000070

use std::collections::HashMap;

pub fn decomp(n: i32) -> String {
    let mut primes: Vec<i32> = Vec::new();
    let mut dict: HashMap<i32, i32> = HashMap::new();

    if n < 2 { return "1".to_string(); }

    for i in 2..n + 1 {
        let mut is_prime = true;
        let mut t = i;
        for &p in &primes {
            while t % p == 0 {
                is_prime = false;
                t /= p;
                dict.insert(p, dict.get(&p).unwrap() + 1);
            }
        }
        if is_prime {
            primes.push(i);
            dict.insert(i, 1);
        }
    }

    let mut v: Vec<_> = dict.into_iter().collect();
    v.sort_by(|x, y| x.0.cmp(&y.0));
    v.iter()
        .map(|&(a, b)| if b == 1 { a.to_string() } else { format!("{}^{}", a, b) })
        .collect::<Vec<String>>()
        .join(" * ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i32, exp: &str) -> () {
        println!("n:{:?}", n);
        let ans = decomp(n);
        println!("actual: {:?}", ans);
        println!("expect: {:?}", exp.to_string());
        println!("{}", ans == exp.to_string());
        assert_eq!(ans, exp.to_string());
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(5, "2^3 * 3 * 5");
        dotest(17, "2^15 * 3^6 * 5^3 * 7^2 * 11 * 13 * 17");
        dotest(22, "2^19 * 3^9 * 5^4 * 7^3 * 11^2 * 13 * 17 * 19");
        dotest(14, "2^11 * 3^5 * 5^2 * 7^2 * 11 * 13");
        dotest(25, "2^22 * 3^10 * 5^6 * 7^3 * 11^2 * 13 * 17 * 19 * 23");
    }
}
