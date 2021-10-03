#![allow(dead_code)]
use std::collections::{BTreeSet, HashMap};

type M = u128;

// Mex when number of queries is unknown
struct Mex {
    map: HashMap<M, usize>,
    complement: BTreeSet<M>, // because HashSet is unsorted
    processed: usize,
}

impl Mex {
    fn new() -> Self {
        let mut complement = BTreeSet::new();
        complement.insert(0);
        Self {
            map: HashMap::new(),
            complement,
            processed: 0,
        }
    }
    fn add(&mut self, element: M) {
        self.map.entry(element).and_modify(|e| *e += 1).or_insert(1);
        self.complement.remove(&element);

        self.processed += 1;
        if self.map.get(&(self.processed as M)).is_none() {
            self.complement.insert(self.processed as M);
        }
    }
    fn remove(&mut self, element: M) {
        if let Some(val) = self.map.get_mut(&element) {
            *val -= 1;
            if *val == 0 {
                self.map.remove_entry(&element);
                self.complement.insert(element);
            }
        }

        self.processed += 1;
        if self.map.get(&(self.processed as M)).is_none() {
            self.complement.insert(self.processed as M);
        }
    }
    fn mex(&self) -> M {
        *self.complement.iter().next().unwrap()
    }
}

// Mex when number of queries is known
struct MexN {
    map: HashMap<M, usize>,
    complement: BTreeSet<M>,
}

impl MexN {
    fn new(n_queries: usize) -> Self {
        let mut complement: BTreeSet<M> = BTreeSet::new();
        for i in 0..n_queries as M {
            complement.insert(i);
        }
        Self {
            map: HashMap::new(),
            complement,
        }
    }
    fn add(&mut self, element: M) {
        self.map.entry(element).and_modify(|e| *e += 1).or_insert(1);
        self.complement.remove(&element);
    }
    fn remove(&mut self, element: M) {
        if let Some(val) = self.map.get_mut(&element) {
            *val -= 1;
            if *val == 0 {
                self.map.remove_entry(&element);
                self.complement.insert(element);
            }
        }
    }
    fn mex(&self) -> M {
        *self.complement.iter().next().unwrap()
    }
}

// Mex if there are no delete operations
struct MexNoDel {
    vec: Vec<M>,
    mex: M,
}

impl MexNoDel {
    fn new(n: usize) -> Self {
        let vec: Vec<M> = vec![0; n + 1];
        Self { vec, mex: 0 }
    }
    fn add(&mut self, element: M) {
        self.vec[element as usize] = 1;
        if element == self.mex {
            while self.vec[self.mex as usize] == 1 {
                self.mex += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Mex;
    #[test]
    fn test_mex() {
        let mut mex = Mex::new();
        mex.add(2);
        assert_eq!(mex.mex(), 0);
        mex.add(100);
        assert_eq!(mex.mex(), 0);
        mex.add(0);
        assert_eq!(mex.mex(), 1);
        mex.add(1);
        assert_eq!(mex.mex(), 3);
        mex.add(2);
        assert_eq!(mex.mex(), 3);
        mex.remove(2);
        assert_eq!(mex.mex(), 3);
        mex.remove(2);
        assert_eq!(mex.mex(), 2);
    }

    use crate::MexN;
    #[test]
    fn test_mex_n() {
        let mut mex = MexN::new(7);
        mex.add(2);
        assert_eq!(mex.mex(), 0);
        mex.add(100);
        assert_eq!(mex.mex(), 0);
        mex.add(0);
        assert_eq!(mex.mex(), 1);
        mex.add(1);
        assert_eq!(mex.mex(), 3);
        mex.add(2);
        assert_eq!(mex.mex(), 3);
        mex.remove(2);
        assert_eq!(mex.mex(), 3);
        mex.remove(2);
        assert_eq!(mex.mex(), 2);
    }

    use crate::MexNoDel;
    #[test]
    fn test_mex_no_del() {
        let mut mex = MexNoDel::new(7);
        mex.add(1);
        assert_eq!(mex.mex, 0);
        mex.add(2);
        assert_eq!(mex.mex, 0);
        mex.add(0);
        assert_eq!(mex.mex, 3);
        mex.add(4);
        assert_eq!(mex.mex, 3);
        mex.add(3);
        assert_eq!(mex.mex, 5);
        mex.add(7);
        assert_eq!(mex.mex, 5);
        mex.add(5);
        assert_eq!(mex.mex, 6);
    }
}
