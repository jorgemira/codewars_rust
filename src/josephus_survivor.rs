// https://www.codewars.com/kata/555624b601231dc7a400017a

pub fn josephus_survivor(n: i32, k: i32) -> i32 {
    let mut v = (1..n + 1).collect::<Vec<i32>>();
    let mut i: usize = 0;

    while v.len() > 1 {
        i = (i + k as usize - 1) % v.len();
        v.remove(i);
    }

    v[0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(josephus_survivor(7, 3), 4);
        assert_eq!(josephus_survivor(11, 19), 10);
        assert_eq!(josephus_survivor(40, 3), 28);
        assert_eq!(josephus_survivor(14, 2), 13);
        assert_eq!(josephus_survivor(100, 1), 100);
        assert_eq!(josephus_survivor(1, 300), 1);
        assert_eq!(josephus_survivor(2, 300), 1);
        assert_eq!(josephus_survivor(5, 300), 1);
        assert_eq!(josephus_survivor(7, 300), 7);
        assert_eq!(josephus_survivor(300, 300), 265);
    }
}