/// creates the reverse complement of a sequence
pub fn reverse_complement(sequence: &str) -> String {
    sequence
        .chars()
        .map(|c| { 
            match c {
                'A' => 'T',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                'N' => 'N',
                _ => panic!("Unexpected nucleotide found in reverse complement"),
            }
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod testing {
    use super::reverse_complement;

    #[test]
    fn test_reverse_complement_1() {
        let seq = "ATCG";
        let rc = reverse_complement(seq);
        assert_eq!(rc, "CGAT");
    }

    #[test]
    fn test_reverse_complement_2() {
        let seq = "ATNCG";
        let rc = reverse_complement(seq);
        assert_eq!(rc, "CGNAT");
    }

    #[test]
    #[should_panic]
    fn test_reverse_complement_3() {
        let seq = "BBBB";
        reverse_complement(seq);
    }
}
