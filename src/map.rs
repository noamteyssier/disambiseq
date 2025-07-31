use hashbrown::{HashMap, HashSet};

use crate::Error;

type ByteSeq = Vec<u8>;
type RefByteSeq<'a> = &'a [u8];
const ALPHABET: &[u8] = b"ACGT";

pub fn generate_mismatches(seq: RefByteSeq, mm_buffer: &mut Vec<ByteSeq>) -> Result<(), Error> {
    // Clear the mismatch buffer
    mm_buffer.clear();

    // Check if the sequence is empty
    if seq.is_empty() {
        return Err(Error::EmptySequence);
    }

    // Iterate over each position in the sequence
    for pos in 0..seq.len() {
        match seq[pos] {
            b'A' | b'C' | b'G' | b'T' | b'N' => {
                // expected
            }
            _ => {
                // unexpected base encountered - invalid nucleotide sequence
                let seq = std::str::from_utf8(seq)?.to_owned();
                return Err(Error::InvalidSequence(seq));
            }
        }

        // For all bases in the alphabet
        for base in ALPHABET {
            // If the current base is different from the base at the current position
            if seq[pos] != *base {
                // generate the mismatch sequence
                let mut mismatch = seq.to_owned();
                mismatch[pos] = *base;
                mm_buffer.push(mismatch);
            }
        }
    }

    Ok(())
}

/// Generates a hashmap of unambiguous one-off mismatches for a given collection of parent sequences.
///
/// This includes parent sequences themselves as well as their one-off mismatches.
///
/// The index will match the original ordering of parent sequences.
pub fn generate_mismatch_hashmap(parents: &[ByteSeq]) -> Result<HashMap<ByteSeq, usize>, Error> {
    // initialize collections
    let mut map = HashMap::default();
    let mut null = HashSet::new();

    // add the parents to the map and nullset
    for (i, parent) in parents.iter().enumerate() {
        if let Some(dup_idx) = map.insert(parent.clone(), i) {
            let dup_seq = std::str::from_utf8(&parents[dup_idx])?.to_owned();
            return Err(Error::DuplicateParent(dup_seq));
        };
        null.insert(parent.clone());
    }

    // process each of the sequences and remove ambiguous sequences
    let mut mm_buffer = Vec::default();
    for (p_idx, p_seq) in parents.iter().enumerate() {
        generate_mismatches(p_seq, &mut mm_buffer)?;

        for mm in mm_buffer.iter() {
            // skip if already marked ambiguous
            if null.contains(mm) {
                continue;
            } else if map.contains_key(mm) {
                map.remove(mm)
                    .map(|_| null.insert(mm.to_owned()))
                    .expect("Failed to insert mismatch");
            } else {
                map.insert(mm.to_owned(), p_idx);
            }
        }
    }

    Ok(map)
}

/// Generates a hashmap of parent sequences to their indices
///
/// This does not generate any mismatches.
///
/// The index will match the original ordering of parent sequences.
pub fn generate_parent_hashmap(parents: &[ByteSeq]) -> Result<HashMap<ByteSeq, usize>, Error> {
    let mut map = HashMap::default();
    for (i, parent) in parents.iter().enumerate() {
        if let Some(dup_idx) = map.insert(parent.clone(), i) {
            let dup_seq = std::str::from_utf8(&parents[dup_idx])?.to_owned();
            return Err(Error::DuplicateParent(dup_seq));
        };
    }
    Ok(map)
}

pub struct Disambiseq {
    parents: Vec<ByteSeq>,
    map: HashMap<ByteSeq, usize>,
}
impl Disambiseq {
    /// Creates a mismatch lookup table for a set of parent sequences.
    ///
    /// This will include the parent sequences themselves.
    pub fn new(parents: &[ByteSeq]) -> Result<Self, Error> {
        let parents_upper = parents
            .iter()
            .map(|p| p.to_ascii_uppercase())
            .collect::<Vec<_>>();
        Ok(Disambiseq {
            map: generate_mismatch_hashmap(&parents_upper)?,
            parents: parents_upper,
        })
    }

