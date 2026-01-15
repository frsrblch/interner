#[cfg(test)]
mod tests;

use std::collections::{HashMap, hash_map::Entry};
use std::hash::{Hash, Hasher};

pub trait Intern<T> {
    type Output;
    fn intern(&mut self, value: T) -> Self::Output;
}

#[derive(Default, Clone)]
pub struct StrInterner {
    buffer: String,
    map: HashMap<HashKey, StrRange>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct StrRange {
    start: u32,
    end: u32,
}

impl Intern<&str> for StrInterner {
    type Output = StrRange;

    fn intern(&mut self, value: &str) -> Self::Output {
        let hash = HashKey::new_str(value);
        match self.map.entry(hash) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let start = self.buffer.len();
                self.buffer.push_str(value);
                let end = self.buffer.len();
                let range = StrRange::new(start, end);
                entry.insert(range);
                range
            }
        }
    }
}

impl Intern<String> for StrInterner {
    type Output = StrRange;

    fn intern(&mut self, value: String) -> Self::Output {
        let hash = HashKey::new_str(&value);
        match self.map.entry(hash) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let start = self.buffer.len();
                self.buffer.push_str(&value);
                let end = self.buffer.len();
                let range = StrRange::new(start, end);
                entry.insert(range);
                range
            }
        }
    }
}

impl StrRange {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start: start as u32,
            end: end as u32,
        }
    }

    fn range(&self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }

    pub fn len(&self) -> usize {
        self.range().len()
    }

    pub fn is_empty(&self) -> bool {
        self.range().is_empty()
    }
}

impl std::ops::Index<StrRange> for StrInterner {
    type Output = str;

    fn index(&self, index: StrRange) -> &Self::Output {
        &self.buffer[index.range()]
    }
}

impl std::ops::Index<&StrRange> for StrInterner {
    type Output = str;

    fn index(&self, index: &StrRange) -> &Self::Output {
        &self.buffer[index.range()]
    }
}

#[derive(Default, Clone)]
pub struct Interner<T> {
    buffer: Vec<T>,
    map: HashMap<HashKey, SliceRange<T>>,
}

impl<T: Hash + Clone> Intern<&[T]> for Interner<T> {
    type Output = SliceRange<T>;

    fn intern(&mut self, values: &[T]) -> Self::Output {
        let hash = HashKey::new_slice(values);
        match self.map.entry(hash) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let start = self.buffer.len();
                self.buffer.extend(values.iter().cloned());
                let end = self.buffer.len();
                let range = SliceRange::new(start, end);
                entry.insert(range);
                range
            }
        }
    }
}

impl<T: Hash> Intern<Vec<T>> for Interner<T> {
    type Output = SliceRange<T>;

    fn intern(&mut self, values: Vec<T>) -> Self::Output {
        let hash = HashKey::new_slice(&values);
        match self.map.entry(hash) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let start = self.buffer.len();
                self.buffer.extend(values);
                let end = self.buffer.len();
                let range = SliceRange::new(start, end);
                entry.insert(range);
                range
            }
        }
    }
}

impl<T: Hash, const N: usize> Intern<[T; N]> for Interner<T> {
    type Output = SliceRange<T>;

    fn intern(&mut self, values: [T; N]) -> Self::Output {
        let hash = HashKey::new_slice(&values);
        match self.map.entry(hash) {
            Entry::Occupied(entry) => *entry.get(),
            Entry::Vacant(entry) => {
                let start = self.buffer.len();
                self.buffer.extend(values);
                let end = self.buffer.len();
                let range = SliceRange::new(start, end);
                entry.insert(range);
                range
            }
        }
    }
}

impl<T> std::ops::Index<SliceRange<T>> for Interner<T> {
    type Output = [T];

    fn index(&self, index: SliceRange<T>) -> &Self::Output {
        &self.buffer[index.range()]
    }
}

impl<T> std::ops::Index<&SliceRange<T>> for Interner<T> {
    type Output = [T];

    fn index(&self, index: &SliceRange<T>) -> &Self::Output {
        &self.buffer[index.range()]
    }
}

#[derive(Debug)]
pub struct SliceRange<T> {
    start: u32,
    end: u32,
    marker: std::marker::PhantomData<fn() -> T>,
}

impl<T> SliceRange<T> {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start: start as u32,
            end: end as u32,
            marker: std::marker::PhantomData,
        }
    }

    fn range(&self) -> std::ops::Range<usize> {
        self.start as usize..self.end as usize
    }

    pub fn len(&self) -> usize {
        self.range().len()
    }

    pub fn is_empty(&self) -> bool {
        self.range().is_empty()
    }
}

impl<T> Copy for SliceRange<T> {}

impl<T> Clone for SliceRange<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> PartialEq for SliceRange<T> {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl<T> Eq for SliceRange<T> {}

impl<T> PartialOrd for SliceRange<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for SliceRange<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.start.cmp(&other.start).then(self.end.cmp(&other.end))
    }
}

impl<T> Hash for SliceRange<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.start.hash(state);
        self.end.hash(state);
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct HashKey(u64);

impl HashKey {
    fn new_str(str: &str) -> Self {
        let key = ahash::RandomState::with_seeds(
            5016128656285951095,
            15991804453339263156,
            869180266196383410,
            16177865525426686551,
        )
        .hash_one(str);
        HashKey(key)
    }

    fn new_slice<T: Hash>(values: &[T]) -> Self {
        let key = ahash::RandomState::with_seeds(
            458768224117184340,
            13440494329435370347,
            12752177437526035150,
            16620102976742681879,
        )
        .hash_one(values);
        HashKey(key)
    }
}
