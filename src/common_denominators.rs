// https://www.codewars.com/kata/54d7660d2daf68c619000d95/

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn convert_fracts(l: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    let l: Vec<(i64, i64)>  = l.iter()
        .map(|&(n, d)| {
            let g = gcd(n, d);
            (n / g, d / g)
        })
        .collect();

    let x = l.iter().fold(1, |ac, &(_, d)| lcm(ac, d));

    l.iter()
        .map(|&(n, d)| (n * (x / d), x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing(l: Vec<(i64, i64)>, exp: Vec<(i64, i64)>) -> () {
        assert_eq!(convert_fracts(l), exp)
    }

    #[test]
    fn basics_convert_fracts() {
        println!("{}", gcd(3, 4));
        println!("{}", lcm(1, 8));
//        testing(vec![(60, 100), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
        testing(vec![(69, 130), (87, 1310), (3, 4)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
        testing(vec![(690, 1300), (87, 1310), (30, 40)], vec![(18078, 34060), (2262, 34060), (25545, 34060)]);
    }
}
