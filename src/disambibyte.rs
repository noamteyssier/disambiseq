use std::{borrow::Borrow, rc::Rc};
use hashbrown::{HashMap, HashSet};
use crate::{sequence::ByteSequence, utils::reverse_complement_bytes};

#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct ByteWrapper(pub Rc<Vec<u8>>);
impl Borrow<[u8]> for ByteWrapper {
    fn borrow(&self) -> &[u8] {
        (*self.0).borrow()
    }
}
impl ByteWrapper {
    pub fn sequence(&self) -> &[u8] {
        self.borrow()
    }
}

#[derive(Debug)]
pub struct Disambibyte {
    unambiguous: HashMap<ByteWrapper, ByteWrapper>,
    parents: HashSet<ByteWrapper>,
    ambiguous: HashSet<ByteWrapper>,
    null: HashSet<ByteWrapper>,
}
impl Disambibyte {
    pub fn new() -> Self {
        Self {
            unambiguous: HashMap::new(),
            parents: HashSet::new(),
            ambiguous: HashSet::new(),
            null: HashSet::new(),
        }
    }

    fn insert_alias(&mut self, child: Vec<u8>, parent: &ByteWrapper) {
        let child = ByteWrapper(Rc::new(child));

        // skip ambigiuous or parental sequences
        if self.ambiguous.contains(&child) | self.parents.contains(&child) | self.null.contains(&child) {
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
    pub fn insert(&mut self, parent: &[u8]) {
        if self.parents.contains(parent) {
            return
        }

        let parent = ByteWrapper(Rc::new(parent.to_vec()));
        self.parents.insert(parent.clone());

        if self.unambiguous.contains_key(&parent) {
            self.unambiguous.remove(&parent);
        }

        ByteSequence::new(parent.borrow())
            .mutate_all()
            .into_iter()
            .for_each(|x| {self.insert_alias(x, &parent)});
        
    }
    
    /// Inserts a parent sequence with which to create all unambiguous
    /// point mutations as well as the reverse complement of those sequences.
    pub fn insert_with_reverse_complement(&mut self, parent: &[u8]) {
        if self.parents.contains(parent) {
            return
        }
        let parent_revc = ByteWrapper(Rc::new(reverse_complement_bytes(parent)));
        let parent = ByteWrapper(Rc::new(parent.to_vec()));
        self.parents.insert(parent.clone());

        if self.unambiguous.contains_key(&parent) {
            self.unambiguous.remove(&parent);
        }

        // insert parent reverse_complement with potential overwriting
        self.unambiguous.insert(parent_revc.clone(), parent.clone());

        // blacklist reverse complement of parent
        self.null.insert(parent_revc.clone());

        ByteSequence::new(parent.borrow())
            .mutate_all()
            .into_iter()
            .for_each(|x| {
                self.insert_alias(reverse_complement_bytes(&x), &parent);
                self.insert_alias(x, &parent);
            });
    }

    pub fn from_slice(sequences: &[Vec<u8>]) -> Self {
        let mut dsb = Self::new();
        sequences.iter().for_each(|x| dsb.insert(x));
        dsb
    }
    pub fn get_parent(&self, seq: &[u8]) -> Option<&ByteWrapper> {
        if let Some(p) = self.parents.get(seq) {
            Some(p)
        } else {
            self.unambiguous.get(seq)
        }
    }
    pub fn parents(&self) -> &HashSet<ByteWrapper> {
        &self.parents
    }
    pub fn ambiguous(&self) -> &HashSet<ByteWrapper> {
        &self.ambiguous
    }
    pub fn unambiguous(&self) -> &HashMap<ByteWrapper, ByteWrapper> {
        &self.unambiguous
    }
}

#[cfg(test)]
mod testing {
    use super::Disambibyte;

    #[test]
    fn init_slice() {
        let sequences = vec![b"ACT".to_vec(), b"AGT".to_vec()];
        let dsb = Disambibyte::from_slice(&sequences);
        assert_eq!(dsb.parents().len(), 2);
        assert_eq!(dsb.ambiguous().len(), 2);
        assert_eq!(dsb.unambiguous().len(), 12);
    }

    #[test]
    fn parental_get() {
        let sequences = vec![b"ACT".to_vec(), b"AGT".to_vec()];
        let dsb = Disambibyte::from_slice(&sequences);
        assert_eq!(dsb.get_parent(b"ACT").unwrap().sequence(), b"ACT");
    }

    #[test]
    fn mutation_get() {
        let sequences = vec![b"ACT".to_vec(), b"AGT".to_vec()];
        let dsb = Disambibyte::from_slice(&sequences);
        assert_eq!(dsb.get_parent(b"TCT").unwrap().sequence(), b"ACT");
    }

    #[test]
    fn ambiguous_get() {
        let sequences = vec![b"ACT".to_vec(), b"AGT".to_vec()];
        let dsb = Disambibyte::from_slice(&sequences);
        assert_eq!(dsb.get_parent(b"ATT"), None);
    }

    #[test]
    fn init() {
        let sequences = vec![b"ACT", b"AGT"];
        let mut dsb = Disambibyte::new();
        dsb.insert(sequences[0]);
        dsb.insert(sequences[1]);
        assert_eq!(dsb.parents().len(), 2);
        assert_eq!(dsb.ambiguous().len(), 2);
        assert_eq!(dsb.unambiguous().len(), 12);
    }

    #[test]
    fn init_rc() {
        let sequences = vec![b"ACTAA", b"AGTAA"];
        let mut dsb = Disambibyte::new();
        dsb.insert_with_reverse_complement(sequences[0]);
        dsb.insert_with_reverse_complement(sequences[1]);
        assert_eq!(dsb.parents().len(), 2);
        assert_eq!(dsb.ambiguous().len(), 4);
        assert_eq!(dsb.unambiguous().len(), 50);
        assert_eq!(dsb.get_parent(b"TTAGT").unwrap().sequence(), b"ACTAA");
        assert_eq!(dsb.get_parent(b"ACTAA").unwrap().sequence(), b"ACTAA");
        assert_eq!(dsb.get_parent(b"TTACT").unwrap().sequence(), b"AGTAA");
        assert_eq!(dsb.get_parent(b"AGTAA").unwrap().sequence(), b"AGTAA");
        assert_eq!(dsb.get_parent(b"ATAGT").unwrap().sequence(), b"ACTAA");
    }
}
