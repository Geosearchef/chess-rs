use std::fmt::{Display, Formatter};
use hashbrown::HashMap;

#[derive(Default, Clone)]
pub struct TranspositionTable {
    hash_map: HashMap<u64, f64>,
    insert_count: u64,
    lookup_count: u64,
    hit_count: u64,
}

impl TranspositionTable {
    // TODO: add debug counting
    #[inline]
    pub fn insert(&mut self, hash: u64, depth: u8, score: f64) { // depth we still want to search until evaluation
        self.hash_map.insert(hash + depth as u64, score);

        #[cfg(debug_assertions)] {
            self.insert_count += 1;
        }
    }

    #[inline]
    pub fn lookup(&mut self, hash: u64, depth: u8) -> Option<&f64> { // depth we still want to search until evaluation
        let res = self.hash_map.get(&(hash + depth as u64));
        #[cfg(debug_assertions)] {
            self.lookup_count += 1;
            if res.is_some() {
                self.hit_count += 1;
            }
        }

        res // TODO: test behaviour against None
    }
}

impl Display for TranspositionTable {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "TranspositionTable - size: {}, inserts: {}, lookups: {}, hit ratio: {:.1} %",
            self.hash_map.len(),
            self.insert_count,
            self.lookup_count,
            self.hit_count as f64 / self.lookup_count as f64 * 100.0
        )
    }
}