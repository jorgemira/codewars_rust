// https://www.codewars.com/kata/523f5d21c841566fde000009

pub fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter()
        .filter(|x| !b.contains(x))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(array_diff(vec![1, 2], vec![1]), vec![2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![1]), vec![2, 2]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![2]), vec![1]);
        assert_eq!(array_diff(vec![1, 2, 2], vec![]), vec![1, 2, 2]);
        assert_eq!(array_diff(vec![], vec![1, 2]), vec![]);
    }
}