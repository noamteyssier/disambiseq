const LEX: [char; 4] = ['A', 'C', 'G', 'T'];

pub struct Sequence<'a> {
    seq: &'a str,
}
impl<'a> Sequence<'a> {
    /// Instantiates a new sequence
    pub fn new(seq: &'a str) -> Self {
        Self { seq }
    }

    /// Returns the internal sequence length
    pub fn len(&self) -> usize {
        self.seq.len()
    }

    /// Creates the sequence from raw parts
    fn build_mutation(&self, prefix: &str, suffix: &str, insertion: &char) -> String {
        let mut sequence = String::with_capacity(self.len());
        sequence.push_str(prefix);
        sequence.push(*insertion);
        sequence.push_str(suffix);
        sequence
    }

    /// Generates all mutations of the sequence at the specified position
    pub fn mutate_position(&self, pos: usize) -> Vec<String> {
        let (prefix, poschar) = self.seq.split_at(pos);
        let (_, suffix) = self.seq.split_at(pos + 1);
        LEX.iter()
            .filter(|c| **c != poschar.chars().nth(0).unwrap())
            .map(|c| self.build_mutation(prefix, suffix, c))
            .collect()
    }

    /// Generates all mutations of the sequence
    pub fn mutate_all(&self) -> Vec<String> {
        (0..self.len())
            .map(|idx| self.mutate_position(idx))
            .flatten()
            .collect()
    }
}

#[cfg(test)]
mod testing {
    use super::Sequence;

    #[test]
    fn init() {
        let bases = "ACTGGACCCATG";
        let seq = Sequence::new(bases);
        assert_eq!(seq.len(), bases.len());
    }

    #[test]
    fn build_mutation() {
        let bases = "ACGT";
        let seq = Sequence::new(bases);
        let prefix = "AC";
        let insertion = 'G';
        let suffix = "T";
        assert_eq!(seq.build_mutation(prefix, suffix, &insertion), "ACGT");
    }

    #[test]
    fn mutate_position() {
        let bases = "ACGT";
        let seq = Sequence::new(bases);
        let muts = seq.mutate_position(0);
        assert_eq!(muts, vec!["CCGT", "GCGT", "TCGT"]);
    }

    #[test]
    fn mutate_all() {
        let bases = "ACGT";
        let seq = Sequence::new(bases);
        let muts = seq.mutate_all();
        assert_eq!(
            muts,
            vec![
                "CCGT", "GCGT", "TCGT", "AAGT", "AGGT", "ATGT", "ACAT", "ACCT", "ACTT", "ACGA",
                "ACGC", "ACGG"
            ]
        );
    }
}