    /// Creates a lookup table for a set of parent sequences.
    ///
    /// This will not include any mismatches.
    pub fn new_exact(parents: &[ByteSeq]) -> Result<Self, Error> {
        Ok(Disambiseq {
            parents: parents.to_vec(),
            map: generate_parent_hashmap(parents)?,
        })
    }

    pub fn get_index(&self, seq: RefByteSeq) -> Option<usize> {
        self.map.get(seq).copied()
    }

    pub fn get_parent(&self, seq: RefByteSeq) -> Option<RefByteSeq> {
        if let Some(idx) = self.get_index(seq) {
            self.index_parent(idx).map(|p| p.as_slice())
        } else {
            None
        }
    }

    pub fn index_parent(&self, index: usize) -> Option<&ByteSeq> {
        self.parents.get(index)
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    // Helper function to convert string slices to ByteSeq vectors
    fn seqs(sequences: &[&str]) -> Vec<ByteSeq> {
        sequences.iter().map(|s| s.as_bytes().to_vec()).collect()
    }

    // Helper function to convert string to ByteSeq
    fn seq(s: &str) -> ByteSeq {
        s.as_bytes().to_vec()
    }

    #[test]
    fn test_generate_mismatches_basic() {
        let input = seq("ACT");
        let mut mm_buffer = Vec::new();
        generate_mismatches(&input, &mut mm_buffer).unwrap();

        // Should generate 9 mismatches (3 positions × 3 alternative bases each)
        assert_eq!(mm_buffer.len(), 9);

        let expected: Vec<ByteSeq> = vec![
            seq("CCT"),
            seq("GCT"),
            seq("TCT"), // Position 0
            seq("AAT"),
            seq("AGT"),
            seq("ATT"), // Position 1
            seq("ACA"),
            seq("ACC"),
            seq("ACG"), // Position 2
        ];

        for expected_seq in expected {
            assert!(mm_buffer.contains(&expected_seq));
        }
    }

    #[test]
    fn test_generate_mismatches_single_base() {
        let input = seq("A");
        let mut mm_buffer = Vec::new();
        generate_mismatches(&input, &mut mm_buffer).unwrap();

        assert_eq!(mm_buffer.len(), 3);
        assert!(mm_buffer.contains(&seq("C")));
        assert!(mm_buffer.contains(&seq("G")));
        assert!(mm_buffer.contains(&seq("T")));
    }

    #[test]
    fn test_generate_mismatches_empty_sequence() {
        let input = seq("");
        let mut mm_buffer = Vec::new();
        let err = generate_mismatches(&input, &mut mm_buffer);
        assert!(err.is_err());
    }

    #[test]
    fn test_generate_mismatches_buffer_reuse() {
        let input = seq("AT");
        let mut mm_buffer = Vec::new();

        // Add some existing data
        mm_buffer.push(seq("GARBAGE"));

        generate_mismatches(&input, &mut mm_buffer).unwrap();

        // Should be cleared and contain only the 6 mismatches
        assert_eq!(mm_buffer.len(), 6);
        assert!(!mm_buffer.contains(&seq("GARBAGE")));
    }

    #[test]
    fn test_generate_parent_hashmap_basic() -> Result<(), Error> {
        let parents = seqs(&["ACT", "AGT", "TCA"]);
        let map = generate_parent_hashmap(&parents)?;

        assert_eq!(map.len(), 3);
        assert_eq!(map.get(&seq("ACT")), Some(&0));
        assert_eq!(map.get(&seq("AGT")), Some(&1));
        assert_eq!(map.get(&seq("TCA")), Some(&2));

        Ok(())
    }

    #[test]
    fn test_generate_parent_hashmap_duplicate_error() {
        let parents = seqs(&["ACT", "AGT", "ACT"]);
        let result = generate_parent_hashmap(&parents);

        assert!(result.is_err());
        match result {
            Err(Error::DuplicateParent(seq)) => assert_eq!(seq, "ACT"),
            _ => panic!("Expected DuplicateParent error"),
        }
    }

    #[test]
    fn test_generate_parent_hashmap_empty() -> Result<(), Error> {
        let parents: Vec<ByteSeq> = vec![];
        let map = generate_parent_hashmap(&parents)?;

        assert_eq!(map.len(), 0);
        Ok(())
    }

    #[test]
    fn test_generate_mismatch_hashmap_basic() -> Result<(), Error> {
        let parents = seqs(&["ACT", "AGT"]);
        let map = generate_mismatch_hashmap(&parents)?;

        // Should contain both parents plus their unambiguous mismatches
        assert!(map.contains_key(&seq("ACT")));
        assert!(map.contains_key(&seq("AGT")));

        // Some specific mismatches should be present
        assert!(map.contains_key(&seq("CCT"))); // ACT mismatch
        assert!(map.contains_key(&seq("CGT"))); // AGT mismatch

        Ok(())
    }

    #[test]
    fn test_generate_mismatch_hashmap_ambiguous_removal() -> Result<(), Error> {
        let parents = seqs(&["ACT", "CCT"]); // CCT is a mismatch of ACT
        let map = generate_mismatch_hashmap(&parents)?;

        // Both parents should be in the map
        assert_eq!(map.get(&seq("ACT")), Some(&0));
        assert_eq!(map.get(&seq("CCT")), Some(&1));

        // But CCT should not appear as a mismatch of ACT since it's a parent
        // The map should handle this correctly by keeping CCT as parent index 1

        Ok(())
    }

    #[test]
    fn test_generate_mismatch_hashmap_overlapping_mismatches() -> Result<(), Error> {
        let parents = seqs(&["ACT", "AGT"]); // These differ by one base
        let map = generate_mismatch_hashmap(&parents)?;

        // ATT is a mismatch of both ACT and AGT, so it should be removed (ambiguous)
        assert!(!map.contains_key(&seq("ATT")));

        Ok(())
    }

    #[test]
    fn test_disambiseq_new() -> Result<(), Error> {
        let sequences = seqs(&["ACT", "AGT", "TCA"]);
        let dsb = Disambiseq::new(&sequences)?;

        assert_eq!(dsb.parents.len(), 3);
        assert!(dsb.map.len() > 3);

        Ok(())
    }

    #[test]
    fn test_disambiseq_new_exact() -> Result<(), Error> {
        let sequences = seqs(&["ACT", "AGT", "TCA"]);
        let dsb = Disambiseq::new_exact(&sequences)?;

        assert_eq!(dsb.parents.len(), 3);
        assert_eq!(dsb.map.len(), 3);

        Ok(())
    }

    #[test]
    fn test_disambiseq_get_index() -> Result<(), Error> {
        let sequences = seqs(&["ACT", "AGT"]);
        let dsb = Disambiseq::new(&sequences)?;

        // Should find exact matches
        assert_eq!(dsb.get_index(&seq("ACT")), Some(0));
        assert_eq!(dsb.get_index(&seq("AGT")), Some(1));

        // Should find some mismatches
        if let Some(idx) = dsb.get_index(&seq("CCT")) {
            assert_eq!(idx, 0); // Should map to ACT
        }

        // Should not find sequences not in the map
        assert_eq!(dsb.get_index(&seq("AAAA")), None);

        Ok(())
    }

    #[test]
    fn test_disambiseq_get_parent() -> Result<(), Error> {
        let sequences = seqs(&["ACT", "AGT", "TCA"]);
        let dsb = Disambiseq::new(&sequences)?;

        assert_eq!(dsb.index_parent(0), Some(&seq("ACT")));
        assert_eq!(dsb.index_parent(1), Some(&seq("AGT")));
        assert_eq!(dsb.index_parent(2), Some(&seq("TCA")));
        assert_eq!(dsb.index_parent(3), None);

        Ok(())
    }

    #[test]
    fn test_round_trip_lookup() -> Result<(), Error> {
        let sequences = seqs(&["ACTG", "AGTC", "TCGA"]);
        let dsb = Disambiseq::new(&sequences)?;

        // Test that we can find a parent and get it back
        if let Some(idx) = dsb.get_index(&seq("ACTG")) {
            if let Some(parent) = dsb.index_parent(idx) {
                assert_eq!(parent, &seq("ACTG"));
            }
        }

        Ok(())
    }

    #[test]
    fn test_large_alphabet_coverage() -> Result<(), Error> {
        let sequences = seqs(&["AAAA"]);
        let dsb = Disambiseq::new(&sequences)?;

        // Should be able to find mismatches using all alphabet characters
        assert!(dsb.get_index(&seq("CAAA")).is_some());
        assert!(dsb.get_index(&seq("GAAA")).is_some());
        assert!(dsb.get_index(&seq("TAAA")).is_some());
        assert!(dsb.get_index(&seq("ACAA")).is_some());
        assert!(dsb.get_index(&seq("AGAA")).is_some());
        assert!(dsb.get_index(&seq("ATAA")).is_some());

        Ok(())
    }

    #[test]
    fn test_edge_case_single_character_sequences() -> Result<(), Error> {
        let sequences = seqs(&["A", "C", "G", "T"]);
        let dsb = Disambiseq::new(&sequences)?;

        // All single characters should be found as parents
        assert_eq!(dsb.get_index(&seq("A")), Some(0));
        assert_eq!(dsb.get_index(&seq("C")), Some(1));
        assert_eq!(dsb.get_index(&seq("G")), Some(2));
        assert_eq!(dsb.get_index(&seq("T")), Some(3));

        // No mismatches should be possible since all alternatives are parents
        assert_eq!(dsb.map.len(), 4);

        Ok(())
    }

    #[test]
    fn test_mismatch_boundary_conditions() -> Result<(), Error> {
        let sequences = seqs(&["AT"]);
        let dsb = Disambiseq::new(&sequences)?;

        // Test all possible single mismatches
        let expected_mismatches = ["CT", "GT", "TT", "AA", "AC", "AG"];
        for mm in expected_mismatches.iter() {
            assert!(
                dsb.get_index(&seq(mm)).is_some(),
                "Mismatch {} not found",
                mm
            );
        }
        assert_eq!(dsb.map.len(), expected_mismatches.len() + 1);

        Ok(())
    }

    #[test]
    fn test_empty_input() -> Result<(), Error> {
        let sequences: Vec<ByteSeq> = vec![];
        let dsb = Disambiseq::new(&sequences)?;

        assert_eq!(dsb.parents.len(), 0);
        assert_eq!(dsb.map.len(), 0);
        assert_eq!(dsb.get_index(&seq("ACT")), None);
        assert_eq!(dsb.index_parent(0), None);

        Ok(())
    }

    #[test]
    fn test_identical_sequences_error() {
        let sequences = seqs(&["ACTG", "AGTC", "ACTG"]);
        let result = Disambiseq::new(&sequences);

        assert!(result.is_err());
        match result {
            Err(Error::DuplicateParent(seq)) => assert_eq!(seq, "ACTG"),
            _ => panic!("Expected DuplicateParent error"),
        }
    }

    #[test]
    fn test_case_sensitivity() -> Result<(), Error> {
        let sequences = vec![b"act".to_vec(), b"ACT".to_vec()];
        let dsb = Disambiseq::new(&sequences);
        assert!(dsb.is_err());
        Ok(())
    }

    #[test]
    fn test_long_sequences() -> Result<(), Error> {
        let sequences = seqs(&["ACTGACTGACTGACTG", "AGTCAGTCAGTCAGTC", "TCGATCGATCGATCGA"]);
        let dsb = Disambiseq::new(&sequences)?;

        // Should handle long sequences
        assert_eq!(dsb.parents.len(), 3);

        // Should find exact matches
        assert!(dsb.get_index(&seq("ACTGACTGACTGACTG")).is_some());

        Ok(())
    }
}
