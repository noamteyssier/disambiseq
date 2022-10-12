use std::{borrow::Borrow, rc::Rc};

use hashbrown::{HashMap, HashSet};
use crate::sequence::Sequence;

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct SeqWrapper(Rc<String>);
impl Borrow<str> for SeqWrapper {
    fn borrow(&self) -> &str {
        (*self.0).borrow()
    }
}
impl SeqWrapper {
    pub fn sequence(&self) -> &str {
        self.borrow()
    }
}

#[derive(Debug)]
pub struct Disambiseq {
    unambiguous: HashMap<SeqWrapper, SeqWrapper>,
    parents: HashSet<SeqWrapper>,
    ambiguous: HashSet<SeqWrapper>,
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
        let parent = SeqWrapper(Rc::new(parent.to_string()));
        self.parents.insert(parent.clone());

        if self.unambiguous.contains_key(&parent) {
            self.unambiguous.remove(&parent);
        }

        // iterate through all sequence mutations
        for mutation in Sequence::new(parent.borrow()).mutate_all() {

            let mutation = SeqWrapper(Rc::new(mutation));
            
            // skip ambigiuous or parental sequences
            if self.ambiguous.contains(&mutation) | self.parents.contains(&mutation) {
                continue
            }
            
            // if the sequence has seen before it becomes ambiguous
            if self.unambiguous.contains_key(&mutation) {
                self.ambiguous.insert(mutation.clone());
                self.unambiguous.remove(&mutation);
            
            // purely unambiguous sequence found
            } else {
                self.unambiguous.insert(mutation.clone(), parent.clone());
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
    pub fn get_parent(&self, seq: &str) -> Option<&SeqWrapper> {
        if let Some(p) = self.parents.get(seq) {
            Some(p)
        } else {
            self.unambiguous.get(seq)
        }
    }
    pub fn parents(&self) -> &HashSet<SeqWrapper> {
        &self.parents
    }
    pub fn ambiguous(&self) -> &HashSet<SeqWrapper> {
        &self.ambiguous
    }
    pub fn mutations(&self) -> &HashMap<SeqWrapper, SeqWrapper> {
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
        assert_eq!(das.get_parent("ACT").unwrap().sequence(), "ACT");
    }

    #[test]
    fn mutation_get() {
        let sequences = vec![
            "ACT".to_string(),
            "AGT".to_string()
        ];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.get_parent("TCT").unwrap().sequence(), "ACT");
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
