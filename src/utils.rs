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

/// creates the reverse complement of a sequence of bytes
pub fn reverse_complement_bytes(sequence: &[u8]) -> Vec<u8> {
    sequence
        .iter()
        .map(|c| { 
            match c {
                b'A' => b'T',
                b'C' => b'G',
                b'G' => b'C',
                b'T' => b'A',
                b'N' => b'N',
                _ => panic!("Unexpected nucleotide found in reverse complement"),
            }
        })
        .rev()
        .collect()
}

#[cfg(test)]
mod testing {
    use crate::utils::reverse_complement_bytes;

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

    #[test]
    fn bytes_test_reverse_complement_1() {
        let seq = b"ATCG";
        let rc = reverse_complement_bytes(seq);
        assert_eq!(rc, b"CGAT");
    }

    #[test]
    fn bytes_test_reverse_complement_2() {
        let seq = b"ATNCG";
        let rc = reverse_complement_bytes(seq);
        assert_eq!(rc, b"CGNAT");
    }

    #[test]
    #[should_panic]
    fn bytes_test_reverse_complement_3() {
        let seq = b"BBBB";
        reverse_complement_bytes(seq);
    }
}
