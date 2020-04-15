//https://www.codewars.com/kata/554e4a2f232cdd87d9000038


use std::collections::HashMap;

pub fn dna_strand(dna: &str) -> String {
    let m: HashMap<char, char> = [('A', 'T'), ('T', 'A'), ('G', 'C'), ('C', 'G')]
        .iter().cloned().collect();

    dna.chars().map(|x| m.get(&x).unwrap()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(dna_strand("AAAA"), "TTTT");
        assert_eq!(dna_strand("ATTGC"), "TAACG");
        assert_eq!(dna_strand("GTAT"), "CATA");
    }
}
