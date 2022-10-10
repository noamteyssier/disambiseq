use hashbrown::{HashMap, HashSet};
use crate::sequence::Sequence;

#[derive(Debug)]
pub struct Disambiseq {
    unambiguous: HashMap<String, String>,
    parents: HashSet<String>,
    ambiguous: HashSet<String>,
}
impl Disambiseq {
    pub fn from_slice(sequences: &[String]) -> Self {
        let mut unambiguous = HashMap::new();
        let mut parents = HashSet::new();
        let mut ambiguous = HashSet::new();

        for parent in sequences {
            parents.insert(parent.to_string());

            // iterate through all sequence mutations
            for mutation in Sequence::new(parent).mutate_all() {
                
                // skip ambigiuous or parental sequences
                if ambiguous.contains(&mutation) | parents.contains(&mutation) {
                    continue
                }
                
                // if the sequence has seen before it becomes ambiguous
                if unambiguous.contains_key(&mutation) {
                    ambiguous.insert(mutation.to_string());
                    unambiguous.remove(&mutation);
                
                // purely unambiguous sequence found
                } else {
                    unambiguous.insert(mutation.to_string(), parent.to_string());
                }
            }
        }

        Self {
            unambiguous, parents, ambiguous
        }
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
    fn init() {
        let sequences = vec![
            "ACT".to_string(),
            "AGT".to_string()
        ];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.parents().len(), 2);
        assert_eq!(das.ambiguous().len(), 2);
        assert_eq!(das.mutations().len(), 13);
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
}
