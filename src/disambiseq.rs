use std::{borrow::Borrow, sync::Arc};

use crate::{sequence::Sequence, utils::reverse_complement};
use hashbrown::{HashMap, HashSet};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct SeqWrapper(pub Arc<String>);
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

#[derive(Debug, Clone, Default)]
pub struct Disambiseq {
    unambiguous: HashMap<SeqWrapper, SeqWrapper>,
    parents: HashSet<SeqWrapper>,
    ambiguous: HashSet<SeqWrapper>,
    null: HashSet<SeqWrapper>,
}
impl Disambiseq {
    pub fn new() -> Self {
        Self::default()
    }

    fn insert_alias(&mut self, child: String, parent: &SeqWrapper) {
        let child = SeqWrapper(Arc::new(child));

        // skip ambigiuous or parental sequences
        if self.ambiguous.contains(&child)
            | self.parents.contains(&child)
            | self.null.contains(&child)
        {
            return;
        }

        // if the sequence has seen before it becomes ambiguous
        if self.unambiguous.contains_key(&child) {
            self.ambiguous.insert(child.clone());
            self.unambiguous.remove(&child);

        // purely unambiguous sequence found
        } else {
            self.unambiguous.insert(child.clone(), parent.clone());
        }
    }

    /// Inserts a parent sequence with which to create all unambiguous
    /// point mutations.
    pub fn insert(&mut self, parent: &str) {
        if self.parents.contains(parent) {
            return;
        }

        let parent = SeqWrapper(Arc::new(parent.to_string()));
        self.parents.insert(parent.clone());

        if self.unambiguous.contains_key(&parent) {
            self.unambiguous.remove(&parent);
        }

        Sequence::new(parent.borrow())
            .mutate_all()
            .into_iter()
            .for_each(|x| self.insert_alias(x, &parent));
    }

    /// Inserts a parent sequence with which to create all unambiguous
    /// point mutations as well as the reverse complement of those sequences.
    pub fn insert_with_reverse_complement(&mut self, parent: &str) {
        if self.parents.contains(parent) {
            return;
        }
        let parent_revc = SeqWrapper(Arc::new(reverse_complement(parent)));
        let parent = SeqWrapper(Arc::new(parent.to_string()));
        self.parents.insert(parent.clone());

        if self.unambiguous.contains_key(&parent) {
            self.unambiguous.remove(&parent);
        }

        // insert parent reverse_complement with potential overwriting
        self.unambiguous.insert(parent_revc.clone(), parent.clone());

        // blacklist reverse complement of parent
        self.null.insert(parent_revc.clone());

        Sequence::new(parent.borrow())
            .mutate_all()
            .into_iter()
            .for_each(|x| {
                self.insert_alias(reverse_complement(&x), &parent);
                self.insert_alias(x, &parent);
            });
    }

    pub fn from_slice(sequences: &[String]) -> Self {
        let mut dsq = Self::new();
        sequences.iter().for_each(|x| dsq.insert(x));
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
    pub fn unambiguous(&self) -> &HashMap<SeqWrapper, SeqWrapper> {
        &self.unambiguous
    }
}

#[cfg(test)]
mod testing {
    use super::Disambiseq;

    #[test]
    fn init_slice() {
        let sequences = vec!["ACT".to_string(), "AGT".to_string()];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.parents().len(), 2);
        assert_eq!(das.ambiguous().len(), 2);
        assert_eq!(das.unambiguous().len(), 12);
    }

    #[test]
    fn parental_get() {
        let sequences = vec!["ACT".to_string(), "AGT".to_string()];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.get_parent("ACT").unwrap().sequence(), "ACT");
    }

    #[test]
    fn mutation_get() {
        let sequences = vec!["ACT".to_string(), "AGT".to_string()];
        let das = Disambiseq::from_slice(&sequences);
        assert_eq!(das.get_parent("TCT").unwrap().sequence(), "ACT");
    }

    #[test]
    fn ambiguous_get() {
        let sequences = vec!["ACT".to_string(), "AGT".to_string()];
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
        assert_eq!(dsq.unambiguous().len(), 12);
    }

    #[test]
    fn init_rc() {
        let sequences = vec!["ACTAA", "AGTAA"];
        let mut dsq = Disambiseq::new();
        dsq.insert_with_reverse_complement(sequences[0]);
        dsq.insert_with_reverse_complement(sequences[1]);
        assert_eq!(dsq.parents().len(), 2);
        assert_eq!(dsq.ambiguous().len(), 4);
        assert_eq!(dsq.unambiguous().len(), 50);
        assert_eq!(dsq.get_parent("TTAGT").unwrap().sequence(), "ACTAA");
        assert_eq!(dsq.get_parent("ACTAA").unwrap().sequence(), "ACTAA");
        assert_eq!(dsq.get_parent("TTACT").unwrap().sequence(), "AGTAA");
        assert_eq!(dsq.get_parent("AGTAA").unwrap().sequence(), "AGTAA");
        assert_eq!(dsq.get_parent("ATAGT").unwrap().sequence(), "ACTAA");
    }
}
