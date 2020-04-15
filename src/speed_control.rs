// https://www.codewars.com/kata/56484848ba95170a8000004d

use std::cmp;

pub fn gps(s: i32, x: Vec<f64>) -> i32 {
    let s = s as f64;
    let mut max = 0;

    for i in 1..x.len() {
        max = cmp::max(max, ((x[i] - x[i - 1]) * 3600.0 / s) as i32);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(s: i32, x: Vec<f64>, exp: i32) -> () {
        println!("s: {:?};", s);
        println!("x: {:?};", x);
        let ans = gps(s, x);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tes1s() {
        let s = 20;
        let x = vec![0.0, 0.23, 0.46, 0.69, 0.92, 1.15, 1.38, 1.61];
        let u = 41;
        dotest(s, x, u);
    }

    #[test]
    fn basic_test2() {
        let x = vec![0.0, 0.11, 0.22, 0.33, 0.44, 0.65, 1.08, 1.26, 1.68, 1.89, 2.1, 2.31, 2.52, 3.25];
        let s = 12;
        let u = 219;
        dotest(s, x, u);
    }
}
