use hashbrown::{HashMap, HashSet};
use crate::sequence::Sequence;

#[derive(Debug)]
pub struct Disambiseq {
    unambiguous: HashMap<String, String>,
    parents: HashSet<String>,
    ambiguous: HashSet<String>,
}
impl Disambiseq {
    pub fn new() -> Self {
        Self {
            unambiguous: HashMap::new(),
            parents: HashSet::new(),
            ambiguous: HashSet::new(),
        }
    }

    pub fn insert(&mut self, parent: &str) {
        self.parents.insert(parent.to_string());

        if self.unambiguous.contains_key(parent) {
            self.unambiguous.remove(parent);
        }

        // iterate through all sequence mutations
        for mutation in Sequence::new(parent).mutate_all() {
            
            // skip ambigiuous or parental sequences
            if self.ambiguous.contains(&mutation) | self.parents.contains(&mutation) {
                continue
            }
            
            // if the sequence has seen before it becomes ambiguous
            if self.unambiguous.contains_key(&mutation) {
                self.ambiguous.insert(mutation.to_string());
                self.unambiguous.remove(&mutation);
            
            // purely unambiguous sequence found
            } else {
                self.unambiguous.insert(mutation.to_string(), parent.to_string());
            }
        }

    }

    pub fn from_slice(sequences: &[String]) -> Self {
        let mut dsq = Self::new();
        sequences
            .iter()
            .for_each(|x| dsq.insert(x));
        dsq
    }
    pub fn get_parent(&self, seq: &str) -> Option<&String> {
        if let Some(p) = self.parents.get(seq) {
            Some(p)
        } else {
            self.unambiguous.get(seq)
        }
    }
    pub fn parents(&self) -> &HashSet<String> {
        &self.parents
    }
    pub fn ambiguous(&self) -> &HashSet<String> {
        &self.ambiguous
    }
    pub fn mutations(&self) -> &HashMap<String, String> {
        &self.unambiguous
    }
}

#[cfg(test)]
mod testing {
    use super::Disambiseq;

    #[test]
    fn init_slice() {
        let sequences = vec![
            "ACT".to_string(),
            "AGT".to_string()
        ];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.parents().len(), 2);
        assert_eq!(das.ambiguous().len(), 2);
        assert_eq!(das.mutations().len(), 12);
    }

    #[test]
    fn parental_get() {
        let sequences = vec![
            "ACT".to_string(),
            "AGT".to_string()
        ];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.get_parent("ACT"), Some(&"ACT".to_string()));
    }

    #[test]
    fn mutation_get() {
        let sequences = vec![
            "ACT".to_string(),
            "AGT".to_string()
        ];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.get_parent("TCT"), Some(&"ACT".to_string()));
    }

    #[test]
    fn ambiguous_get() {
        let sequences = vec![
            "ACT".to_string(),
            "AGT".to_string()
        ];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.get_parent("ATT"), None);
    }

    #[test]
    fn init() {
        let sequences = vec!["ACT", "AGT"];
        let mut dsq = Disambiseq::new();
        dsq.insert(sequences[0]);
        dsq.insert(sequences[1]);
        assert_eq!(dsq.parents().len(), 2);
        assert_eq!(dsq.ambiguous().len(), 2);
        assert_eq!(dsq.mutations().len(), 12);
    }
}
