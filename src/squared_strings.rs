// https://www.codewars.com/kata/56dbe0e313c2f63be4000b25

pub fn hor_mirror(s: String) -> String {
    let mut a: String = s.rsplit('\n').fold(String::new(), |r, c| r + c + "\n");
    a.pop();
    a
}

pub fn vert_mirror(s: String) -> String {
    let mut a: String = s.split('\n')
        .fold(String::new(), |r, c| r + c.chars().rev().collect::<String>().as_str() + "\n");
    a.pop();
    a
}

pub fn oper(f: fn(String) -> String, s: String) -> String {
    f(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn testing1(s: &str, exp: &str) -> () {
        assert_eq!(oper(hor_mirror, s.to_string()), exp)
    }

    fn testing2(s: &str, exp: &str) -> () {
        assert_eq!(oper(vert_mirror, s.to_string()), exp)
    }

    #[test]
    fn basics_oper1() {
        testing1("lVHt\nJVhv\nCSbg\nyeCt", "yeCt\nCSbg\nJVhv\nlVHt");
    }

    #[test]
    fn basics_oper2() {
        testing1("njMK\ndbrZ\nLPKo\ncEYz", "cEYz\nLPKo\ndbrZ\nnjMK");
    }

    #[test]
    fn basics_oper3() {
        testing1("QMxo\ntmFe\nWLUG\nowoq", "owoq\nWLUG\ntmFe\nQMxo");
    }

    #[test]
    fn basics_oper4() {
        testing2("hSgdHQ\nHnDMao\nClNNxX\niRvxxH\nbqTVvA\nwvSyRu", "QHdgSh\noaMDnH\nXxNNlC\nHxxvRi\nAvVTqb\nuRySvw");
    }

    #[test]
    fn basics_oper5() {
        testing2("IzOTWE\nkkbeCM\nWuzZxM\nvDddJw\njiJyHF\nPVHfSx", "EWTOzI\nMCebkk\nMxZzuW\nwJddDv\nFHyJij\nxSfHVP");
    }

    #[test]
    fn basics_oper6() {
        testing2("cuQW\nxOuD\nfZwp\neqFx", "WQuc\nDuOx\npwZf\nxFqe");
    }
}